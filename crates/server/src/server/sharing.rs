//! Capability lifecycle, derived replay chunks and latest-only live delivery.

use super::{db_enums::ShareDeleteReason, state::AppState};
use crate::server::response::{AppError, AppResult, Trace};
use actix_web::HttpRequest;
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use math::riot::RiotRealtime;
use protocol::{
    CodeMode, GameMode, RawSnapshot, RecordingSummary, SetShareMode, ShareMode, ShareState,
    code_from_random, display_code, inspect, normalize_code,
};
use sqlx::PgPool;
use std::{
    collections::{HashMap, VecDeque},
    io::Read,
    sync::Mutex,
    time::{Duration, Instant},
};
use types::BINCODE_CONFIG;
use uuid::Uuid;

pub use super::routes::{
    recordings::{live::push_live, share::owner_share, share::set_share},
    share::{frames::frames, info::info, live::live},
};
pub use ::api::After;

pub const MAX_LIVE_BYTES: usize = 64 << 10;
pub(crate) const MAX_LIVE_DECOMPRESSED: usize = 512 << 10;

pub struct ViewerPolicy(pub bool);

#[derive(Default)]
pub struct MissingCodes(Mutex<HashMap<String, VecDeque<Instant>>>);

impl MissingCodes {
    pub(crate) fn allow(&self, key: String, record_miss: bool) -> bool {
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
    pub(crate) fn check_lookup_budget(
        &self,
        req: &HttpRequest,
        record_miss: bool,
    ) -> AppResult<()> {
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
            return Err(AppError::rate_limited("Too many unknown code lookups").retry_after(60));
        }

        Ok(())
    }

    pub(crate) fn missing_code(&self, req: &HttpRequest) -> AppError {
        self.check_lookup_budget(req, true)
            .err()
            .unwrap_or_else(|| AppError::not_found("Code not found"))
    }

