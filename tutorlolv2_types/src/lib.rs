#![no_std]
pub mod ability_name;

pub use ability_name::AbilityName;

/// Enum that represents one ability of a champion, with a custom display name.
/// - [`AbilityId::P`] represents the passive of a champion
/// - Other variants correspond to the abilities `Q`, `W`, `E`, and `R` (ultimate)
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
pub enum AbilityId {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl AbilityId {
    pub const fn as_char(&self) -> char {
        match self {
            AbilityId::P(_) => 'P',
            AbilityId::Q(_) => 'Q',
            AbilityId::W(_) => 'W',
            AbilityId::E(_) => 'E',
            AbilityId::R(_) => 'R',
        }
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

#[cfg(feature = "dev")]
impl AbilityId {
    pub const fn from_fn(&self) -> fn(AbilityName) -> Self {
        match self {
            AbilityId::P(_) => AbilityId::P,
            AbilityId::Q(_) => AbilityId::Q,
            AbilityId::W(_) => AbilityId::W,
            AbilityId::E(_) => AbilityId::E,
            AbilityId::R(_) => AbilityId::R,
        }
    }

    pub const fn from_ability_name(&self, ability_name: AbilityName) -> Self {
        match self {
            AbilityId::P(_) => AbilityId::P(ability_name),
            AbilityId::Q(_) => AbilityId::Q(ability_name),
            AbilityId::W(_) => AbilityId::W(ability_name),
            AbilityId::E(_) => AbilityId::E(ability_name),
            AbilityId::R(_) => AbilityId::R(ability_name),
        }
    }

    pub fn as_const_lit(&self) -> String {
        format!("{}::{:?}.cast()", self.as_char(), self.ability_name())
    }

    pub fn as_literal(&self) -> String {
        format!(
            "AbilityId::{}(AbilityName::{:?})",
            self.as_char(),
            self.ability_name()
        )
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
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum StatName {
    AbilityHaste(u16),
    AbilityPower(u16),
    AdaptiveForce(u16),
    Armor(u16),
    ArmorPenetration(u16),
    AttackDamage(u16),
    AttackSpeed(u16),
    BaseHealthRegen(u16),
    BaseManaRegen(u16),
    CriticalStrikeChance(u16),
    CriticalStrikeDamage(u16),
    GoldPer10Seconds(u16),
    HealAndShieldPower(u16),
    Health(u16),
    Lethality(u16),
    LifeSteal(u16),
    MagicPenetration(u16),
    MagicResist(u16),
    Mana(u16),
    MoveSpeed(u16),
    Omnivamp(u16),
    Tenacity(u16),
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct MergeData {
    pub minimum_damage: u8,
    pub maximum_damage: u8,
    pub alias: AbilityId,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct DevMergeData {
    pub minimum_damage: AbilityId,
    pub maximum_damage: AbilityId,
    pub alias: AbilityId,
}
