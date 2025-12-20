#![no_std]
//! Declaration of structs, constants, enums and methods that are
//! used in all other modules of this library.
//!
//! ### Features
//! - `bincode` - implements `Encode` `Decode` or `BorrowDecode` traits
//! to all eligible structs and enums
//! - `serde` - implements `Serialize` `Deserialize` traits to all eligible
//! structs and enums. Structs that have `'static` lifetimes do not implement
//! `Deserialize`

extern crate alloc;

use alloc::boxed::Box;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tutorlolv2_gen::*;

#[cfg(feature = "recv")]
mod recv;

#[cfg(feature = "send")]
mod send;

#[cfg(feature = "recv")]
pub use recv::*;

#[cfg(feature = "send")]
pub use send::*;

/// Exact number of resistence variations for jungle monsters
pub const L_MSTR: usize = 7;

/// Number of different plates a tower can have. Each tower can have `0..=5` plates
pub const L_TWRD: usize = 6;

/// Holds all champion stats provided by Riot's API.
/// Generic parameter `T` is intended to be a numeric type,
/// like [`f32`], [`f64`], or [`i32`]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Encode, Decode)]
#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
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

/// Enum that defines the team of some player.
/// - `CHAOS` is converted to [`Team::Red`],
/// - `ORDER` and any other variant matches [`Team::Blue`]
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub enum Team {
    Blue,
    Red,
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        match value {
            "ORDER" => Team::Blue,
            _ => Team::Red,
        }
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct RangeDamage {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

impl RangeDamage {
    /// Takes an attribute and adds the appropriate damage value based on it.
    /// [`Attrs::Onhit`] and any other variant like this one indicates that the
    /// current instance deals a constant onhit damage, not being defined as
    /// minimum-only, or maximum-only
    pub const fn inc_attr(&mut self, attr: Attrs, damage: i32) {
        match attr {
            Attrs::OnhitMin | Attrs::AreaOnhitMin => self.minimum_damage += damage,
            Attrs::OnhitMax | Attrs::AreaOnhitMax => self.maximum_damage += damage,
            Attrs::Onhit | Attrs::AreaOnhit => {
                self.minimum_damage += damage;
                self.maximum_damage += damage;
            }
            _ => {}
        };
    }
}

/// Struct holding the core champion stats of a player, where `T` is a
/// numeric type
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct BasicStats<T> {
    pub armor: T,
    pub health: T,
    pub attack_damage: T,
    pub magic_resist: T,
    pub mana: T,
}

/// Holds the damage of the basic attack, critical strike damage, and onhits
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct Attacks {
    /// Damage of the basic attack hit
    pub basic_attack: i32,
    /// Damage of the critical strike. For most champions, it represents a
    /// multipler of 1.75x the damage of the basic attack.
    pub critical_strike: i32,
    /// The onhit damage variant, containing the necessary information to
    /// display it as a range `{min} - {max}`
    pub onhit_damage: RangeDamage,
}

/// Struct that holds the values that can reduce the enemie's armor or
/// magic resistence benefits
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct ResistShred {
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
}

impl ResistShred {
    /// Creates a [`ResistShred`] struct from the current player stats. This API
    /// uses percent penetration as a value in range `[0.0, 100.0]`. Values over
    /// `100.0` will be clamped
    pub const fn new(stats: &Stats<f32>) -> Self {
        let Stats {
            armor_penetration_flat,
            armor_penetration_percent,
            magic_penetration_flat,
            magic_penetration_percent,
            ..
        } = *stats;
        Self {
            armor_penetration_flat: armor_penetration_flat,
            armor_penetration_percent: armor_penetration_percent.clamp(0.0, 100.0),
            magic_penetration_flat: magic_penetration_flat,
            magic_penetration_percent: magic_penetration_percent.clamp(0.0, 100.0),
        }
    }
}

/// Holds the most simple stats that need to be used to calculate
/// the damage against this enemy. Note that it is similar to struct
/// [`BasicStats`], but without the `attack_damage` and `mana` fields,
/// which are fields that do not quantify any damage reduction the enemy
/// champion may take. Generic parameter `T` is supposed to be a numeric type
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct SimpleStats<T> {
    pub armor: T,
    pub health: T,
    pub magic_resist: T,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: Box<[i32]>,
    pub items: Box<[i32]>,
    pub runes: Box<[i32]>,
}

/// Wrapper around the type [`u32`], whose first [`Self::DISC_BITS`] are used to
/// identify the enum type of the current value, which is either [`ItemId`] or [`RuneId`],
/// and the remaining [`Self::VAL_BITS`] are used to store the actual number of stacks held
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct ValueException(u32);

impl ValueException {
    pub const DISC_BITS: u32 = Self::find_disc_bits(NUMBER_OF_ITEMS as u32, NUMBER_OF_RUNES as u32);
    pub const VAL_BITS: u32 = 32 - Self::DISC_BITS;
    pub const VAL_MASK: u32 = (1u32 << Self::VAL_BITS) - 1;
    pub const DISC_MASK: u32 = !Self::VAL_MASK;
    pub const DISC_LOW_MASK: u32 = (1u32 << Self::DISC_BITS) - 1;

    /// Returns a u32 with the number of leading zeros of the maximum between `a` and `b`
    const fn find_disc_bits(a: u32, b: u32) -> u32 {
        u32::BITS - if a > b { a } else { b }.leading_zeros()
    }

    /// Returns how many stacks are stored. Note that it returns an [`u32`] but whose
    /// maximum value is [`Self::VAL_MASK`]
    pub const fn stacks(&self) -> u32 {
        self.0 & Self::VAL_MASK
    }

    /// Returns an [`u16`], which is large enough to represent both [`ItemId`] and [`RuneId`]
    /// enums. This value is taken from the first [`Self::DISC_BITS`] bits
    const fn enum_id(&self) -> u16 {
        ((self.0 >> Self::VAL_BITS) & ((1u32 << Self::DISC_BITS) - 1)) as u16
    }

    /// Returns if the current value is a [`RuneId`]
    pub const fn get_rune_id(&self) -> Option<RuneId> {
        RuneId::from_u8(self.enum_id() as u8)
    }

    /// Returns if the current value is an [`ItemId`]
    pub const fn get_item_id(&self) -> Option<ItemId> {
        ItemId::from_u16(self.enum_id())
    }

    /// If the value to be stored is greater than [`Self::VAL_MASK`],
    /// the value is truncated
    const fn truncate_value(v: u32) -> u32 {
        v & Self::VAL_MASK
    }

    /// Creates a new instance of [`Self`] from a [`RuneId`] and a number of stacks
    pub const fn pack_rune_id(r: RuneId, v: u32) -> Self {
        let disc = (r as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }

    /// Creates a new instance of [`Self`] from an [`ItemId`] and a number of stacks
    pub const fn pack_item_id(i: ItemId, v: u32) -> Self {
        let disc = (i as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }
}

/// Holds the number of dragons and their types, associated to the ally or enemy team.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct Dragons {
    pub ally_fire_dragons: u16,
    pub ally_earth_dragons: u16,
    pub ally_chemtech_dragons: u16,
    pub enemy_earth_dragons: u16,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Box<[InputMinData<SimpleStats<i32>>]>,
    pub dragons: Dragons,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct InputActivePlayer {
    pub runes: Box<[RuneId]>,
    pub rune_exceptions: Box<[ValueException]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData<Stats<i32>>,
}

/// Minimum required data to qualify a valid enemy player, and calculate
/// damages against this target. Field `stats` is required, but if `infer_stats`
/// is set to true, the enemy's stats will be inferred and this field will be ignored.
/// The same happens with `is_mega_gnar`, which can be set to true, but will only
/// have effect if field `champion_id` is also of type [`ChampionId::Gnar`].
/// Field `stacks` is useless if the associated champion does not have any special
/// characteristics that are related to stack-scaling
#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct InputMinData<T> {
    pub stats: T,
    pub items: Box<[ItemId]>,
    pub item_exceptions: Box<[ValueException]>,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

/// Returned data by the function [`crate::calculator::calculator`]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

/// Holds values that will be multiplied by all damages, depending on their
/// damage types, defined by the metadata [`tutorlolv2_gen::DamageType`]. Note
/// that the value `1.0` means no modifiers, and `global_mod` is applied regardless
/// of the damage type provided
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct DamageModifiers {
    pub physical_mod: f32,
    pub magic_mod: f32,
    pub true_mod: f32,
    pub global_mod: f32,
}

impl DamageModifiers {
    pub const fn modifier(&self, damage_type: DamageType) -> f32 {
        self.global_mod
            * match damage_type {
                DamageType::Physical => self.physical_mod,
                DamageType::Magic => self.magic_mod,
                DamageType::True => self.true_mod,
                _ => 1.0,
            }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct Modifiers {
    pub damages: DamageModifiers,
    pub abilities: AbilityModifiers,
}

/// Holds float values that will be multiplied by the damages of each ability
/// depending on their letters, which can be obtained through the method
/// [`AbilityId::as_char`] with simple branching. Values of `1.0` mean no modifiers
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct AbilityModifiers {
    pub q: f32,
    pub w: f32,
    pub e: f32,
    pub r: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct OutputCurrentPlayer {
    pub current_stats: Stats<i32>,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: Box<[i32]>,
    pub items: Box<[i32]>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct OutputGame {
    pub monster_damages: [MonsterDamage; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[OutputEnemy]>,
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: &'static [TypeMetadata<AbilityId>],
    pub abilities_to_merge: &'static [(usize, usize)],
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}

/// Holds the levels of the abilities of a champion
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

/// Holds the real resistence that a enemy has, after applying the appropriate
/// armor or magic penetrations that the current player has, and other buffs or
/// penalties applied. Field `modifier` is a value between `0.0` and `1.0` that
/// represents the number that will be multiplied by the raw damage to obtain
/// the precise final damage
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

/// ZST with constant methods to calculate some values for the game
/// League of Legends. These formulas may change over time
pub struct RiotFormulas;

impl RiotFormulas {
    /// Constant growth formula used to calculate base-stats and other scaling
    /// related fields
    pub const fn growth(level: u8) -> f32 {
        let factor = level as f32 - 1.0;
        factor * (0.7025 + 0.0175 * factor)
    }

    /// Given the base stats and growth factors, return a number after applying the formula
    pub const fn stat_growth(base: f32, per_level: f32, growth_factor: f32) -> f32 {
        base + per_level * growth_factor
    }

    /// Takes in a slice of numbers between `0.0` and `100.0` and treats them as percentage
    /// armor or magic penetration, returning a number that represents the final penetration
    /// value of the current player. This happens because two `30%` penetrations do not sum
    /// up to `60%` directly, instead they return `51%` penetration.
    pub const fn percent_value(from_vec: &[f32]) -> f32 {
        let mut i = 0;
        let mut prod = 1f32;
        let mut div = 1f32;

        while i < from_vec.len() {
            div *= 10f32;
            prod *= 100f32 - from_vec[i];
            i += 1;
        }

        100f32 - prod / div
    }

    /// Returns the real resistence value in a struct [`ResistValue`], taking into account
    /// if it can go below zero or not. If `accept_negatives` is set to `false`, then the
    /// `real` field will be clamped to `0.0`, and the modifier will be set to `1.0`, meaning
    /// that the enemy would have zero resistences after applying the percentage and flat
    /// penetration values
    pub const fn real_resist(
        percent_pen: f32,
        flat_pen: f32,
        resist: f32,
        accept_negatives: bool,
    ) -> ResistValue {
        let real_val = ((1.0 - percent_pen / 100.0) * resist - flat_pen).max(if accept_negatives {
            f32::NEG_INFINITY
        } else {
            0.0
        });
        let modf_val = 100.0 / (100.0 + real_val);
        ResistValue {
            real: real_val,
            modifier: modf_val,
        }
    }

    /// Returns an [`i32`] that represents the damage against some turret
    pub const fn tower_damage(
        plates: f32,
        base_attack_damage: f32,
        bonus_attack_damage: f32,
        ability_power: f32,
        pen_percent: f32,
        pen_flat: f32,
    ) -> i32 {
        (base_attack_damage
            + bonus_attack_damage
            + ability_power
                * 0.6
                * (100.0 / (140.0 + (-25.0 + 50.0 * plates) * pen_percent - pen_flat)))
            as i32
    }

    /// Returns the adaptative type of the current player, given its bonus attack_damage
    /// and ability_power. If they tie, it will return `None`, wich should be unwraped to
    /// the default adaptative type of the current champion.
    /// ```rs
    /// let adaptative_type = RiotFormulas::adaptative_type(
    ///     current_player_bonus_stats.attack_damage,
    ///     champion_stats.ability_power,
    /// )
    /// .unwrap_or(current_player_cache.adaptative_type);
    /// ```
    pub const fn adaptative_type(
        bonus_attack_damage: f32,
        ability_power: f32,
    ) -> Option<AdaptativeType> {
        let lhs = 0.35 * bonus_attack_damage;
        let rhs = 0.2 * ability_power;

        if lhs == rhs {
            None
        } else if lhs > rhs {
            Some(AdaptativeType::Physical)
        } else {
            Some(AdaptativeType::Magic)
        }
    }
}

/// Implements traits [`From<T<f32>>`] and [`From<T<i32>>`] for some struct
/// in which all of its fields are numeric types with simple and
/// deterministic castings from between [`f32`] and [`i32`]
macro_rules! impl_cast_from {
    ($stru:ident, $($fields:ident),*) => {
        impl $stru<f32> {
            pub const fn from_i32(value: &$stru<i32>) -> Self {
                $stru {
                    $($fields: value.$fields as f32),*
                }
            }
        }

        impl $stru<i32> {
            pub const fn from_f32(value: &$stru<f32>) -> Self {
                $stru {
                    $($fields: value.$fields as i32),*
                }
            }
        }

        impl From<$stru<f32>> for $stru<i32> {
            fn from(value: $stru<f32>) -> Self {
                $stru::from_f32(&value)
            }
        }

        impl From<$stru<i32>> for $stru<f32> {
            fn from(value: $stru<i32>) -> Self {
                $stru::from_i32(&value)
            }
        }

        impl From<&$stru<f32>> for $stru<i32> {
            fn from(value: &$stru<f32>) -> Self {
                $stru::from_f32(value)
            }
        }

        impl From<&$stru<i32>> for $stru<f32> {
            fn from(value: &$stru<i32>) -> Self {
                $stru::from_i32(value)
            }
        }
    };
}

impl_cast_from!(SimpleStats, health, armor, magic_resist);
impl_cast_from!(BasicStats, armor, health, attack_damage, magic_resist, mana);
impl_cast_from!(
    Stats,
    ability_power,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_range,
    attack_speed,
    crit_chance,
    crit_damage,
    current_health,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    health,
    mana,
    current_mana
);

/// Implements trait [`Default`] and a constant method [`Self::default`] for some struct.
/// The first argument of this macro is the struct in which these functions will be generated,
/// the second one is the default value. Often `0` or `1`, and the third one is the type
/// of the default value, that could not be inferred. This macro does not need to know the
/// fields of the target struct since it uses [`core::mem::transmute`]
macro_rules! impl_default {
    ($ty:ty, $init:literal, $typedef:ty) => {
        impl $ty {
            pub const fn default() -> Self {
                unsafe { core::mem::transmute([$init; size_of::<$ty>() / size_of::<$typedef>()]) }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                <$ty>::default()
            }
        }
    };
}

impl_default!(Stats<f32>, 0.0f32, f32);
impl_default!(SimpleStats<f32>, 0.0f32, f32);
impl_default!(BasicStats<f32>, 0.0f32, f32);
impl_default!(DamageModifiers, 1.0f32, f32);
impl_default!(AbilityModifiers, 1.0f32, f32);
impl_default!(Modifiers, 1.0f32, f32);
impl_default!(Dragons, 0u8, u8);
impl_default!(RangeDamage, 0i32, i32);
