use serde::{Deserialize, Serialize};
use shared_types::*;

pub struct EvalContext {
    pub chogath_stacks: f64,
    pub veigar_stacks: f64,
    pub nasus_stacks: f64,
    pub smolder_stacks: f64,
    pub aurelion_sol_stacks: f64,
    pub thresh_stacks: f64,
    pub kindred_stacks: f64,
    pub belveth_stacks: f64,
    pub adaptative_damage: f64,
    pub level: f64,
    pub physical_multiplier: f64,
    pub magic_multiplier: f64,
    pub steelcaps_effect: f64,
    pub randuin_effect: f64,
    pub rocksolid_effect: f64,
    pub enemy_bonus_health: f64,
    pub enemy_armor: f64,
    pub enemy_max_health: f64,
    pub enemy_health: f64,
    pub enemy_current_health: f64,
    pub enemy_missing_health: f64,
    pub enemy_magic_resist: f64,
    pub base_health: f64,
    pub base_ad: f64,
    pub base_armor: f64,
    pub base_magic_resist: f64,
    pub base_mana: f64,
    pub bonus_ad: f64,
    pub bonus_armor: f64,
    pub bonus_magic_resist: f64,
    pub bonus_health: f64,
    pub bonus_mana: f64,
    pub bonus_move_speed: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub max_mana: f64,
    pub current_mana: f64,
    pub max_health: f64,
    pub current_health: f64,
    pub armor: f64,
    pub magic_resist: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub attack_speed: f64,
    pub missing_health: f64,
    pub ap: f64,
    pub ad: f64,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Copy, Clone)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

#[derive(Copy, Clone, Serialize)]
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
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub struct CachedChampionStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resistance: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: f64,
    pub critical_strike_damage: f64,
    pub critical_strike_damage_modifier: f64,
    pub attack_speed_ratio: f64,
    pub attack_range: f64,
    pub aram_damage_taken: f64,
    pub aram_damage_dealt: f64,
    pub urf_damage_taken: f64,
    pub urf_damage_dealt: f64,
}

pub struct CachedItemDamages {
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub enum StatName {
    AbilityHaste(f64),
    AbilityPower(f64),
    Armor(f64),
    ArmorPenetration(f64),
    MagicPenetration(f64),
    AttackDamage(f64),
    AttackSpeed(f64),
    GoldPer10Seconds(f64),
    AdaptiveForce(f64),
    CriticalStrikeChance(f64),
    CriticalStrikeDamage(f64),
    Health(f64),
    LifeSteal(f64),
    MagicResist(f64),
    Mana(f64),
    MoveSpeed(f64),
    Omnivamp(f64),
    BaseHealthRegen(f64),
    BaseManaRegen(f64),
    Tenacity(f64),
    HealAndShieldPower(f64),
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
    pub ranged: fn(u8, &EvalContext) -> f64,
    pub melee: fn(u8, &EvalContext) -> f64,
}

pub struct CachedItemStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_percent: f64,
    pub armor_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub critical_strike_chance: f64,
    pub critical_strike_damage: f64,
    pub health: f64,
    pub lifesteal: f64,
    pub magic_resistance: f64,
    pub mana: f64,
    pub movespeed: f64,
    pub omnivamp: f64,
}

#[inline(always)]
pub const fn zero(_: u8, _: &EvalContext) -> f64 {
    0.0
}

#[derive(Copy, Clone)]
pub struct DamageExpression {
    pub level: u8,
    pub attributes: Attrs,
    pub damage_type: DamageType,
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

// macro_rules! impl_unsafe_cast {
//     ($t:ty, $n:ty) => {
//         impl $t {
//             pub fn unsafe_cast(n: $n) -> $t {
//                 unsafe { std::mem::transmute(n) }
//             }
//         }
//     };
// }

// impl_unsafe_cast!(ChampionId, u8);
// impl_unsafe_cast!(ItemId, u16);
// impl_unsafe_cast!(RuneId, u8);
