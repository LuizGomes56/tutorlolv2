use crate::model::Effect;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffect {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub raw_description: Option<String>,
    pub effect: Effect,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffects {
    pub pass: Option<ItemEffect>,
    pub act: Option<ItemEffect>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WikiItem {
    pub id: u32,
    pub name: String,
    pub item_id: String,
    pub tier: Option<u8>,

    #[serde(default)]
    pub modes: BTreeMap<String, bool>,

    #[serde(default)]
    pub stats: BTreeMap<String, f64>,

    #[serde(default)]
    pub effects: ItemEffects,

    #[serde(default)]
    pub recipe: Vec<String>,

    pub buy: Option<u16>,
    pub purchasable: bool,
    pub custom: bool,
}
