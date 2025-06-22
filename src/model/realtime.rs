use super::base::{BasicStats, ComparedItem, Damages, RealResists, Stats};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DamageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
}

#[derive(Serialize)]
pub struct CurrentPlayer<'a> {
    pub damaging_abilities: FxHashMap<&'a str, &'a str>,
    pub damaging_items: FxHashMap<usize, &'a str>,
    pub damaging_runes: FxHashMap<usize, &'a str>,
    pub riot_id: &'a str,
    pub level: usize,
    pub team: &'a str,
    pub position: &'a str,
    pub champion_name: &'a str,
    pub champion_id: &'a str,
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
    pub champion_id: &'a str,
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

#[derive(Deserialize, Serialize)]
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
pub struct Scoreboard<'a> {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub riot_id: &'a str,
    pub champion_id: Option<&'a str>,
    pub champion_name: &'a str,
    pub team: &'a str,
    pub position: &'a str,
}

#[derive(Serialize)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub compared_items: FxHashMap<usize, ComparedItem<'a>>,
    pub scoreboard: Vec<Scoreboard<'a>>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
