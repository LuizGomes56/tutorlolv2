#![allow(unreachable_patterns)]

pub mod check;
pub mod docs;
pub mod impls;
pub mod model;

pub(self) use {
    bincode::{Decode, Encode},
    core::fmt::Debug,
    serde::{Deserialize, Serialize},
    tutorlolv2_types::{
        AbilityId::{self, *},
        AbilityName::*,
        AdaptiveType,
        AttackType::{self, *},
        Attrs::*,
        ComboElement::*,
        Ctx,
        CtxVar::*,
        DamageType::*,
        GameMap::*,
        MergeData,
        Position::*,
        StatName, TypeMetadata,
    },
};

pub use {
    champions_code::ChampionId,
    impls::traits::{CastId, ValueId},
    items_code::ItemId,
    model::*,
    runes_code::RuneId,
};

pub mod champions_code {
    use super::{champions::*, *};
    include!(concat!(env!("OUT_DIR"), "/champions_code.rs"));
}

pub mod champions {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/champions.rs"));
    type X = Champion;
}

pub mod items_code {
    use super::{items::*, *};
    include!(concat!(env!("OUT_DIR"), "/items_code.rs"));
}

pub mod items {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/items.rs"));
    type X = Item;
}

pub mod runes_code {
    use super::{runes::*, *};
    include!(concat!(env!("OUT_DIR"), "/runes_code.rs"));
}

pub mod runes {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/runes.rs"));
    type X = Rune;
}

pub const fn zero(_: &Ctx) -> f32 {
    0.0
}

pub const fn ignite(level: u8) -> i32 {
    let n = level as i32;
    let nth = if n > 4 { n - 4 } else { 0 };
    70 + 20 * n + 5 * nth
}

pub const L_MSTR: usize = 7;
pub const L_TWRD: usize = 6;
