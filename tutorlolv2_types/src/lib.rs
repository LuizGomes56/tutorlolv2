#![no_std]
mod ability_name;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

pub use ability_name::AbilityName;

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
