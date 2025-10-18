use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tutorlolv2_gen::{AbilityLike, AdaptativeType, AttackType, Attrs, DamageType, Position};

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
    pub fn format(&self, damage: Vec<String>) -> Ability {
        Ability {
            name: self.name.clone(),
            damage_type: DamageType::from(self.damage_type.clone().unwrap_or_default()),
            attributes: Attrs::None,
            damage,
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
    pub fn format(
        self,
        abilities: HashMap<AbilityLike, Ability>,
        merge_data: Vec<(AbilityLike, AbilityLike)>,
    ) -> Champion {
        Champion {
            name: self.name,
            adaptative_type: AdaptativeType::from(self.adaptive_type),
            attack_type: AttackType::from(self.attack_type),
            positions: self
                .positions
                .into_iter()
                .map(|pos| Position::from_raw(&pos).unwrap_or_default())
                .collect(),
            stats: self.stats,
            abilities,
            merge_data,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ability {
    pub name: String,
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub damage: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Champion {
    pub name: String,
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: Vec<Position>,
    pub stats: ChampionCdnStats,
    pub abilities: HashMap<AbilityLike, Ability>,
    pub merge_data: Vec<(AbilityLike, AbilityLike)>,
}

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
