use super::base::{AbilityLevels, BasicStats, DamageLike, Stats};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_abilities: Vec<&'static str>,
    pub damaging_items: Vec<u32>,
    pub damaging_runes: Vec<u32>,
    pub level: u8,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct CalculatorDamages {
    pub abilities: DamageLike<&'static str>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
}

#[derive(Serialize)]
pub struct OutputEnemy {
    pub champion_name: String,
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Serialize)]
pub struct OutputGame {
    pub current_player: OutputCurrentPlayer,
    pub enemies: FxHashMap<&'static str, OutputEnemy>,
    pub recommended_items: &'static [u32],
}

#[derive(Deserialize)]
pub struct InputActivePlayer {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<u32>,
    pub runes: Vec<u32>,
    pub level: u8,
    pub stacks: u32,
    // Determines if stats should be inferred or used as <literal>[Stats] from Sender
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputEnemyPlayers {
    pub champion_name: String,
    pub items: Vec<u32>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Vec<InputEnemyPlayers>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    pub stack_exceptions: FxHashMap<u32, u32>,
}
