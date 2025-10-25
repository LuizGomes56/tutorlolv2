use bincode::{Decode, Encode};
use tutorlolv2_gen::{
    AdaptativeType, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, INTERNAL_RUNES, SIMULATED_ITEMS,
};

pub mod calculator;
pub mod helpers;
pub mod model;
pub mod realtime;
pub mod riot;

pub use calculator::*;
pub use helpers::*;
pub use model::*;
pub use realtime::*;
pub use riot::*;

#[derive(Encode, Decode, Clone, Copy)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

pub const NUMBER_OF_CHAMPIONS: usize = INTERNAL_CHAMPIONS.len();
pub const NUMBER_OF_ITEMS: usize = INTERNAL_ITEMS.len();
pub const NUMBER_OF_RUNES: usize = INTERNAL_RUNES.len();
pub const NUMBER_OF_ABILITIES: usize = {
    let mut i = 0;
    let mut sum = 0;
    while i < NUMBER_OF_CHAMPIONS {
        let data = INTERNAL_CHAMPIONS[i];
        sum += data.closures.len();
        i += 1;
    }
    sum
};
pub const L_TEAM: usize = 5;
pub const L_PLYR: usize = L_TEAM << 1;
pub const L_MSTR: usize = 7;
pub const L_CENM: usize = 1;
pub const L_TWRD: usize = 6;
pub const L_SIML: usize = SIMULATED_ITEMS.len();
pub const L_RUNE: usize = 2;
pub const L_ITEM: usize = 5;
pub const L_ABLT: usize = NUMBER_OF_ABILITIES / NUMBER_OF_CHAMPIONS;

pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

pub struct RiotFormulas;

impl RiotFormulas {
    const fn growth(level: u8) -> f32 {
        let factor = level as f32 - 1.0;
        factor * (0.7025 + 0.0175 * factor)
    }

    pub const fn stat_growth(base: f32, per_level: f32, growth_factor: f32) -> f32 {
        base + per_level * growth_factor
    }

    pub fn percent_value(from_vec: &[f32]) -> f32 {
        from_vec
            .iter()
            .map(|value: &f32| 100.0 - value)
            .product::<f32>()
            / 10f32.powi((from_vec.len() << 1) as i32)
    }

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

    pub const fn adaptative_type(attack_damage: f32, ability_power: f32) -> AdaptativeType {
        if 0.35 * attack_damage >= 0.2 * ability_power {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }
}
