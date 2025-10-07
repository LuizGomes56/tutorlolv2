use crate::__v2::{
    AbilityLevels, GameMap, L_ABLT, L_CENM, L_ITEM, L_MSTR, L_PLYR, L_RUNE, L_SIML, L_STCK, L_TEAM,
    L_TWRD, ResistValue, riot::StatsF32,
};
use bincode::{Decode, Encode};
use smallvec::SmallVec;
use tinyset::SetU32;
use tutorlolv2_gen::{
    AbilityLike, AdaptativeType, Attrs, ChampionId, DamageType, EvalContext, ItemId, Position,
    RuneId,
};

#[derive(Encode, PartialEq, Clone, Copy)]
pub enum Team {
    Blue,
    Red,
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        match value {
            "ORDER" => Team::Blue,
            _ => Team::Red,
        }
    }
}

pub struct RangeDamageF32 {
    pub minimum_damage: f32,
    pub maximum_damage: f32,
}

#[derive(Encode, Default)]
pub struct RangeDamageI32 {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

#[derive(Encode)]
pub struct StatsI32 {
    pub ability_power: i32,
    pub armor: i32,
    pub armor_penetration_flat: i32,
    pub armor_penetration_percent: i32,
    pub attack_damage: i32,
    pub attack_range: i32,
    pub attack_speed: i32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub current_health: i32,
    pub magic_penetration_flat: i32,
    pub magic_penetration_percent: i32,
    pub magic_resist: i32,
    pub health: i32,
    pub mana: i32,
    pub current_mana: i32,
}

#[derive(Clone, Copy)]
pub struct BasicStatsF32 {
    pub armor: f32,
    pub health: f32,
    pub attack_damage: f32,
    pub magic_resist: f32,
    pub mana: f32,
}

#[derive(Encode)]
pub struct BasicStatsI32 {
    pub armor: i32,
    pub health: i32,
    pub attack_damage: i32,
    pub magic_resist: i32,
    pub mana: i32,
}

#[derive(Decode, Default)]
pub struct Dragons {
    pub earth: u8,
    pub fire: u8,
}

#[derive(Encode)]
pub struct Attacks {
    pub basic_attack: RangeDamageI32,
    pub critical_strike: RangeDamageI32,
    pub onhit_damage: RangeDamageI32,
}

#[derive(Encode)]
pub struct TypeMetadata<T> {
    pub level: u8,
    pub kind: T,
    pub meta: Meta,
}

#[derive(Encode, Clone, Copy)]
pub struct Meta(pub u8);

impl Meta {
    pub const fn from_bytes(damage_type: DamageType, attributes: Attrs) -> Self {
        Self(((damage_type as u8 & 0b0000_0111) << 5) | attributes as u8 & 0b0001_1111)
    }
    pub const fn damage_type(&self) -> DamageType {
        unsafe { std::mem::transmute((self.0 >> 5) & 0b0000_0111) }
    }
    pub const fn attributes(&self) -> Attrs {
        unsafe { std::mem::transmute(self.0 & 0b0001_1111) }
    }
}

pub struct DamageClosure {
    pub minimum_damage: fn(u8, &EvalContext) -> f32,
    pub maximum_damage: fn(u8, &EvalContext) -> f32,
}

pub struct DamageKind<const N: usize, T> {
    pub metadata: SmallVec<[TypeMetadata<T>; N]>,
    pub closures: SmallVec<[DamageClosure; N]>,
}

#[derive(Encode)]
pub struct ConstItemMetadata {
    pub kind: ItemId,
    pub meta: Meta,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[Enemy<'a>; L_TEAM]>,
    pub scoreboard: SmallVec<[Scoreboard<'a>; L_PLYR]>,
    pub abilities_meta: SmallVec<[TypeMetadata<AbilityLike>; L_ABLT]>,
    pub items_meta: SmallVec<[TypeMetadata<ItemId>; L_ITEM]>,
    pub runes_meta: SmallVec<[TypeMetadata<RuneId>; L_RUNE]>,
    pub siml_meta: [ConstItemMetadata; L_SIML],
    pub game_time: u32,
    pub ability_levels: AbilityLevels,
}

#[derive(Encode)]
pub struct Scoreboard<'a> {
    pub riot_id: &'a str,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

#[derive(Encode)]
pub struct CurrentPlayer<'a> {
    pub riot_id: &'a str,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub current_stats: StatsI32,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub game_map: GameMap,
}

#[derive(Clone, Copy)]
pub struct ResistShred {
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
}

pub struct EnemyState {
    pub base_stats: SimpleStatsF32,
    pub items: SetU32,
    pub stacks: u32,
    pub champion_id: ChampionId,
    pub level: u8,
}

#[derive(Copy, Clone)]
pub struct SelfState {
    pub current_stats: StatsF32,
    pub bonus_stats: BasicStatsF32,
    pub base_stats: BasicStatsF32,
    pub level: u8,
}

pub struct EnemyFullState {
    pub current_stats: SimpleStatsF32,
    pub bonus_stats: SimpleStatsF32,
    pub modifiers: DamageModifiers,
    pub armor_values: ResistValue,
    pub magic_values: ResistValue,
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
}

#[derive(Decode, Clone, Copy)]
pub struct SimpleStatsF32 {
    pub armor: f32,
    pub health: f32,
    pub magic_resist: f32,
}

pub struct DamageEvalData {
    pub abilities: DamageKind<L_ABLT, AbilityLike>,
    pub items: DamageKind<L_ITEM, ItemId>,
    pub runes: DamageKind<L_RUNE, RuneId>,
}

#[derive(Encode)]
pub struct SimpleStatsI32 {
    pub armor: i32,
    pub health: i32,
    pub magic_resist: i32,
}

#[derive(Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub damages: Damages,
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStatsI32,
    pub bonus_stats: SimpleStatsI32,
    pub current_stats: SimpleStatsI32,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub team: Team,
    pub position: Position,
}

#[derive(Encode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: SmallVec<[RangeDamageI32; L_ABLT]>,
    pub items: SmallVec<[RangeDamageI32; L_ITEM]>,
    pub runes: SmallVec<[RangeDamageI32; L_RUNE]>,
}

