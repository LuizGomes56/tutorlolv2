use crate::{ItemId, RuneId};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

macro_rules! create_eval_ident {
    (u8($($bytes:ident),* $(,)?), f32($($floats:ident),* $(,)?)) => {
        tutorlolv2_types::paste! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum EvalIdent {
                $(
                    [<$bytes:camel>],
                )*
                $(
                    [<$floats:camel>],
                )*
            }

            #[derive(Default, Debug, Copy, Clone)]
            pub struct EvalContext {
                $(
                    pub $bytes: u8,
                )*
                $(
                    pub $floats: f32,
                )*
            }

            impl ::core::fmt::Display for EvalIdent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        $(
                            Self::[<$bytes:camel>]  => write!(f, concat!("ctx.", stringify!($bytes))),
                        )*
                        $(
                            Self::[<$floats:camel>] => write!(f, concat!("ctx.", stringify!($floats))),
                        )*
                    }
                }
            }
        }
    };
}

create_eval_ident!(
    u8(q_level, w_level, e_level, r_level),
    f32(
        level,
        chogath_stacks,
        veigar_stacks,
        nasus_stacks,
        smolder_stacks,
        aurelion_sol_stacks,
        thresh_stacks,
        kindred_stacks,
        belveth_stacks,
        adaptative_damage,
        physical_multiplier,
        magic_multiplier,
        steelcaps_effect,
        randuin_effect,
        rocksolid_effect,
        enemy_bonus_health,
        enemy_armor,
        enemy_max_health,
        enemy_health,
        enemy_current_health,
        enemy_missing_health,
        enemy_magic_resist,
        base_health,
        base_ad,
        base_armor,
        base_magic_resist,
        base_mana,
        bonus_ad,
        bonus_armor,
        bonus_magic_resist,
        bonus_health,
        bonus_mana,
        bonus_move_speed,
        armor_penetration_flat,
        armor_penetration_percent,
        magic_penetration_flat,
        magic_penetration_percent,
        max_mana,
        current_mana,
        max_health,
        current_health,
        armor,
        magic_resist,
        crit_chance,
        crit_damage,
        attack_speed,
        missing_health,
        ap,
        ad
    )
);

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

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize)]
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
    pub range_closure: &'static [ConstClosure],
    pub melee_closure: &'static [ConstClosure],
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
