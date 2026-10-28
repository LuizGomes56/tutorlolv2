#![no_std]
#![allow(clippy::missing_transmute_annotations)]

extern crate alloc;

mod entity;
pub use entity::{EntityKind, Tag};

#[cfg(test)]
extern crate std;

#[cfg(all(test, feature = "bake"))]
mod bake_tests;

#[cfg(test)]
mod strum_tests;

use bincode::{Decode, Encode};
use core::{convert::Infallible, fmt::Display, ops::Index, str::FromStr};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

// Keep the existing string error type when deriving parsers.
type ParseError = &'static str;

/// A generic metadata holder for [`AbilityId`], [`ItemId`], or [`RuneId`].
/// Contains its damage type, attributes, and which instance of the enum the value is.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Encode, Decode, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub struct TypeMetadata<T> {
    pub kind: T,
    pub damage_type: DamageType,
    pub attributes: Attrs,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Key {
    #[default]
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

    pub const fn as_ctx_var(&self) -> CtxVar {
        match self {
            Key::P => CtxVar::Level,
            Key::Q => CtxVar::QLevel,
            Key::W => CtxVar::WLevel,
            Key::E => CtxVar::ELevel,
            Key::R => CtxVar::RLevel,
        }
    }
}

impl TryFrom<char> for Key {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' | 'A' | 'P' => Ok(Key::P),
            'Q' => Ok(Key::Q),
            'W' => Ok(Key::W),
            'E' => Ok(Key::E),
            'R' => Ok(Key::R),
            _ => Err("Invalid char when calling Key::try_from"),
        }
    }
}

/// Enum that represents one ability of a champion, with a custom display name.
/// - [`AbilityId::P`] represents the passive of a champion
/// - Other variants correspond to the abilities `Q`, `W`, `E`, and `R` (ultimate)
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
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
        value.as_key()
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

    pub fn discriminant(&self) -> alloc::string::String {
        let letter = self.as_char();
        let ability_name = self.ability_name().short();
        alloc::format!("{letter}{ability_name}")
    }
}

/// A generic metadata holder that determines what buffs one item give
/// to the current player when bought. For example `StatName::AbilityPower(80)`
/// means that when bought, the player gets extra 80 ability power. This struct
/// weighs 4 bytes and the maximum stat buff for one item is [`u16::MAX`]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
    EnumCount,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
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
    MagicPenetrationPercent,
    MagicResist,
    Mana,
    MoveSpeed,
    MoveSpeedPercent,
    Omnivamp,
    Tenacity,
}

