use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use super::base::{BasicStats, ComparedItem, Damages, RealResists, Stats};

#[derive(Serialize)]
pub struct CurrentPlayerX {
    pub champion_id: String,
    pub damaging_abilities: HashMap<String, String>,
    pub damaging_items: HashMap<usize, String>,
    pub damaging_runes: HashMap<usize, String>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct EnemyX {
    pub champion_name: String,
    pub champion_id: String,
    pub level: usize,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_resists: RealResists,
}

#[derive(Serialize)]
pub struct Calculator {
    pub current_player: CurrentPlayerX,
    pub enemies: Vec<EnemyX>,
    pub recommended_items: Vec<usize>,
    pub compared_items: HashMap<usize, ComparedItem>,
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
}

#[derive(Deserialize)]
pub struct EnemyPlayersX {
    pub champion_id: String,
    pub items: Vec<usize>,
    pub level: usize,
    pub stats: BasicStats,
}

#[derive(Deserialize)]
pub struct GameX {
    pub active_player: ActivePlayerX,
    pub enemy_players: Vec<EnemyPlayersX>,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
    pub stack_exceptions: BTreeMap<usize, usize>,
}
