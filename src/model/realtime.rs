use super::base::{BasicStats, Damages, DragonMultipliers, Stats};
use rustc_hash::FxHashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct CurrentPlayer<'a> {
    pub damaging_abilities: FxHashMap<&'static str, &'static str>,
    pub damaging_items: FxHashMap<usize, &'static str>,
    pub damaging_runes: FxHashMap<usize, &'static str>,
    pub riot_id: &'a str,
    pub level: usize,
    pub team: &'a str,
    pub position: &'a str,
    pub champion_name: &'a str,
    pub champion_id: &'static str,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Serialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Serialize)]
pub struct Enemy<'a> {
    pub champion_id: &'static str,
    pub champion_name: &'a str,
    pub riot_id: &'a str,
    pub team: &'a str,
    pub level: usize,
    pub position: &'a str,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Serialize)]
pub struct Scoreboard<'a> {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub riot_id: &'a str,
    pub champion_id: &'static str,
    pub champion_name: &'a str,
    pub position: &'a str,
}

#[derive(Serialize)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub game_information: GameInformation,
    pub recommended_items: &'static [usize],
    pub scoreboard: Scoreboard<'a>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
