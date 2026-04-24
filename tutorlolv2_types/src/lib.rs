#![no_std]

#[cfg(feature = "dev")]
extern crate alloc;
#[cfg(feature = "dev")]
use alloc::{format, string::String};
use bincode::{Decode, Encode};
use core::{convert::Infallible, fmt::Display, ops::Index, str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Key {
    P,
    Q,
    W,
    E,
    R,
}

impl Key {
    pub const fn as_char(&self) -> char {
        match self {
            Key::P => 'P',
            Key::Q => 'Q',
            Key::W => 'W',
            Key::E => 'E',
            Key::R => 'R',
        }
    }
}

/// Enum that represents one ability of a champion, with a custom display name.
/// - [`AbilityId::P`] represents the passive of a champion
/// - Other variants correspond to the abilities `Q`, `W`, `E`, and `R` (ultimate)
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub enum AbilityId {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl From<Key> for AbilityId {
    fn from(value: Key) -> Self {
        AbilityId::from_key_fn(value)(AbilityName::Void)
    }
}

impl From<AbilityId> for Key {
    fn from(value: AbilityId) -> Self {
        match value {
            AbilityId::P(_) => Key::P,
            AbilityId::Q(_) => Key::Q,
            AbilityId::W(_) => Key::W,
            AbilityId::E(_) => Key::E,
            AbilityId::R(_) => Key::R,
        }
    }
}

impl AbilityId {
    pub const fn const_eq(self, other: AbilityId) -> bool {
        unsafe {
            let a: [u8; 2] = core::mem::transmute(self);
            let b: [u8; 2] = core::mem::transmute(other);
            a[0] == b[0] && a[1] == b[1]
        }
    }

    pub const fn setter(&self) -> fn(AbilityName) -> Self {
        match self {
            AbilityId::P(_) => AbilityId::P,
            AbilityId::Q(_) => AbilityId::Q,
            AbilityId::W(_) => AbilityId::W,
            AbilityId::E(_) => AbilityId::E,
            AbilityId::R(_) => AbilityId::R,
        }
    }

    pub const fn from_key_fn(key: Key) -> fn(AbilityName) -> Self {
        match key {
            Key::P => AbilityId::P,
            Key::Q => AbilityId::Q,
            Key::W => AbilityId::W,
            Key::E => AbilityId::E,
            Key::R => AbilityId::R,
        }
    }

    pub const fn as_key(&self) -> Key {
        match self {
            AbilityId::P(_) => Key::P,
            AbilityId::Q(_) => Key::Q,
            AbilityId::W(_) => Key::W,
            AbilityId::E(_) => Key::E,
            AbilityId::R(_) => Key::R,
        }
    }

    pub const fn as_char(&self) -> char {
        self.as_key().as_char()
    }

    pub const fn ability_name(&self) -> AbilityName {
        match self {
            AbilityId::P(v) => *v,
            AbilityId::Q(v) => *v,
            AbilityId::W(v) => *v,
            AbilityId::E(v) => *v,
            AbilityId::R(v) => *v,
        }
    }
}

#[cfg(feature = "dev")]
impl AbilityId {
    pub fn as_literal(&self) -> String {
        format!(
            "AbilityId::{}(AbilityName::{:?})",
            self.as_char(),
            self.ability_name()
        )
    }

    pub fn discriminant(&self) -> String {
        let letter = self.as_char();
        let ability_name = self.ability_name();
        let mut result = String::from(letter);
        match ability_name {
            AbilityName::Void => {}
            _ => result.push_str(&format!("{ability_name:?}").trim_start_matches('_')),
        }
        result
    }
}

/// A generic metadata holder that determines what buffs one item give
/// to the current player when bought. For example `StatName::AbilityPower(80)`
/// means that when bought, the player gets extra 80 ability power. This struct
/// weighs 4 bytes and the maximum stat buff for one item is [`u16::MAX`]
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum StatName {
    AbilityHaste,
    AbilityPower,
    AdaptiveForce,
    Armor,
    ArmorPenetration,
    AttackDamage,
    AttackSpeed,
    BaseHealthRegen,
    BaseManaRegen,
    CritChance,
    CritDamage,
    GoldPer10Seconds,
    HealAndShieldPower,
    Health,
    Lethality,
    LifeSteal,
    MagicPenetration,
    MagicResist,
    Mana,
    MoveSpeed,
    Omnivamp,
    Tenacity,
}

impl StatName {
    pub const VARIANTS: usize = 22;

    pub const fn name(&self) -> &'static str {
        match self {
            StatName::AbilityHaste => "Ability Haste",
            StatName::AbilityPower => "Ability Power",
            StatName::AdaptiveForce => "Adaptive Force",
            StatName::Armor => "Armor",
            StatName::ArmorPenetration => "Armor Penetration",
            StatName::AttackDamage => "Attack Damage",
            StatName::AttackSpeed => "Attack Speed",
            StatName::BaseHealthRegen => "Base Health Regen",
            StatName::BaseManaRegen => "Base Mana Regen",
            StatName::CritChance => "Crit Chance",
            StatName::CritDamage => "Crit Damage",
            StatName::GoldPer10Seconds => "Gold / 10s",
            StatName::HealAndShieldPower => "Heal & Shield Power",
            StatName::Health => "Health",
            StatName::Lethality => "Lethality",
            StatName::LifeSteal => "Life Steal",
            StatName::MagicPenetration => "Magic Penetration",
            StatName::MagicResist => "Magic Resist",
            StatName::Mana => "Mana",
            StatName::MoveSpeed => "Move Speed",
            StatName::Omnivamp => "Omnivamp",
            StatName::Tenacity => "Tenacity",
        }
    }

    pub const fn from_u8_unchecked(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Display for StatName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = self.name();
        write!(f, "{value}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct MergeData {
    pub minimum_damage: u8,
    pub maximum_damage: u8,
    pub alias: AbilityId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DevMergeData {
    pub minimum_damage: AbilityId,
    pub maximum_damage: AbilityId,
    pub alias: AbilityId,
}

#[derive(Clone, Copy, Debug, Deserialize, Encode, Hash, PartialEq, PartialOrd, Serialize)]
pub enum ComboElement {
    Ability(AbilityId),
    Attack,
}

impl AbilityName {
    pub const fn display(&self) -> Option<&'static str> {
        match self {
            AbilityName::_1 => Some("1"),
            AbilityName::_2 => Some("2"),
            AbilityName::_3 => Some("3"),
            AbilityName::_4 => Some("4"),
            AbilityName::_5 => Some("5"),
            AbilityName::_6 => Some("6"),
            AbilityName::_7 => Some("7"),
            AbilityName::_8 => Some("8"),
            AbilityName::Min => Some("MIN"),
            AbilityName::Max => Some("MAX"),
            AbilityName::Mega => Some("MEGA"),
            AbilityName::_1Min => Some("1-MIN"),
            AbilityName::_2Min => Some("2-MIN"),
            AbilityName::_3Min => Some("3-MIN"),
            AbilityName::_4Min => Some("4-MIN"),
            AbilityName::_5Min => Some("5-MIN"),
            AbilityName::_6Min => Some("6-MIN"),
            AbilityName::_7Min => Some("7-MIN"),
            AbilityName::_8Min => Some("8-MIN"),
            AbilityName::_1Max => Some("1-MAX"),
            AbilityName::_2Max => Some("2-MAX"),
            AbilityName::_3Max => Some("3-MAX"),
            AbilityName::_4Max => Some("4-MAX"),
            AbilityName::_5Max => Some("5-MAX"),
            AbilityName::_6Max => Some("6-MAX"),
            AbilityName::_7Max => Some("7-MAX"),
            AbilityName::_8Max => Some("8-MAX"),
            _ => None,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum AbilityName {
    Void,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Min,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
    Max,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    Mega,
    Minion,
    Minion1,
    Minion2,
    Minion3,
    MinionMax,
    Monster,
    Monster1,
    Monster2,
    Monster3,
    Monster4,
    MonsterMax,
}

impl AbilityName {
    pub const JMP: u8 = Self::Min as u8;

    pub const fn cast_max(&self) -> Self {
        let byte = *self as u8;
        assert!(byte < Self::Mega as u8 - 1);
        match byte >= Self::Max as u8 {
            true => *self,
            false if byte < Self::Min as u8 => unsafe {
                core::mem::transmute(byte + (Self::JMP << 1))
            },
            false => unsafe { core::mem::transmute(byte + Self::JMP) },
        }
    }

    pub const fn cast_min(&self) -> Self {
        Self::from_u8(self.cast_max() as u8 - Self::JMP).unwrap()
    }

    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            x if x < Self::MonsterMax as u8 => Some(unsafe { core::mem::transmute(x) }),
            _ => None,
        }
    }
}

/// A champion can have either melee or ranged damage. Ranged champions
/// often have some damage penalty for items and runes, which are considered
/// by branching over this enum
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum AttackType {
    #[default]
    Melee,
    Ranged,
}

impl FromStr for AttackType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Melee" | "MELEE" => Ok(AttackType::Melee),
            "Ranged" | "RANGED" => Ok(AttackType::Ranged),
            _ => Err("No matches when calling AttackType::from_str"),
        }
    }
}

impl FromStr for AdaptiveType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Magic" | "MAGIC_DAMAGE" => Ok(Self::Magic),
            "Physical" | "PHYSICAL_DAMAGE" => Ok(Self::Physical),
            _ => Err("No matches when calling AdaptiveType::from_str"),
        }
    }
}

