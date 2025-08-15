use super::{
    SIZE_ABILITIES, SIZE_ENEMIES_EXPECTED, WrapSetU32,
    base::{BasicStats, DamageLike, DragonMultipliers, Stats},
};
use crate::{SIZE_DAMAGING_ITEMS, SIZE_SIMULATED_ITEMS};
use bincode::Encode;
use internal_comptime::{AbilityLike, ChampionId, ItemId, Position, RuneId};
use smallvec::SmallVec;

#[derive(Encode)]
pub struct SimulatedDamages {
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<SIZE_DAMAGING_ITEMS, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct Damages {
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<5, ItemId>,
    pub runes: DamageLike<3, RuneId>,
    pub compared_items: SmallVec<[(ItemId, SimulatedDamages); SIZE_SIMULATED_ITEMS]>,
}

#[derive(Encode)]
pub struct CurrentPlayer<'a> {
    pub damaging_items: WrapSetU32,
    pub damaging_runes: WrapSetU32,
    pub riot_id: &'a str,
    pub level: u8,
    pub team: Team,
    pub position: Position,
    pub champion_id: ChampionId,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Encode)]
pub struct GameInformation {
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Encode)]
pub enum Team {
    Blue,
    Red,
}

impl Team {
    pub fn from_raw(raw: &str) -> Self {
        match raw {
            "ORDER" => Team::Blue,
            "CHAOS" => Team::Red,
            _ => Team::Blue,
        }
    }
}

#[derive(Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub team: Team,
    pub level: u8,
    pub position: Position,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Encode)]
pub struct Scoreboard<'a> {
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: &'a str,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[(ChampionId, Enemy<'a>); SIZE_ENEMIES_EXPECTED]>,
    pub game_information: GameInformation,
    pub recommended_items: &'static [ItemId],
    pub scoreboard: Scoreboard<'a>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
