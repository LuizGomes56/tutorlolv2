use crate::server::endpoint::EndpointExt;
use crate::server::response::Trace;
use crate::server::{google::Owner, state::AppState};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use tutorlolv2_protocol::*;
use uuid::Uuid;

use crate::server::me::*;

pub async fn details(
    state: Data<AppState>,
    auth: Owner,
    id: Path<Uuid>,
) -> crate::server::response::ApiResult<RecordingDetails> {
    let row = sqlx::query_as!(
        HistoryRow,
        r#"
        SELECT r.id,
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
               r.is_practice,
               r.ended_at,
               r.end_reason,
               r.coverage_s,
               r.kills,
               r.deaths,
               r.assists,
               r.creep_score,
               r.level,
               s.code AS "code?",
               s.mode AS "mode?"
        FROM recordings r
        LEFT JOIN share_codes s ON s.recording_id = r.id
        AND s.deleted_at IS NULL
        WHERE ((r.device_id = $1
                AND $1 IS NOT NULL)
               OR (r.account_id = $4
                   AND $4 IS NOT NULL))
            AND r.deleted_at IS NULL
            AND EXISTS
                (SELECT 1
                 FROM devices d
                 WHERE d.id = r.device_id
                     AND d.deleted_at IS NULL)
            AND r.id = $2
            AND $3::integer IS NULL
        "#,
        auth.device_id,
        *id,
        Option::<i32>::None,
        auth.account_id,
    )
    .fetch_optional(&state.db)
    .await
    .trace_ctx("History storage unavailable")?
    .trace_ctx(("Unknown recording", StatusCode::NOT_FOUND))?;

    let chunks = sqlx::query!(
        r#"
        SELECT c.seq,
               c.first_game_time,
               c.last_game_time,
               c.snapshot_count,
               c.frames_format
        FROM recording_chunks c
        JOIN recordings r ON r.id = c.recording_id
        WHERE ((r.device_id = $1
                AND $1 IS NOT NULL)
               OR (r.account_id = $4
                   AND $4 IS NOT NULL))
            AND r.deleted_at IS NULL
            AND EXISTS
                (SELECT 1
                 FROM devices d
                 WHERE d.id = r.device_id
                     AND d.deleted_at IS NULL)
            AND r.id = $2
            AND $3::integer IS NULL
        ORDER BY c.first_game_time,
                 c.seq
        "#,
        auth.device_id,
        *id,
        Option::<i32>::None,
        auth.account_id,
    )
    .fetch_all(&state.db)
    .await
    .trace_ctx("History storage unavailable")?
    .iter()
    .map(|r| ChunkInfo {
        seq: r.seq as u32,
        first_game_time: r.first_game_time,
        last_game_time: r.last_game_time,
        snapshot_count: r.snapshot_count as u16,
        frames_format: r.frames_format as u16,
    })
    .collect();

    let item = (HistoryService { state: &state }).item(&row)?;

    Ok(tutorlolv2_api::GetRecording::reply(RecordingDetails {
        summary: item.summary,
        share: item.share,
        chunks,
        frames_format: FRAMES_FORMAT,
    }))
    .map(|reply| reply.no_store())
}

api_endpoint! {
    pub GetRecording {
        handler: details, method: GET, path: "/api/me/recordings/{id}",
        path_params: { id: Uuid }, query: none, body: none,
        response: envelope(tutorlolv2_protocol::RecordingDetails), success: [200], auth: owner,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
