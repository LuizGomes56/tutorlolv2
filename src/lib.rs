#![no_std]

pub use tutorlolv2_gen::{
    AdaptativeType, AttackType, Attrs, ChampionId, DamageType, EvalContext, GameMap, ItemId,
    Position, RuneId,
};

pub use calculator::calculator;
pub use realtime::realtime;

pub mod constants {
    pub use tutorlolv2_gen::{CHAMPION_CACHE, ITEM_CACHE, RUNE_CACHE};
}

pub mod champions {
    pub use tutorlolv2_gen::champions::*;
}

pub mod items {
    pub use tutorlolv2_gen::items::*;
}

pub mod runes {
    pub use tutorlolv2_gen::runes::*;
}

extern crate alloc;

pub mod calculator;
pub mod const_eval;
pub mod helpers;
pub mod model;
pub mod realtime;
pub mod riot;
