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

#[derive(Copy, Clone)]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

macro_rules! impl_key {
    ($field:ident) => {
        paste::paste! {
            impl AbilityLike {
                pub fn [<to_str_ $field>](&self) -> &'static str {
                    match self {
                        AbilityLike::[<$field:upper>](v) => {
                            match v {
                                AbilityName::_1 => stringify!(concat!($field, "1")),
                                AbilityName::_2 => stringify!(concat!($field, "2")),
                                AbilityName::_3 => stringify!(concat!($field, "3")),
                                AbilityName::_4 => stringify!(concat!($field, "4")),
                                AbilityName::_5 => stringify!(concat!($field, "5")),
                                AbilityName::_6 => stringify!(concat!($field, "6")),
                                AbilityName::_7 => stringify!(concat!($field, "7")),
                                AbilityName::_8 => stringify!(concat!($field, "8")),
                                AbilityName::Mega => stringify!(concat!($field, "MEGA")),
                                AbilityName::Max => stringify!(concat!($field, "MAX")),
                                AbilityName::Min => stringify!(concat!($field, "MIN")),
                                AbilityName::Minion => stringify!(concat!($field, "MNX")),
                                AbilityName::MinionMax => stringify!(concat!($field, "MMNX")),
                                AbilityName::Monster => stringify!(concat!($field, "MSTR")),
                                AbilityName::MonsterMax => stringify!(concat!($field, "MMST")),
                                AbilityName::None => stringify!($field),
                                AbilityName::_1Max => stringify!(concat!($field, "MAX1")),
                                AbilityName::_2Max => stringify!(concat!($field, "MAX2")),
                                AbilityName::_3Max => stringify!(concat!($field, "MAX3")),
                                AbilityName::_4Max => stringify!(concat!($field, "MAX4")),
                                AbilityName::_5Max => stringify!(concat!($field, "MAX5")),
                                AbilityName::_6Max => stringify!(concat!($field, "MAX6")),
                                AbilityName::_7Max => stringify!(concat!($field, "MAX7")),
                                AbilityName::_8Max => stringify!(concat!($field, "MAX8")),
                                AbilityName::_1Min => stringify!(concat!($field, "MIN1")),
                                AbilityName::_2Min => stringify!(concat!($field, "MIN2")),
                                AbilityName::_3Min => stringify!(concat!($field, "MIN3")),
                                AbilityName::_4Min => stringify!(concat!($field, "MIN4")),
                                AbilityName::_5Min => stringify!(concat!($field, "MIN5")),
                                AbilityName::_6Min => stringify!(concat!($field, "MIN6")),
                                AbilityName::_7Min => stringify!(concat!($field, "MIN7")),
                                AbilityName::_8Min => stringify!(concat!($field, "MIN8")),
                            }
                        },
                        _ => {
                            "UNDF"
                        }
                    }
                }
            }
        }
    };
}

impl_key!(p);
impl_key!(q);
impl_key!(w);
impl_key!(e);
impl_key!(r);

pub enum AbilityName {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Mega,
    Max,
    Min,
    Minion,
    MinionMax,
    Monster,
    MonsterMax,
    None,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
}

pub struct CachedChampion {
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub abilities: &'static [(&'static str, CachedChampionAbility)],
}

struct _V((AbilityLike, CachedChampionAbility));

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
    pub damage_type: Option<DamageType>,
    pub stats: CachedItemStats,
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
