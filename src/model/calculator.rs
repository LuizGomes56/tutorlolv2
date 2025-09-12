use super::{
    SIZE_ABILITIES, SIZE_ITEMS_EXPECTED, SIZE_RUNES_EXPECTED, Sizer,
    base::{AbilityLevels, Attacks, BasicStats, DamageLike, MonsterDamages, Stats},
    functions::WrapSetU32,
};
use bincode::{Decode, Encode};
use internal_comptime::{AbilityLike, AdaptativeType, ChampionId, ItemId, RuneId};
use smallvec::SmallVec;

#[derive(Encode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    pub damaging_items: WrapSetU32,
    pub damaging_runes: WrapSetU32,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: Stats<i32>,
}

#[derive(Encode)]
pub struct CalculatorDamages {
    pub attacks: Attacks<i32>,
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<5, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct OutputEnemy {
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: BasicStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages<i32>,
    pub tower_damages: [i32; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[(ChampionId, OutputEnemy); 1]>,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub champion_id: ChampionId,
    pub champion_stats: Stats<i32>,
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
    pub items: SmallVec<[ItemId; SIZE_ITEMS_EXPECTED]>,
    pub level: u8,
    pub stats: BasicStats<i32>,
    pub infer_stats: bool,
    pub stacks: u32,
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputEnemyPlayers; 1]>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    pub stack_exceptions: SmallVec<[(u8, u16, u8); 5]>,
}

impl Sizer for OutputGame {
    fn size(&self) -> usize {
        let cp = &self.current_player;
        self.tower_damages.size()
            + self.monster_damages.size()
            + cp.adaptative_type.size()
            + cp.base_stats.size()
            + cp.bonus_stats.size()
            + cp.champion_id.size()
            + cp.current_stats.size()
            + cp.damaging_items.size()
            + cp.damaging_runes.size()
            + cp.level.size()
            + self.enemies.len().size()
            + self
                .enemies
                .iter()
                .map(|(k, e)| {
                    let dmgs = &e.damages;
                    k.size()
                        + e.base_stats.size()
                        + e.bonus_stats.size()
                        + e.current_stats.size()
                        + dmgs.abilities.size()
                        + dmgs.attacks.size()
                        + dmgs.items.size()
                        + dmgs.runes.size()
                        + e.level.size()
                        + e.real_armor.size()
                        + e.real_magic_resist.size()
                })
                .sum::<usize>()
    }
}