impl FromStr for StatName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ah" => Ok(Self::AbilityHaste),
            "hp" => Ok(Self::Health),
            "mr" => Ok(Self::MagicResist),
            "ap" => Ok(Self::AbilityPower),
            "mana" => Ok(Self::Mana),
            "ms" => Ok(Self::MoveSpeedPercent),
            "hsp" => Ok(Self::HealAndShieldPower),
            "mp5" => Ok(Self::BaseManaRegen),
            "armor" => Ok(Self::Armor),
            "msflat" => Ok(Self::MoveSpeed),
            "crit" => Ok(Self::CritChance),
            "ad" => Ok(Self::AttackDamage),
            "armpen" => Ok(Self::ArmorPenetration),
            "lethality" => Ok(Self::Lethality),
            "as" => Ok(Self::AttackSpeed),
            "lifesteal" => Ok(Self::LifeSteal),
            "mpen" => Ok(Self::MagicPenetrationPercent),
            "mpenflat" => Ok(Self::MagicPenetration),
            "gp10" => Ok(Self::GoldPer10Seconds),
            "hp5" => Ok(Self::BaseHealthRegen),
            "tenacity" => Ok(Self::Tenacity),
            "spec" => Ok(Self::AdaptiveForce),
            "omnivamp" => Ok(Self::Omnivamp),
            "hp5flat" => Ok(Self::BaseHealthRegen),
            "critdamage" => Ok(Self::CritDamage),
            _ => Err("Unknown stat in StatName::from_str"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub struct MergeData {
    pub min: u8,
    pub max: u8,
    pub alias: AbilityId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DevMergeData {
    pub min: AbilityId,
    pub max: AbilityId,
    pub alias: AbilityId,
}

#[derive(Clone, Copy, Debug, Deserialize, Encode, Hash, PartialEq, PartialOrd, Serialize)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub enum ComboElement {
    Ability(AbilityId),
    Attack,
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
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
    strum::VariantNames,
    strum::IntoStaticStr,
    strum::FromRepr,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
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

    pub const fn short(&self) -> &'static str {
        match self {
            Self::Void => "",
            Self::_1 => "1",
            Self::_2 => "2",
            Self::_3 => "3",
            Self::_4 => "4",
            Self::_5 => "5",
            Self::_6 => "6",
            Self::_7 => "7",
            Self::_8 => "8",
            Self::Min => "min",
            Self::_1Min => "1min",
            Self::_2Min => "2min",
            Self::_3Min => "3min",
            Self::_4Min => "4min",
            Self::_5Min => "5min",
            Self::_6Min => "6min",
            Self::_7Min => "7min",
            Self::_8Min => "8min",
            Self::Max => "max",
            Self::_1Max => "1max",
            Self::_2Max => "2max",
            Self::_3Max => "3max",
            Self::_4Max => "4max",
            Self::_5Max => "5max",
            Self::_6Max => "6max",
            Self::_7Max => "7max",
            Self::_8Max => "8max",
            Self::Mega => "mega",
            Self::Minion => "c",
            Self::Minion1 => "c1",
            Self::Minion2 => "c2",
            Self::Minion3 => "c3",
            Self::MinionMax => "cmax",
            Self::Monster => "m",
            Self::Monster1 => "m1",
            Self::Monster2 => "m2",
            Self::Monster3 => "m3",
            Self::Monster4 => "m4",
            Self::MonsterMax => "mmax",
        }
    }

    pub const fn cast_max(&self) -> Self {
        let byte = *self as u8;
        assert!(byte < Self::Mega as u8 - 1);

        match byte >= Self::Max as u8 {
            true => *self,
            false if byte < Self::Min as u8 => Self::from_repr(byte + (Self::JMP << 1)).unwrap(),
            false => Self::from_repr(byte + Self::JMP).unwrap(),
        }
    }

    pub const fn cast_min(&self) -> Self {
        Self::from_repr(self.cast_max() as u8 - Self::JMP).unwrap()
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
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::FromRepr,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = AttackType::parse_error)]
pub enum AttackType {
    #[default]
    #[strum(to_string = "Melee", serialize = "MELEE")]
    Melee,
    #[strum(to_string = "Ranged", serialize = "RANGED")]
    Ranged,
}

impl AttackType {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling AttackType::from_str"
    }
}

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
    Serialize,
    Deserialize,
    strum::EnumCount,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
)]
#[repr(u8)]
pub enum DamageIndex {
    #[default]
    Min,
    Max,
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
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = AdaptiveType::parse_error)]
pub enum AdaptiveType {
    #[default]
    #[strum(to_string = "Physical", serialize = "PHYSICAL_DAMAGE")]
    Physical,
    #[strum(to_string = "Magic", serialize = "MAGIC_DAMAGE")]
    Magic,
}

impl AdaptiveType {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling AdaptiveType::from_str"
    }

    pub const fn try_infer(bonus_attack_damage: f32, ability_power: f32) -> Option<Self> {
        let lhs = 0.35 * bonus_attack_damage;
        let rhs = 0.2 * ability_power;

        if lhs == rhs {
            None
        } else if lhs > rhs {
            Some(Self::Physical)
        } else {
            Some(Self::Magic)
        }
    }
}

impl TryFrom<(f32, f32)> for AdaptiveType {
    type Error = ();

    fn try_from(value: (f32, f32)) -> Result<Self, Self::Error> {
        let (bonus_attack_damage, ability_power) = value;
        Self::try_infer(bonus_attack_damage, ability_power).ok_or(())
    }
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
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::VariantArray,
    strum::EnumCount,
    strum::FromRepr,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = Position::parse_error)]
pub enum Position {
    #[default]
    #[strum(to_string = "Top", serialize = "TOP")]
    Top,
    #[strum(to_string = "Jungle", serialize = "JUNGLE")]
    Jungle,
    #[strum(to_string = "Middle", serialize = "MIDDLE")]
    Middle,
    #[strum(to_string = "Bottom", serialize = "BOTTOM")]
    Bottom,
    #[strum(to_string = "Support", serialize = "SUPPORT", serialize = "UTILITY")]
    Support,
}

impl Position {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling Position::from_str"
    }

    pub const fn index(&self) -> usize {
        *self as _
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
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
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
    OneForAll,
    UnsealedSpellbook,
    SwiftPlay,
}

impl GameMap {
    /// Constant conversion of a [`u8`] into a [`GameMap`] enum,
    /// where the byte represents the code of the current map
    pub const fn from_code(value: u8) -> Self {
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
            0xFC => GameMap::OneForAll,
            0xFD => GameMap::UnsealedSpellbook,
            0xFE => GameMap::SwiftPlay,
            0xFF => GameMap::Urf,
            _ => GameMap::Unknown,
        }
    }
}

