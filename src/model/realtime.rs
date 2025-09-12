use super::{
    SIZE_ABILITIES, SIZE_ENEMIES_EXPECTED, Sizer,
    base::{Attacks, BasicStats, DamageLike, DragonMultipliers, Stats},
    functions::WrapSetU32,
};
use crate::{SIZE_DAMAGING_ITEMS, SIZE_SIMULATED_ITEMS};
use bincode::Encode;
use internal_comptime::{AbilityLike, AdaptativeType, ChampionId, ItemId, Position, RuneId};
use smallvec::SmallVec;

#[derive(Encode)]
pub struct SimulatedDamages {
    pub attacks: Attacks<i32>,
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<SIZE_DAMAGING_ITEMS, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct Damages {
    pub attacks: Attacks<i32>,
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
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: Stats<i32>,
}

#[derive(Encode)]
pub struct GameInformation {
    pub game_time: i32,
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
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: BasicStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
}

#[derive(Encode)]
pub struct Scoreboard<'a> {
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub riot_id: &'a str,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[(ChampionId, Enemy<'a>); SIZE_ENEMIES_EXPECTED]>,
    pub game_information: GameInformation,
    pub scoreboard: Scoreboard<'a>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}

impl Sizer for Realtime<'_> {
    fn size(&self) -> usize {
        let sc = &self.scoreboard;
        let cp = &self.current_player;
        let en = &self.enemies;
        self.ally_dragon_multipliers.size()
            + self.enemy_dragon_multipliers.size()
            + self.game_information.game_time.size()
            + self.game_information.map_number.size()
            + sc.assists.size()
            + sc.champion_id.size()
            + sc.creep_score.size()
            + sc.deaths.size()
            + sc.kills.size()
            + sc.position.size()
            + (sc.riot_id.size() << 1)
            + cp.adaptative_type.size()
            + cp.base_stats.size()
            + cp.bonus_stats.size()
            + cp.champion_id.size()
            + cp.current_stats.size()
            + cp.damaging_items.size()
            + cp.damaging_runes.size()
            + cp.level.size()
            + cp.position.size()
            + cp.team.size()
            + en.len().size()
            + en.iter()
                .map(|(id, e)| {
                    id.size()
                        + e.base_stats.size()
                        + e.bonus_stats.size()
                        + e.current_stats.size()
                        + e.damages.abilities.size()
                        + e.damages.attacks.size()
                        + e.damages.items.size()
                        + e.damages.runes.size()
                        + e.damages.compared_items.size()
                        + e.level.size()
                        + e.position.size()
                        + e.real_armor.size()
                        + e.real_magic_resist.size()
                        + e.riot_id.size()
                        + e.team.size()
                })
                .sum::<usize>()
    }
}
