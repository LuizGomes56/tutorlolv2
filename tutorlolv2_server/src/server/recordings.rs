//! Device registration and recording control (spec §10.2).

use super::{
    auth::{DeviceAuth, secret_hash},
    codec::{Bincode, Response, respond},
    db_enums::{RiotGameSource, ShareDeleteReason},
    ingest::{IngestError, derive_chunk, hex},
    riot_api::{EnrichJob, log_unmapped_region},
    state::AppState,
};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{
        ErrorConflict, ErrorInternalServerError, ErrorNotFound, ErrorPayloadTooLarge,
        ErrorUnprocessableEntity,
    },
    web::{self, Bytes, Data, Path},
};
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgPool};
use tutorlolv2_protocol::{
    CodeMode, DeviceCredentials, EndReason, EndRecording, GameMode, MatchDecision, Platform, Probe,
    RAW_CHUNK_FORMAT, RecordingResult, Region, RegisterDevice, SessionStarted, ShareMode,
    ShareState, StartSession, fingerprint,
};
use uuid::Uuid;

pub async fn register_device(
    state: Data<AppState>,
    Bincode(request): Bincode<RegisterDevice>,
) -> Response {
    let mut device_secret = [0u8; 32];

    getrandom::fill(&mut device_secret).map_err(ErrorInternalServerError)?;

    let device_id = sqlx::query_scalar!(
        "
        INSERT INTO devices (secret_hash, app_version)
        VALUES ($1, $2)
        RETURNING id
        ",
        &secret_hash(&device_secret),
        request.app_version,
    )
    .fetch_one(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    respond(DeviceCredentials {
        device_id: device_id.into_bytes(),
        device_secret,
    })
}

/// Start-or-resume (spec §8.3): resumes the device's recording of the same
/// match when there is one, otherwise ends the device's open recordings as
/// superseded and creates a new one.
pub async fn start_session(
    state: Data<AppState>,
    auth: DeviceAuth,
    Bincode(request): Bincode<StartSession>,
) -> Response {
    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;
    let now = Utc::now();

    let candidates = sqlx::query!(
        r#"
        SELECT id,
            probe,
            last_ingest_at,
            ended_at IS NULL AS "open!"
        FROM recordings
        WHERE device_id = $1
            AND deleted_at IS NULL
            AND (ended_at IS NULL OR (
                end_reason = $2 AND ended_at > now() - $3 * interval '1 second')
            )
        ORDER BY last_ingest_at DESC
        "#,
        auth.device_id,
        EndReason::Timeout,
        RESUME_WINDOW_SECS,
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    let same = candidates.iter().find(|c| {
        decode_probe(&c.probe).is_some_and(|stored| {
            stored.same_match(
                c.last_ingest_at.timestamp_millis(),
                &request.probe,
                now.timestamp_millis(),
            ) == MatchDecision::Same
        })
    });

    for other in candidates
        .iter()
        .filter(|c| c.open && Some(c.id) != same.map(|s| s.id))
    {
        end_recording_tx(&mut tx, other.id, EndReason::Superseded, now)
            .await
            .map_err(ErrorInternalServerError)?;
    }

    let started = match same {
        Some(found) => {
            if !found.open {
                sqlx::query!(
                    "
                    UPDATE recordings
                    SET ended_at = NULL,
                        end_reason = NULL,
                        last_ingest_at = now()
                    WHERE id = $1
                    ",
                    found.id
                )
                .execute(&mut *tx)
                .await
                .map_err(ErrorInternalServerError)?;
            }

            let last_seq = sqlx::query_scalar!(
                "
                SELECT max(seq)
                FROM recording_chunks
                WHERE recording_id = $1
                ",
                found.id
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(ErrorInternalServerError)?;

            SessionStarted {
                recording_id: found.id.into_bytes(),
                resumed: true,
                next_chunk_seq: last_seq.map_or(0, |seq| seq as u32 + 1),
                share: PRIVATE,
            }
        }
        None => {
            let id = insert_recording(&mut tx, auth.device_id, &request)
                .await
                .map_err(ErrorInternalServerError)?;

            SessionStarted {
                recording_id: id.into_bytes(),
                resumed: false,
                next_chunk_seq: 0,
                share: PRIVATE,
            }
        }
    };

    tx.commit().await.map_err(ErrorInternalServerError)?;

    if !started.resumed
        && let Some(region) = &request.region
    {
        match region.platform() {
            Some(platform) => state.enrich.push(EnrichJob {
                recording_id: Uuid::from_bytes(started.recording_id),
                riot_id: request.probe.active_riot_id.clone(),
                platform,
            }),
            None => log_unmapped_region(region),
        }
    }

    respond(started)
}

/// A recording ended by timeout this recently can still be resumed (spec §8.3).
const RESUME_WINDOW_SECS: f64 = 6.0 * 3600.0;
const PRIVATE: ShareState = ShareState {
    mode: ShareMode::Private,
    code: None,
    url: None,
};

fn decode_probe(bytes: &[u8]) -> Option<Probe> {
    bincode::decode_from_slice(bytes, tutorlolv2_types::BINCODE_CONFIG)
        .ok()
        .map(|(probe, _)| probe)
}

async fn insert_recording(
    tx: &mut PgConnection,
    device_id: Uuid,
    request: &StartSession,
) -> sqlx::Result<Uuid> {
    let probe = &request.probe;
    let probe_bytes = bincode::encode_to_vec(probe, tutorlolv2_types::BINCODE_CONFIG)
        .expect("encoding into a Vec cannot fail");

    let champion = probe
        .roster
        .iter()
        .find(|e| e.riot_id == probe.active_riot_id)
        .map_or("", |e| e.champion.as_str());

    let platform = request.region.as_ref().and_then(Region::platform);
    let lcu = request.lcu.as_ref();

    sqlx::query_scalar!(
        r#"
        INSERT INTO recordings (
            device_id,
            game_mode,
            map_number,
            fingerprint,
            probe,
            platform_id,
            riot_game_id,
            riot_game_source,
            queue_id,
            is_custom,
            is_practice,
            active_riot_id,
            champion,
            started_at_estimate,
            first_game_time,
            last_game_time,
            app_version
        )
        VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8,
            $9, $10, $11, $12,
            $13, now() - $14 * interval '1 second',
            $15, $15, $16
        )
        RETURNING id
        "#,
        device_id,
        probe.game_mode.as_str(),
        probe.map_number as i16,
        &fingerprint(probe)[..],
        probe_bytes,
        platform.as_ref().map(Platform::as_str),
        lcu.map(|l| l.game_id as i64),
        lcu.map(|_| RiotGameSource::Lcu),
        lcu.and_then(|l| l.queue_id).map(i32::from),
        lcu.and_then(|l| l.is_custom),
        probe.game_mode == GameMode::PracticeTool,
        probe.active_riot_id,
        champion,
        probe.game_time as f64,
        probe.game_time,
        request.app_version,
    )
    .fetch_one(tx)
    .await
}

/// Ends a recording if it is still open and expires its Live code in the same
/// transaction. Returns whether the recording was open.
pub async fn end_recording_tx(
    tx: &mut PgConnection,
    id: Uuid,
    reason: EndReason,
    ended_at: DateTime<Utc>,
) -> sqlx::Result<bool> {
    let ended = sqlx::query!(
        "
        UPDATE recordings
        SET ended_at = $2, end_reason = $3
        WHERE id = $1
            AND ended_at IS NULL
        ",
        id,
        ended_at,
        reason,
    )
    .execute(&mut *tx)
    .await?
    .rows_affected()
        == 1;

    sqlx::query!(
        "
        UPDATE share_codes
        SET deleted_at = now(), delete_reason = $2
        WHERE recording_id = $1
            AND mode = $3
            AND deleted_at IS NULL
        ",
        id,
        ShareDeleteReason::GameEnded,
        CodeMode::Live,
    )
    .execute(&mut *tx)
    .await?;

    Ok(ended)
}

/// Header carrying the hex SHA-256 of the uncompressed chunk (spec §6.3).
pub const CONTENT_SHA256: &str = "X-Content-Sha256";

/// Stores one sealed chunk (spec §10.3). Idempotent per `(recording, seq)`;
/// late chunks are accepted but never reopen an ended recording.
pub async fn upload_chunk(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<(Uuid, u32)>,
    req: HttpRequest,
    body: Bytes,
) -> Response {
    let (id, seq) = path.into_inner();
    let sha_header = req
        .headers()
        .get(CONTENT_SHA256)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .ok_or_else(|| ErrorUnprocessableEntity("Missing X-Content-Sha256"))?;

    let recording = sqlx::query!(
        "
        SELECT probe, active_riot_id
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        ",
        id,
        auth.device_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    if let Some(response) = existing_chunk(&state.db, id, seq, &sha_header).await? {
        return Ok(response);
    }

    let compressed = body.clone();
    let derived = web::block(move || derive_chunk(&compressed, seq, &sha_header))
        .await
        .map_err(ErrorInternalServerError)?
        .map_err(|e| match e {
            IngestError::TooLarge => ErrorPayloadTooLarge("Chunk too large once decompressed"),
            IngestError::Invalid(reason) => ErrorUnprocessableEntity(reason),
            IngestError::Mismatch(reason) => ErrorConflict(reason),
        })?;

    let stored = decode_probe(&recording.probe)
        .ok_or_else(|| ErrorInternalServerError("Stored probe is unreadable"))?;

    if derived.first.probe.active_riot_id != recording.active_riot_id
        || stored.consistent(&derived.first.probe).is_err()
    {
        return Err(ErrorConflict("The chunk belongs to another match"));
    }

    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    let inserted = sqlx::query!(
        "
        INSERT INTO recording_chunks (
            recording_id,
            seq,
            first_game_time,
            last_game_time,
            snapshot_count,
            raw_format,
            raw_sha256,
            raw_size,
            raw,
            frames_format,
            frames
        )
        VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8,
            $9, $10, $11
        )
        ON CONFLICT (recording_id, seq)
        DO NOTHING",
        id,
        seq as i32,
        derived.first_game_time,
        derived.last_game_time,
        derived.snapshot_count as i16,
        RAW_CHUNK_FORMAT as i16,
        &derived.raw_sha256,
        derived.raw_size as i32,
        body.as_ref(),
        derived.frames_format as i16,
        derived.frames_br,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .rows_affected()
        == 1;

    if !inserted {
        // Lost a race with an identical or conflicting upload of the same seq
        drop(tx);

        let sha = hex(&derived.raw_sha256);

        return existing_chunk(&state.db, id, seq, &sha)
            .await?
            .ok_or_else(|| ErrorInternalServerError("Chunk vanished"));
    }

    let last = &derived.last;
    let last_probe = bincode::encode_to_vec(&last.probe, tutorlolv2_types::BINCODE_CONFIG)
        .map_err(ErrorInternalServerError)?;

    let coverage = (derived.last_game_time - derived.first_game_time + 1.0).round() as i32;

    // SET expressions read the row as it was, so every CASE compares against the old last_game_time
    sqlx::query!(
        "
        UPDATE recordings SET
            last_ingest_at = now(),
            coverage_s = coverage_s + $2,
            first_game_time = least(first_game_time, $3),
            last_game_time = greatest(last_game_time, $4),
            probe = CASE WHEN $4 > last_game_time THEN $5 ELSE probe END,
            kills = CASE WHEN $4 > last_game_time THEN $6 ELSE kills END,
            deaths = CASE WHEN $4 > last_game_time THEN $7 ELSE deaths END,
            assists = CASE WHEN $4 > last_game_time THEN $8 ELSE assists END,
            creep_score = CASE WHEN $4 > last_game_time THEN $9 ELSE creep_score END,
            level = CASE WHEN $4 > last_game_time THEN $10 ELSE level END,
            champion = CASE WHEN $4 > last_game_time THEN $11 ELSE champion END
        WHERE id = $1
        ",
        id,
        coverage,
        derived.first_game_time,
        derived.last_game_time,
        last_probe,
        last.summary.kills as i16,
        last.summary.deaths as i16,
        last.summary.assists as i16,
        last.summary.creep_score as i16,
        last.summary.level as i16,
        last.summary.champion,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())
}

/// `200` when `seq` already holds this exact chunk, `409` when it holds another one.
async fn existing_chunk(
    db: &PgPool,
    id: Uuid,
    seq: u32,
    sha_hex: &str,
) -> actix_web::Result<Option<HttpResponse>> {
    let stored = sqlx::query_scalar!(
        "
        SELECT raw_sha256
        FROM recording_chunks
        WHERE recording_id = $1
            AND seq = $2
        ",
        id,
        seq as i32,
    )
    .fetch_optional(db)
    .await
    .map_err(ErrorInternalServerError)?;

    match stored {
        None => Ok(None),
        Some(stored) if hex(&stored).eq_ignore_ascii_case(sha_hex) => {
            Ok(Some(HttpResponse::Ok().finish()))
        }
        Some(_) => Err(ErrorConflict("Another chunk is stored under this seq")),
    }
}

/// The device's recording, or `404`: ownership is never revealed to other devices.
async fn owned(db: &PgPool, id: Uuid, device_id: Uuid) -> actix_web::Result<Owned> {
    sqlx::query_as!(
        Owned,
        r#"
        SELECT end_reason
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        "#,
        id,
        device_id,
    )
    .fetch_optional(db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))
}

struct Owned {
    end_reason: Option<EndReason>,
}

/// Keeps a recording alive while no chunk is sent (spec §7.3). Reopens a
/// recording the janitor timed out when the host comes back (spec §8.5).
pub async fn heartbeat(state: Data<AppState>, auth: DeviceAuth, path: Path<Uuid>) -> Response {
    let id = path.into_inner();

    // One statement, so an end committing meanwhile is re-checked by Postgres
    // instead of being overwritten
    let alive = sqlx::query!(
        "
        UPDATE recordings
        SET last_ingest_at = now(),
            ended_at = NULL,
            end_reason = NULL
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
            AND (ended_at IS NULL
                OR (end_reason = $3
                    AND NOT EXISTS (
                        SELECT 1 FROM recordings other
                        WHERE other.device_id = $2
                            AND other.id <> $1
                            AND other.ended_at IS NULL
                            AND other.deleted_at IS NULL
                    )
                )
            )",
        id,
        auth.device_id,
        EndReason::Timeout,
    )
    .execute(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .rows_affected()
        == 1;

    if alive {
        return Ok(HttpResponse::NoContent().finish());
    }

    // Explain why: unknown recording, a real end, or another open recording
    let recording = owned(&state.db, id, auth.device_id).await?;

    if recording.end_reason == Some(EndReason::Timeout) {
        Err(ErrorConflict("Another recording of this device is open"))
    } else {
        Err(ErrorConflict("The recording has ended"))
    }
}

/// Ends a recording (spec §8.4). Idempotent: the first end wins, later calls
/// only fill in a result or game length that is still missing.
pub async fn end(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<Uuid>,
    Bincode(request): Bincode<EndRecording>,
) -> Response {
    let id = path.into_inner();

    if request.reason == EndReason::Timeout {
        return Err(ErrorUnprocessableEntity(
            "Only the server ends recordings by timeout",
        ));
    }

    owned(&state.db, id, auth.device_id).await?;

    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    end_recording_tx(&mut tx, id, request.reason, Utc::now())
        .await
        .map_err(ErrorInternalServerError)?;

    sqlx::query!(
        "
        UPDATE recordings
        SET result = coalesce(result, $2),
            game_length_s = coalesce(game_length_s, $3)
        WHERE id = $1",
        id,
        request.result,
        request.final_game_time.map(|t| t.round() as i32),
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

/// Final result from the League client's end-of-game stats (spec §8.4). The
/// client's game ids never replace Riot-verified ones.
pub async fn set_result(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<Uuid>,
    Bincode(request): Bincode<RecordingResult>,
) -> Response {
    let id = path.into_inner();

    owned(&state.db, id, auth.device_id).await?;

    let lcu = request.riot_game.as_ref();

    sqlx::query!(
        "
        UPDATE recordings SET
            result = $2,
            game_length_s = coalesce($3, game_length_s),
            riot_game_id = CASE WHEN $4::bigint IS NOT NULL
                AND riot_game_source IS DISTINCT FROM $7
                THEN $4
                ELSE riot_game_id END,
            queue_id = CASE WHEN $4::bigint IS NOT NULL
                AND riot_game_source IS DISTINCT FROM $7
                THEN coalesce($5, queue_id)
                ELSE queue_id END,
            is_custom = CASE WHEN $4::bigint IS NOT NULL
                AND riot_game_source IS DISTINCT FROM $7
                THEN coalesce($6, is_custom)
                ELSE is_custom END,
            riot_game_source = CASE WHEN $4::bigint IS NOT NULL
                AND riot_game_source IS DISTINCT FROM $7
                THEN $8
                ELSE riot_game_source END
        WHERE id = $1
        ",
        id,
        request.result,
        request.game_length_s.map(|s| s as i32),
        lcu.map(|l| l.game_id as i64),
        lcu.and_then(|l| l.queue_id).map(i32::from),
        lcu.and_then(|l| l.is_custom),
        RiotGameSource::Spectator,
        RiotGameSource::Lcu,
    )
    .execute(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
