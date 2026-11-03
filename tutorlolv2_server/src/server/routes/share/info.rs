use crate::server::{
    codec::{Response, respond},
    state::AppState,
};
use actix_web::{
    HttpRequest,
    error::ErrorInternalServerError,
    web::{Data, Path},
};
use tutorlolv2_protocol::{ChunkInfo, FRAMES_FORMAT, ShareInfo, ShareStatus};

use crate::server::sharing::*;

pub async fn info(state: Data<AppState>, code: Path<String>, req: HttpRequest) -> Response {
    let (code, cap) = state.capability(&code, &req).await?;

    let chunks = sqlx::query!(
        r#"
        SELECT seq,
               first_game_time,
               last_game_time,
               snapshot_count,
               frames_format
        FROM recording_chunks
        WHERE recording_id = $1
        ORDER BY seq
        "#,
        cap.recording,
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
