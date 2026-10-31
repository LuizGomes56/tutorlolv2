//! Device-owned metadata and derived replay access. Never mints capabilities on reads.
use super::{
    auth::DeviceAuth,
    codec::{Bincode, OCTET_STREAM, Response, respond},
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
use sqlx::Row;
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
// SQL composition below uses only these fixed fragments; all request data is bound.
const COLUMNS: &str = "r.id, r.champion, r.game_mode, r.queue_id, r.active_riot_id, r.started_at_estimate, r.first_game_time, r.last_game_time, r.game_length_s, r.result, r.is_custom, r.is_practice, r.ended_at, r.end_reason, r.coverage_s, r.kills, r.deaths, r.assists, r.creep_score, r.level, s.code, s.mode";
const OWNER: &str = "r.device_id=$1 AND r.deleted_at IS NULL AND EXISTS (SELECT 1 FROM devices d WHERE d.id=r.device_id AND d.deleted_at IS NULL)";
const JOIN: &str =
    "FROM recordings r LEFT JOIN share_codes s ON s.recording_id=r.id AND s.deleted_at IS NULL";

fn internal(_: sqlx::Error) -> actix_web::Error {
    ErrorInternalServerError("History storage unavailable")
}

fn checked<T, U: TryFrom<T>>(value: T) -> actix_web::Result<U> {
    U::try_from(value).map_err(|_| ErrorInternalServerError("Invalid stored history metadata"))
}

fn item(state: &AppState, row: &sqlx::postgres::PgRow) -> actix_web::Result<HistoryListItem> {
    let code: Option<String> = row.get("code");
    let mode: Option<CodeMode> = row.get("mode");

    Ok(HistoryListItem {
        summary: HistorySummary {
            recording: RecordingSummary {
                id: row.get::<Uuid, _>("id").into_bytes(),
                champion: row.get("champion"),
                game_mode: GameMode::from(row.get::<String, _>("game_mode")),
                queue_id: row
                    .get::<Option<i32>, _>("queue_id")
                    .map(checked)
                    .transpose()?,
                active_riot_id: row.get("active_riot_id"),
                started_at_ms: row
                    .get::<DateTime<Utc>, _>("started_at_estimate")
                    .timestamp_millis(),
                first_game_time: row.get("first_game_time"),
                last_game_time: row.get("last_game_time"),
                game_length_s: row
                    .get::<Option<i32>, _>("game_length_s")
                    .map(checked)
                    .transpose()?,
                result: row.get("result"),
                is_custom: row.get("is_custom"),
                is_practice: row.get("is_practice"),
            },
            ended_at_ms: row
                .get::<Option<DateTime<Utc>>, _>("ended_at")
                .map(|v| v.timestamp_millis()),
            end_reason: row.get("end_reason"),
            coverage_s: checked(row.get::<i32, _>("coverage_s"))?,
            kills: checked(row.get::<i16, _>("kills"))?,
            deaths: checked(row.get::<i16, _>("deaths"))?,
            assists: checked(row.get::<i16, _>("assists"))?,
            creep_score: checked(row.get::<i16, _>("creep_score"))?,
            level: checked(row.get::<i16, _>("level"))?,
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

pub async fn list(state: Data<AppState>, auth: DeviceAuth, query: Query<Pagination>) -> Response {
    let limit = query.limit.unwrap_or(50);

    if !(1..=100).contains(&limit) {
        return Err(ErrorBadRequest("limit must be between 1 and 100"));
    }

    let rows = sqlx::query(sqlx::AssertSqlSafe(format!("SELECT {COLUMNS} {JOIN} WHERE {OWNER} AND ($2::uuid IS NULL OR r.id < $2) ORDER BY r.id DESC LIMIT $3")))
        .bind(auth.device_id).bind(query.before).bind(i64::from(limit)+1).fetch_all(&state.db).await.map_err(internal)?;

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

pub async fn details(state: Data<AppState>, auth: DeviceAuth, id: Path<Uuid>) -> Response {
    let row = sqlx::query(sqlx::AssertSqlSafe(format!(
        "SELECT {COLUMNS} {JOIN} WHERE {OWNER} AND r.id = $2"
    )))
    .bind(auth.device_id)
    .bind(*id)
    .fetch_optional(&state.db)
    .await
    .map_err(internal)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    let chunks = sqlx::query(sqlx::AssertSqlSafe(format!("SELECT c.seq,c.first_game_time,c.last_game_time,c.snapshot_count,c.frames_format FROM recording_chunks c JOIN recordings r ON r.id=c.recording_id WHERE {OWNER} AND r.id=$2 ORDER BY c.first_game_time,c.seq")))
        .bind(auth.device_id).bind(*id).fetch_all(&state.db).await.map_err(internal)?.iter().map(|r|ChunkInfo {seq:r.get::<i32,_>("seq") as u32, first_game_time:r.get("first_game_time"), last_game_time:r.get("last_game_time"), snapshot_count:r.get::<i16,_>("snapshot_count") as u16, frames_format:r.get::<i16,_>("frames_format") as u16}).collect();

    let item = item(&state, &row)?;

    respond(RecordingDetails {
        summary: item.summary,
        share: item.share,
        chunks,
        frames_format: FRAMES_FORMAT,
    })
    .map(no_store)
}
pub async fn frames(state: Data<AppState>, auth: DeviceAuth, path: Path<(Uuid, u32)>) -> Response {
    let (id, seq) = path.into_inner();
    let seq = i32::try_from(seq).map_err(|_| ErrorNotFound("Chunk not found"))?;
    let row=sqlx::query(sqlx::AssertSqlSafe(format!("SELECT c.frames,c.frames_format FROM recording_chunks c JOIN recordings r ON r.id=c.recording_id WHERE {OWNER} AND r.id=$2 AND c.seq=$3")))
        .bind(auth.device_id).bind(id).bind(seq).fetch_optional(&state.db).await.map_err(internal)?.ok_or_else(||ErrorNotFound("Chunk not found"))?;

    if row.get::<i16, _>("frames_format") != FRAMES_FORMAT as i16 {
        return Err(ErrorUnprocessableEntity(
            "Frames unavailable for this chunk",
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Cache-Control", "no-store"))
        .insert_header(("Content-Encoding", "br"))
        .body(row.get::<Vec<u8>, _>("frames")))
}

pub async fn set_share(
    state: Data<AppState>,
    auth: DeviceAuth,
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
