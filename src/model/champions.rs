#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsMap {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
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
pub struct Modifiers {
    pub units: Vec<String>,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct ModifierLike {
    pub attribute: Option<String>,
    pub modifiers: Vec<Modifiers>,
}

#[derive(Deserialize)]
pub struct Effect {
    pub description: String,
    pub leveling: Vec<ModifierLike>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnAbility {
    pub cooldown: Option<ModifierLike>,
    pub damage_type: Option<String>,
    pub effects: Vec<Effect>,
    pub name: String,
    pub spell_effects: Option<String>,
}

impl CdnAbility {
    pub fn format(&self, minimum_damage: Vec<String>, maximum_damage: Vec<String>) -> Ability {
        Ability {
            damage_type: self.damage_type.clone().unwrap_or_default(),
            damages_in_area: self
                .damage_type
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains("aoe"),
            minimum_damage,
            maximum_damage,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Abilities {
    pub p: Vec<CdnAbility>,
    pub q: Vec<CdnAbility>,
    pub w: Vec<CdnAbility>,
    pub e: Vec<CdnAbility>,
    pub r: Vec<CdnAbility>,
}

impl Abilities {
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &Vec<CdnAbility>)> {
        [
            ("q", &self.q),
            ("w", &self.w),
            ("e", &self.e),
            ("r", &self.r),
            ("p", &self.p),
        ]
        .into_iter()
    }
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

impl CdnChampion {
    pub fn format(self, abilities: HashMap<String, Ability>) -> Champion {
        Champion {
            name: self.name,
            adaptative_type: self.adaptive_type,
            attack_type: self.attack_type,
            positions: self.positions,
            stats: self.stats,
            abilities,
        }
    }
}
