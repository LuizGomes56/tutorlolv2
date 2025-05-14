use std::collections::HashMap;

use super::champions::Champion;
use super::items::CdnItem;
// use super::runes::{Rune};

pub struct GlobalCache {
    pub champions: HashMap<String, Champion>,
    pub items: HashMap<String, CdnItem>,
    // pub runes: Vec<Rune>,
    pub champion_names: HashMap<String, String>,
}
