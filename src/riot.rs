//! This module only have structs related to Riot's port 2999 endpoint,
//! which can be used to deserialize the JSON data to this library's
//! own data types
use crate::{ChampionId, model::*};
use alloc::boxed::Box;
use bincode::{BorrowDecode, Decode, Encode};
use core::num::NonZeroU8;
use serde::{Deserialize, Serialize};

#[cfg(feature = "engine")]
use crate::realtime::RealtimeError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotAbility {
    pub ability_level: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Serialize)]
pub struct RiotAbilities {
    pub q: RiotAbility,
    pub w: RiotAbility,
    pub e: RiotAbility,
    pub r: RiotAbility,
}

impl RiotAbilities {
    /// Returns a struct [`AbilityLevels`] which contains
    /// the levels of `Q`, `W`, `E` and `R` as bytes.
    /// Most champions have their ability levels variying from
    /// 0 to 5, while others may have 6 levels. Note that this
    /// library doesn't panic if the input level is invalid, it
    /// will keep with calculations while the input satisfies the
    /// bounds of type [`u8`]
    pub const fn ability_levels(&self) -> AbilityLevels {
        AbilityLevels {
            q: self.q.ability_level,
            w: self.w.ability_level,
            e: self.e.ability_level,
            r: self.r.ability_level,
        }
    }
}

/// Field `id` is the rune identifier in Riot's API,
/// which has to be translated to enum [`crate::RuneId`],
/// using the function [`crate::RuneId::from_riot_id`]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotGeneralRunes {
    pub id: u32,
}

/// Field `general_runes` is an array, but might be undefined
/// in gamemodes that doesn't allow champions to use runes, such
/// as Arena, defined at [`crate::GameMap::Arena`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotFullRunes {
    pub general_runes: Option<Box<[RiotGeneralRunes]>>,
}

/// All useful fields from the `active_player` object
#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotActivePlayer<'a> {
    pub abilities: RiotAbilities,
    pub champion_stats: PlayerStats,
    pub full_runes: RiotFullRunes,
    pub level: NonZeroU8,
    #[serde(borrow)]
    pub riot_id: &'a str,
}

/// A champion's score. `kills` and `deaths` are of type [`u8`],
/// meaning that their maximum values are 255. Note that field
/// `assists` is of type [`u16`] since it is possible that in
/// ARAM mode ([`crate::GameMap::Aram`]), this many
/// assistences is achieved. However, having more than 255
/// assistences will truncate in the returned [`Scoreboard`] struct
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotScoreboard {
    pub kills: u8,
    pub deaths: u8,
    pub assists: u16,
    pub creep_score: u16,
}

/// Holds the riot identifier of an item, which will
/// be converted to enum [`crate::ItemId`],
/// using function [`crate::ItemId::from_riot_id`]
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize, Serialize,
)]
#[repr(transparent)]
pub struct RiotItems {
    #[serde(rename = "itemID")]
    pub item_id: u32,
}

/// All useful fields from the `all_players` object.
/// Field `champion_name` can be coerced to enum [`crate::ChampionId`]
/// by using the `phf` stored in variable [`crate::CHAMPION_NAME_TO_ID`],
/// which supports all languages. For example, `آتروكس` translates to
/// [`crate::ChampionId::Aatrox`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotAllPlayers<'a> {
    #[serde(borrow)]
    pub champion_name: &'a str,
    /// Not localized, but follows Neeko's disguise and Gnar's form, e.g.
    /// `game_character_displayname_GnarBig` for Mega Gnar
    #[serde(borrow, default)]
    pub raw_champion_name: &'a str,
    /// Keeps the real champion in every form and disguise, e.g.
    /// `game_character_skin_displayname_Neeko_22` while disguised
    #[serde(borrow, default)]
    pub raw_skin_name: &'a str,
    pub items: Box<[RiotItems]>,
    pub level: NonZeroU8,
    #[serde(borrow)]
    pub position: &'a str,
    #[serde(borrow)]
    pub riot_id: &'a str,
    #[serde(borrow)]
    pub team: &'a str,
    pub scores: RiotScoreboard,
}

