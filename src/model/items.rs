#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Effect {
    pub effects: String,
    pub name: String,
    pub unique: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatsMap {
    pub flat: f64,
    pub per_level: f64,
    pub percent: f64,
    pub percent_base: f64,
    pub percent_bonus: f64,
    pub percent_per_level: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Stats {
    pub ability_power: StatsMap,
    pub armor: StatsMap,
    pub armor_penetration: StatsMap,
    pub attack_damage: StatsMap,
    pub attack_speed: StatsMap,
    pub critical_strike_chance: StatsMap,
    pub critical_strike_damage: StatsMap,
    pub health: StatsMap,
    pub lifesteal: StatsMap,
    pub magic_resistance: StatsMap,
    pub mana: StatsMap,
    pub movement_speed: StatsMap,
    pub omnivamp: StatsMap,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnItem {
    pub active: Vec<Effect>,
    pub builds_from: Vec<u32>,
    pub builds_into: Vec<u32>,
    pub name: String,
    pub passives: Vec<Effect>,
    pub required_ally: String,
    pub required_champion: String,
    pub removed: bool,
    pub stats: Stats,
}
