pub mod export_champions;
pub mod export_code;
pub mod export_items;
pub mod export_runes;
pub mod meta_items;
pub mod utils;

use crate::{compress_bytes, init_map};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};
use utils::*;

pub use export_champions::{Attrs, export_champions};
pub use export_code::export_code;
pub use export_items::export_items;
pub use export_runes::export_runes;
pub use meta_items::internal_meta_items;

pub static CRITICAL_STRIKE: &'static str = r#"pub static CRITICAL_STRIKE: DamageExpression = DamageExpression {
    level: 0,
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    minimum_damage: |_, ctx| {
        ctx.ad * ctx.physical_multiplier * ctx.crit_damage / 100.0
    },
    maximum_damage: zero,
};"#;

pub static BASIC_ATTACK: &'static str = r#"pub static BASIC_ATTACK: DamageExpression = DamageExpression {
    level: 0,
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    minimum_damage: |_, ctx| ctx.ad * ctx.physical_multiplier,
    maximum_damage: zero,
};"#;
