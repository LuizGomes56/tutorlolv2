#![no_std]

#[cfg(feature = "dev")]
extern crate alloc;

#[cfg(feature = "dev")]
use alloc::{format, string::String};

use bincode::{Decode, Encode};
use core::{convert::Infallible, fmt::Display, ops::Index, str::FromStr};
use serde::{Deserialize, Serialize};

/// A generic metadata holder for [`AbilityId`], [`ItemId`], or [`RuneId`].
/// Contains its damage type, attributes, and which instance of the enum the value is.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Encode, Decode, Serialize, Deserialize,
)]
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
    pub fn discriminant(&self) -> String {
        let letter = self.as_char();
        let ability_name = self.ability_name();
        format!("{letter}{ability_name}")
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
            _ => return Err("Unknown stat in StatName::from_str"),
        }
    }
}

impl StatName {
    pub const VARIANTS: usize = 24;

    pub const fn name(&self) -> &'static str {
        match self {
            Self::AbilityHaste => "Ability Haste",
            Self::AbilityPower => "Ability Power",
            Self::AdaptiveForce => "Adaptive Force",
            Self::Armor => "Armor",
            Self::ArmorPenetration => "Armor Penetration",
            Self::AttackDamage => "Attack Damage",
            Self::AttackSpeed => "Attack Speed",
            Self::BaseHealthRegen => "Base Health Regen",
            Self::BaseManaRegen => "Base Mana Regen",
            Self::CritChance => "Crit Chance",
            Self::CritDamage => "Crit Damage",
            Self::GoldPer10Seconds => "Gold / 10s",
            Self::HealAndShieldPower => "Heal & Shield Power",
            Self::Health => "Health",
            Self::Lethality => "Lethality",
            Self::LifeSteal => "Life Steal",
            Self::MagicPenetration | Self::MagicPenetrationPercent => "Magic Penetration",
            Self::MagicResist => "Magic Resist",
            Self::Mana => "Mana",
            Self::MoveSpeed | Self::MoveSpeedPercent => "Move Speed",
            Self::Omnivamp => "Omnivamp",
            Self::Tenacity => "Tenacity",
        }
    }

    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }

    pub const fn from_u8(value: u8) -> Option<Self> {
        unsafe {
            if value <= Self::VARIANTS as _ {
                Some(Self::from_u8_unchecked(value))
            } else {
                None
            }
        }
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
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
    strum::IntoStaticStr,
)]
#[repr(u8)]
pub enum AbilityName {
    #[strum(serialize = "")]
    Void,
    #[strum(serialize = "1")]
    _1,
    #[strum(serialize = "2")]
    _2,
    #[strum(serialize = "3")]
    _3,
    #[strum(serialize = "4")]
    _4,
    #[strum(serialize = "5")]
    _5,
    #[strum(serialize = "6")]
    _6,
    #[strum(serialize = "7")]
    _7,
    #[strum(serialize = "8")]
    _8,
    #[strum(serialize = "min")]
    Min,
    #[strum(serialize = "1min")]
    _1Min,
    #[strum(serialize = "2min")]
    _2Min,
    #[strum(serialize = "3min")]
    _3Min,
    #[strum(serialize = "4min")]
    _4Min,
    #[strum(serialize = "5min")]
    _5Min,
    #[strum(serialize = "6min")]
    _6Min,
    #[strum(serialize = "7min")]
    _7Min,
    #[strum(serialize = "8min")]
    _8Min,
    #[strum(serialize = "max")]
    Max,
    #[strum(serialize = "1max")]
    _1Max,
    #[strum(serialize = "2max")]
    _2Max,
    #[strum(serialize = "3max")]
    _3Max,
    #[strum(serialize = "4max")]
    _4Max,
    #[strum(serialize = "5max")]
    _5Max,
    #[strum(serialize = "6max")]
    _6Max,
    #[strum(serialize = "7max")]
    _7Max,
    #[strum(serialize = "8max")]
    _8Max,
    #[strum(serialize = "mega")]
    Mega,
    #[strum(serialize = "c")]
    Minion,
    #[strum(serialize = "c1")]
    Minion1,
    #[strum(serialize = "c2")]
    Minion2,
    #[strum(serialize = "c3")]
    Minion3,
    #[strum(serialize = "cmax")]
    MinionMax,
    #[strum(serialize = "m")]
    Monster,
    #[strum(serialize = "m1")]
    Monster1,
    #[strum(serialize = "m2")]
    Monster2,
    #[strum(serialize = "m3")]
    Monster3,
    #[strum(serialize = "m4")]
    Monster4,
    #[strum(serialize = "mmax")]
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

impl AttackType {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Melee),
            1 => Some(Self::Ranged),
            _ => None,
        }
    }

    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
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
    strum::Display,
)]
pub enum DamageIndex {
    #[default]
    Min,
    Max,
}

impl DamageIndex {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Min),
            1 => Some(Self::Max),
            _ => None,
        }
    }

    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }
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

impl AdaptiveType {
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
    pub const VARIANTS: usize = 5;
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
        match value as usize {
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
            "Top" | "TOP" => Ok(Position::Top),
            "Jungle" | "JUNGLE" => Ok(Position::Jungle),
            "Middle" | "MIDDLE" => Ok(Position::Middle),
            "Bottom" | "BOTTOM" => Ok(Position::Bottom),
            "Support" | "SUPPORT" => Ok(Position::Support),
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
    strum::Display,
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
    OneForAll,
    UnsealedSpellbook,
    SwiftPlay,
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
            0xFC => GameMap::OneForAll,
            0xFD => GameMap::UnsealedSpellbook,
            0xFE => GameMap::SwiftPlay,
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
        Unspecified,
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
                strum::FromRepr
            )]
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

            impl ::core::str::FromStr for CtxVar {
                type Err = &'static str;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $(stringify!($value) | stringify!([<$value:camel>]) => Ok(Self::[<$value:camel>]),)*
                        _ => Err("CtxVar::from_str: Invalid variable provided"),
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
);

impl CtxVar {
    pub const VARIANTS: usize = size_of::<Ctx>() / size_of::<f32>();
    pub const SKIP: usize = Self::SteelcapsEffect as usize + 1;
}

macro_rules! impl_display {
    ($($stru:ty),*) => {
        $(impl Display for $stru {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{self:?}")
            }
        })*
    };
}

impl_display!(AttackType, AdaptiveType, Position);
