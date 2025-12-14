use bincode::{Decode, Encode};
use tutorlolv2_gen::{
    AdaptativeType, NUMBER_OF_ABILITIES, NUMBER_OF_CHAMPIONS, NUMBER_OF_SIMULATED_ITEMS,
};

pub mod calculator;
pub mod const_impl;
pub mod helpers;
pub mod model;
pub mod realtime;
pub mod riot;

pub use calculator::*;
pub use const_impl::*;
pub use helpers::*;
pub use model::*;
pub use realtime::*;
pub use riot::*;

/// Holds the levels of the abilities of a champion
#[derive(Encode, Decode, Clone, Copy)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

/// Expected array length to a team in the `SummonersRift` gamemode
pub const L_TEAM: usize = 5;

/// Expected number of players per game
pub const L_PLYR: usize = L_TEAM << 1;

/// Exact number of resistence variations for jungle monsters
pub const L_MSTR: usize = 7;

/// Most common number of enemies in a regular input for the calculator section
pub const L_CENM: usize = 1;

/// Number of different plates a tower can have. Each tower can have `0..=5` plates
pub const L_TWRD: usize = 6;

/// Alias to the number of simulated items
pub const L_SIML: usize = NUMBER_OF_SIMULATED_ITEMS;

/// Most common number of damaging runes the current player has
pub const L_RUNE: usize = 2;

/// Upper bound of the number of damaging items the current player usually have
pub const L_ITEM: usize = 5;

/// Estimate of how many abilities it is more common that a champion has. If
/// it happens to have more, the whole data will spill to the heap to avoid
/// stack overflows.
pub const L_ABLT: usize = NUMBER_OF_ABILITIES / NUMBER_OF_CHAMPIONS;

/// Holds the real resistence that a enemy has, after applying the appropriate
/// armor or magic penetrations that the current player has, and other buffs or
/// penalties applied. Field `modifier` is a value between `0.0` and `1.0` that
/// represents the number that will be multiplied by the raw damage to obtain
/// the precise final damage
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
        let mut prod = 1.0_f32;

        while i < from_vec.len() {
            prod *= 100.0_f32 - from_vec[i];
            i += 1;
        }

        let mut div = 1.0_f32;
        let mut j = 0;
        let exp = (from_vec.len() as u32) << 1;

        while j < exp {
            div *= 10.0_f32;
            j += 1;
        }

        prod / div
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
        let real_val = (percent_pen * resist - flat_pen).max(if accept_negatives {
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
