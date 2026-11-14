//! Device-owned metadata and derived replay access. Never mints capabilities on reads.

use super::state::AppState;
use crate::server::response::{AppError, AppResult, Trace};
use chrono::{DateTime, Utc};
use protocol::*;
use uuid::Uuid;

pub use super::routes::{
    me::{frames::frames, recording::details, recordings::list, share::set_share},
    me_scope as scope,
};
pub use ::api::Pagination;

// Deliberately excludes probe/raw/frame payloads and player identifiers beyond the host Riot ID.

pub(crate) struct HistoryRow {
    pub(crate) id: Uuid,
    pub(crate) champion: String,
    pub(crate) game_mode: String,
    pub(crate) queue_id: Option<i32>,
    pub(crate) active_riot_id: String,
    pub(crate) started_at_estimate: DateTime<Utc>,
    pub(crate) first_game_time: f32,
    pub(crate) last_game_time: f32,
    pub(crate) game_length_s: Option<i32>,
    pub(crate) result: Option<GameResult>,
    pub(crate) is_custom: Option<bool>,
    pub(crate) is_practice: bool,
    pub(crate) ended_at: Option<DateTime<Utc>>,
    pub(crate) end_reason: Option<EndReason>,
    pub(crate) coverage_s: i32,
    pub(crate) kills: i16,
    pub(crate) deaths: i16,
    pub(crate) assists: i16,
    pub(crate) creep_score: i16,
    pub(crate) level: i16,
    pub(crate) code: Option<String>,
    pub(crate) mode: Option<CodeMode>,
}

pub(crate) struct HistoryService<'a> {
    pub(crate) state: &'a AppState,
}

impl HistoryService<'_> {
    fn checked<T, U: TryFrom<T>>(value: T) -> AppResult<U>
    where
        U::Error: Into<AppError>,
    {
        U::try_from(value).trace_ctx("Invalid stored history metadata")
    }

    pub(crate) fn item(&self, row: &HistoryRow) -> AppResult<HistoryListItem> {
        let state = self.state;
        let code = row.code.clone();
        let mode = row.mode;

        Ok(HistoryListItem {
            summary: HistorySummary {
                recording: RecordingSummary {
                    id: row.id.into_bytes(),
                    champion: row.champion.clone(),
                    game_mode: GameMode::from(row.game_mode.clone()),
                    queue_id: row.queue_id.map(Self::checked).transpose()?,
                    active_riot_id: row.active_riot_id.clone(),
                    started_at_ms: row.started_at_estimate.timestamp_millis(),
                    first_game_time: row.first_game_time,
                    last_game_time: row.last_game_time,
                    game_length_s: row.game_length_s.map(Self::checked).transpose()?,
                    result: row.result,
                    is_custom: row.is_custom,
                    is_practice: row.is_practice,
                },
                ended_at_ms: row.ended_at.map(|v| v.timestamp_millis()),
                end_reason: row.end_reason,
                coverage_s: Self::checked(row.coverage_s)?,
                kills: Self::checked(row.kills)?,
                deaths: Self::checked(row.deaths)?,
                assists: Self::checked(row.assists)?,
                creep_score: Self::checked(row.creep_score)?,
                level: Self::checked(row.level)?,
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
}
