#![no_std]
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
extern crate alloc;
#[cfg(feature = "dev")]
use alloc::{format, string::String};
use bincode::{Decode, Encode};
use core::fmt::Display;
use serde::{Deserialize, Serialize};

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
