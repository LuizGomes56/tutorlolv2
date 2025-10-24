use serde::{Deserialize, Serialize};
use tutorlolv2_gen::{Attrs, DamageType, GameMap, ItemId};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    pub name: Option<String>,
    pub unique: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItemsCdnStatsMap {
    pub flat: f64,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Default)]
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

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub riot_id: u32,
    pub item_id: ItemId,
    pub name: String,
    pub price: u32,
    pub sell: u32,
    pub tier: u8,
    pub maps: Vec<(GameMap, bool)>,
    pub prettified_stats: Vec<String>,
    pub damage_type: DamageType,
    pub stats: ItemStats,
    pub builds_from_riot_ids: Vec<u32>,
    pub builds_from_item_ids: Vec<ItemId>,
    pub builds_into_riot_ids: Vec<u32>,
    pub builds_into_item_ids: Vec<ItemId>,
    pub ranged: DamageObject,
    pub melee: DamageObject,
    pub attributes: Attrs,
    pub purchasable: bool,
}
