use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::riot::RiotChampionStats;

#[derive(Deserialize, Serialize, Clone)]
pub struct DamageLike {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
}

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
pub struct CurrentPlayer<'a> {
    pub damaging_abilities: Vec<String>,
    pub damaging_items: Vec<usize>,
    pub damaging_runes: Vec<usize>,
    pub riot_id: &'a str,
    pub level: usize,
    pub team: &'a str,
    pub position: &'a str,
    pub champion_name: &'a str,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
    pub recommended_items: Vec<usize>,
}

#[derive(Serialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Serialize)]
pub struct InstanceDamage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
    pub damage_type: String,
    pub damages_in_area: bool,
    pub damages_onhit: bool,
}

#[derive(Serialize)]
pub struct Damages {
    pub abilities: HashMap<String, InstanceDamage>,
    pub items: HashMap<usize, InstanceDamage>,
    pub runes: HashMap<usize, InstanceDamage>,
}

#[derive(Serialize)]
pub struct Enemy<'a> {
    pub champion_name: &'a str,
    pub riot_id: &'a str,
    pub team: &'a str,
    pub level: usize,
    pub position: &'a str,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
}

#[derive(Serialize)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub game_information: GameInformation,
}

pub struct DamageMultipliers {
    pub magic_damage: f64,
    pub physical_damage: f64,
    pub true_damage: f64,
    pub all_sources: f64,
}

pub struct RealResistances {
    pub magic_resistance: f64,
    pub armor: f64,
}

pub struct EnemyFullStats<'a> {
    pub current_stats: &'a BasicStats,
    pub base_stats: &'a BasicStats,
    pub bonus_stats: &'a BasicStats,
    pub takes_extra_damage_from: DamageMultipliers,
    pub real_resistances: RealResistances,
}

pub struct SelfFullStats<'a> {
    pub current_stats: &'a Stats,
    pub base_stats: &'a BasicStats,
    pub bonus_stats: &'a BasicStats,
    pub is_ranged: bool,
    pub level: usize,
    pub deals_extra_damage_from: DamageMultipliers,
    pub is_physical_adaptative_type: bool,
}

pub struct FullStats<'a> {
    pub missing_health: f64,
    pub enemy_has_steelcaps: bool,
    pub enemy_has_rocksolid: bool,
    pub enemy_has_randuin: bool,
    pub current_player: SelfFullStats<'a>,
    pub enemy_player: EnemyFullStats<'a>,
    pub physical_damage_multiplier: f64,
    pub magic_damage_multiplier: f64,
}
