use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_gen::{Attrs, DamageType, GameMap, StatName};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    pub name: Option<String>,
    pub unique: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerakiItemStatsMap {
    pub flat: f64,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    pub ability_power: MerakiItemStatsMap,
    pub armor: MerakiItemStatsMap,
    pub armor_penetration: MerakiItemStatsMap,
    pub magic_penetration: MerakiItemStatsMap,
    pub attack_damage: MerakiItemStatsMap,
    pub attack_speed: MerakiItemStatsMap,
    pub critical_strike_chance: MerakiItemStatsMap,
    pub critical_strike_damage: MerakiItemStatsMap,
    pub health: MerakiItemStatsMap,
    pub lifesteal: MerakiItemStatsMap,
    pub magic_resistance: MerakiItemStatsMap,
    pub mana: MerakiItemStatsMap,
    pub movespeed: MerakiItemStatsMap,
    pub omnivamp: MerakiItemStatsMap,
    #[serde(default)]
    pub adaptive_force: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiItem {
    pub active: Vec<Effect>,
    pub builds_from: Vec<u32>,
    pub builds_into: Vec<u32>,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageObject {
    pub minimum_damage: String,
    pub maximum_damage: String,
}

impl Default for DamageObject {
    fn default() -> Self {
        Self {
            minimum_damage: "zero".to_string(),
            maximum_damage: "zero".to_string(),
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Item {
    pub version: String,
    pub riot_id: u32,
    pub name: String,
    pub price: u32,
    pub sell: u32,
    pub tier: u8,
    pub attributes: Attrs,
    pub purchasable: bool,
    pub damage_type: DamageType,
    pub prettified_stats: BTreeMap<StatName, u16>,
    pub ranged: DamageObject,
    pub melee: DamageObject,
    pub stats: ItemStats,
    pub builds_from_riot_ids: BTreeSet<u32>,
    pub builds_into_riot_ids: BTreeSet<u32>,
    pub maps: BTreeMap<GameMap, bool>,
}
