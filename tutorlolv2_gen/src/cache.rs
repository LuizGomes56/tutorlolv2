use crate::{Attrs, DamageType, ItemId, RuneId, eval::EvalContext};
use bincode::Encode;
use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

/// A champion can have either melee or ranged damage. Ranged champions
/// often have some damage penalty for items and runes, which are considered
/// by branching over this enum
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttackType {
    Melee,
    Ranged,
}

impl<T: AsRef<str>> From<T> for AttackType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "RANGED" => AttackType::Ranged,
            _ => AttackType::Melee,
        }
    }
}

impl<T: AsRef<str>> From<T> for AdaptativeType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "MAGIC_DAMAGE" => AdaptativeType::Magic,
            _ => AdaptativeType::Physical,
        }
    }
}

#[derive(Encode, Copy, Clone, Serialize, Deserialize)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

/// Represents each playable position or `lane` that a champion can
/// play in the standard gamemode `SummonersRift`, whose definition
/// is [`GameMap::SummonersRift`]. If we don't know a champion's position,
/// it is set to [`Position::Top`].
#[derive(Copy, Clone, Encode, Serialize, Deserialize, Default)]
pub enum Position {
    #[default]
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

impl Position {
    /// Converts a raw string into a [`Position`].
    pub fn from_raw(raw: &str) -> Option<Self> {
        match raw {
            "TOP" => Some(Position::Top),
            "JUNGLE" => Some(Position::Jungle),
            "MIDDLE" => Some(Position::Middle),
            "BOTTOM" => Some(Position::Bottom),
            "SUPPORT" => Some(Position::Support),
            _ => None,
        }
    }
}

/// All possible maps and codes that can be played. Most of them are
/// event maps that may never return to the game, and don't have a
/// deterministic code. [`GameMap::SummonersRift`] is the default map.
#[derive(Default, Encode, Deserialize, Serialize)]
pub enum GameMap {
    #[default]
    SummonersRift,
    Tutorial,
    TwistedTreeline,
    Dominion,
    Aram,
    DarkStar,
    Invasion,
    Project,
    StarGuardian,
    Odyssey,
    NexusBlitz,
    Tft,
    Arena,
    Urf,
}

impl GameMap {
    /// Constant conversion of a [`u8`] into a [`GameMap`] enum,
    /// where the byte represents the code of the current map
    pub const fn from_u8(value: u8) -> Self {
        match value {
            3 => GameMap::Tutorial,
            4 | 10 => GameMap::TwistedTreeline,
            8 => GameMap::Dominion,
            12 | 14 => GameMap::Aram,
            13 => GameMap::Invasion,
            16 => GameMap::DarkStar,
            18 => GameMap::StarGuardian,
            19 => GameMap::Project,
            20 => GameMap::Odyssey,
            21 => GameMap::NexusBlitz,
            22 => GameMap::Tft,
            30 => GameMap::Arena,
            // Unknown
            0xFF => GameMap::Urf,
            /* 1 | 2 | 11 | */ _ => GameMap::SummonersRift,
        }
    }
}

/// A generic metadata holder for [`AbilityId`], [`ItemId`], or [`RuneId`].
/// Contains its damage type, attributes, and which instance of the enum the value is.
#[derive(Copy, Clone, Encode)]
pub struct TypeMetadata<T> {
    /// Represents a variety of values:
    /// - [`AbilityId`]: Which ability key it represents, and its name
    /// - [`ItemId`]: Can be casted to [`usize`] and indexes into [`crate::ITEM_CACHE`]
    /// - [`RuneId`]: Can be casted to [`usize`] and indexes into [`crate::RUNE_CACHE`]
    pub kind: T,
    /// Represents the damage type of the current instance
    pub damage_type: DamageType,
    /// A variety of possible extra attributes the current instance can have.
    /// See [`Attrs`] for more details
    pub attributes: Attrs,
}

/// Definition of a closure that lives in the generated static variables of
/// cache fields, such as [`crate::CHAMPION_CACHE`], [`crate::ITEM_CACHE`], or
/// [`crate::RUNE_CACHE`]. They all receive a [`EvalContext`], which contains
/// more than the necessary information to calculate the damage of some ability,
/// item, passive, or rune, and they return an [`f32`], which represents the calculated
/// damage. All of them must be qualified as `const`, capturing no variables
pub type ConstClosure = fn(&EvalContext) -> f32;

/// Generated data about some champion, held in the static variable [`crate::CHAMPION_CACHE`]
pub struct CachedChampion {
    pub name: &'static str,
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub metadata: &'static [TypeMetadata<AbilityId>],
    pub closures: &'static [ConstClosure],
    pub merge_data: &'static [(usize, usize)],
}

pub struct CachedChampionAbility {
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}

pub struct CachedChampionStatsMap {
    pub flat: f32,
    pub per_level: f32,
}

pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resist: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: f32,
    pub critical_strike_damage: f32,
    pub critical_strike_damage_modifier: f32,
    pub attack_speed_ratio: f32,
    pub attack_range: f32,
    pub aram_damage_taken: f32,
    pub aram_damage_dealt: f32,
    pub urf_damage_taken: f32,
    pub urf_damage_dealt: f32,
}

pub struct CachedItemDamages {
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}

/// Generated data about some item, held in the static variable [`crate::ITEM_CACHE`]
pub struct CachedItem {
    pub gold: u16,
    pub prettified_stats: &'static [StatName],
    pub damage_type: DamageType,
    pub stats: CachedItemStats,
    pub metadata: TypeMetadata<ItemId>,
    pub ranged_closure: [ConstClosure; 2],
    pub melee_closure: [ConstClosure; 2],
    pub attributes: Attrs,
    pub is_simulated: bool,
    pub is_damaging: bool,
    pub riot_id: u32,
}

/// Generated data about some rune, held in the static variable [`crate::RUNE_CACHE`]
pub struct CachedRune {
    pub damage_type: DamageType,
    pub metadata: TypeMetadata<RuneId>,
    pub melee_closure: ConstClosure,
    pub ranged_closure: ConstClosure,
    pub riot_id: u32,
    pub undeclared: bool,
}

pub struct CachedItemStats {
    pub ability_power: f32,
    pub armor: f32,
    pub armor_penetration_percent: f32,
    pub armor_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub health: f32,
    pub lifesteal: f32,
    pub magic_resist: f32,
    pub mana: f32,
    pub movespeed: f32,
    pub omnivamp: f32,
}

/// A constant function that returns `0.0f32`. This is used as a placeholder
/// when some item, rune, or ability doesn't deal any damage, but must be defined
/// in the static variable to meet the requirements of the compiler. In the best-case
/// scenario, this function should never be called, to avoid wasting CPU time with
/// a compile-time known result
pub const fn zero(_: &EvalContext) -> f32 {
    0.0
}
