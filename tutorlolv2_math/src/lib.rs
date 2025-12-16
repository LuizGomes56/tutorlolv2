#![no_std]
use bincode::{Decode, Encode};
use tutorlolv2_gen::{AdaptativeType, NUMBER_OF_SIMULATED_ITEMS};

extern crate alloc;

pub(crate) use alloc::boxed::Box;

pub mod calculator;
pub mod const_eval;
pub mod helpers;
mod math_test;
pub mod model;
pub mod realtime;
pub mod riot;

pub use calculator::*;
pub use const_eval::*;
pub use helpers::*;
pub use model::*;
pub use realtime::*;
pub use riot::*;

/// Holds the levels of the abilities of a champion
#[derive(Clone, Copy, Debug, Decode, Encode)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

/// Exact number of resistence variations for jungle monsters
pub const L_MSTR: usize = 7;

/// Number of different plates a tower can have. Each tower can have `0..=5` plates
pub const L_TWRD: usize = 6;

/// Alias to the number of simulated items
pub const L_SIML: usize = NUMBER_OF_SIMULATED_ITEMS;

/// Holds the real resistence that a enemy has, after applying the appropriate
/// armor or magic penetrations that the current player has, and other buffs or
/// penalties applied. Field `modifier` is a value between `0.0` and `1.0` that
/// represents the number that will be multiplied by the raw damage to obtain
/// the precise final damage
#[derive(Clone, Copy, Debug)]
pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

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
