use super::{
    SIZE_ABILITIES, SIZE_ITEMS_EXPECTED, SIZE_RUNES_EXPECTED,
    base::{AbilityLevels, BasicStats, DamageLike, MonsterDamages, Stats},
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use tinyset::SetU32;

#[derive(Serialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_items: SetU32,
    pub damaging_runes: SetU32,
    pub level: u8,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct CalculatorDamages {
    pub abilities: DamageLike<SIZE_ABILITIES, &'static str>,
    pub items: DamageLike<5, u32>,
    pub runes: DamageLike<3, u32>,
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
    pub monster_damages: MonsterDamages,
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[(&'static str, OutputEnemy); 1]>,
    pub recommended_items: &'static [u32],
}

#[derive(Deserialize)]
pub struct InputActivePlayer {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: SmallVec<[u32; SIZE_ITEMS_EXPECTED]>,
    pub runes: SmallVec<[u32; SIZE_RUNES_EXPECTED]>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputEnemyPlayers {
    pub champion_name: String,
    pub items: SmallVec<[u32; SIZE_ITEMS_EXPECTED]>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Deserialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputEnemyPlayers; 1]>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    // #![todo] well defined struct instead of a Map or Vec
    // pub stack_exceptions: SmallVec<[(u32, u32); SIZE_STACK_EXCEPTIONS]>,
}
