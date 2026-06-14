use super::*;
include!(concat!(env!("OUT_DIR"), "/docs.rs"));

use super::{champions::ChampionId, items::ItemId, runes::RuneId};

pub static RAW_BLOCK: &str = include_str!(concat!(env!("OUT_DIR"), "/docs.txt"));
pub const RAW_BLOCK_LEN: usize = RAW_BLOCK.len();

// const BR_BLOCK: &[u8] = include_bytes!("block.br");
// pub static mut BLOCK: &[u8] = BR_BLOCK;

// pub const BLOCK_LEN: usize = BR_BLOCK.len();