/// Enum that holds the current adaptive type of some champion, which
/// can be either physical or magic. It is mainly used to determine if runes
/// should deal physical or magic damage, or to convert `Adaptive Force`
/// stats to either `Attack Damage` or `Ability Power`. Check the enum [`StatName`]
/// for more details about all the possibilities
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum AdaptiveType {
    #[default]
    Physical,
    Magic,
}

/// Represents each playable position or `lane` that a champion can
/// play in the standard gamemode `SummonersRift`, whose definition
/// is [`GameMap::SummonersRift`]. If we don't know a champion's position,
/// it is set to [`Position::Top`].
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Position {
    #[default]
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

impl Position {
    pub const VARIANTS: u8 = 5;
    pub const ARRAY: [Self; Self::VARIANTS as _] = [
        Position::Top,
        Position::Jungle,
        Position::Middle,
        Position::Bottom,
        Position::Support,
    ];

    pub const fn index(&self) -> usize {
        *self as _
    }

    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0..Self::VARIANTS => Some(unsafe { Self::from_u8_unchecked(value) }),
            _ => None,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Position::Top => "Top",
            Position::Jungle => "Jungle",
            Position::Middle => "Mid",
            Position::Bottom => "Adc / Bottom",
            Position::Support => "Support",
        }
    }

    pub const fn role(&self) -> &'static str {
        match self {
            Position::Top => "top",
            Position::Jungle => "jungle",
            Position::Middle => "mid",
            Position::Bottom => "adc",
            Position::Support => "support",
        }
    }

    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TOP" => Ok(Position::Top),
            "JUNGLE" => Ok(Position::Jungle),
            "MIDDLE" => Ok(Position::Middle),
            "BOTTOM" => Ok(Position::Bottom),
            "SUPPORT" => Ok(Position::Support),
            _ => Err("No matches when calling Position::from_str"),
        }
    }
}

