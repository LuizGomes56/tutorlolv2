use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RiotCdnItem {
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RiotCdnStandard {
    pub data: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct RiotCdnImage {
    pub full: String,
}

#[derive(Deserialize)]
pub struct RiotCdnInstance {
    pub image: RiotCdnImage,
}

#[derive(Deserialize)]
pub struct RiotCdnRuneTree {
    pub icon: String,
    pub id: usize,
}

#[derive(Deserialize)]
pub struct RiotCdnRuneSlot {
    pub runes: Vec<RiotCdnRuneTree>,
}

#[derive(Deserialize)]
pub struct RiotCdnRune {
    pub icon: String,
    pub id: usize,
    pub slots: Vec<RiotCdnRuneSlot>,
}

#[derive(Deserialize)]
pub struct RiotCdnSkin {
    pub num: usize,
}

#[derive(Deserialize)]
pub struct RiotCdnChampion {
    pub id: String,
    pub image: RiotCdnImage,
    pub passive: RiotCdnInstance,
    pub spells: Vec<RiotCdnInstance>,
    pub skins: Vec<RiotCdnSkin>,
}
