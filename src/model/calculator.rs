#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    base::{BasicStats, ComparedItem, Damages, RealResists, Stats},
    realtime::DragonMultipliers,
    riot::RiotChampionStats,
};

#[derive(Serialize)]
pub struct CurrentPlayerX {
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
    pub best_item: usize,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion_stats: Option<RiotChampionStats>,
    pub abilities: AbilitiesX,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: usize,
}

#[derive(Deserialize)]
pub struct EnemyPlayersX {
    pub champion_id: String,
    pub items: Vec<usize>,
    pub level: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<BasicStats>,
}

#[derive(Deserialize)]
pub struct GameX {
    pub active_player: ActivePlayerX,
    pub enemy_players: Vec<EnemyPlayersX>,
    pub ally_dragon_multipliers: DragonMultipliers,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
}
