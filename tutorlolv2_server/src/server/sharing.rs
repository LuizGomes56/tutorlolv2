//! Capability lifecycle, derived replay chunks and latest-only live delivery.
use super::{
    auth::DeviceAuth,
    codec::{Bincode, OCTET_STREAM, Response, respond},
    db_enums::ShareDeleteReason,
    state::AppState,
};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{
        ErrorConflict, ErrorForbidden, ErrorGone, ErrorInternalServerError, ErrorNotFound,
        ErrorPayloadTooLarge, ErrorTooManyRequests, ErrorUnprocessableEntity,
    },
    web::{self, Bytes, Data, Path, Query},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use std::{
    collections::{HashMap, VecDeque},
    io::Read,
    sync::Mutex,
    time::{Duration, Instant},
};
use tutorlolv2::riot::RiotRealtime;
use tutorlolv2_protocol::{
    ChunkInfo, CodeMode, FRAMES_FORMAT, GameMode, RawSnapshot, RecordingSummary, SetShareMode,
    ShareInfo, ShareMode, ShareState, ShareStatus, code_from_random, display_code, inspect,
    normalize_code,
};
use tutorlolv2_types::BINCODE_CONFIG;
use uuid::Uuid;

pub const MAX_LIVE_BYTES: usize = 64 << 10;
const MAX_LIVE_DECOMPRESSED: usize = 512 << 10;

pub struct ViewerPolicy(pub bool);

#[derive(Default)]
pub struct MissingCodes(Mutex<HashMap<String, VecDeque<Instant>>>);

impl MissingCodes {
    fn allow(&self, key: String, record_miss: bool) -> bool {
        let now = Instant::now();
        let mut clients = self.0.lock().unwrap();

        clients.retain(|_, hits| {
            hits.back()
                .is_some_and(|last| now.duration_since(*last) < Duration::from_secs(60))
        });

        let hits = clients.entry(key).or_default();

        while hits
            .front()
            .is_some_and(|first| now.duration_since(*first) >= Duration::from_secs(60))
        {
            hits.pop_front();
        }

        if hits.len() >= 20 {
            return false;
        }

        if record_miss {
            hits.push_back(now);
        }

        true
    }
}

impl AppState {
    fn check_lookup_budget(&self, req: &HttpRequest, record_miss: bool) -> actix_web::Result<()> {
        let info = req.connection_info();
        let raw = if self.config.trusted_proxy {
            info.realip_remote_addr()
        } else {
            info.peer_addr()
        };

        let key = raw
            .and_then(super::routes::ClientKey::address)
            .map_or_else(|| "unknown".into(), |ip| ip.to_string());

        if req.app_data::<ViewerPolicy>().is_none_or(|p| p.0)
            && !self.missing_codes.allow(key, record_miss)
        {
            return Err(actix_web::error::InternalError::from_response(
                "Too many unknown code lookups",
                HttpResponse::TooManyRequests()
                    .insert_header(("Retry-After", "60"))
                    .body("Too many unknown code lookups"),
            )
            .into());
        }

        Ok(())
    }

    fn missing_code(&self, req: &HttpRequest) -> actix_web::Error {
        self.check_lookup_budget(req, true)
            .err()
            .unwrap_or_else(|| ErrorNotFound("Code not found"))
    }

