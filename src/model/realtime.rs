use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::base::{BasicStats, ComparedItem, Damages, RealResists, Stats};

#[derive(Deserialize, Serialize, Clone)]
pub struct DamageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
}

#[derive(Serialize)]
pub struct CurrentPlayer<'a> {
    pub damaging_abilities: HashMap<String, String>,
    pub damaging_items: HashMap<usize, String>,
    pub damaging_runes: HashMap<usize, String>,
    pub riot_id: &'a str,
    pub level: usize,
    pub team: &'a str,
    pub position: &'a str,
    pub champion_name: &'a str,
    pub champion_id: String,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Serialize)]
pub struct Enemy<'a> {
    pub champion_id: String,
    pub champion_name: &'a str,
    pub riot_id: &'a str,
    pub team: &'a str,
    pub level: usize,
    pub position: &'a str,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_resists: RealResists,
}

#[derive(Serialize)]
pub struct DragonMultipliers {
    pub earth: f64,
    pub fire: f64,
    pub chemtech: f64,
}

impl DragonMultipliers {
    pub fn new() -> DragonMultipliers {
        DragonMultipliers {
            earth: 1.0,
            fire: 1.0,
            chemtech: 1.0,
        }
    }
}

#[derive(Serialize)]
pub struct Scoreboard {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub riot_id: String,
    pub champion_id: Option<String>,
    pub champion_name: String,
    pub team: String,
    pub position: String,
}

#[derive(Serialize)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub compared_items: HashMap<usize, ComparedItem>,
    pub best_item: usize,
    pub scoreboard: Vec<Scoreboard>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
