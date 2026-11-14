//! Device registration and recording control (spec §10.2).

use super::{
    db_enums::{RiotGameSource, ShareDeleteReason},
    ingest::{hex, rederive_frames},
};
use crate::server::response::{AppError, AppResult, Trace};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};
use protocol::{
    CodeMode, EndReason, FRAMES_FORMAT, GameMode, Platform, Probe, Region, ShareMode, ShareState,
    StartSession,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

/// Keeps a recording alive while no chunk is sent (spec §7.3). Reopens a
/// recording the janitor timed out when the host comes back (spec §8.5).
///
/// Ends a recording (spec §8.4). Idempotent: the first end wins, later calls
/// only fill in a result or game length that is still missing.
///
/// Final result from the League client's end-of-game stats (spec §8.4). The
/// client's game ids never replace Riot-verified ones.
pub use super::routes::{
    devices::register_device,
    recordings::{
        chunk::upload_chunk, end::end, heartbeat::heartbeat, result::set_result,
        sessions::start_session,
    },
};

/// Start-or-resume (spec §8.3): resumes the device's recording of the same
/// match when there is one, otherwise ends the device's open recordings as
/// superseded and creates a new one.
///
/// A recording ended by timeout this recently can still be resumed (spec §8.3).
pub(crate) const RESUME_WINDOW_SECS: f64 = 6.0 * 3600.0;
pub(crate) const PRIVATE: ShareState = ShareState {
    mode: ShareMode::Private,
    code: None,
    url: None,
};

/// Ends a recording if it is still open and expires its Live code in the same
/// transaction. Returns whether the recording was open.
///
/// Header carrying the hex SHA-256 of the uncompressed chunk (spec §6.3).
pub const CONTENT_SHA256: &str = "X-Content-Sha256";

/// Stores one sealed chunk (spec §10.3). Idempotent per `(recording, seq)`;
/// late chunks are accepted but never reopen an ended recording.
///
/// `200` when `seq` already holds this exact chunk, `409` when it holds another one.
///
/// The device's recording, or `404`: ownership is never revealed to other devices.
pub(crate) struct Owned {
    pub(crate) end_reason: Option<EndReason>,
}

pub(crate) struct RecordingService<'a> {
    pub(crate) db: &'a PgPool,
}

impl RecordingService<'_> {
    pub(crate) fn decode_probe(bytes: &[u8]) -> Option<Probe> {
        bincode::decode_from_slice(bytes, types::BINCODE_CONFIG)
            .ok()
            .map(|(probe, _)| probe)
    }

    pub(crate) async fn insert_recording(
        tx: &mut PgConnection,
        device_id: Uuid,
        request: &StartSession,
    ) -> sqlx::Result<Uuid> {
        let probe = &request.probe;
        let probe_bytes = bincode::encode_to_vec(probe, types::BINCODE_CONFIG)
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
                account_id,
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
            ) VALUES (
                $1,
                (
                    SELECT d.account_id
                    FROM devices d
                    JOIN accounts a ON a.id = d.account_id
                    JOIN account_sessions s ON s.id = d.account_session_id
                    WHERE d.id = $1
                    AND d.deleted_at IS NULL
                        AND a.deleted_at IS NULL
                        AND s.account_id = d.account_id
                        AND s.device_id = d.id
                        AND s.kind = 'Desktop'
                        AND s.desktop_confirmed_at IS NOT NULL
                        AND s.revoked_at IS NULL
                        AND s.expires_at > clock_timestamp()
                ),
                $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11,
                $12, $13,
                now() - $14 * interval '1 second',
                $15, $15, $16
            )
            RETURNING id
            "#,
            device_id,
            probe.game_mode.as_str(),
            probe.map_number as i16,
            &probe.fingerprint()[..],
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

    /// Frames of a chunk in the current [`FRAMES_FORMAT`]. Frames stored with
    /// another format are derived again from the raw chunk, which is kept
    /// verbatim, and stored so later requests serve them directly
    pub(crate) async fn current_frames(
        &self,
        id: Uuid,
        seq: i32,
        frames: Vec<u8>,
        frames_format: i16,
    ) -> AppResult<Vec<u8>> {
        if frames_format == FRAMES_FORMAT as i16 {
            return Ok(frames);
        }

        let db = self.db;
        let raw = sqlx::query_scalar!(
            r#"
            SELECT RAW
            FROM recording_chunks
            WHERE recording_id = $1
                AND seq = $2
            "#,
            id,
            seq,
        )
        .fetch_optional(db)
        .await
        .trace()?
        .trace_ctx(("Chunk not found", StatusCode::NOT_FOUND))?;

        let frames = web::block(move || rederive_frames(&raw))
            .await
            .trace()?
            .trace_ctx((
                "Frames unavailable for this chunk",
                StatusCode::UNPROCESSABLE_ENTITY,
            ))?;

        // Matching the old format keeps a concurrent upgrade from being overwritten
        sqlx::query!(
            r#"
            UPDATE recording_chunks
            SET frames = $3,
                frames_format = $4
            WHERE recording_id = $1
                AND seq = $2
                AND frames_format = $5
            "#,
            id,
            seq,
            &frames,
            FRAMES_FORMAT as i16,
            frames_format,
        )
        .execute(db)
        .await
        .trace()?;

        Ok(frames)
    }

    pub(crate) async fn existing_chunk(
        &self,
        id: Uuid,
        seq: u32,
        sha_hex: &str,
    ) -> AppResult<Option<HttpResponse>> {
        let db = self.db;
        let stored = sqlx::query_scalar!(
            r#"
            SELECT raw_sha256
            FROM recording_chunks
            WHERE recording_id = $1
                AND seq = $2
            "#,
            id,
            seq as i32,
        )
        .fetch_optional(db)
        .await
        .trace()?;

        match stored {
            None => Ok(None),
            Some(stored) if hex(&stored).eq_ignore_ascii_case(sha_hex) => {
                Ok(Some(HttpResponse::Ok().finish()))
            }
            Some(_) => Err(AppError::conflict("Another chunk is stored under this seq")),
        }
    }

    pub(crate) async fn owned(&self, id: Uuid, device_id: Uuid) -> AppResult<Owned> {
        let db = self.db;

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
        .trace()?
        .trace_ctx(("Unknown recording", StatusCode::NOT_FOUND))
    }
    pub async fn end_recording_tx(
        tx: &mut PgConnection,
        id: Uuid,
        reason: EndReason,
        ended_at: DateTime<Utc>,
    ) -> sqlx::Result<bool> {
        let ended = sqlx::query!(
            r#"
            UPDATE recordings
            SET ended_at = $2,
                end_reason = $3
            WHERE id = $1
                AND ended_at IS NULL
            "#,
            id,
            ended_at,
            reason,
        )
        .execute(&mut *tx)
        .await?
        .rows_affected()
            == 1;

        sqlx::query!(
            r#"
            UPDATE share_codes
            SET deleted_at = now(),
                delete_reason = $2
            WHERE recording_id = $1
                AND MODE = $3
                AND deleted_at IS NULL
            "#,
            id,
            ShareDeleteReason::GameEnded,
            CodeMode::Live,
        )
        .execute(&mut *tx)
        .await?;

        Ok(ended)
    }
}