    async fn capability(
        &self,
        input: &str,
        req: &HttpRequest,
    ) -> actix_web::Result<(String, Capability)> {
        self.check_lookup_budget(req, false)?;
        let code = normalize_code(input).ok_or_else(|| self.missing_code(req))?;

        match Capability::load(&self.db, &code).await {
            Ok(cap) => Ok((code, cap)),
            Err(error)
                if error.as_response_error().status_code()
                    == actix_web::http::StatusCode::NOT_FOUND =>
            {
                Err(self.missing_code(req))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn share_state(&self, id: Uuid) -> actix_web::Result<ShareState> {
        let row = sqlx::query!(
            "
            SELECT code, mode
            FROM share_codes
            WHERE recording_id = $1
                AND deleted_at IS NULL
            ",
            id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(ErrorInternalServerError)?;

        Ok(match row {
            Some(row) => self.code_state(&row.code, row.mode),
            None => ShareState {
                mode: ShareMode::Private,
                code: None,
                url: None,
            },
        })
    }

    pub(super) fn code_state(&self, code: &str, mode: CodeMode) -> ShareState {
        let code = display_code(code);

        ShareState {
            mode: mode.into(),
            url: Some(format!("{}/watch/{code}", self.config.public_base_url)),
            code: Some(code),
        }
    }
}

pub async fn owner_share(state: Data<AppState>, auth: DeviceAuth, id: Path<Uuid>) -> Response {
    let exists = sqlx::query_scalar!(
        "
        SELECT id
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        ",
        *id,
        auth.device_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    exists.ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    respond(state.share_state(*id).await?)
}

pub async fn set_share(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> Response {
    change_share(&state, auth, id.into_inner(), request, false).await
}

/// Both owner surfaces serialize lifecycle changes on the recording row.
pub(super) async fn change_share(
    state: &AppState,
    auth: DeviceAuth,
    id: Uuid,
    request: SetShareMode,
    finished_only: bool,
) -> Response {
    if finished_only && request.mode == ShareMode::Live {
        return Err(actix_web::error::ErrorBadRequest(
            "Live is unavailable in history",
        ));
    }

    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;
    let recording = sqlx::query!(
        "
        SELECT
            ended_at,
            is_custom,
            is_practice
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
            AND EXISTS (
                SELECT 1
                FROM devices
                WHERE devices.id = recordings.device_id
                    AND devices.deleted_at IS NULL
            )
        FOR UPDATE
        ",
        id,
        auth.device_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    if finished_only && recording.ended_at.is_none() {
        return Err(ErrorConflict("The recording is still active"));
    }

    if request.mode == ShareMode::Public
        && (recording.is_practice || recording.is_custom != Some(false))
    {
        return Err(ErrorForbidden("public_not_allowed"));
    }

    if request.mode == ShareMode::Live && recording.ended_at.is_some() {
        return Err(ErrorConflict("The recording has ended"));
    }

    if request.mode == ShareMode::Private {
        sqlx::query!(
            "
            UPDATE share_codes
            SET deleted_at = now(), delete_reason = $2
            WHERE recording_id = $1
                AND deleted_at IS NULL
            ",
            id,
            ShareDeleteReason::Revoked
        )
        .execute(&mut *tx)
        .await
        .map_err(ErrorInternalServerError)?;

        tx.commit().await.map_err(ErrorInternalServerError)?;
        state.live.close(id);

        return respond(state.share_state(id).await?);
    }

    let mode = if request.mode == ShareMode::Live {
        CodeMode::Live
    } else {
        CodeMode::Public
    };

    let existing = sqlx::query!(
        "
        SELECT id, code
        FROM share_codes
        WHERE recording_id = $1
            AND deleted_at IS NULL
        ",
        id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    let code = match existing {
        Some(existing) => {
            sqlx::query!(
                "UPDATE share_codes SET mode = $2, mode_changed_at = now() WHERE id = $1",
                existing.id,
                mode
            )
            .execute(&mut *tx)
            .await
            .map_err(ErrorInternalServerError)?;

            existing.code
        }
        None => {
            let mut chosen = None;

            for _ in 0..5 {
                let mut random = [0u8; 5];

                getrandom::fill(&mut random).map_err(ErrorInternalServerError)?;

                let code = code_from_random(random);
                let inserted = sqlx::query_scalar!(
                    "
                    INSERT INTO share_codes (
                        recording_id,
                        code,
                        mode
                    )
                    VALUES ($1, $2, $3)
                    ON CONFLICT (code)
                    DO NOTHING
                    RETURNING code
                    ",
                    id,
                    code,
                    mode
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(ErrorInternalServerError)?;

                if inserted.is_some() {
                    chosen = inserted;
                    break;
                }
            }

            chosen.ok_or_else(|| ErrorInternalServerError("Could not allocate a share code"))?
        }
    };

    tx.commit().await.map_err(ErrorInternalServerError)?;
    respond(state.code_state(&code, mode))
}

struct Capability {
    recording: Uuid,
    mode: CodeMode,
    ended: Option<DateTime<Utc>>,
    deleted: bool,
    summary: RecordingSummary,
}

impl Capability {
    async fn load(db: &PgPool, code: &str) -> actix_web::Result<Self> {
        let row = sqlx::query!(
            r#"
            SELECT
                s.recording_id,
                s.mode,
                s.deleted_at AS code_deleted,
                r.deleted_at AS recording_deleted,
                r.ended_at,
                r.champion,
                r.game_mode,
                r.queue_id,
                r.active_riot_id,
                r.started_at_estimate,
                r.first_game_time,
                r.last_game_time,
                r.game_length_s,
                r.result,
                r.is_custom,
                r.is_practice
            FROM share_codes s
            JOIN recordings r ON r.id = s.recording_id
            WHERE s.code = $1
            "#,
            code
        )
        .fetch_optional(db)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("Code not found"))?;

        let cap = Self {
            recording: row.recording_id,
            mode: row.mode,
            ended: row.ended_at,
            deleted: row.code_deleted.is_some() || row.recording_deleted.is_some(),
            summary: RecordingSummary {
                id: row.recording_id.into_bytes(),
                champion: row.champion,
                game_mode: GameMode::from(row.game_mode),
                queue_id: row.queue_id.and_then(|q| q.try_into().ok()),
                active_riot_id: row.active_riot_id,
                started_at_ms: row.started_at_estimate.timestamp_millis(),
                first_game_time: row.first_game_time,
                last_game_time: row.last_game_time,
                game_length_s: row.game_length_s.and_then(|s| s.try_into().ok()),
                result: row.result,
                is_custom: row.is_custom,
                is_practice: row.is_practice,
            },
        };

        cap.active()?;

        Ok(cap)
    }

    fn active(&self) -> actix_web::Result<()> {
        if self.mode == CodeMode::Public
            && (self.summary.is_practice || self.summary.is_custom != Some(false))
        {
            return Err(ErrorGone("public_not_allowed"));
        }

        if self.deleted || (self.mode == CodeMode::Live && self.ended.is_some()) {
            Err(ErrorGone("This match is no longer shared"))
        } else {
            Ok(())
        }
    }
}

pub async fn info(state: Data<AppState>, code: Path<String>, req: HttpRequest) -> Response {
    let (code, cap) = state.capability(&code, &req).await?;

    let chunks = sqlx::query!(
        "
        SELECT seq,
            first_game_time,
            last_game_time,
            snapshot_count,
            frames_format
        FROM recording_chunks
        WHERE recording_id = $1
        ORDER BY seq
        ",
        cap.recording
    )
    .fetch_all(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .into_iter()
    .map(|row| ChunkInfo {
        seq: row.seq as u32,
        first_game_time: row.first_game_time,
        last_game_time: row.last_game_time,
        snapshot_count: row.snapshot_count as u16,
        frames_format: row.frames_format as u16,
    })
    .collect();

    // Revalidate after the chunk query, which may have raced with revocation.
    let cap = Capability::load(&state.db, &code).await?;
    let status = if cap.ended.is_some() {
        ShareStatus::Ended {
            result: cap.summary.result,
        }
    } else {
        match state.live.latest(cap.recording) {
            Some(frame) if frame.age_ms() <= 10_000 => ShareStatus::Live {
                last_frame_age_ms: frame.age_ms(),
            },
            _ => ShareStatus::HostDisconnected,
        }
    };

    respond(ShareInfo {
        mode: cap.mode.into(),
        status,
        summary: cap.summary,
        chunks,
        frames_format: FRAMES_FORMAT,
    })
}

pub async fn frames(
    state: Data<AppState>,
    path: Path<(String, u32)>,
    req: HttpRequest,
) -> Response {
    let (code, seq) = path.into_inner();
    let (code, cap) = state.capability(&code, &req).await?;

    let chunk = sqlx::query!(
        "
        SELECT frames, frames_format
        FROM recording_chunks
        WHERE recording_id = $1
            AND seq = $2
        ",
        cap.recording,
        i32::try_from(seq).map_err(|_| ErrorNotFound("Chunk not found"))?
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Chunk not found"))?;

    if chunk.frames_format != FRAMES_FORMAT as i16 {
        return Err(ErrorUnprocessableEntity(
            "Frames unavailable for this chunk",
        ));
    }

    let latest = Capability::load(&state.db, &code).await?;

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Content-Encoding", "br"))
        .insert_header((
            "Cache-Control",
            if latest.mode == CodeMode::Live {
                "no-store"
            } else {
                "private, max-age=300"
            },
        ))
        .body(chunk.frames))
}

pub async fn push_live(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
    body: Bytes,
) -> Response {
    let id = id.into_inner();

    if body.len() > MAX_LIVE_BYTES {
        return Err(ErrorPayloadTooLarge("Live snapshot too large"));
    }

    // Ownership first, before decompressing any untrusted data.
    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    let recording = sqlx::query!(
        "
        SELECT ended_at, probe
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        FOR UPDATE
        ",
        id,
        auth.device_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    if recording.ended_at.is_some() {
        return Err(ErrorConflict("The recording has ended"));
    }

    let shared = sqlx::query_scalar!(
        "
        SELECT id
        FROM share_codes
        WHERE recording_id = $1
            AND deleted_at IS NULL
        ",
        id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .is_some();

    if !shared {
        return Err(ErrorConflict("The recording is private"));
    }

    let bytes = body.to_vec();
    let derived = web::block(move || SelfContainedFrame::decode(&bytes))
        .await
        .map_err(ErrorInternalServerError)?
        .map_err(|e| {
            if e.too_large {
                ErrorPayloadTooLarge(e.message)
            } else {
                ErrorUnprocessableEntity(e.message)
            }
        })?;

    let (stored, _) = bincode::decode_from_slice::<tutorlolv2_protocol::Probe, _>(
        &recording.probe,
        BINCODE_CONFIG,
    )
    .map_err(ErrorInternalServerError)?;

    if stored.consistent(&derived.probe).is_err() {
        return Err(ErrorConflict("The snapshot belongs to another match"));
    }

    sqlx::query!(
        "
        UPDATE recordings
        SET last_ingest_at = now()
        WHERE id = $1
        ",
        id
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    // Publish under the same row lock used by end/share mutations. Their
    // post-commit close cannot race ahead and leave a stale frame behind.
    state.live.publish(
        id,
        derived.captured_at,
        derived.game_time,
        derived.bytes,
        state.config.max_viewers_per_code,
    );

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

struct SelfContainedFrame {
    captured_at: i64,
    game_time: f32,
    probe: tutorlolv2_protocol::Probe,
    bytes: Vec<u8>,
}

struct LiveDecodeError {
    too_large: bool,
    message: String,
}

impl LiveDecodeError {
    fn invalid(error: impl std::fmt::Display) -> Self {
        Self {
            too_large: false,
            message: error.to_string(),
        }
    }

    fn large() -> Self {
        Self {
            too_large: true,
            message: "Live snapshot too large once decompressed".into(),
        }
    }
}

impl SelfContainedFrame {
    fn decode(body: &[u8]) -> Result<Self, LiveDecodeError> {
        let mut raw = Vec::new();

        brotli::Decompressor::new(body, 4096)
            .take(MAX_LIVE_DECOMPRESSED as u64 + 1)
            .read_to_end(&mut raw)
            .map_err(LiveDecodeError::invalid)?;

        if raw.len() > MAX_LIVE_DECOMPRESSED {
            return Err(LiveDecodeError::large());
        }

        let (snapshot, used) = bincode::decode_from_slice::<RawSnapshot, _>(
            &raw,
            BINCODE_CONFIG.with_limit::<MAX_LIVE_DECOMPRESSED>(),
        )
        .map_err(LiveDecodeError::invalid)?;

        if used != raw.len() {
            return Err(LiveDecodeError::invalid("Trailing snapshot bytes"));
        }

        let inspected =
            inspect(&snapshot.json).map_err(|e| LiveDecodeError::invalid(format!("{e:?}")))?;

        if !snapshot.game_time.is_finite()
            || (snapshot.game_time - inspected.game_time).abs() > 0.01
        {
            return Err(LiveDecodeError::invalid("Snapshot time mismatch"));
        }

        let plain = serde_json::to_vec(
            &serde_json::from_slice::<serde_json::Value>(&snapshot.json)
                .map_err(LiveDecodeError::invalid)?,
        )
        .map_err(LiveDecodeError::invalid)?;

        let frame =
            serde_json::from_slice::<RiotRealtime<'_>>(&plain).map_err(LiveDecodeError::invalid)?;

        let bytes =
            bincode::encode_to_vec(frame, BINCODE_CONFIG).map_err(LiveDecodeError::invalid)?;

        Ok(Self {
            captured_at: snapshot.captured_at_ms,
            game_time: inspected.game_time,
            probe: inspected.probe,
            bytes,
        })
    }
}

#[derive(Deserialize)]
pub struct After {
    #[serde(default)]
    after: u64,
}

pub async fn live(
    state: Data<AppState>,
    code: Path<String>,
    query: Query<After>,
    req: HttpRequest,
) -> Response {
    let (code, cap) = state.capability(&code, &req).await?;

    if cap.ended.is_some() {
        return Err(ErrorGone("The match has ended"));
    }

    let (mut receiver, _permit) = state
        .live
        .subscribe(cap.recording, state.config.max_viewers_per_code)
        .ok_or_else(|| ErrorTooManyRequests("Viewer limit reached"))?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);

    loop {
        let cap = Capability::load(&state.db, &code).await?;

        if cap.ended.is_some() {
            return Err(ErrorGone("The match has ended"));
        }

        let latest = receiver.borrow_and_update().clone();

        if let Some(frame) = latest.filter(|frame| frame.seq > query.after) {
            return Ok(HttpResponse::Ok()
                .content_type(OCTET_STREAM)
                .insert_header(("Cache-Control", "no-store"))
                .insert_header(("X-Frame-Seq", frame.seq.to_string()))
                .insert_header(("X-Captured-At", frame.captured_at.to_string()))
                .insert_header(("X-Game-Time", frame.game_time.to_string()))
                .body(frame.bytes.clone()));
        }

        tokio::select! {
            changed = receiver.changed() => if changed.is_err() {
                let cap = Capability::load(&state.db, &code).await?;

                if cap.ended.is_some() {
                    return Err(ErrorGone("The match has ended"));
                }

                return Ok(HttpResponse::NoContent().finish());
            },
            _ = tokio::time::sleep_until(deadline) => return Ok(HttpResponse::NoContent().finish()),
            _ = tokio::time::sleep(Duration::from_secs(1)) => {},
        }
    }
}
