use crate::*;
use bincode::{Decode, Encode};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAbility {
    pub ability_level: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
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
    pub fn get_ability_levels(&self) -> AbilityLevels {
        AbilityLevels {
            q: self.q.ability_level,
            w: self.w.ability_level,
            e: self.e.ability_level,
            r: self.r.ability_level,
        }
    }
}

/// Holds all champion stats provided by Riot's API.
/// Generic parameter `T` is intended to be a numeric type,
/// like [`f32`], [`f64`], or [`i32`]. This struct is used
/// as both input and output, which means that it implements
/// [`Encode`], [`Decode`], and [`Deserialize`].
#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode)]
#[serde(rename_all = "camelCase")]
pub struct Stats<T> {
    pub ability_power: T,
    pub armor: T,
    #[serde(rename = "physicalLethality")]
    pub armor_penetration_flat: T,
    pub armor_penetration_percent: T,
    pub attack_damage: T,
    pub attack_range: T,
    pub attack_speed: T,
    pub crit_chance: T,
    pub crit_damage: T,
    pub current_health: T,
    pub magic_penetration_flat: T,
    pub magic_penetration_percent: T,
    pub magic_resist: T,
    #[serde(rename = "maxHealth")]
    pub health: T,
    #[serde(rename = "resourceMax")]
    pub mana: T,
    #[serde(rename = "resourceValue")]
    pub current_mana: T,
}

/// Field `id` is the rune identifier in Riot's API,
/// which has to be translated to enum [`tutorlolv2_gen::RuneId`],
/// using the function [`tutorlolv2_gen::RuneId::from_riot_id`]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotGeneralRunes {
    pub id: u32,
}

/// Field `general_runes` is an array, but might be undefined
/// in gamemodes that doesn't allow champions to use runes, such
/// as Arena, defined at [`tutorlolv2_gen::GameMap::Arena`]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotFullRunes {
    pub general_runes: Option<Box<[RiotGeneralRunes]>>,
}

/// All useful fields from the `active_player` object
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotScoreboard {
    pub kills: u8,
    pub deaths: u8,
    pub assists: u16,
    pub creep_score: u16,
}

/// Holds the riot identifier of an item, which will
/// be converted to enum [`tutorlolv2_gen::ItemId`],
/// using function [`tutorlolv2_gen::ItemId::from_riot_id`]
#[derive(Deserialize)]
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
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RealtimeEvent<'a> {
    pub dragon_type: Option<&'a str>,
    pub killer_name: Option<&'a str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RiotRealtimeEvents<'a> {
    #[serde(borrow)]
    pub events: Box<[RealtimeEvent<'a>]>,
}

/// Struct holding all the useful information provided in Riot's API
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtime<'a> {
    #[serde(borrow)]
    pub active_player: RiotActivePlayer<'a>,
    pub all_players: Box<[RiotAllPlayers<'a>]>,
    #[serde(borrow)]
    pub events: RiotRealtimeEvents<'a>,
    pub game_data: RiotRealtimeGameData,
}
