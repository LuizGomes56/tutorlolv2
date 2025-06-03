use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionCdnStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionCdnStats {
    pub health: ChampionCdnStatsMap,
    pub mana: ChampionCdnStatsMap,
    pub armor: ChampionCdnStatsMap,
    pub magic_resistance: ChampionCdnStatsMap,
    pub attack_damage: ChampionCdnStatsMap,
    pub attack_speed: ChampionCdnStatsMap,
    pub movespeed: ChampionCdnStatsMap,
    pub critical_strike_damage: ChampionCdnStatsMap,
    pub critical_strike_damage_modifier: ChampionCdnStatsMap,
    pub attack_speed_ratio: ChampionCdnStatsMap,
    pub attack_range: ChampionCdnStatsMap,
    pub aram_damage_taken: ChampionCdnStatsMap,
    pub aram_damage_dealt: ChampionCdnStatsMap,
    pub urf_damage_taken: ChampionCdnStatsMap,
    pub urf_damage_dealt: ChampionCdnStatsMap,
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
}

impl CdnAbility {
    pub fn format(&self, minimum_damage: Vec<String>, maximum_damage: Vec<String>) -> Ability {
        Ability {
            name: self.name.clone(),
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
    pub fn into_iterator(&self) -> impl Iterator<Item = (&'static str, &Vec<CdnAbility>)> {
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
    pub stats: ChampionCdnStats,
    pub abilities: Abilities,
    pub positions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ability {
    pub name: String,
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
    pub stats: ChampionCdnStats,
    pub abilities: HashMap<String, Ability>,
}

impl CdnChampion {
    pub fn format(self, abilities: HashMap<String, Ability>) -> Champion {
        Champion {
            abilities,
            name: self.name,
            adaptative_type: self.adaptive_type,
            attack_type: self.attack_type,
            positions: self.positions,
            stats: self.stats,
        }
    }
}
