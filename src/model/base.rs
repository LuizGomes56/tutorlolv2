use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{calculator::CurrentPlayerX, realtime::CurrentPlayer, riot::RiotChampionStats};

#[derive(Serialize)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: String,
    pub damages_in_area: bool,
    pub damages_onhit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_dmg_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dmg_change: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize)]
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

pub trait ToRiotFormat {
    fn format(&self) -> RiotChampionStats;
}

impl ToRiotFormat for BasicStats {
    fn format(&self) -> RiotChampionStats {
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

impl ToRiotFormat for RiotChampionStats {
    fn format(&self) -> RiotChampionStats {
        (*self).clone()
    }
}

impl RiotChampionStats {
    pub fn transform(&self) -> Stats {
        Stats {
            ability_power: self.ability_power,
            armor: self.armor,
            armor_penetration_flat: self.physical_lethality,
            armor_penetration_percent: self.armor_penetration_percent,
            attack_damage: self.attack_damage,
            attack_range: self.attack_range,
            attack_speed: self.attack_speed,
            crit_chance: self.crit_chance,
            crit_damage: self.crit_damage,
            current_health: self.current_health,
            magic_penetration_flat: self.magic_penetration_flat,
            magic_penetration_percent: self.magic_penetration_percent,
            magic_resist: self.magic_resist,
            max_health: self.max_health,
            max_mana: self.resource_max,
            current_mana: self.resource_value,
        }
    }
}

impl ToRiotFormat for Stats {
    fn format(&self) -> RiotChampionStats {
        RiotChampionStats {
            ability_power: self.ability_power,
            armor: self.armor,
            physical_lethality: self.armor_penetration_flat,
            armor_penetration_percent: self.armor_penetration_percent,
            attack_damage: self.attack_damage,
            attack_range: self.attack_range,
            attack_speed: self.attack_speed,
            crit_chance: self.crit_chance,
            crit_damage: self.crit_damage,
            current_health: self.current_health,
            magic_penetration_flat: self.magic_penetration_flat,
            magic_penetration_percent: self.magic_penetration_percent,
            magic_resist: self.magic_resist,
            max_health: self.max_health,
            resource_max: self.max_mana,
            resource_value: self.current_mana,
        }
    }
}

pub type DamageLike<T> = HashMap<T, InstanceDamage>;

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Serialize)]
pub struct ComparedItem {
    pub name: String,
    pub gold_cost: usize,
    pub prettified_stats: HashMap<String, Value>,
}

#[derive(Serialize)]
pub struct RealResists {
    pub magic_resist: f64,
    pub armor: f64,
}

#[derive(Serialize)]
pub struct ComparedDamage<T> {
    pub total: f64,
    pub change: f64,
    pub damages: DamageLike<T>,
}

#[derive(Serialize)]
pub struct SimulatedDamages {
    pub abilities: ComparedDamage<String>,
    pub items: ComparedDamage<usize>,
    pub runes: ComparedDamage<usize>,
}

#[derive(Serialize)]
pub struct Damages {
    pub abilities: DamageLike<String>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
    pub compared_items: HashMap<usize, SimulatedDamages>,
}

pub struct DamageMultipliers {
    pub magic_damage: f64,
    pub physical_damage: f64,
    pub true_damage: f64,
    pub all_sources: f64,
}

pub struct EnemyFullStats<'a> {
    pub current_stats: &'a mut BasicStats,
    pub bonus_stats: &'a mut BasicStats,
    pub damage_mod: DamageMultipliers,
    pub real_resists: RealResists,
}

pub struct SelfFullStats<'a> {
    pub current_stats: &'a Stats,
    pub base_stats: &'a BasicStats,
    pub bonus_stats: &'a BasicStats,
    pub is_ranged: bool,
    pub level: usize,
    pub damage_mod: DamageMultipliers,
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

pub enum AttackType {
    Melee,
    Ranged,
    Other,
}

impl From<&str> for AttackType {
    fn from(s: &str) -> Self {
        match s {
            "MELEE" => AttackType::Melee,
            "RANGED" => AttackType::Ranged,
            _ => AttackType::Other,
        }
    }
}

pub trait CurrentPlayerLike {
    fn get_base_stats(&self) -> &BasicStats;
    fn get_bonus_stats(&self) -> &BasicStats;
    fn get_level(&self) -> usize;
    fn get_current_stats(&self) -> &Stats;
}

impl<'a> CurrentPlayerLike for CurrentPlayer<'a> {
    fn get_base_stats(&self) -> &BasicStats {
        &self.base_stats
    }
    fn get_bonus_stats(&self) -> &BasicStats {
        &self.bonus_stats
    }
    fn get_level(&self) -> usize {
        self.level
    }
    fn get_current_stats(&self) -> &Stats {
        &self.current_stats
    }
}

impl CurrentPlayerLike for CurrentPlayerX {
    fn get_base_stats(&self) -> &BasicStats {
        &self.base_stats
    }
    fn get_bonus_stats(&self) -> &BasicStats {
        &self.bonus_stats
    }
    fn get_level(&self) -> usize {
        self.level
    }
    fn get_current_stats(&self) -> &Stats {
        &self.current_stats
    }
}