/// All possible maps and codes that can be played. Most of them are
/// event maps that may never return to the game, and don't have a
/// deterministic code. [`GameMap::SummonersRift`] is the default map.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum GameMap {
    Aram,
    Arena,
    DarkStar,
    Dominion,
    Invasion,
    NexusBlitz,
    Odyssey,
    Project,
    StarGuardian,
    SummonersRift,
    Tft,
    Tutorial,
    TwistedTreeline,
    Urf,
    #[default]
    Unknown,
    UnknownMap33,
    UnknownMap35,
}

impl GameMap {
    /// Constant conversion of a [`u8`] into a [`GameMap`] enum,
    /// where the byte represents the code of the current map
    pub const fn from_u8(value: u8) -> Self {
        match value {
            3 => GameMap::Tutorial,
            4 | 10 => GameMap::TwistedTreeline,
            8 => GameMap::Dominion,
            11 => GameMap::SummonersRift,
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
            33 => GameMap::UnknownMap33,
            35 => GameMap::UnknownMap35,
            0xFF => GameMap::Urf,
            _ => GameMap::Unknown,
        }
    }
}

/// Creates an enum and associates constants that represents each of its
/// variants, using the same name and ignores `upper_case` lints
macro_rules! const_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $Variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $(#[$vmeta])*
                $Variant,
            )+
        }
    };
}

