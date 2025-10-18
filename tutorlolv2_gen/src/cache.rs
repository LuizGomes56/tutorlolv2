use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

use crate::{ItemId, RuneId};

#[derive(Default)]
pub struct EvalContext {
    pub q_level: u8,
    pub w_level: u8,
    pub e_level: u8,
    pub r_level: u8,
    pub level: f32,
    pub chogath_stacks: f32,
    pub veigar_stacks: f32,
    pub nasus_stacks: f32,
    pub smolder_stacks: f32,
    pub aurelion_sol_stacks: f32,
    pub thresh_stacks: f32,
    pub kindred_stacks: f32,
    pub belveth_stacks: f32,
    pub adaptative_damage: f32,
    pub physical_multiplier: f32,
    pub magic_multiplier: f32,
    pub steelcaps_effect: f32,
    pub randuin_effect: f32,
    pub rocksolid_effect: f32,
    pub enemy_bonus_health: f32,
    pub enemy_armor: f32,
    pub enemy_max_health: f32,
    pub enemy_health: f32,
    pub enemy_current_health: f32,
    pub enemy_missing_health: f32,
    pub enemy_magic_resist: f32,
    pub base_health: f32,
    pub base_ad: f32,
    pub base_armor: f32,
    pub base_magic_resist: f32,
    pub base_mana: f32,
    pub bonus_ad: f32,
    pub bonus_armor: f32,
    pub bonus_magic_resist: f32,
    pub bonus_health: f32,
    pub bonus_mana: f32,
    pub bonus_move_speed: f32,
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub max_mana: f32,
    pub current_mana: f32,
    pub max_health: f32,
    pub current_health: f32,
    pub armor: f32,
    pub magic_resist: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub attack_speed: f32,
    pub missing_health: f32,
    pub ap: f32,
    pub ad: f32,
}

#[derive(Default, Copy, Serialize, Deserialize, Clone, Encode, Decode)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    #[default]
    Unknown,
}

impl<T: AsRef<str>> From<T> for DamageType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "PHYSICAL_DAMAGE" => DamageType::Physical,
            "MAGIC_DAMAGE" => DamageType::Magic,
            "MIXED_DAMAGE" => DamageType::Mixed,
            "TRUE_DAMAGE" => DamageType::True,
            "ADAPTATIVE_DAMAGE" => DamageType::Adaptative,
            _ => DamageType::Unknown,
        }
    }
}

#[derive(Copy, Clone, Encode, Serialize, Deserialize)]
pub enum Attrs {
    None,
    Onhit,
    OnhitMin,
    OnhitMax,
    Area,
    AreaOnhit,
    AreaOnhitMin,
    AreaOnhitMax,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum AttackType {
    Melee,
    Ranged,
}

impl<T: AsRef<str>> From<T> for AttackType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "MELEE" => AttackType::Melee,
            "RANGED" => AttackType::Ranged,
            _ => AttackType::Melee,
        }
    }
}

impl<T: AsRef<str>> From<T> for AdaptativeType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "PHYSICAL_DAMAGE" => AdaptativeType::Physical,
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

#[derive(Copy, Clone, Encode)]
pub struct TypeMetadata<T> {
    pub kind: T,
    pub damage_type: DamageType,
    pub attributes: Attrs,
}

pub type ConstClosure = fn(&EvalContext) -> f32;

#[derive(Copy, Clone)]
pub struct DamageClosures {
    pub minimum_damage: ConstClosure,
    pub maximum_damage: ConstClosure,
}

pub struct CachedChampion {
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub metadata: &'static [TypeMetadata<AbilityLike>],
    pub closures: &'static [fn(&EvalContext) -> f32],
}

pub struct CachedChampionAbility {
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub minimum_damage: fn(u8, &EvalContext) -> f32,
    pub maximum_damage: fn(u8, &EvalContext) -> f32,
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
    pub minimum_damage: fn(u8, &EvalContext) -> f32,
    pub maximum_damage: fn(u8, &EvalContext) -> f32,
}

pub enum StatName {
    AbilityHaste(u16),
    AbilityPower(u16),
    Armor(u16),
    ArmorPenetration(u16),
    MagicPenetration(u16),
    AttackDamage(u16),
    AttackSpeed(u16),
    GoldPer10Seconds(u16),
    AdaptiveForce(u16),
    CriticalStrikeChance(u16),
    CriticalStrikeDamage(u16),
    Health(u16),
    LifeSteal(u16),
    MagicResist(u16),
    Mana(u16),
    MoveSpeed(u16),
    Omnivamp(u16),
    BaseHealthRegen(u16),
    BaseManaRegen(u16),
    Tenacity(u16),
    HealAndShieldPower(u16),
}

pub struct CachedItem {
    pub gold: u16,
    pub prettified_stats: &'static [StatName],
    pub damage_type: DamageType,
    pub stats: CachedItemStats,
    pub metadata: TypeMetadata<ItemId>,
    pub range_closure: DamageClosures,
    pub melee_closure: DamageClosures,
    pub attributes: Attrs,
}

pub struct CachedRune {
    pub damage_type: DamageType,
    pub metadata: TypeMetadata<RuneId>,
    pub range_closure: ConstClosure,
    pub melee_closure: ConstClosure,
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
