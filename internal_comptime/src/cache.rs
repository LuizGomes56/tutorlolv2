use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use shared_types::*;

#[derive(Default)]
pub struct EvalContext {
    pub chogath_stacks: f32,
    pub veigar_stacks: f32,
    pub nasus_stacks: f32,
    pub smolder_stacks: f32,
    pub aurelion_sol_stacks: f32,
    pub thresh_stacks: f32,
    pub kindred_stacks: f32,
    pub belveth_stacks: f32,
    pub adaptative_damage: f32,
    pub level: f32,
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

#[derive(Default, Copy, Clone, Encode, Decode)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    #[default]
    Unknown,
}

impl ToString for DamageType {
    fn to_string(&self) -> String {
        let res = match self {
            DamageType::Physical => "PHYSICAL_DAMAGE",
            DamageType::Magic => "MAGIC_DAMAGE",
            DamageType::Mixed => "MIXED_DAMAGE",
            DamageType::True => "TRUE_DAMAGE",
            DamageType::Adaptative => "ADAPTATIVE_DAMAGE",
            DamageType::Unknown => "UNKNOWN_DAMAGE",
        };
        res.to_string()
    }
}

#[derive(Copy, Clone, Encode, Serialize, Deserialize)]
pub enum Attrs {
    None,
    Onhit,
    OnhitMin,
    OnhitMax,
}

#[derive(Copy, Clone)]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Encode, Copy, Clone)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

#[derive(Copy, Clone, Encode)]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

impl Position {
    pub fn from_raw(raw: &str) -> Self {
        match raw {
            "TOP" => Position::Top,
            "JUNGLE" => Position::Jungle,
            "MIDDLE" => Position::Middle,
            "BOTTOM" => Position::Bottom,
            "SUPPORT" => Position::Support,
            _ => Position::Top,
        }
    }
}

pub struct CachedChampion {
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub abilities: &'static [(AbilityLike, CachedChampionAbility)],
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
    pub magic_resistance: CachedChampionStatsMap,
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
    pub ranged: CachedItemDamages,
    pub melee: CachedItemDamages,
    pub attributes: Attrs,
}

pub struct CachedRune {
    pub damage_type: DamageType,
    pub ranged: fn(u8, &EvalContext) -> f32,
    pub melee: fn(u8, &EvalContext) -> f32,
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
    pub critical_strike_chance: f32,
    pub critical_strike_damage: f32,
    pub health: f32,
    pub lifesteal: f32,
    pub magic_resistance: f32,
    pub mana: f32,
    pub movespeed: f32,
    pub omnivamp: f32,
}

#[inline(always)]
pub const fn zero(_: u8, _: &EvalContext) -> f32 {
    0.0
}

#[derive(Copy, Clone)]
pub struct DamageExpression {
    pub level: u8,
    pub attributes: Attrs,
    pub damage_type: DamageType,
    pub minimum_damage: fn(u8, &EvalContext) -> f32,
    pub maximum_damage: fn(u8, &EvalContext) -> f32,
}
