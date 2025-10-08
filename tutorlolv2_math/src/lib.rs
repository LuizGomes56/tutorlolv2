use bincode::{Decode, Encode};
use tutorlolv2_gen::{AdaptativeType, SIMULATED_ITEMS};

pub mod calc;
pub mod helpers;
pub mod model;
pub mod riot;
pub mod rt;

pub use calc::*;
pub use helpers::*;
pub use model::*;
pub use riot::*;
pub use rt::*;

#[derive(Encode, Decode, Clone, Copy)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

pub const L_SIML: usize = SIMULATED_ITEMS.len();
pub const L_RUNE: usize = 2;
pub const L_ITEM: usize = 4;
pub const L_ABLT: usize = 7;
pub const L_TEAM: usize = 5;
pub const L_PLYR: usize = L_TEAM << 1;
pub const L_MSTR: usize = 7;
pub const L_CENM: usize = 1;
pub const L_TWRD: usize = 6;
pub const L_STCK: usize = 3;

#[derive(Encode)]
pub enum GameMap {
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

impl From<u8> for GameMap {
    fn from(value: u8) -> Self {
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
            0xFF => GameMap::Urf,
            1 | 2 | 11 | _ => GameMap::SummonersRift,
        }
    }
}

pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

pub struct RiotFormulas;

impl RiotFormulas {
    pub const fn stat_growth(base: f32, growth_per_level: f32, level: u8) -> f32 {
        base + growth_per_level * (level as f32 - 1.0) * (0.7025 + 0.0175 * (level as f32 - 1.0))
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
