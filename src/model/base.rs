use crate::model::cache::EvalContext;

use super::riot::RiotChampionStats;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: &'static str,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
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

impl RiotChampionStats {
    pub fn to_stats(&self) -> Stats {
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

pub type DamageLike<T> = FxHashMap<T, InstanceDamage>;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Serialize)]
pub struct ComparedItem {
    pub name: &'static str,
    pub gold_cost: usize,
    pub prettified_stats: FxHashMap<&'static str, f64>,
}

#[derive(Serialize)]
pub struct SimulatedDamages {
    pub abilities: DamageLike<&'static str>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
}

#[derive(Serialize)]
pub struct Damages {
    pub abilities: DamageLike<&'static str>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
    pub compared_items: FxHashMap<usize, SimulatedDamages>,
}

pub enum AdaptativeType {
    Physical,
    Magic,
}

impl From<bool> for AdaptativeType {
    fn from(value: bool) -> Self {
        if value {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }
}

pub enum AttackType {
    Melee,
    Ranged,
    Other,
}

impl ToString for AttackType {
    fn to_string(&self) -> String {
        match self {
            Self::Melee => "MELEE".to_string(),
            Self::Ranged => "RANGED".to_string(),
            Self::Other => "OTHER".to_string(),
        }
    }
}

impl PartialEq<AttackType> for String {
    fn eq(&self, other: &AttackType) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Clone, Copy)]
pub struct GenericStats {
    pub real_armor: f64,
    pub real_magic: f64,
    pub armor_mod: f64,
    pub magic_mod: f64,
    pub enemy_mod: (f64, f64, f64, f64),
    pub self_mod: (f64, f64, f64, f64),
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
}

#[derive(Copy, Clone)]
pub struct DamageExpression {
    pub level: usize,
    pub damage_type: &'static str,
    pub minimum_damage: fn(usize, &EvalContext) -> f64,
    pub maximum_damage: fn(usize, &EvalContext) -> f64,
}

pub struct DamageMultipliers {
    pub self_mod: (f64, f64, f64, f64),
    pub enemy_mod: (f64, f64, f64, f64),
    pub damage_mod: (f64, f64),
}
