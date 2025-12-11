use crate::{Attrs, DamageType, ItemId, RuneId, eval::EvalContext};
use bincode::Encode;
use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttackType {
    Melee,
    Ranged,
}

impl<T: AsRef<str>> From<T> for AttackType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "RANGED" => AttackType::Ranged,
            _ => AttackType::Melee,
        }
    }
}

impl<T: AsRef<str>> From<T> for AdaptativeType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "MAGIC_DAMAGE" => AdaptativeType::Magic,
            _ => AdaptativeType::Physical,
        }
    }
}

#[derive(Encode, Copy, Clone, Serialize, Deserialize)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

#[derive(Copy, Clone, Encode, Serialize, Deserialize, Default)]
pub enum Position {
    #[default]
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

impl Position {
    pub fn from_raw(raw: &str) -> Option<Self> {
        match raw {
            "TOP" => Some(Position::Top),
            "JUNGLE" => Some(Position::Jungle),
            "MIDDLE" => Some(Position::Middle),
            "BOTTOM" => Some(Position::Bottom),
            "SUPPORT" => Some(Position::Support),
            _ => None,
        }
    }
}

#[derive(Encode, Deserialize, Serialize)]
pub enum GameMap {
    SummonersRift,
    Tutorial,
    TwistedTreeline,
    Dominion,
    Aram,
    DarkStar,
    Invasion,
    Project,
    StarGuardian,
    Odyssey,
    NexusBlitz,
    Tft,
    Arena,
    Urf,
}

impl From<u8> for GameMap {
    fn from(value: u8) -> Self {
        match value {
            3 => GameMap::Tutorial,
            4 | 10 => GameMap::TwistedTreeline,
            8 => GameMap::Dominion,
            12 | 14 => GameMap::Aram,
            13 => GameMap::Invasion,
            16 => GameMap::DarkStar,
            18 => GameMap::StarGuardian,
            19 => GameMap::Project,
            20 => GameMap::Odyssey,
            21 => GameMap::NexusBlitz,
            22 => GameMap::Tft,
            30 => GameMap::Arena,
            // Unknown
            0xFF => GameMap::Urf,
            /* 1 | 2 | 11 | */ _ => GameMap::SummonersRift,
        }
    }
}

#[derive(Copy, Clone, Encode)]
pub struct TypeMetadata<T> {
    pub kind: T,
    pub damage_type: DamageType,
    pub attributes: Attrs,
}

pub type ConstClosure = fn(&EvalContext) -> f32;

pub struct CachedChampion {
    pub name: &'static str,
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub metadata: &'static [TypeMetadata<AbilityLike>],
    pub closures: &'static [ConstClosure],
    pub merge_data: &'static [(usize, usize)],
}

pub struct CachedChampionAbility {
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}

pub struct CachedChampionStatsMap {
    pub flat: f32,
    pub per_level: f32,
}

pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resist: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: f32,
    pub critical_strike_damage: f32,
    pub critical_strike_damage_modifier: f32,
    pub attack_speed_ratio: f32,
    pub attack_range: f32,
    pub aram_damage_taken: f32,
    pub aram_damage_dealt: f32,
    pub urf_damage_taken: f32,
    pub urf_damage_dealt: f32,
}

pub struct CachedItemDamages {
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}

pub struct CachedItem {
    pub gold: u16,
    pub prettified_stats: &'static [StatName],
    pub damage_type: DamageType,
    pub stats: CachedItemStats,
    pub metadata: TypeMetadata<ItemId>,
    pub range_closure: &'static [ConstClosure],
    pub melee_closure: &'static [ConstClosure],
    pub attributes: Attrs,
    pub is_simulated: bool,
    pub is_damaging: bool,
    pub riot_id: u32,
}

pub struct CachedRune {
    pub damage_type: DamageType,
    pub metadata: TypeMetadata<RuneId>,
    pub range_closure: ConstClosure,
    pub melee_closure: ConstClosure,
    pub riot_id: u32,
    pub undeclared: bool,
}

pub struct CachedItemStats {
    pub ability_power: f32,
    pub armor: f32,
    pub armor_penetration_percent: f32,
    pub armor_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub health: f32,
    pub lifesteal: f32,
    pub magic_resist: f32,
    pub mana: f32,
    pub movespeed: f32,
    pub omnivamp: f32,
}

#[inline(always)]
pub const fn zero(_: &EvalContext) -> f32 {
    0.0
}

#[derive(Copy, Clone)]
pub struct DamageExpression {
    pub attributes: Attrs,
    pub damage_type: DamageType,
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}
