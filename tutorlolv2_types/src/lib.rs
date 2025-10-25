use bincode::Encode;
use serde::{Deserialize, Serialize};

pub use paste::paste;

#[derive(
    Serialize, Deserialize, Copy, Clone, Encode, Debug, Eq, PartialEq, PartialOrd, Hash, Ord,
)]
#[serde(tag = "ability", content = "name")]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl ToString for AbilityLike {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl AbilityLike {
    pub fn as_char(&self) -> char {
        match self {
            AbilityLike::P(_) => 'P',
            AbilityLike::Q(_) => 'Q',
            AbilityLike::W(_) => 'W',
            AbilityLike::E(_) => 'E',
            AbilityLike::R(_) => 'R',
        }
    }

    pub fn ability_name(&self) -> AbilityName {
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

#[derive(
    Clone, Copy, Serialize, Deserialize, Encode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
#[repr(u8)]
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
    Void,
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
