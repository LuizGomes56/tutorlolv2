#![no_std]
use bincode::Encode;
use serde::{Deserialize, Serialize};

pub mod ability_name;

pub use ability_name::*;

extern crate alloc;
use alloc::{format, string::String};

#[derive(
    Serialize, Deserialize, Copy, Clone, Encode, Debug, Eq, PartialEq, PartialOrd, Hash, Ord,
)]
#[serde(tag = "type", content = "name")]
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

    pub const fn ability_name(&self) -> AbilityName {
        match self {
            AbilityId::P(v) => *v,
            AbilityId::Q(v) => *v,
            AbilityId::W(v) => *v,
            AbilityId::E(v) => *v,
            AbilityId::R(v) => *v,
        }
    }

    pub fn as_literal(&self) -> String {
        format!(
            "AbilityId::{}(AbilityName::{:?})",
            self.as_char(),
            self.ability_name()
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(tag = "name", content = "value")]
pub enum StatName {
    AbilityHaste(u16),
    AbilityPower(u16),
    Armor(u16),
    Lethality(u16),
    ArmorPenetration(u16),
    MagicPenetration(u16),
    AttackDamage(u16),
    AttackSpeed(u16),
    GoldPer10Seconds(u16),
    AdaptiveForce(u16),
    CriticalStrikeChance(u16),
    CriticalStrikeDamage(u16),
    Health(u16),
    LifeSteal(u16),
    MagicResist(u16),
    Mana(u16),
    MoveSpeed(u16),
    Omnivamp(u16),
    BaseHealthRegen(u16),
    BaseManaRegen(u16),
    Tenacity(u16),
    HealAndShieldPower(u16),
}
