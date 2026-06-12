use crate::model::Effect;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiRune {
    pub name: String,
    pub rune_id: String,
    pub effects: BTreeMap<String, Effect>,
    pub descriptions: Vec<String>,
    pub riot_id: usize,
    pub custom: bool,
}
