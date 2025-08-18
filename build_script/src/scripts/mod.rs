pub mod export_champions;
pub mod export_items;
pub mod export_runes;
pub mod utils;

pub use crate::*;
pub use export_champions::*;
pub use export_items::*;
pub use export_runes::*;
pub use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use serde::Deserialize;
pub use std::{
    collections::{BTreeMap, HashMap},
    fs,
};
pub use utils::*;

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
