//! Reading one `GET /liveclientdata/allgamedata` body (spec §6.2, §8.2, §8.6).

use crate::{EventKey, GameMode, GameResult, Probe, RosterEntry, Team, open_enum};
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, de::IgnoredAny};

const SKIN_PREFIX: &str = "game_character_skin_displayname_";
const DISPLAY_PREFIX: &str = "game_character_displayname_";
const NEEKO_KEY: &str = "neeko";
const NEEKO_RAW_CHAMPION: &str = "game_character_displayname_Neeko";

open_enum!(
    /// `events.Events[*].EventName`; only the names this crate acts on are listed.
    pub enum EventName {
        GameStart = "GameStart",
        GameEnd = "GameEnd",
    }
);

open_enum!(
    /// `GameEnd.Result`. Other shapes (surrender, remake, Arena) aren't verified
    /// yet (V2), so unknown values must not reject the snapshot.
    pub enum GameEndResult {
        Win = "Win",
        Lose = "Lose",
    }
);

impl GameEndResult {
    pub fn result(&self) -> Option<GameResult> {
        match self {
            Self::Win => Some(GameResult::Win),
            Self::Lose => Some(GameResult::Lose),
            Self::Other(_) => None,
        }
    }
}

/// A valid snapshot, ready for match identity and summaries.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub probe: Probe,
    pub game_time: f32,
    pub game_end: Option<GameEnd>,
    pub is_practice: bool,
    pub summary: Summary,
}

/// The `GameEnd` event (spec V2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameEnd {
    pub event_time: f32,
    pub result: Option<GameResult>,
}

/// The recording player's state in this snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Summary {
    /// Lowercase internal key from [`champion_key`].
    pub champion: String,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u16,
    pub creep_score: u16,
    /// Corrected for a disguised Neeko (spec §8.6).
    pub level: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotError {
    Json,
    /// No `activePlayer.riotId`: a replay, spectator mode, or an error body such as the loading screen's 404.
    NoActivePlayer,
    ActivePlayerNotInRoster,
    BadGameTime,
}

/// Validates a snapshot (spec §6.2) and extracts its probe and summary.
pub fn inspect(json: &[u8]) -> Result<Snapshot, SnapshotError> {
    let data = serde_json::from_slice::<AllGameData>(json).map_err(|_| SnapshotError::Json)?;

    let Some(ActivePlayerDto::Player(active)) = data.active_player else {
        return Err(SnapshotError::NoActivePlayer);
    };

    let game_data = data.game_data.ok_or(SnapshotError::Json)?;

    if !game_data.game_time.is_finite() || game_data.game_time < 0.0 {
        return Err(SnapshotError::BadGameTime);
    }

    let me = data
        .all_players
        .iter()
        .find(|p| p.riot_id == active.riot_id)
        .ok_or(SnapshotError::ActivePlayerNotInRoster)?;

    let champion = champion_key(&me.raw_skin_name, &me.champion_name);

    let level = if champion == NEEKO_KEY && me.raw_champion_name != NEEKO_RAW_CHAMPION {
        // Disguised as a non-champion, Neeko reports the disguise's level; her
        // ability points can't exceed her real level.
        active.level.max(active.abilities.points())
    } else {
        active.level
    };

    let summary = Summary {
        champion,
        kills: me.scores.kills,
        deaths: me.scores.deaths,
        assists: me.scores.assists,
        creep_score: me.scores.creep_score,
        level,
    };

    let mut roster = data
        .all_players
        .iter()
        .map(|p| RosterEntry {
            team: p.team.clone(),
            riot_id: p.riot_id.clone(),
            champion: champion_key(&p.raw_skin_name, &p.champion_name),
            skin_id: p.skin_id,
            keystone: p
                .runes
                .as_ref()
                .and_then(|r| r.keystone.as_ref())
                .map(|k| k.id),
            is_bot: p.is_bot,
        })
        .collect::<Vec<_>>();

    roster.sort();

    let events = data
        .events
        .events
        .iter()
        .map(|e| EventKey {
            id: e.id,
            name_hash: fnv1a32(e.name.as_str().as_bytes()),
            time_bits: (e.time as f32).to_bits(),
        })
        .collect();

    let game_end = data
        .events
        .events
        .iter()
        .find(|e| e.name == EventName::GameEnd)
        .map(|e| GameEnd {
            event_time: e.time as f32,
            result: e.result.as_ref().and_then(GameEndResult::result),
        });

    let is_practice = game_data.game_mode == GameMode::PracticeTool;

    Ok(Snapshot {
        probe: Probe {
            active_riot_id: active.riot_id,
            game_mode: game_data.game_mode,
            map_number: game_data.map_number,
            roster,
            game_time: game_data.game_time,
            events,
            riot_game: None,
        },
        game_time: game_data.game_time,
        game_end,
        is_practice,
        summary,
    })
}

