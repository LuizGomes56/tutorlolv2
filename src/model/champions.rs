#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatsMap {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Stats {
    pub health: StatsMap,
    pub mana: StatsMap,
    pub armor: StatsMap,
    pub magic_resistance: StatsMap,
    pub attack_damage: StatsMap,
    pub attack_speed: StatsMap,
    pub movespeed: StatsMap,
    pub critical_strike_damage: StatsMap,
    pub critical_strike_damage_modifier: StatsMap,
    pub attack_speed_ratio: StatsMap,
    pub attack_range: StatsMap,
    pub aram_damage_taken: StatsMap,
    pub aram_damage_dealt: StatsMap,
    pub urf_damage_taken: StatsMap,
    pub urf_damage_dealt: StatsMap,
}

#[derive(Deserialize)]
pub(crate) struct Modifiers {
    pub units: Vec<String>,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub(crate) struct ModifierLike {
    pub attribute: Option<String>,
    pub modifiers: Vec<Modifiers>,
}

#[derive(Deserialize)]
pub(crate) struct Effect {
    pub leveling: Vec<ModifierLike>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CdnAbility {
    pub cooldown: Option<ModifierLike>,
    pub damage_type: Option<String>,
    pub effects: Vec<Effect>,
    pub name: String,
    pub spell_effects: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) struct Abilities {
    pub p: Vec<CdnAbility>,
    pub q: Vec<CdnAbility>,
    pub w: Vec<CdnAbility>,
    pub e: Vec<CdnAbility>,
    pub r: Vec<CdnAbility>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnChampion {
    pub name: String,
    pub attack_type: String,
    pub adaptive_type: String,
    pub stats: Stats,
    pub abilities: Abilities,
    pub positions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ability {
    pub damage_type: String,
    pub damages_in_area: bool,
    pub minimum_damage: Vec<String>,
    pub maximum_damage: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Champion {
    pub name: String,
    pub adaptative_type: String,
    pub attack_type: String,
    pub positions: Vec<String>,
    pub stats: Stats,
    pub abilities: HashMap<String, Ability>,
}
