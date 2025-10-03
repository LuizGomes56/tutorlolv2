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
