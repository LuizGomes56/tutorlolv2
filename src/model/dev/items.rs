use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::model::cache::Attrs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    // pub name: Option<String>,
    // pub unique: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsCdnStatsMap {
    pub flat: f64,
    pub percent: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    pub ability_power: ItemsCdnStatsMap,
    pub armor: ItemsCdnStatsMap,
    pub armor_penetration: ItemsCdnStatsMap,
    pub magic_penetration: ItemsCdnStatsMap,
    pub attack_damage: ItemsCdnStatsMap,
    pub attack_speed: ItemsCdnStatsMap,
    pub critical_strike_chance: ItemsCdnStatsMap,
    pub critical_strike_damage: ItemsCdnStatsMap,
    pub health: ItemsCdnStatsMap,
    pub lifesteal: ItemsCdnStatsMap,
    pub magic_resistance: ItemsCdnStatsMap,
    pub mana: ItemsCdnStatsMap,
    pub movespeed: ItemsCdnStatsMap,
    pub omnivamp: ItemsCdnStatsMap,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnItem {
    pub active: Vec<Effect>,
    pub builds_from: Vec<u32>,
    // pub builds_into: Vec<usize>,
    pub name: String,
    pub tier: u8,
    pub passives: Vec<Effect>,
    pub id: u32,
    pub stats: ItemStats,
    pub shop: Shop,
}

#[derive(Serialize, Deserialize)]
pub struct ItemPrices {
    pub total: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub purchasable: bool,
    pub prices: ItemPrices,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DamageObject {
    pub minimum_damage: Option<String>,
    pub maximum_damage: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartialStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_percent: f64,
    pub armor_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub critical_strike_chance: f64,
    pub critical_strike_damage: f64,
    pub health: f64,
    pub lifesteal: f64,
    pub magic_resistance: f64,
    pub mana: f64,
    pub movespeed: f64,
    pub omnivamp: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub gold: u32,
    pub tier: u8,
    pub prettified_stats: HashMap<String, Value>,
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub builds_from: Vec<u32>,
    pub ranged: Option<DamageObject>,
    pub melee: Option<DamageObject>,
    pub attributes: Attrs,
    pub purchasable: bool,
}
