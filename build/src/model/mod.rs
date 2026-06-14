use serde::{Deserialize, Serialize};
use std::ops::{Range, RangeFrom, RangeTo};
use tutorlolv2_types::CtxVar;

pub mod champions;
pub mod items;
pub mod runes;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EffectInner {
    pub description: String,
    pub leveling: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Effect {
    pub index: usize,
    #[serde(default)]
    pub formula: Option<String>,
    pub inner: EffectInner,
    pub scalings: Vec<Scaling>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<f64>>,
    pub base: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Scaling {
    Simple {
        value: f64,
        ctx_var: CtxVar,
    },
    Ranked {
        values: Vec<f64>,
        ctx_var: CtxVar,
    },
    RankedPer100 {
        values: Vec<f64>,
        ctx_var: CtxVar,
    },
    Per100 {
        value: f64,
        ctx_var: CtxVar,
    },
    PercentAttr {
        value: f64,
        debug: String,
        ctx_var: CtxVar,
    },
    BasedOnLevel {
        level_var: CtxVar,
        arms: Vec<LevelArm>,
        debug: String,
        ctx_var: CtxVar,
    },
    Flat {
        values: Vec<f64>,
    },
    RangePercentAttr {
        min: f64,
        max: f64,
        debug: String,
        ctx_var: CtxVar,
    },
    Multiplier {
        raw: String,
        base: f64,
        inner: Vec<Scaling>,
    },
    Nested {
        raw: String,
        outer: Box<Scaling>,
        inner: Vec<Scaling>,
    },
    Other {
        raw: String,
    },
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum LevelArm {
    To { range: RangeTo<u8>, value: f64 },
    Range { range: Range<u8>, value: f64 },
    From { range: RangeFrom<u8>, value: f64 },
}
