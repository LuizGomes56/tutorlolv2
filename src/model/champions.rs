#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatsMap {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Deserialize)]
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
    pub modifiers: Vec<Modifiers>,
}

#[derive(Deserialize)]
pub(crate) struct Effect {
    pub leveling: Vec<ModifierLike>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ability {
    pub cooldown: Option<ModifierLike>,
    pub damage_type: Option<String>,
    pub effects: Vec<Effect>,
    pub name: String,
    pub spell_effects: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) struct Abilities {
    pub p: Vec<Ability>,
    pub q: Vec<Ability>,
    pub w: Vec<Ability>,
    pub e: Vec<Ability>,
    pub r: Vec<Ability>,
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
