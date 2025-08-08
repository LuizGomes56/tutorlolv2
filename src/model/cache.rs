use serde::{Deserialize, Serialize};

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

#[derive(Copy, Clone, Serialize, Default)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    #[default]
    Unknown,
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

#[derive(Copy, Clone)]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

pub struct CachedChampion {
    pub name: &'static str,
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub abilities: &'static [(&'static str, CachedChampionAbility)],
}

pub struct CachedChampionAbility {
    pub name: &'static str,
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
    pub movespeed: CachedChampionStatsMap,
    pub critical_strike_damage: CachedChampionStatsMap,
    pub critical_strike_damage_modifier: CachedChampionStatsMap,
    pub attack_speed_ratio: CachedChampionStatsMap,
    pub attack_range: CachedChampionStatsMap,
    pub aram_damage_taken: CachedChampionStatsMap,
    pub aram_damage_dealt: CachedChampionStatsMap,
    pub urf_damage_taken: CachedChampionStatsMap,
    pub urf_damage_dealt: CachedChampionStatsMap,
}

pub struct CachedItemDamages {
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub struct CachedItem {
    pub name: &'static str,
    pub gold: u16,
    pub tier: u8,
    pub prettified_stats: &'static [(&'static str, f64)],
    pub damage_type: Option<DamageType>,
    pub stats: CachedItemStats,
    pub builds_from: &'static [u32],
    pub ranged: CachedItemDamages,
    pub melee: CachedItemDamages,
    pub attributes: Attrs,
}

pub struct CachedMetaItem {
    pub jungle: &'static [u32],
    pub top: &'static [u32],
    pub mid: &'static [u32],
    pub adc: &'static [u32],
    pub support: &'static [u32],
}

pub struct CachedRune {
    pub name: &'static str,
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
pub const fn __zero(_: u8, _: &EvalContext) -> f64 {
    0.0
}
