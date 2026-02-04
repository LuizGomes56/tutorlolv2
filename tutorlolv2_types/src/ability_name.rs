use crate::AbilityId;

macro_rules! ability_name {
    ($($field:ident),+$(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum AbilityName { $($field),+ }

        $(
            #[allow(non_upper_case_globals)]
            pub const $field: AbilityName = AbilityName::$field;
        )+

        #[derive(Clone, Copy)]
        pub enum P { $($field),+ }
        impl P {
            pub const fn cast(self) -> AbilityId { match self {
            $(P::$field => AbilityId::P(AbilityName::$field)),+ } }
        }
        impl Into<AbilityId> for P { fn into(self) -> AbilityId { self.cast() } }

        #[derive(Clone, Copy)]
        pub enum Q { $($field),+ }
        impl Q {
            pub const fn cast(self) -> AbilityId { match self {
            $(Q::$field => AbilityId::Q(AbilityName::$field)),+ } }
        }
        impl Into<AbilityId> for Q { fn into(self) -> AbilityId { self.cast() } }

        #[derive(Clone, Copy)]
        pub enum W { $($field),+ }
        impl W {
            pub const fn cast(self) -> AbilityId { match self {
            $(W::$field => AbilityId::W(AbilityName::$field)),+ } }
        }
        impl Into<AbilityId> for W { fn into(self) -> AbilityId { self.cast() } }

        #[derive(Clone, Copy)]
        pub enum E { $($field),+ }
        impl E {
            pub const fn cast(self) -> AbilityId { match self {
            $(E::$field => AbilityId::E(AbilityName::$field)),+ } }
        }
        impl Into<AbilityId> for E { fn into(self) -> AbilityId { self.cast() } }

        #[derive(Clone, Copy)]
        pub enum R { $($field),+ }
        impl R {
            pub const fn cast(self) -> AbilityId { match self {
            $(R::$field => AbilityId::R(AbilityName::$field)),+ } }
        }
        impl Into<AbilityId> for R { fn into(self) -> AbilityId { self.cast() } }
    };
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
