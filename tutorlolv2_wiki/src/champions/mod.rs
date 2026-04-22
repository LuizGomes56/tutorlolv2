use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tutorlolv2_types::{AdaptiveType, AttackType, DamageType, Key};

use crate::client::MayFail;

pub mod abilities;
pub mod full;
pub mod template;

pub async fn run() -> MayFail {
    full::download().await?;
    full::parse()?;
    template::download().await?;
    template::parse()?;
    abilities::download().await?;
    abilities::parse()?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiChampion {
    pub champion_id: String,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub abilities: BTreeMap<Key, WikiAbility>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiStats {
    pub health: (f32, f32),
    pub mana: (f32, f32),
    pub armor: (f32, f32),
    pub magic_resist: (f32, f32),
    pub attack_damage: (f32, f32),
    pub attack_speed: (f32, f32),
    pub crit: (f32, f32),
    pub movespeed: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiModifiers {
    pub ofa: (f64, f64),
    pub usb: (f64, f64),
    pub aram: (f64, f64),
    pub ar: (f64, f64),
    pub nb: (f64, f64),
    pub swift: (f64, f64),
    pub urf: (f64, f64),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiAbility {
    pub name: String,
    pub damage_type: DamageType,
    pub effects: Vec<WikiEffect>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiEffect {
    pub description: String,
    pub levelings: Vec<WikiLeveling>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiLeveling {
    pub attribute: String,
    pub formula_attempt: Vec<String>,
    pub data: String,
}

pub fn concat() -> MayFail {
    todo!()
}

pub fn clean_text(s: &str) -> String {
    s.replace('\u{a0}', " ")
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}