/// The real champion as a lowercase internal key (`"neeko"`, `"gnar"`,
/// `"monkeyking"`), independent of locale, Neeko's disguise and Gnar's form.
/// `rawSkinName` keeps the real champion in every case observed (spec §20, V5).
pub fn champion_key(raw_skin_name: &str, champion_name: &str) -> String {
    raw_skin_key(raw_skin_name)
        .unwrap_or(champion_name)
        .to_ascii_lowercase()
}

/// `game_character_skin_displayname_<Key>_<digits>` or `game_character_displayname_<Key>`.
fn raw_skin_key(raw_skin_name: &str) -> Option<&str> {
    if let Some(rest) = raw_skin_name.strip_prefix(SKIN_PREFIX) {
        let (key, skin) = rest.rsplit_once('_')?;

        (!key.is_empty() && !skin.is_empty() && skin.bytes().all(|b| b.is_ascii_digit()))
            .then_some(key)
    } else {
        raw_skin_name
            .strip_prefix(DISPLAY_PREFIX)
            .filter(|key| !key.is_empty())
    }
}

/// 32-bit FNV-1a.
pub fn fnv1a32(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0x811C_9DC5, |hash, &b| {
        (hash ^ b as u32).wrapping_mul(0x0100_0193)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllGameData {
    #[serde(default)]
    active_player: Option<ActivePlayerDto>,
    #[serde(default)]
    all_players: Vec<PlayerDto>,
    #[serde(default)]
    events: EventsDto,
    #[serde(default)]
    game_data: Option<GameDataDto>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ActivePlayerDto {
    Player(ActivePlayer),
    /// e.g. `{"error": "Spectator mode doesn't currently support this feature"}`
    #[allow(dead_code)]
    Other(IgnoredAny),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivePlayer {
    riot_id: String,
    level: u8,
    #[serde(default)]
    abilities: AbilitiesDto,
}

#[derive(Default, Deserialize)]
struct AbilitiesDto {
    #[serde(rename = "Q", default)]
    q: AbilityDto,
    #[serde(rename = "W", default)]
    w: AbilityDto,
    #[serde(rename = "E", default)]
    e: AbilityDto,
    #[serde(rename = "R", default)]
    r: AbilityDto,
}

impl AbilitiesDto {
    fn points(&self) -> u8 {
        [&self.q, &self.w, &self.e, &self.r]
            .iter()
            .map(|a| a.ability_level.unwrap_or(0))
            .fold(0u8, u8::saturating_add)
    }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AbilityDto {
    #[serde(default)]
    ability_level: Option<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerDto {
    riot_id: String,
    #[serde(default)]
    champion_name: String,
    #[serde(default)]
    raw_champion_name: String,
    #[serde(default)]
    raw_skin_name: String,
    #[serde(rename = "skinID", default)]
    skin_id: u16,
    team: Team,
    #[serde(default)]
    is_bot: bool,
    #[serde(default)]
    runes: Option<RunesDto>,
    #[serde(default)]
    scores: ScoresDto,
}

#[derive(Deserialize)]
struct RunesDto {
    #[serde(default)]
    keystone: Option<IdDto>,
}

#[derive(Deserialize)]
struct IdDto {
    id: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScoresDto {
    #[serde(default)]
    kills: u8,
    #[serde(default)]
    deaths: u8,
    #[serde(default)]
    assists: u16,
    #[serde(default)]
    creep_score: u16,
}

#[derive(Default, Deserialize)]
struct EventsDto {
    #[serde(rename = "Events", default)]
    events: Vec<EventDto>,
}

#[derive(Deserialize)]
struct EventDto {
    #[serde(rename = "EventID")]
    id: u32,
    #[serde(rename = "EventName")]
    name: EventName,
    #[serde(rename = "EventTime")]
    time: f64,
    #[serde(rename = "Result", default)]
    result: Option<GameEndResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameDataDto {
    game_mode: GameMode,
    game_time: f32,
    map_number: u8,
}
