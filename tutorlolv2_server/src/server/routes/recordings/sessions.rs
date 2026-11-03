use crate::server::{
    auth::DeviceAuth,
    codec::{Bincode, Response, respond},
    riot_api::{EnrichJob, log_unmapped_region},
    state::AppState,
};
use actix_web::{error::ErrorInternalServerError, web::Data};
use chrono::Utc;
use tutorlolv2_protocol::{EndReason, MatchDecision, SessionStarted, StartSession};
use uuid::Uuid;

use crate::server::recordings::*;

pub async fn start_session(
    state: Data<AppState>,
    auth: DeviceAuth,
    Bincode(request): Bincode<StartSession>,
) -> Response {
    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;
    // Serialize ownership with redeem/logout before reading the account link.
    // A foreign-key lock taken by INSERT is too late: its account subquery may
    // already have observed the pre-login device row.
    let device = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM devices
        WHERE id = $1
            AND deleted_at IS NULL
            FOR
            UPDATE
        "#,
        auth.device_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;
    if device.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Device unavailable"));
    }
    // The native client drops expired credentials, but ingest still uses its
    // DeviceAuth. A stale link must not silently attach later games to that
    // signed-out account. Check the current binding under the same device lock
    // used by redeem/logout so a newer login cannot be unlinked here.
    sqlx::query!(
        r#"
        UPDATE devices d
        SET account_id = NULL,
            account_session_id = NULL
        WHERE d.id = $1
            AND (d.account_id IS NOT NULL
                 OR d.account_session_id IS NOT NULL)
            AND NOT EXISTS
                (SELECT 1
                 FROM account_sessions s
                 JOIN accounts a ON a.id = s.account_id
                 WHERE s.id = d.account_session_id
                     AND s.account_id = d.account_id
                     AND s.device_id = d.id
                     AND s.kind = 'Desktop'
                     AND s.revoked_at IS NULL
                     AND s.expires_at > now()
                     AND a.deleted_at IS NULL)
        "#,
        auth.device_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;
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
            AND (ended_at IS NULL
                 OR (end_reason = $2
                     AND ended_at > now() - $3 * interval '1 second'))
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
        RecordingService::decode_probe(&c.probe).is_some_and(|stored| {
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
        RecordingService::end_recording_tx(&mut tx, other.id, EndReason::Superseded, now)
            .await
            .map_err(ErrorInternalServerError)?;
    }

    let mut started = match same {
        Some(found) => {
            if !found.open {
                sqlx::query!(
                    r#"
                    UPDATE recordings
                    SET ended_at = NULL,
                        end_reason = NULL,
                        last_ingest_at = now()
                    WHERE id = $1
                    "#,
                    found.id,
                )
                .execute(&mut *tx)
                .await
                .map_err(ErrorInternalServerError)?;
            }

            let last_seq = sqlx::query_scalar!(
                r#"
                SELECT max(seq)
                FROM recording_chunks
                WHERE recording_id = $1
                "#,
                found.id,
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
            let id = RecordingService::insert_recording(&mut tx, auth.device_id, &request)
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

    for other in candidates
        .iter()
        .filter(|c| c.open && Some(c.id) != same.map(|s| s.id))
    {
        state.live.close(other.id);
    }

    started.share = state
        .share_state(Uuid::from_bytes(started.recording_id))
        .await?;

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
