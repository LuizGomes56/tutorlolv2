use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone)]
pub struct DamageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    pub name: Option<String>,
    pub unique: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsCdnStatsMap {
    pub flat: f64,
    pub percent: f64,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnItem {
    pub active: Vec<Effect>,
    pub builds_from: Vec<usize>,
    pub builds_into: Vec<usize>,
    pub name: String,
    pub tier: usize,
    pub passives: Vec<Effect>,
    pub required_ally: String,
    pub required_champion: String,
    pub removed: bool,
    pub id: usize,
    pub stats: ItemStats,
    pub shop: Shop,
}

#[derive(Deserialize)]
pub struct ItemPrices {
    pub total: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub purchasable: bool,
    pub prices: ItemPrices,
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
    pub gold: usize,
    pub tier: usize,
    pub prettified_stats: FxHashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub builds_from: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levelings: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranged: Option<DamageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<DamageObject>,
    pub damages_onhit: bool,
}
