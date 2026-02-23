#![no_std]
mod ability_name;

use core::fmt::Display;

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
        //  format!(
        //         "{letter}_{ability_name}",
        //         ability_name = format!("{:?}", ability_id.ability_name()).trim_start_matches('_')
        //     )
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
    AbilityHaste,
    AbilityPower,
    AdaptiveForce,
    Armor,
    ArmorPenetration,
    AttackDamage,
    AttackSpeed,
    BaseHealthRegen,
    BaseManaRegen,
    CriticalStrikeChance,
    CriticalStrikeDamage,
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
}

impl Display for StatName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = match self {
            StatName::AbilityHaste => "Ability Haste",
            StatName::AbilityPower => "Ability Power",
            StatName::AdaptiveForce => "Adaptive Force",
            StatName::Armor => "Armor",
            StatName::ArmorPenetration => "Armor Penetration",
            StatName::AttackDamage => "Attack Damage",
            StatName::AttackSpeed => "Attack Speed",
            StatName::BaseHealthRegen => "Base Health Regen",
            StatName::BaseManaRegen => "Base Mana Regen",
            StatName::CriticalStrikeChance => "Crit Chance",
            StatName::CriticalStrikeDamage => "Crit Damage",
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
        };
        write!(f, "{value}")
    }
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
