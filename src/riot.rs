use crate::model::*;
use alloc::boxed::Box;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotAbility {
    pub ability_level: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "UPPERCASE")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

/// Holds all champion stats provided by Riot's API.
/// Generic parameter `T` is intended to be a numeric type,
/// like [`f32`], [`f64`], or [`i32`]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Stats<T> {
    pub ability_power: T,
    pub armor: T,
    #[cfg_attr(
        any(feature = "serde", feature = "livegame"),
        serde(rename = "physicalLethality")
    )]
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
    #[cfg_attr(
        any(feature = "serde", feature = "livegame"),
        serde(rename = "maxHealth")
    )]
    pub health: T,
    #[cfg_attr(
        any(feature = "serde", feature = "livegame"),
        serde(rename = "resourceMax")
    )]
    pub mana: T,
    #[cfg_attr(
        any(feature = "serde", feature = "livegame"),
        serde(rename = "resourceValue")
    )]
    pub current_mana: T,
}

impl Stats<f32> {
    /// Returns a new struct [`Stats`] with the same original values except the ones
    /// that involve percent penetration, which are resolved and converted to the
    /// `[0.0, 100.0]` range used in this library
    pub const fn base100(&self) -> Self {
        Self {
            armor_penetration_percent: (1.0 - self.armor_penetration_percent).clamp(0.0, 1.0)
                * 100.0,
            magic_penetration_percent: (1.0 - self.magic_penetration_percent).clamp(0.0, 1.0)
                * 100.0,
            ..*self
        }
    }
}

/// Field `id` is the rune identifier in Riot's API,
/// which has to be translated to enum [`tutorlolv2_gen::RuneId`],
/// using the function [`tutorlolv2_gen::RuneId::from_riot_id`]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotGeneralRunes {
    pub id: u32,
}

/// Field `general_runes` is an array, but might be undefined
/// in gamemodes that doesn't allow champions to use runes, such
/// as Arena, defined at [`tutorlolv2_gen::GameMap::Arena`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotFullRunes {
    pub general_runes: Option<Box<[RiotGeneralRunes]>>,
}

/// All useful fields from the `active_player` object
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotActivePlayer<'a> {
    pub abilities: RiotAbilities,
    pub champion_stats: Stats<f32>,
    pub full_runes: RiotFullRunes,
    pub level: u8,
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub riot_id: &'a str,
}

/// A champion's score. `kills` and `deaths` are of type [`u8`],
/// meaning that their maximum values are 255. Note that field
/// `assists` is of type [`u16`] since it is possible that in
/// ARAM mode ([`tutorlolv2_gen::GameMap::Aram`]), this many
/// assistences is achieved. However, having more than 255
/// assistences will truncate in the returned [`Scoreboard`] struct
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotScoreboard {
    pub kills: u8,
    pub deaths: u8,
    pub assists: u16,
    pub creep_score: u16,
}

/// Holds the riot identifier of an item, which will
/// be converted to enum [`tutorlolv2_gen::ItemId`],
/// using function [`tutorlolv2_gen::ItemId::from_riot_id`]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(transparent)]
pub struct RiotItems {
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(rename = "itemID"))]
    pub item_id: u32,
}

/// All useful fields from the `all_players` object.
/// Field `champion_name` can be coerced to enum [`tutorlolv2_gen::ChampionId`]
/// by using the `phf` stored in variable [`tutorlolv2_gen::CHAMPION_NAME_TO_ID`],
/// which supports all languages. For example, `آتروكس` translates to
/// [`tutorlolv2_gen::ChampionId::Aatrox`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotAllPlayers<'a> {
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub champion_name: &'a str,
    pub items: Box<[RiotItems]>,
    pub level: u8,
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub position: &'a str,
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub riot_id: &'a str,
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub team: &'a str,
    pub scores: RiotScoreboard,
}

/// Field `game_time` is in seconds, and field `map_number` can
/// be coerced to enum [`tutorlolv2_gen::GameMap`] using
/// function [`tutorlolv2_gen::GameMap::from_u8`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "PascalCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RealtimeEvent<'a> {
    pub dragon_type: Option<&'a str>,
    pub killer_name: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "PascalCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotRealtimeEvents<'a> {
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub events: Box<[RealtimeEvent<'a>]>,
}

/// Struct holding all the useful information provided in Riot's API
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(
    any(feature = "livegame", feature = "serde"),
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RiotRealtime<'a> {
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub active_player: RiotActivePlayer<'a>,
    pub all_players: Box<[RiotAllPlayers<'a>]>,
    #[cfg_attr(any(feature = "serde", feature = "livegame"), serde(borrow))]
    pub events: RiotRealtimeEvents<'a>,
    pub game_data: RiotRealtimeGameData,
}
