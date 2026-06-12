use crate::model::Effect;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_types::{AdaptiveType, AttackType, DamageType, Key, Position};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiChampion {
    pub name: String,
    pub champion_id: String,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub positions: Vec<Position>,
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub wiki_abilities: BTreeMap<Key, Vec<WikiAbility>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WikiAbility {
    pub champion_id: String,
    pub damage_type: DamageType,
    pub name: String,
    pub skill: Key,
    pub spell_effects: Option<String>,
    pub effects: BTreeMap<String, Effect>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct WikiStats {
    pub health: Stat,
    pub mana: Stat,
    pub armor: Stat,
    pub magic_resist: Stat,
    pub attack_damage: Stat,
    pub attack_speed: Stat,
    pub attack_speed_ratio: f32,
    pub crit_modifier: f32,
    pub crit_base: f32,
    pub move_speed: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Stat {
    pub base: f32,
    pub per_level: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Modifier {
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct WikiModifiers {
    pub ofa: Modifier,
    pub usb: Modifier,
    pub aram: Modifier,
    pub ar: Modifier,
    pub nb: Modifier,
    pub swift: Modifier,
    pub urf: Modifier,
}
