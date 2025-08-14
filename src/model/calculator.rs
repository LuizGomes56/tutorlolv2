use super::{
    SIZE_ABILITIES, SIZE_ITEMS_EXPECTED, SIZE_RUNES_EXPECTED,
    base::{AbilityLevels, BasicStats, DamageLike, MonsterDamages, Stats},
};
use bincode::{Decode, Encode};
use internal_comptime::{AbilityLike, ChampionId, ItemId, RuneId};
use smallvec::SmallVec;
use tinyset::SetU32;

#[derive(Encode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    #[bincode(with_serde)]
    pub damaging_items: SetU32,
    #[bincode(with_serde)]
    pub damaging_runes: SetU32,
    pub level: u8,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Encode)]
pub struct CalculatorDamages {
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<5, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct OutputEnemy {
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Encode)]
pub struct _Test {
    pub a: Vec<(u32, u32)>,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages,
    pub tower_damages: [f64; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[(ChampionId, OutputEnemy); 1]>,
    pub recommended_items: &'static [ItemId],
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub champion_id: ChampionId,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: SmallVec<[ItemId; SIZE_ITEMS_EXPECTED]>,
    pub runes: SmallVec<[RuneId; SIZE_RUNES_EXPECTED]>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Decode)]
pub struct InputEnemyPlayers {
    pub champion_id: ChampionId,
    pub items: SmallVec<[u32; SIZE_ITEMS_EXPECTED]>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputEnemyPlayers; 1]>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    // #![todo] well defined struct instead of a Map or Vec
    // pub stack_exceptions: SmallVec<[(u32, u32); SIZE_STACK_EXCEPTIONS]>,
}
