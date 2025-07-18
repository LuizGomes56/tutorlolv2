use super::base::{AbilityLevels, BasicStats, DamageLike, Stats};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_abilities: Vec<&'static str>,
    pub damaging_items: Vec<usize>,
    pub damaging_runes: Vec<usize>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct CalculatorDamages {
    pub abilities: DamageLike<&'static str>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
}

#[derive(Serialize)]
pub struct OutputEnemy {
    pub champion_name: String,
    pub level: usize,
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
    pub recommended_items: &'static [usize],
}

#[derive(Deserialize)]
pub struct InputActivePlayer {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: usize,
    pub stacks: usize,
    // Determines if stats should be inferred or used as <literal>[Stats] from Sender
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputEnemyPlayers {
    pub champion_name: String,
    pub items: Vec<usize>,
    pub level: usize,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Vec<InputEnemyPlayers>,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
    pub stack_exceptions: FxHashMap<usize, usize>,
}
