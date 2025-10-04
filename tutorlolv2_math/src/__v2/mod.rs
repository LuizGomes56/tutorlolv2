use bincode::{Decode, Encode};
use tutorlolv2_gen::SIMULATED_ITEMS;

pub mod arena;
pub mod riot;
pub mod stack;
pub mod stdalloc;

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
    ARAM,
    DarkStar,
    Invasion,
    Project,
    StarGuardian,
    Odyssey,
    NexusBlitz,
    TFT,
    Arena,
    URF,
}

impl From<u8> for GameMap {
    fn from(value: u8) -> Self {
        match value {
            1 | 2 | 11 => GameMap::SummonersRift,
            3 => GameMap::Tutorial,
            4 | 10 => GameMap::TwistedTreeline,
            8 => GameMap::Dominion,
            12 | 14 => GameMap::ARAM,
            13 => GameMap::Invasion,
            16 => GameMap::DarkStar,
            18 => GameMap::StarGuardian,
            19 => GameMap::Project,
            20 => GameMap::Odyssey,
            21 => GameMap::NexusBlitz,
            22 => GameMap::TFT,
            30 => GameMap::Arena,
            0xFF => GameMap::URF,
            _ => GameMap::SummonersRift,
        }
    }
}