#[derive(Decode)]
pub struct StackExceptionKind<T> {
    pub kind: T,
    pub stacks: u16,
    pub offset: u8,
}

#[derive(Decode)]
pub enum StackException {
    Item(StackExceptionKind<ItemId>),
    Rune(StackExceptionKind<RuneId>),
    Champion(StackExceptionKind<ChampionId>),
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputMinData<SimpleStatsF32>; L_CENM]>,
    pub stack_exceptions: SmallVec<[StackException; L_STCK]>,
    pub ally_dragons: Dragons,
    pub enemy_earth_dragons: u8,
    // pub padding: u32 + u8,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub runes: SmallVec<[RuneId; L_RUNE]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData<StatsF32>,
}

#[derive(Decode)]
pub struct InputMinData<T> {
    pub stats: T,
    pub items: SmallVec<[ItemId; L_ITEM]>,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStatsI32,
    pub bonus_stats: SimpleStatsI32,
    pub current_stats: SimpleStatsI32,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

#[derive(Clone, Copy, Default)]
pub struct DamageModifiers {
    pub physical_mod: f32,
    pub magic_mod: f32,
    pub true_mod: f32,
    pub global_mod: f32,
}

#[derive(Encode)]
pub struct OutputCurrentPlayer {
    pub current_stats: StatsI32,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: SmallVec<[RangeDamageI32; L_ABLT]>,
    pub items: SmallVec<[RangeDamageI32; L_ITEM]>,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: [MonsterDamage; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[OutputEnemy; L_CENM]>,
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: SmallVec<[TypeMetadata<AbilityLike>; L_ABLT]>,
    pub items_meta: SmallVec<[TypeMetadata<ItemId>; L_ITEM]>,
    pub runes_meta: SmallVec<[TypeMetadata<RuneId>; L_RUNE]>,
}

macro_rules! transmute_from {
    ($type1:ty, $type2:ty) => {
        impl From<$type1> for $type2 {
            fn from(value: $type1) -> Self {
                unsafe { std::mem::transmute(value) }
            }
        }
    };
}

transmute_from!(StatsF32, StatsI32);
transmute_from!(BasicStatsF32, BasicStatsI32);
transmute_from!(SimpleStatsF32, SimpleStatsI32);
