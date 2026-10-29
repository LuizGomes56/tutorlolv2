//! Ends recordings whose host stopped reporting (spec §8.5).

use super::{db_enums::ShareDeleteReason, state::AppState};
use actix_web::web::Data;
use sqlx::PgPool;
use std::time::Duration;
use tutorlolv2_protocol::{CodeMode, EndReason};

const INTERVAL: Duration = Duration::from_secs(60);

/// Ends every open recording with no ingest or heartbeat for `timeout_secs`
/// (`ended_at` = the last time it was heard from) and expires their Live codes.
/// Returns how many recordings were ended.
pub async fn expire_stale(db: &PgPool, timeout_secs: i64) -> sqlx::Result<u64> {
    let mut tx = db.begin().await?;

    let ended = sqlx::query_scalar!(
        "
        UPDATE recordings SET ended_at = last_ingest_at, end_reason = $2
        WHERE ended_at IS NULL
            AND deleted_at IS NULL
            AND last_ingest_at < now() - $1 * interval '1 second'
        RETURNING id
        ",
        timeout_secs as f64,
        EndReason::Timeout,
    )
    .fetch_all(&mut *tx)
    .await?;

    sqlx::query!(
        "
        UPDATE share_codes SET deleted_at = now(), delete_reason = $2
        WHERE recording_id = ANY($1)
            AND mode = $3
            AND deleted_at IS NULL
        ",
        &ended,
        ShareDeleteReason::GameEnded,
        CodeMode::Live,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(ended.len() as u64)
}

/// Runs [`expire_stale`] every minute for the lifetime of the server.
pub fn spawn(state: Data<AppState>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(INTERVAL);

        loop {
            interval.tick().await;

            match expire_stale(&state.db, state.config.recording_timeout_secs).await {
                Ok(0) => {}
                Ok(n) => println!("Janitor ended {n} idle recording(s)"),
                Err(e) => eprintln!("Janitor failed: {e}"),
            }
        }
    });
}
