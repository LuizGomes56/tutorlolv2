use std::collections::HashMap;

use serde::Serialize;

use super::riot::RiotChampionStats;

#[derive(Serialize)]
pub struct Stats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub max_mana: f64,
    pub current_mana: f64,
}

#[derive(Serialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

impl BasicStats {
    pub fn to_riot_format(&self) -> RiotChampionStats {
        RiotChampionStats {
            armor: self.armor,
            max_health: self.health,
            attack_damage: self.attack_damage,
            magic_resist: self.magic_resist,
            resource_max: self.mana,
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
pub struct CurrentPlayer {
    pub damaging_abilities: Vec<String>,
    pub damaging_items: Vec<String>,
    pub damaging_runes: Vec<String>,
    pub riot_id: String,
    pub level: usize,
    pub team: String,
    pub position: String,
    pub champion_name: String,
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
pub struct Damages {
    pub abilities: HashMap<String, f64>,
    pub items: HashMap<String, f64>,
    pub runes: HashMap<String, f64>,
}

#[derive(Serialize)]
pub struct Enemy {
    pub champion_name: String,
    pub riot_id: String,
    pub team: String,
    pub level: usize,
    pub position: String,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
}

#[derive(Serialize)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
    pub game_information: GameInformation,
}
