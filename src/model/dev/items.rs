use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effects: String,
    // pub name: Option<String>,
    // pub unique: bool,
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
    pub builds_from: Vec<u32>,
    // pub builds_into: Vec<usize>,
    pub name: String,
    pub tier: u8,
    pub passives: Vec<Effect>,
    pub id: u32,
    pub stats: ItemStats,
    pub shop: Shop,
}

#[derive(Deserialize)]
pub struct ItemPrices {
    pub total: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub purchasable: bool,
    pub prices: ItemPrices,
}