const_enum! {
    /// Defines what is the damage type of some entity.
    /// - [`DamageType::Physical`] and [`DamageType::Magic`] Represents any damage related
    /// to how much (armor or magic resistence) the enemy player has, and is affected by the
    /// percent and flat values or (armor or magic) penetration of the current player
    /// - [`DamageType::Mixed`] Damages of this type are treated as a special case of
    /// [`DamageType::True`], where the closure has to multiply on its own the `physical_mod`
    /// and `magic_mod` modifiers of the [`tutorlolv2_math::DamageModifiers`] struct. It is
    /// usually used when a single ability or item deals both physical and magic damage in the
    /// same hit.
    /// - [`DamageType::True`] Damages of this type are not affected by armor or magic resistence,
    /// their values are in general irreducible.
    /// - [`DamageType::Adaptive`] Damages of this type will vary into the [`DamageType::Physical`]
    /// or [`DamageType::Magic`] depending on how much bonus armor or ability power the current player
    /// has.
    /// - [`DamageType::Unknown`] is the default value when no damage type is set
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[derive(bincode::Encode, bincode::Decode)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum DamageType {
        Physical,
        Magic,
        Mixed,
        True,
        Adaptive,
        #[default]
        Unknown,
    }
}

const_enum! {
    /// An enum with several variants that can be used to add up to `255` attributes
    /// to some ability, item or rune. It is mostly used to determine if the current
    /// instance damages onhit only for the `maximum`, `minimum` or both damage kinds.
    /// [`Attrs::Undefined`] is set to be the default variant, representing no extra data. This
    /// is also used to determine if some ability has area damage
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[derive(bincode::Encode, bincode::Decode)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum Attrs {
        #[default]
        Undefined,
        Onhit,
        OnhitMin,
        OnhitMax,
        Area,
        AreaOnhit,
        AreaOnhitMin,
        AreaOnhitMax,
    }
}

impl FromStr for DamageType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PHYSICAL_DAMAGE" | "Physical" => Ok(DamageType::Physical),
            "MAGIC_DAMAGE" | "Magic" | "magic" => Ok(DamageType::Magic),
            "MIXED_DAMAGE" | "Mixed" | "Magic True" => Ok(DamageType::Mixed),
            "TRUE_DAMAGE" | "True" | "true" => Ok(DamageType::True),
            "ADAPTIVE_DAMAGE" | "adaptive" => Ok(DamageType::Adaptive),
            _ => Ok(DamageType::Unknown),
        }
    }
}

/// Creates the `CtxVar` and `Ctx` structs, associating
/// the appropriate names and numeric types that it will hold. This struct
/// is essential to the application since it is used to evaluate all the
/// generated closures contained in cache static variables
macro_rules! create_eval_struct {
    ($($value:ident),*$(,)?) => {
        pastey::paste! {
            /// Defines a standard type that implements trait [`core::fmt::Display`]
            /// and is used to create constant closures in the static variables of
            /// this module. For example:
            /// [`CtxVar::QLevel`] is converted to: `ctx.q_level`
            #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Encode, Decode, Serialize, Deserialize)]
            #[repr(u8)]
            pub enum CtxVar {
                $([<$value:camel>],)*
            }

            impl CtxVar {
                pub const ARRAY: [Self; Self::VARIANTS] = [$(Self::[<$value:camel>],)*];
                pub const fn as_var(&self) -> &'static str {
                    match self {
                        $(
                            Self::[<$value:camel>] => concat!("ctx.", stringify!($value)),
                        )*
                    }
                }
            }

            impl AsRef<str> for CtxVar {
                fn as_ref(&self) -> &str {
                    self.as_var()
                }
            }

            impl Index<CtxVar> for Ctx {
                type Output = f32;
                fn index(&self, index: CtxVar) -> &Self::Output {
                    match index {
                        $(CtxVar::[<$value:camel>] => &self.$value),*
                    }
                }
            }

            #[derive(Clone, Copy, Debug, Decode, Default, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
            #[repr(C)]
            pub struct Ctx {
                $(pub $value: f32,)*
            }

            impl ::core::fmt::Display for CtxVar {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{}", self.as_var())
                }
            }
        }
    };
}

create_eval_struct!(
    ability_power,
    adaptive_damage,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_speed,
    base_ad,
    base_armor,
    base_health,
    base_magic_resist,
    base_mana,
    bonus_ad,
    bonus_armor,
    bonus_health,
    bonus_magic_resist,
    bonus_mana,
    bonus_move_speed,
    crit_chance,
    crit_damage,
    current_health,
    current_mana,
    level,
    q_level,
    w_level,
    e_level,
    r_level,
    magic_multiplier,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    max_health,
    max_mana,
    missing_health,
    physical_multiplier,
    randuin_effect,
    rocksolid_effect,
    stacks,
    steelcaps_effect,
    enemy_armor,
    enemy_bonus_armor,
    enemy_bonus_health,
    enemy_bonus_magic_resist,
    enemy_current_health,
    enemy_magic_resist,
    enemy_max_health,
    enemy_missing_health,
);

impl CtxVar {
    pub const VARIANTS: usize = size_of::<Ctx>() / size_of::<f32>();
    pub const SKIP: usize = Self::SteelcapsEffect as usize + 1;
}
