use super::{
    SIZE_ENEMIES_EXPECTED,
    base::{BasicStats, Damages, DragonMultipliers, Stats},
};
use serde::Serialize;
use smallvec::SmallVec;
use tinyset::SetU32;

#[derive(Serialize)]
pub struct CurrentPlayer<'a> {
    pub damaging_items: SetU32,
    pub damaging_runes: SetU32,
    pub riot_id: &'a str,
    pub level: u8,
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
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Serialize)]
pub struct Enemy<'a> {
    pub champion_name: &'a str,
    pub riot_id: &'a str,
    pub team: &'a str,
    pub level: u8,
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
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: &'a str,
    pub champion_id: &'static str,
    pub champion_name: &'a str,
    pub position: &'a str,
}

#[derive(Serialize)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[(&'static str, Enemy<'a>); SIZE_ENEMIES_EXPECTED]>,
    pub game_information: GameInformation,
    pub recommended_items: &'static [u32],
    pub scoreboard: Scoreboard<'a>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
