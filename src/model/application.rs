use std::collections::HashMap;

use super::champions::Champion;
use super::internal::Positions;
use super::items::Item;
use super::runes::Rune;

pub struct GlobalCache {
    pub champions: HashMap<String, Champion>,
    pub items: HashMap<usize, Item>,
    pub runes: HashMap<usize, Rune>,
    pub champion_names: HashMap<String, String>,
    pub meta_items: HashMap<String, Positions>,
}