/// Defines what is the damage type of some entity.
/// - [`DamageType::Physical`] and [`DamageType::Magic`] Represents any damage related
///
/// to how much (armor or magic resistence) the enemy player has, and is affected by the
/// percent and flat values or (armor or magic) penetration of the current player
/// - [`DamageType::Mixed`] Damages of this type are treated as a special case of [`DamageType::True`],
///
/// where the closure has to multiply on its own the `physical_mod`
///
/// and `magic_mod` modifiers of the [`tutorlolv2_math::DamageModifiers`] struct. It is
/// usually used when a single ability or item deals both physical and magic damage in the same hit.
///
/// - [`DamageType::True`] Damages of this type are not affected by armor or magic resistence,
///
/// their values are in general irreducible.
///
/// - [`DamageType::Adaptive`] Damages of this type will vary into the [`DamageType::Physical`]
///
/// or [`DamageType::Magic`] depending on how much bonus armor or ability power the current player has.
/// - [`DamageType::Unknown`] is the default value when no damage type is set
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
    strum::EnumIter,
    strum::VariantNames,
    strum::IntoStaticStr,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptive,
    #[default]
    Unspecified,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value: &str = self.into();
        write!(f, "{value}")
    }
}

/// An enum with several variants that can be used to add up to `255` attributes
/// to some ability, item or rune. It is mostly used to determine if the current
/// instance damages onhit only for the `maximum`, `minimum` or both damage kinds.
/// [`Attrs::Undefined`] is set to be the default variant, representing no extra data. This
/// is also used to determine if some ability has area damage
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
    strum::EnumIter,
    strum::VariantNames,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
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

impl FromStr for DamageType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PHYSICAL_DAMAGE" | "Physical" => Ok(DamageType::Physical),
            "MAGIC_DAMAGE" | "Magic" | "magic" => Ok(DamageType::Magic),
            "MIXED_DAMAGE" | "Mixed" | "Magic True" => Ok(DamageType::Mixed),
            "TRUE_DAMAGE" | "True" | "true" => Ok(DamageType::True),
            "ADAPTIVE_DAMAGE" | "adaptive" => Ok(DamageType::Adaptive),
            _ => Ok(DamageType::Unspecified),
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
            #[derive(
                Copy,
                Clone,
                Debug,
                Eq,
                Hash,
                PartialEq,
                Ord,
                PartialOrd,
                Encode,
                Decode,
                Serialize,
                Deserialize,
                strum::FromRepr,
                strum::EnumIter,
                strum::VariantArray,
                strum::EnumCount,
                strum::EnumString,
            )]
            #[repr(u8)]
            #[cfg_attr(feature = "bake", derive(databake::Bake))]
            #[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
            #[strum(parse_err_ty = ParseError, parse_err_fn = CtxVar::parse_error)]
            pub enum CtxVar {
                $(
                    #[strum(serialize = "" $value, serialize = "" [<$value:camel>])]
                    [<$value:camel>],
                )*
            }

            impl CtxVar {
                const fn parse_error(_: &str) -> &'static str {
                    "CtxVar::from_str: Invalid variable provided"
                }

                pub const fn to_snake_case(&self) -> &'static str {
                    self.as_var().split_at(4).1
                }

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

            #[derive(
                Clone,
                Copy,
                Debug,
                Decode,
                Default,
                Deserialize,
                Encode,
                PartialEq,
                PartialOrd,
                Serialize
            )]
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
    base_attack_speed,
    base_health,
    base_magic_resist,
    base_mana,
    bonus_ad,
    bonus_armor,
    bonus_attack_speed,
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
    life_steal,
    enemy_armor,
    enemy_bonus_armor,
    enemy_bonus_health,
    enemy_bonus_magic_resist,
    enemy_current_health,
    enemy_magic_resist,
    enemy_max_health,
    enemy_missing_health,
    // Variables that the wiki parser could not identify, always zero
    unknown,
);

impl CtxVar {
    pub const SKIP: usize = Self::SteelcapsEffect as usize + 1;

    pub const ENEMY_VARS: [Self; 7] = [
        CtxVar::EnemyArmor,
        CtxVar::EnemyBonusArmor,
        CtxVar::EnemyBonusHealth,
        CtxVar::EnemyBonusMagicResist,
        CtxVar::EnemyCurrentHealth,
        CtxVar::EnemyMagicResist,
        CtxVar::EnemyMaxHealth,
    ];
}
