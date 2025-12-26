use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tutorlolv2_gen::{AdaptativeType, AttackType, Attrs, DamageType, Position};
use tutorlolv2_types::{AbilityId, MergeData};

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
pub struct MerakiAbility {
    pub cooldown: Option<ModifierLike>,
    pub damage_type: Option<String>,
    pub effects: Vec<Effect>,
    pub name: String,
}

impl MerakiAbility {
    /// Receives a `damage` vector and returns a new [`Ability`] struct,
    /// filling the information that can be inferred from the [`Self`] object,
    /// such as name, attributes, and damage type
    pub fn format(&self, damage: Vec<String>) -> Ability {
        Ability {
            name: self.name.clone(),
            damage_type: DamageType::from(self.damage_type.clone().unwrap_or_default()),
            attributes: Attrs::Undefined,
            damage,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Abilities {
    pub p: Vec<MerakiAbility>,
    pub q: Vec<MerakiAbility>,
    pub w: Vec<MerakiAbility>,
    pub e: Vec<MerakiAbility>,
    pub r: Vec<MerakiAbility>,
}

impl Abilities {
    pub fn into_iter(self) -> impl Iterator<Item = (char, Vec<MerakiAbility>)> {
        [
            ('P', self.p),
            ('Q', self.q),
            ('W', self.w),
            ('E', self.e),
            ('R', self.r),
        ]
        .into_iter()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampion {
    pub name: String,
    pub attack_type: String,
    pub adaptive_type: String,
    pub stats: MerakiChampionStats,
    pub abilities: Abilities,
    pub positions: Vec<String>,
}

impl MerakiChampion {
    /// Receives the collected abilities and their generated merge information
    /// and creates a new [`Champion`] struct which represents the result of
    /// a champion generator file. Other fields are filled based on the information
    /// of the [`Self`] object
    pub fn format(
        self,
        abilities: HashMap<AbilityId, Ability>,
        merge_data: Vec<MergeData>,
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
            abilities: abilities.into_iter().collect(),
            merge_data,
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
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
    pub stats: MerakiChampionStats,
    pub abilities: Vec<(AbilityId, Ability)>,
    pub merge_data: Vec<MergeData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStats {
    pub health: MerakiChampionStatsMap,
    pub mana: MerakiChampionStatsMap,
    pub armor: MerakiChampionStatsMap,
    pub magic_resistance: MerakiChampionStatsMap,
    pub attack_damage: MerakiChampionStatsMap,
    pub attack_speed: MerakiChampionStatsMap,
    pub movespeed: MerakiChampionStatsMap,
    pub critical_strike_damage: MerakiChampionStatsMap,
    pub critical_strike_damage_modifier: MerakiChampionStatsMap,
    pub attack_speed_ratio: MerakiChampionStatsMap,
    pub attack_range: MerakiChampionStatsMap,
    pub aram_damage_taken: MerakiChampionStatsMap,
    pub aram_damage_dealt: MerakiChampionStatsMap,
    pub urf_damage_taken: MerakiChampionStatsMap,
    pub urf_damage_dealt: MerakiChampionStatsMap,
}
