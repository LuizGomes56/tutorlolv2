use crate::{
    AbilityLevels, Attacks, Dragons, L_MSTR, L_TWRD, OutputCurrentPlayer, SimpleStats, Stats,
    ValueException,
};
use tutorlolv2_gen::{ChampionId, ItemId, RuneId, TypeMetadata};
use tutorlolv2_types::AbilityId;

#[derive(Debug)]
pub struct ConstInputGame<
    const _ENEMIES: usize,
    const _RUNES: usize,
    const _RUNES_EXC: usize,
    const _ITEMS: usize,
    const _ITEMS_EXC: usize,
> {
    pub active_player: ConstInputActivePlayer<_ITEMS, _ITEMS_EXC, _RUNES, _RUNES_EXC>,
    pub enemy_players: [ConstInputMinData<SimpleStats<i32>, _ITEMS, _ITEMS_EXC>; _ENEMIES],
    pub dragons: Dragons,
}

#[derive(Debug)]
pub struct ConstInputActivePlayer<
    const _ITEMS: usize,
    const _ITEMS_EXC: usize,
    const _RUNES: usize,
    const _RUNES_EXC: usize,
> {
    pub runes: [RuneId; _RUNES],
    pub rune_exceptions: [ValueException; _RUNES_EXC],
    pub abilities: AbilityLevels,
    pub data: ConstInputMinData<Stats<i32>, _ITEMS, _ITEMS_EXC>,
}

#[derive(Clone, Copy, Debug)]
pub struct ConstInputMinData<T, const _ITEMS: usize, const _ITEMS_EXC: usize> {
    pub stats: T,
    pub items: [ItemId; _ITEMS],
    pub item_exceptions: [ValueException; _ITEMS_EXC],
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

#[derive(Debug)]
pub struct ConstDamages<const _ABILITIES: usize, const _ITEMS: usize, const _RUNES: usize> {
    pub attacks: Attacks,
    pub abilities: [i32; _ABILITIES],
    pub items: [i32; _ITEMS],
    pub runes: [i32; _RUNES],
}

#[derive(Debug)]
pub struct ConstMonsterDamage<const _ABILITIES: usize, const _ITEMS: usize> {
    pub attacks: Attacks,
    pub abilities: [i32; _ABILITIES],
    pub items: [i32; _ITEMS],
}

#[derive(Debug)]
pub struct ConstOutputEnemy<const _ABILITIES: usize, const _ITEMS: usize, const _RUNES: usize> {
    pub damages: ConstDamages<_ABILITIES, _ITEMS, _RUNES>,
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

#[derive(Debug)]
pub struct ConstOutputGame<
    const _ABILITIES: usize,
    const _ENEMIES: usize,
    const _ITEMS: usize,
    const _RUNES: usize,
> {
    pub monster_damages: [ConstMonsterDamage<_ABILITIES, _ITEMS>; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: [ConstOutputEnemy<_ABILITIES, _ITEMS, _RUNES>; _ENEMIES],
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: &'static [TypeMetadata<AbilityId>],
    pub abilities_to_merge: &'static [(usize, usize)],
    pub items_meta: [TypeMetadata<ItemId>; _ITEMS],
    pub runes_meta: [TypeMetadata<RuneId>; _RUNES],
}
