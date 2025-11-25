use crate::*;
use bincode::{Decode, Encode};
use smallvec::SmallVec;
use tutorlolv2_gen::{
    AdaptativeType, ChampionId, ConstClosure, GameMap, ItemId, Position, RuneId, TypeMetadata,
};
use tutorlolv2_types::AbilityLike;

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

#[derive(Encode)]
pub struct Attacks {
    pub basic_attack: i32,
    pub critical_strike: i32,
    pub onhit_damage: RangeDamage,
}

pub struct ConstDamageKind<T: 'static> {
    pub metadata: &'static [TypeMetadata<T>],
    pub closures: &'static [ConstClosure],
}

pub struct DamageKind<const N: usize, T> {
    pub metadata: SmallVec<[TypeMetadata<T>; N]>,
    pub closures: SmallVec<[ConstClosure; N]>,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[Enemy<'a>; L_TEAM]>,
    pub scoreboard: SmallVec<[Scoreboard<'a>; L_PLYR]>,
    pub abilities_meta: &'static [TypeMetadata<AbilityLike>],
    pub items_meta: SmallVec<[TypeMetadata<ItemId>; L_ITEM]>,
    pub runes_meta: SmallVec<[TypeMetadata<RuneId>; L_RUNE]>,
    pub siml_meta: [TypeMetadata<ItemId>; L_SIML],
    pub abilities_to_merge: &'static [(usize, usize)],
    pub items_to_merge: SmallVec<[(usize, usize); L_ITEM]>,
    pub game_time: u32,
    pub ability_levels: AbilityLevels,
    pub dragons: Dragons,
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
    // pub _padding: u16,
}

#[derive(Clone, Copy)]
pub struct ResistShred {
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
}

pub struct EnemyState<'a> {
    pub base_stats: SimpleStats<f32>,
    pub items: BitArray,
    pub stacks: u32,
    pub champion_id: ChampionId,
    pub earth_dragons: u16,
    pub level: u8,
    pub item_exceptions: &'a [ValueException],
    // _padding: u32,
}

#[derive(Copy, Clone)]
pub struct SelfState {
    pub ability_levels: AbilityLevels,
    pub current_stats: Stats<f32>,
    pub bonus_stats: BasicStats<f32>,
    pub base_stats: BasicStats<f32>,
    pub level: u8,
    // _padding: u32 - u8
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
    // _padding: u8
}

#[derive(Encode, Decode, Clone, Copy)]
pub struct SimpleStats<T> {
    pub armor: T,
    pub health: T,
    pub magic_resist: T,
}

pub struct DamageEvalData {
    pub abilities: ConstDamageKind<AbilityLike>,
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
    pub abilities: SmallVec<[i32; L_ABLT]>,
    pub items: SmallVec<[i32; L_ITEM]>,
    pub runes: SmallVec<[i32; L_RUNE]>,
}

#[derive(Decode, Copy, Clone)]
#[repr(transparent)]
pub struct ValueException(u32);

impl ValueException {
    pub const DISC_BITS: u32 = Self::find_disc_bits(NUMBER_OF_ITEMS as u32, NUMBER_OF_RUNES as u32);
    pub const VAL_BITS: u32 = 32 - Self::DISC_BITS;
    pub const VAL_MASK: u32 = (1u32 << Self::VAL_BITS) - 1;
    pub const DISC_MASK: u32 = !Self::VAL_MASK;
    pub const DISC_LOW_MASK: u32 = (1u32 << Self::DISC_BITS) - 1;

    const fn find_disc_bits(a: u32, b: u32) -> u32 {
        u32::BITS - if a > b { a } else { b }.leading_zeros()
    }

    pub const fn stacks(&self) -> u32 {
        self.0 & Self::VAL_MASK
    }

    const fn enum_id(&self) -> u16 {
        ((self.0 >> Self::VAL_BITS) & ((1u32 << Self::DISC_BITS) - 1)) as u16
    }

    pub const fn get_rune_id(&self) -> Option<RuneId> {
        let disc = self.enum_id();
        if disc < NUMBER_OF_RUNES as u16 {
            unsafe { Some(std::mem::transmute::<u8, RuneId>(disc as u8)) }
        } else {
            None
        }
    }

    pub const fn get_item_id(&self) -> Option<ItemId> {
        let disc = self.enum_id();
        if disc < NUMBER_OF_ITEMS as u16 {
            unsafe { Some(std::mem::transmute::<u16, ItemId>(disc)) }
        } else {
            None
        }
    }

    const fn truncate_value(v: u32) -> u32 {
        v & Self::VAL_MASK
    }

    pub const fn pack_rune_id(r: RuneId, v: u32) -> Self {
        let disc = (r as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }

    pub const fn pack_item_id(i: ItemId, v: u32) -> Self {
        let disc = (i as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }
}

#[derive(Encode, Decode, Copy, Clone, Default)]
pub struct Dragons {
    pub ally_fire_dragons: u16,
    pub ally_earth_dragons: u16,
    pub ally_chemtech_dragons: u16,
    pub enemy_earth_dragons: u16,
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputMinData<SimpleStats<i32>>; L_CENM]>,
    pub dragons: Dragons,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub runes: SmallVec<[RuneId; L_RUNE]>,
    pub rune_exceptions: SmallVec<[ValueException; L_RUNE]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData<Stats<i32>>,
}

#[derive(Decode)]
pub struct InputMinData<T> {
    pub stats: T,
    pub items: SmallVec<[ItemId; L_ITEM]>,
    pub item_exceptions: SmallVec<[ValueException; L_ITEM]>,
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

#[derive(Clone, Copy)]
pub struct DamageModifiers {
    pub physical_mod: f32,
    pub magic_mod: f32,
    pub true_mod: f32,
    pub global_mod: f32,
}

#[derive(Clone, Copy)]
pub struct Modifiers {
    pub damages: DamageModifiers,
    pub abilities: AbilityModifiers,
}

macro_rules! impl_default {
    ($ty:ty, $initializer:literal) => {
        impl $ty {
            pub const fn default() -> Self {
                unsafe { std::mem::transmute([$initializer; size_of::<$ty>() / size_of::<f32>()]) }
            }
        }
    };
}

impl_default!(DamageModifiers, 1.0f32);
impl_default!(AbilityModifiers, 1.0f32);
impl_default!(Modifiers, 1.0f32);

#[derive(Clone, Copy)]
pub struct AbilityModifiers {
    pub q: f32,
    pub w: f32,
    pub e: f32,
    pub r: f32,
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
    pub abilities: SmallVec<[i32; L_ABLT]>,
    pub items: SmallVec<[i32; L_ITEM]>,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: [MonsterDamage; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[OutputEnemy; L_CENM]>,
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: &'static [TypeMetadata<AbilityLike>],
    pub abilities_to_merge: &'static [(usize, usize)],
    pub items_to_merge: SmallVec<[(usize, usize); L_ITEM]>,
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
