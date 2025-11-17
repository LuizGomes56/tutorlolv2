use bincode::Encode;
use serde::{Deserialize, Serialize};

pub use paste::paste;

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

impl From<usize> for AbilityLike {
    fn from(value: usize) -> Self {
        match value {
            0 => AbilityLike::P(AbilityName::Void),
            1 => AbilityLike::Q(AbilityName::Void),
            2 => AbilityLike::W(AbilityName::Void),
            3 => AbilityLike::E(AbilityName::Void),
            4 => AbilityLike::R(AbilityName::Void),
            _ => unreachable!(),
        }
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

    pub fn from_fn(&self) -> fn(AbilityName) -> Self {
        match self {
            AbilityLike::P(_) => AbilityLike::P,
            AbilityLike::Q(_) => AbilityLike::Q,
            AbilityLike::W(_) => AbilityLike::W,
            AbilityLike::E(_) => AbilityLike::E,
            AbilityLike::R(_) => AbilityLike::R,
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

macro_rules! ability_name {
    ($($field:ident),+$(,)?) => {
        #[derive(Clone, Copy, Serialize, Deserialize, Encode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum AbilityName { $($field),+ }

        $(
            #[allow(non_upper_case_globals)]
            pub const $field: AbilityName = AbilityName::$field;
        )+

        pub enum P { $($field),+ }
        impl Into<AbilityLike> for P { fn into(self) -> AbilityLike { match self {
            $(P::$field => AbilityLike::P(AbilityName::$field)),+ } }
        }

        pub enum Q { $($field),+ }
        impl Into<AbilityLike> for Q { fn into(self) -> AbilityLike { match self {
            $(Q::$field => AbilityLike::Q(AbilityName::$field)),+ } }
        }

        pub enum W { $($field),+ }
        impl Into<AbilityLike> for W { fn into(self) -> AbilityLike { match self {
            $(W::$field => AbilityLike::W(AbilityName::$field)),+ } }
        }

        pub enum E { $($field),+ }
        impl Into<AbilityLike> for E { fn into(self) -> AbilityLike { match self {
            $(E::$field => AbilityLike::E(AbilityName::$field)),+ } }
        }

        pub enum R { $($field),+ }
        impl Into<AbilityLike> for R { fn into(self) -> AbilityLike { match self {
            $(R::$field => AbilityLike::R(AbilityName::$field)),+ } }
        }
    };
}

ability_name! {
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
