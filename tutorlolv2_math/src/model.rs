use crate::*;
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

#[derive(Encode, Default)]
pub struct RangeDamage {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

#[derive(Encode, Clone, Copy)]
pub struct BasicStats<T> {
    pub armor: T,
    pub health: T,
    pub attack_damage: T,
    pub magic_resist: T,
    pub mana: T,
}

#[derive(Decode, Default)]
pub struct Dragons {
    pub earth: u8,
    pub fire: u8,
}

#[derive(Encode)]
pub struct Attacks {
    pub basic_attack: RangeDamage,
    pub critical_strike: RangeDamage,
    pub onhit_damage: RangeDamage,
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
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[Enemy<'a>; L_TEAM]>,
    pub scoreboard: SmallVec<[Scoreboard<'a>; L_PLYR]>,
    pub abilities_meta: SmallVec<[TypeMetadata<AbilityLike>; L_ABLT]>,
    pub items_meta: SmallVec<[TypeMetadata<ItemId>; L_ITEM]>,
    pub runes_meta: SmallVec<[TypeMetadata<RuneId>; L_RUNE]>,
    pub siml_meta: [TypeMetadata<ItemId>; L_SIML],
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
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: Stats<i32>,
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
    pub base_stats: SimpleStats<f32>,
    pub items: SetU32,
    pub stacks: u32,
    pub champion_id: ChampionId,
    pub level: u8,
}

#[derive(Copy, Clone)]
pub struct SelfState {
    pub current_stats: Stats<f32>,
    pub bonus_stats: BasicStats<f32>,
    pub base_stats: BasicStats<f32>,
    pub level: u8,
}

pub struct EnemyFullState {
    pub current_stats: SimpleStats<f32>,
    pub bonus_stats: SimpleStats<f32>,
    pub modifiers: DamageModifiers,
    pub armor_values: ResistValue,
    pub magic_values: ResistValue,
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
}

#[derive(Encode, Decode, Clone, Copy)]
pub struct SimpleStats<T> {
    pub armor: T,
    pub health: T,
    pub magic_resist: T,
}

pub struct DamageEvalData {
    pub abilities: DamageKind<L_ABLT, AbilityLike>,
    pub items: DamageKind<L_ITEM, ItemId>,
    pub runes: DamageKind<L_RUNE, RuneId>,
}

#[derive(Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub damages: Damages,
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
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
    pub abilities: SmallVec<[RangeDamage; L_ABLT]>,
    pub items: SmallVec<[RangeDamage; L_ITEM]>,
    pub runes: SmallVec<[RangeDamage; L_RUNE]>,
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
    pub enemy_players: SmallVec<[InputMinData<SimpleStats<i32>>; L_CENM]>,
    pub stack_exceptions: SmallVec<[StackException; L_STCK]>,
    pub ally_dragons: Dragons,
    pub enemy_earth_dragons: u8,
    // pub padding: u32 + u8,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub runes: SmallVec<[RuneId; L_RUNE]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData<Stats<i32>>,
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
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
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
    pub current_stats: Stats<i32>,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: SmallVec<[RangeDamage; L_ABLT]>,
    pub items: SmallVec<[RangeDamage; L_ITEM]>,
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

macro_rules! impl_cast_from {
    ($stru:ident, $($fields:ident),*) => {
        impl From<$stru<f32>> for $stru<i32> {
            fn from(value: $stru<f32>) -> Self {
                $stru {
                    $($fields: value.$fields as i32),*
                }
            }
        }

        impl From<$stru<i32>> for $stru<f32> {
            fn from(value: $stru<i32>) -> Self {
                $stru {
                    $($fields: value.$fields as f32),*
                }
            }
        }
    };
}

impl_cast_from!(
    Stats,
    ability_power,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_range,
    attack_speed,
    crit_chance,
    crit_damage,
    current_health,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    health,
    mana,
    current_mana
);
impl_cast_from!(SimpleStats, health, armor, magic_resist);
impl_cast_from!(BasicStats, armor, health, attack_damage, magic_resist, mana);
