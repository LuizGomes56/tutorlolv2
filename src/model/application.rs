use super::{champions::Champion, internal::Positions, items::Item, runes::Rune};
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct GlobalCache {
    pub champions: FxHashMap<String, Champion>,
    pub items: FxHashMap<usize, Item>,
    pub runes: FxHashMap<usize, Rune>,
    pub champion_names: FxHashMap<String, String>,
    pub meta_items: FxHashMap<String, Positions>,
    pub simulated_items: Vec<usize>,
}