/// Field `game_time` is in seconds, and field `map_number` can
/// be coerced to enum [`crate::GameMap`] using
/// function [`crate::GameMap::from_u8`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotRealtimeGameData {
    pub game_time: f32,
    pub map_number: u8,
}

/// Captures the fields that can be used to infer how many
/// dragons were killed in that game, by looking at the Events history.
/// Most events contains no meaningful data, we care only about the
/// objective kills such as dragons. Field `dragon_type` is used to
/// determine which dragon was killed, and field `killer_name` is used
/// to dettermine to which team its buffs should be assigned
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize,
)]
#[serde(rename_all = "PascalCase")]
#[derive(Serialize)]
pub struct RealtimeEvent<'a> {
    pub dragon_type: Option<&'a str>,
    pub killer_name: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Serialize)]
pub struct RiotRealtimeEvents<'a> {
    #[serde(borrow)]
    pub events: Box<[RealtimeEvent<'a>]>,
}

/// Struct holding all the useful information provided in Riot's API
#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotRealtime<'a> {
    #[serde(borrow)]
    pub active_player: RiotActivePlayer<'a>,
    pub all_players: Box<[RiotAllPlayers<'a>]>,
    #[serde(borrow)]
    pub events: RiotRealtimeEvents<'a>,
    pub game_data: RiotRealtimeGameData,
}

const SKIN_PREFIX: &str = "game_character_skin_displayname_";
const DISPLAY_PREFIX: &str = "game_character_displayname_";
const NEEKO_SKIN_KEY: &str = "Neeko";
const NEEKO_RAW_CHAMPION: &str = "game_character_displayname_Neeko";
const MEGA_GNAR_RAW_CHAMPION: &str = "game_character_displayname_GnarBig";

/// Internal champion key of `rawSkinName`: `game_character_skin_displayname_<Key>_<digits>`
/// or `game_character_displayname_<Key>`. Same rules as `tutorlolv2_protocol::champion_key`,
/// without lowercasing
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

/// Parses an internal key, retrying in lowercase because the game's casing
/// is not consistent (`FiddleSticks`)
fn parse_key(key: &str) -> Option<ChampionId> {
    if let Ok(id) = key.parse() {
        return Some(id);
    }
    let mut buffer = [0u8; 32];
    let lower = buffer.get_mut(..key.len())?;

    lower.copy_from_slice(key.as_bytes());
    lower.make_ascii_lowercase();

    core::str::from_utf8(lower).ok()?.parse().ok()
}

impl RiotAllPlayers<'_> {
    /// The real champion. Extra work happens only for the champions that can
    /// transform: Neeko can wear another champion's name, which would parse
    /// successfully as the wrong champion, and names like `"Mega Gnar"` don't
    /// parse at all
    pub fn champion_id(&self) -> ChampionId {
        let skin_key = raw_skin_key(self.raw_skin_name);

        if skin_key == Some(NEEKO_SKIN_KEY) {
            return ChampionId::Neeko;
        }

        self.champion_name
            .parse()
            .ok()
            .or_else(|| skin_key.and_then(parse_key))
            // Unknown entity names used to mean a Neeko disguise
            .unwrap_or(ChampionId::Neeko)
    }

    /// Whether this Gnar is in Mega form, in any client language
    pub fn is_mega_gnar(&self, id: ChampionId) -> bool {
        id == ChampionId::Gnar && self.raw_champion_name == MEGA_GNAR_RAW_CHAMPION
    }

    /// Whether this Neeko is disguised. Disguised as a non-champion, the API
    /// reports the disguise's level instead of hers
    pub fn is_disguised_neeko(&self, id: ChampionId) -> bool {
        id == ChampionId::Neeko && self.raw_champion_name != NEEKO_RAW_CHAMPION
    }
}

#[cfg(feature = "engine")]
impl RiotRealtime<'_> {
    pub fn eval(&self) -> Result<Realtime<'_>, RealtimeError<'_>> {
        crate::realtime::realtime(self)
    }
}
