//! Device-owned metadata and derived replay access. Never mints capabilities on reads.
use super::{
    codec::{Bincode, OCTET_STREAM, Response, respond},
    google::Owner,
    sharing,
    state::AppState,
};
use actix_web::{
    HttpResponse, Scope,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity},
    web::{self, Data, Path, Query},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tutorlolv2_protocol::*;
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("/me/recordings")
        .app_data(
            web::PathConfig::default()
                .error_handler(|_, _| ErrorBadRequest("Invalid recording ID or sequence")),
        )
        .route("", web::get().to(list))
        .route("/{id}", web::get().to(details))
        .route("/{id}/frames/{seq}", web::get().to(frames))
        .route("/{id}/share", web::put().to(set_share))
}

#[derive(Deserialize)]
pub struct Pagination {
    before: Option<Uuid>,
    limit: Option<u32>,
}

// Deliberately excludes probe/raw/frame payloads and player identifiers beyond the host Riot ID.

fn internal(_: sqlx::Error) -> actix_web::Error {
    ErrorInternalServerError("History storage unavailable")
}

fn checked<T, U: TryFrom<T>>(value: T) -> actix_web::Result<U> {
    U::try_from(value).map_err(|_| ErrorInternalServerError("Invalid stored history metadata"))
}

struct HistoryRow {
    id: Uuid,
    champion: String,
    game_mode: String,
    queue_id: Option<i32>,
    active_riot_id: String,
    started_at_estimate: DateTime<Utc>,
    first_game_time: f32,
    last_game_time: f32,
    game_length_s: Option<i32>,
    result: Option<GameResult>,
    is_custom: Option<bool>,
    is_practice: bool,
    ended_at: Option<DateTime<Utc>>,
    end_reason: Option<EndReason>,
    coverage_s: i32,
    kills: i16,
    deaths: i16,
    assists: i16,
    creep_score: i16,
    level: i16,
    code: Option<String>,
    mode: Option<CodeMode>,
}

fn item(state: &AppState, row: &HistoryRow) -> actix_web::Result<HistoryListItem> {
    let code = row.code.clone();
    let mode = row.mode;

    Ok(HistoryListItem {
        summary: HistorySummary {
            recording: RecordingSummary {
                id: row.id.into_bytes(),
                champion: row.champion.clone(),
                game_mode: GameMode::from(row.game_mode.clone()),
                queue_id: row.queue_id.map(checked).transpose()?,
                active_riot_id: row.active_riot_id.clone(),
                started_at_ms: row.started_at_estimate.timestamp_millis(),
                first_game_time: row.first_game_time,
                last_game_time: row.last_game_time,
                game_length_s: row.game_length_s.map(checked).transpose()?,
                result: row.result,
                is_custom: row.is_custom,
                is_practice: row.is_practice,
            },
            ended_at_ms: row.ended_at.map(|v| v.timestamp_millis()),
            end_reason: row.end_reason,
            coverage_s: checked(row.coverage_s)?,
            kills: checked(row.kills)?,
            deaths: checked(row.deaths)?,
            assists: checked(row.assists)?,
            creep_score: checked(row.creep_score)?,
            level: checked(row.level)?,
        },
        share: match (code, mode) {
            (Some(c), Some(m)) => state.code_state(&c, m),
            _ => ShareState {
                mode: ShareMode::Private,
                code: None,
                url: None,
            },
        },
    })
}

fn no_store(mut response: HttpResponse) -> HttpResponse {
    response.headers_mut().insert(
        actix_web::http::header::CACHE_CONTROL,
        actix_web::http::header::HeaderValue::from_static("no-store"),
    );
    response
}

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
        .map(|row| item(&state, row))
        .collect::<actix_web::Result<Vec<_>>>()?;

    let next_before = if more {
        items.last().map(|v| v.summary.recording.id)
    } else {
        None
    };

    respond(HistoryPage { items, next_before }).map(no_store)
}

pub async fn details(state: Data<AppState>, auth: Owner, id: Path<Uuid>) -> Response {
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
    .map_err(internal)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

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
    .map_err(internal)?
    .iter()
    .map(|r| ChunkInfo {
        seq: r.seq as u32,
        first_game_time: r.first_game_time,
        last_game_time: r.last_game_time,
        snapshot_count: r.snapshot_count as u16,
        frames_format: r.frames_format as u16,
    })
    .collect();

    let item = item(&state, &row)?;

    respond(RecordingDetails {
        summary: item.summary,
        share: item.share,
        chunks,
        frames_format: FRAMES_FORMAT,
    })
    .map(no_store)
}
pub async fn frames(state: Data<AppState>, auth: Owner, path: Path<(Uuid, u32)>) -> Response {
    let (id, seq) = path.into_inner();
    let seq = i32::try_from(seq).map_err(|_| ErrorNotFound("Chunk not found"))?;
    let row = sqlx::query!(
        r#"
        SELECT c.frames,
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
            AND c.seq = $3
        "#,
        auth.device_id,
        id,
        seq,
        auth.account_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal)?
    .ok_or_else(|| ErrorNotFound("Chunk not found"))?;

    if row.frames_format != FRAMES_FORMAT as i16 {
        return Err(ErrorUnprocessableEntity(
            "Frames unavailable for this chunk",
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Cache-Control", "no-store"))
        .insert_header(("Content-Encoding", "br"))
        .body(row.frames))
}

pub async fn set_share(
    state: Data<AppState>,
    auth: Owner,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> Response {
    sharing::change_share(&state, auth, id.into_inner(), request, true)
        .await
        .map(no_store)
        .map_err(|error| {
            if error.as_response_error().status_code().is_server_error() {
                ErrorInternalServerError("History storage unavailable")
            } else {
                error
            }
        })
}
