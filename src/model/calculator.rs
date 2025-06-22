use rustc_hash::FxHashMap;

use serde::{Deserialize, Serialize};

use super::base::{BasicStats, ComparedItem, Damages, RealResists, Stats};

#[derive(Serialize)]
pub struct CurrentPlayerX<'a> {
    pub champion_id: String,
    pub damaging_abilities: FxHashMap<&'a str, &'a str>,
    pub damaging_items: FxHashMap<usize, &'a str>,
    pub damaging_runes: FxHashMap<usize, &'a str>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct EnemyX<'a> {
    pub champion_name: &'a str,
    pub champion_id: String,
    pub level: usize,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_resists: RealResists,
}

#[derive(Serialize)]
pub struct Calculator<'a> {
    pub current_player: CurrentPlayerX<'a>,
    pub enemies: Vec<EnemyX<'a>>,
    pub recommended_items: Vec<usize>,
    pub compared_items: FxHashMap<usize, ComparedItem<'a>>,
}

#[derive(Deserialize)]
pub struct AbilitiesX {
    pub q: usize,
    pub w: usize,
    pub e: usize,
    pub r: usize,
}

#[derive(Deserialize)]
pub struct ActivePlayerX {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilitiesX,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: usize,
    pub stacks: usize,
    // Determines if stats should be inferred or used as <literal>[Stats] from Sender
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct EnemyPlayersX {
    pub champion_id: String,
    pub items: Vec<usize>,
    pub level: usize,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct GameX {
    pub active_player: ActivePlayerX,
    pub enemy_players: Vec<EnemyPlayersX>,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
    pub stack_exceptions: FxHashMap<usize, usize>,
}
