use crate::server::{
    codec::{Response, respond},
    google::Owner,
    state::AppState,
};
use actix_web::{
    error::ErrorBadRequest,
    web::{Data, Query},
};
use tutorlolv2_protocol::*;

use crate::server::me::*;

pub async fn list(state: Data<AppState>, auth: Owner, query: Query<Pagination>) -> Response {
    let limit = query.limit.unwrap_or(50);

    if !(1..=100).contains(&limit) {
        return Err(ErrorBadRequest("limit must be between 1 and 100"));
    }

    let rows = sqlx::query_as!(
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
            AND ($2::UUID IS NULL
                 OR r.id < $2)
        ORDER BY r.id DESC
        LIMIT $3
        "#,
        auth.device_id,
        query.before,
        i64::from(limit) + 1,
        auth.account_id,
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal)?;

    let more = rows.len() > limit as usize;
    let items: Vec<_> = rows
        .iter()
        .take(limit as usize)
        .map(|row| (HistoryService { state: &state }).item(row))
        .collect::<actix_web::Result<Vec<_>>>()?;

    let next_before = if more {
        items.last().map(|v| v.summary.recording.id)
    } else {
        None
    };

    respond(HistoryPage { items, next_before }).map(no_store)
}

api_contract! { pub struct Pagination { pub before: Option<Uuid>, pub limit: Option<u32> } }

api_endpoint! {
    pub ListRecordings {
        handler: list, method: GET, path: "/api/me/recordings",
        path_params: {  }, query: form(Pagination), body: none,
        response: envelope(tutorlolv2_protocol::HistoryPage), success: [200], auth: owner,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
