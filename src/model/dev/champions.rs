use crate::{
    generators::{Ability, Champion},
    model::champions::ChampionCdnStats,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Modifiers {
    pub units: Vec<String>,
    pub values: Vec<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct ModifierLike {
    pub attribute: Option<String>,
    pub modifiers: Vec<Modifiers>,
}

#[derive(Deserialize, Serialize)]
pub struct Effect {
    pub description: String,
    pub leveling: Vec<ModifierLike>,
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnChampion {
    pub name: String,
    pub attack_type: String,
    pub adaptive_type: String,
    pub stats: ChampionCdnStats,
    pub abilities: Abilities,
    pub positions: Vec<String>,
}

impl CdnChampion {
    pub fn format(self, abilities: HashMap<String, Ability>) -> Champion {
        Champion {
            abilities: abilities.into_iter().collect(),
            name: self.name,
            adaptative_type: self.adaptive_type,
            attack_type: self.attack_type,
            positions: self.positions,
            stats: self.stats,
        }
    }
}
