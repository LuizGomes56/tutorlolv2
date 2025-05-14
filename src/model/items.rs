#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    pub name: Option<String>,
    pub unique: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsMap {
    pub flat: f64,
    pub per_level: f64,
    pub percent: f64,
    pub percent_base: f64,
    pub percent_bonus: f64,
    pub percent_per_level: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    pub ability_power: StatsMap,
    pub armor: StatsMap,
    pub armor_penetration: StatsMap,
    pub magic_penetration: StatsMap,
    pub attack_damage: StatsMap,
    pub attack_speed: StatsMap,
    pub critical_strike_chance: StatsMap,
    pub critical_strike_damage: StatsMap,
    pub health: StatsMap,
    pub lifesteal: StatsMap,
    pub magic_resistance: StatsMap,
    pub mana: StatsMap,
    pub movespeed: StatsMap,
    pub omnivamp: StatsMap,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnItem {
    pub active: Vec<Effect>,
    pub builds_from: Vec<usize>,
    pub builds_into: Vec<usize>,
    pub name: String,
    pub passives: Vec<Effect>,
    pub required_ally: String,
    pub required_champion: String,
    pub removed: bool,
    pub id: usize,
    pub stats: ItemStats,
    pub shop: Shop,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub purchasable: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ItemDamage {
    pub minimum_damage: Vec<String>,
    pub maximum_damage: Vec<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartialStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_penetration_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_penetration_flat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_penetration_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_penetration_flat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_damage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_strike_chance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_strike_damage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifesteal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_resistance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movespeed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omnivamp: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub builds_from: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levelings: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranged: Option<ItemDamage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<ItemDamage>,
}