    pub(crate) async fn capability(
        &self,
        input: &str,
        req: &HttpRequest,
    ) -> AppResult<(String, Capability)> {
        self.check_lookup_budget(req, false).trace()?;
        let code = normalize_code(input).ok_or_else(|| self.missing_code(req))?;

        match Capability::load(&self.db, &code).await {
            Ok(cap) => Ok((code, cap)),
            Err(error) if error.status_code() == actix_web::http::StatusCode::NOT_FOUND => {
                Err(self.missing_code(req))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn share_state(&self, id: Uuid) -> AppResult<ShareState> {
        let row = sqlx::query!(
            r#"
            SELECT code, MODE
            FROM share_codes
            WHERE recording_id = $1
                AND deleted_at IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.db)
        .await
        .trace()?;

        Ok(match row {
            Some(row) => self.code_state(&row.code, row.mode),
            None => ShareState {
                mode: ShareMode::Private,
                code: None,
                url: None,
            },
        })
    }

    pub(crate) fn code_state(&self, code: &str, mode: CodeMode) -> ShareState {
        let code = display_code(code);

        ShareState {
            mode: mode.into(),
            url: Some(format!("{}/watch/{code}", self.config.public_base_url)),
            code: Some(code),
        }
    }
}

/// Both owner surfaces serialize lifecycle changes on the recording row.
pub(crate) struct Capability {
    pub(crate) recording: Uuid,
    pub(crate) mode: CodeMode,
    pub(crate) ended: Option<DateTime<Utc>>,
    pub(crate) deleted: bool,
    pub(crate) summary: RecordingSummary,
}

impl Capability {
    pub(crate) async fn load(db: &PgPool, code: &str) -> AppResult<Self> {
        let row = sqlx::query!(
            r#"
            SELECT s.recording_id,
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
            code,
        )
        .fetch_optional(db)
        .await
        .trace()?
        .trace_ctx(("Code not found", StatusCode::NOT_FOUND))?;

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

        cap.active().trace()?;

        Ok(cap)
    }

    pub(crate) fn active(&self) -> AppResult<()> {
        if self.mode == CodeMode::Public
            && (self.summary.is_practice || self.summary.is_custom != Some(false))
        {
            return Err(AppError::gone("public_not_allowed"));
        }

        if self.deleted || (self.mode == CodeMode::Live && self.ended.is_some()) {
            Err(AppError::gone("This match is no longer shared"))
        } else {
            Ok(())
        }
    }
}

pub(crate) struct SelfContainedFrame {
    pub(crate) captured_at: i64,
    pub(crate) game_time: f32,
    pub(crate) probe: protocol::Probe,
    pub(crate) bytes: Vec<u8>,
}

pub(crate) struct LiveDecodeError {
    pub(crate) too_large: bool,
    pub(crate) message: String,
}

impl LiveDecodeError {
    pub(crate) fn invalid(error: impl std::fmt::Display) -> Self {
        Self {
            too_large: false,
            message: error.to_string(),
        }
    }

    pub(crate) fn large() -> Self {
        Self {
            too_large: true,
            message: "Live snapshot too large once decompressed".into(),
        }
    }
}

impl SelfContainedFrame {
    pub(crate) fn decode(body: &[u8]) -> Result<Self, LiveDecodeError> {
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

pub(crate) struct SharingService<'a> {
    pub(crate) state: &'a AppState,
}

impl SharingService<'_> {
    pub(crate) async fn change_share(
        &self,
        auth: super::google::Owner,
        id: Uuid,
        request: SetShareMode,
        finished_only: bool,
    ) -> AppResult<ShareState> {
        let state = self.state;

        if finished_only && request.mode == ShareMode::Live {
            return Err(AppError::bad_request("Live is unavailable in history"));
        }

        let mut tx = state.db.begin().await.trace()?;
        let recording = sqlx::query!(
            r#"
            SELECT ended_at,
                is_custom,
                is_practice
            FROM recordings
            WHERE id = $1
                AND ((device_id = $2
                        AND $2 IS NOT NULL)
                        OR (account_id = $3
                            AND $3 IS NOT NULL))
                AND deleted_at IS NULL
                AND EXISTS (
                    SELECT 1
                    FROM devices
                    WHERE devices.id = recordings.device_id
                        AND devices.deleted_at IS NULL
                )
            FOR UPDATE
            "#,
            id,
            auth.device_id,
            auth.account_id,
        )
        .fetch_optional(&mut *tx)
        .await
        .trace()?
        .trace_ctx(("Unknown recording", StatusCode::NOT_FOUND))?;

        if finished_only && recording.ended_at.is_none() {
            return Err(AppError::conflict("The recording is still active"));
        }

        if request.mode == ShareMode::Public
            && (recording.is_practice || recording.is_custom != Some(false))
        {
            return Err(AppError::forbidden("public_not_allowed"));
        }

        if request.mode == ShareMode::Live && recording.ended_at.is_some() {
            return Err(AppError::conflict("The recording has ended"));
        }

        if request.mode == ShareMode::Private {
            sqlx::query!(
                r#"
                UPDATE share_codes
                SET deleted_at = now(),
                    delete_reason = $2
                WHERE recording_id = $1
                    AND deleted_at IS NULL
                "#,
                id,
                ShareDeleteReason::Revoked,
            )
            .execute(&mut *tx)
            .await
            .trace()?;

            tx.commit().await.trace()?;
            state.live.close(id);

            return state.share_state(id).await.trace();
        }

        let mode = if request.mode == ShareMode::Live {
            CodeMode::Live
        } else {
            CodeMode::Public
        };

        let existing = sqlx::query!(
            r#"
            SELECT id, code
            FROM share_codes
            WHERE recording_id = $1
                AND deleted_at IS NULL
            "#,
            id,
        )
        .fetch_optional(&mut *tx)
        .await
        .trace()?;

        let code = match existing {
            Some(existing) => {
                sqlx::query!(
                    r#"
                    UPDATE share_codes
                    SET MODE = $2, mode_changed_at = now()
                    WHERE id = $1
                    "#,
                    existing.id,
                    mode,
                )
                .execute(&mut *tx)
                .await
                .trace()?;

                existing.code
            }
            None => {
                let mut chosen = None;

                for _ in 0..5 {
                    let mut random = [0u8; 5];

                    getrandom::fill(&mut random).trace()?;

                    let code = code_from_random(random);
                    let inserted = sqlx::query_scalar!(
                        r#"
                        INSERT INTO share_codes (
                            recording_id,
                            code,
                            MODE
                        ) VALUES (
                            $1,
                            $2,
                            $3
                        )
                        ON CONFLICT(code) DO NOTHING RETURNING code
                        "#,
                        id,
                        code,
                        mode,
                    )
                    .fetch_optional(&mut *tx)
                    .await
                    .trace()?;

                    if inserted.is_some() {
                        chosen = inserted;
                        break;
                    }
                }

                chosen.trace_ctx((
                    "Could not allocate a share code",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?
            }
        };

        tx.commit().await.trace()?;
        Ok(state.code_state(&code, mode))
    }
}

impl From<LiveDecodeError> for AppError {
    fn from(error: LiveDecodeError) -> Self {
        AppError::new(
            if error.too_large {
                StatusCode::PAYLOAD_TOO_LARGE
            } else {
                StatusCode::UNPROCESSABLE_ENTITY
            },
            error.message,
        )
    }
}
