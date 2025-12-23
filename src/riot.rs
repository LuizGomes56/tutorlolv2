//! This module only have structs related to Riot's port 2999 endpoint,
//! which can be used to deserialize the JSON data to this library's
//! own data types

use crate::model::*;
use alloc::boxed::Box;
use bincode::{BorrowDecode, Decode, Encode};
use serde::{Deserialize, Serialize};

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
    pub const fn get_ability_levels(&self) -> AbilityLevels {
        AbilityLevels {
            q: self.q.ability_level,
            w: self.w.ability_level,
            e: self.e.ability_level,
            r: self.r.ability_level,
        }
    }
}

/// Field `id` is the rune identifier in Riot's API,
/// which has to be translated to enum [`tutorlolv2_gen::RuneId`],
/// using the function [`tutorlolv2_gen::RuneId::from_riot_id`]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotGeneralRunes {
    pub id: u32,
}

/// Field `general_runes` is an array, but might be undefined
/// in gamemodes that doesn't allow champions to use runes, such
/// as Arena, defined at [`tutorlolv2_gen::GameMap::Arena`]
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
    pub champion_stats: Stats<f32>,
    pub full_runes: RiotFullRunes,
    pub level: u8,
    #[serde(borrow)]
    pub riot_id: &'a str,
}

/// A champion's score. `kills` and `deaths` are of type [`u8`],
/// meaning that their maximum values are 255. Note that field
/// `assists` is of type [`u16`] since it is possible that in
/// ARAM mode ([`tutorlolv2_gen::GameMap::Aram`]), this many
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
/// be converted to enum [`tutorlolv2_gen::ItemId`],
/// using function [`tutorlolv2_gen::ItemId::from_riot_id`]
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Deserialize, Serialize,
)]
#[repr(transparent)]
pub struct RiotItems {
    #[serde(rename = "itemID")]
    pub item_id: u32,
}

/// All useful fields from the `all_players` object.
/// Field `champion_name` can be coerced to enum [`tutorlolv2_gen::ChampionId`]
/// by using the `phf` stored in variable [`tutorlolv2_gen::CHAMPION_NAME_TO_ID`],
/// which supports all languages. For example, `آتروكس` translates to
/// [`tutorlolv2_gen::ChampionId::Aatrox`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct RiotAllPlayers<'a> {
    #[serde(borrow)]
    pub champion_name: &'a str,
    pub items: Box<[RiotItems]>,
    pub level: u8,
    #[serde(borrow)]
    pub position: &'a str,
    #[serde(borrow)]
    pub riot_id: &'a str,
    #[serde(borrow)]
    pub team: &'a str,
    pub scores: RiotScoreboard,
}

/// Field `game_time` is in seconds, and field `map_number` can
/// be coerced to enum [`tutorlolv2_gen::GameMap`] using
/// function [`tutorlolv2_gen::GameMap::from_u8`]
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
