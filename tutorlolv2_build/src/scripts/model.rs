use serde::Deserialize;
use tutorlolv2_types::{AbilityId, DevMergeData, StatName};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStatMap {
    pub flat: f64,
    pub per_level: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStats {
    pub health: MerakiChampionStatMap,
    pub mana: MerakiChampionStatMap,
    pub armor: MerakiChampionStatMap,
    #[serde(rename = "magicResistance")]
    pub magic_resist: MerakiChampionStatMap,
    pub attack_damage: MerakiChampionStatMap,
    pub attack_speed: MerakiChampionStatMap,
    pub movespeed: MerakiChampionStatMap,
    pub critical_strike_damage: MerakiChampionStatMap,
    pub critical_strike_damage_modifier: MerakiChampionStatMap,
    pub attack_speed_ratio: MerakiChampionStatMap,
    pub attack_range: MerakiChampionStatMap,
    pub aram_damage_taken: MerakiChampionStatMap,
    pub aram_damage_dealt: MerakiChampionStatMap,
    pub urf_damage_taken: MerakiChampionStatMap,
    pub urf_damage_dealt: MerakiChampionStatMap,
}

#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub enum Attrs {
    #[default]
    Undefined,
    Onhit,
    OnhitMin,
    OnhitMax,
    Area,
    AreaOnhit,
    AreaOnhitMin,
    AreaOnhitMax,
}

#[derive(Deserialize)]
pub struct Ability {
    pub name: String,
    pub damage_type: String,
    #[serde(default)]
    pub attributes: Attrs,
    pub damage: Vec<String>,
}

#[derive(Deserialize)]
pub struct Champion {
    pub name: String,
    pub adaptative_type: String,
    pub attack_type: String,
    pub positions: Vec<String>,
    pub stats: MerakiChampionStats,
    pub abilities: Vec<(AbilityId, Ability)>,
    pub merge_data: Vec<DevMergeData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiItemStatMap {
    pub flat: f64,
    pub percent: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    pub ability_power: MerakiItemStatMap,
    pub armor: MerakiItemStatMap,
    pub armor_penetration: MerakiItemStatMap,
    pub magic_penetration: MerakiItemStatMap,
    pub attack_damage: MerakiItemStatMap,
    pub attack_speed: MerakiItemStatMap,
    #[serde(rename = "criticalStrikeChance")]
    pub crit_chance: MerakiItemStatMap,
    #[serde(rename = "criticalStrikeDamage")]
    pub crit_damage: MerakiItemStatMap,
    pub health: MerakiItemStatMap,
    pub lifesteal: MerakiItemStatMap,
    #[serde(rename = "magicResistance")]
    pub magic_resist: MerakiItemStatMap,
    pub mana: MerakiItemStatMap,
    pub movespeed: MerakiItemStatMap,
    pub omnivamp: MerakiItemStatMap,
}

#[derive(Deserialize)]
pub struct DamageObject {
    pub minimum_damage: String,
    pub maximum_damage: String,
}

#[derive(Deserialize)]
pub struct Item {
    pub riot_id: usize,
    pub name: String,
    pub price: usize,
    pub tier: u8,
    pub prettified_stats: Vec<StatName>,
    pub damage_type: String,
    pub stats: ItemStats,
    pub ranged: DamageObject,
    pub melee: DamageObject,
    pub attributes: Attrs,
    pub purchasable: bool,
}

#[derive(Deserialize)]
pub struct Rune {
    pub name: String,
    pub damage_type: String,
    pub ranged: String,
    pub melee: String,
}
