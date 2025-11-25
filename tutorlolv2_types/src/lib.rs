use bincode::Encode;
use serde::{Deserialize, Serialize};

pub mod ability_name;

pub use ability_name::*;

#[derive(
    Serialize, Deserialize, Copy, Clone, Encode, Debug, Eq, PartialEq, PartialOrd, Hash, Ord,
)]
#[serde(tag = "type", content = "name")]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl AbilityLike {
    pub const fn as_char(&self) -> char {
        match self {
            AbilityLike::P(_) => 'P',
            AbilityLike::Q(_) => 'Q',
            AbilityLike::W(_) => 'W',
            AbilityLike::E(_) => 'E',
            AbilityLike::R(_) => 'R',
        }
    }

    pub const fn from_fn(&self) -> fn(AbilityName) -> Self {
        match self {
            AbilityLike::P(_) => AbilityLike::P,
            AbilityLike::Q(_) => AbilityLike::Q,
            AbilityLike::W(_) => AbilityLike::W,
            AbilityLike::E(_) => AbilityLike::E,
            AbilityLike::R(_) => AbilityLike::R,
        }
    }

    pub const fn from_ability_name(&self, ability_name: AbilityName) -> Self {
        match self {
            AbilityLike::P(_) => AbilityLike::P(ability_name),
            AbilityLike::Q(_) => AbilityLike::Q(ability_name),
            AbilityLike::W(_) => AbilityLike::W(ability_name),
            AbilityLike::E(_) => AbilityLike::E(ability_name),
            AbilityLike::R(_) => AbilityLike::R(ability_name),
        }
    }

    pub const fn ability_name(&self) -> AbilityName {
        match self {
            AbilityLike::P(v) => *v,
            AbilityLike::Q(v) => *v,
            AbilityLike::W(v) => *v,
            AbilityLike::E(v) => *v,
            AbilityLike::R(v) => *v,
        }
    }

    pub fn as_literal(&self) -> String {
        format!(
            "AbilityLike::{}(AbilityName::{:?})",
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
