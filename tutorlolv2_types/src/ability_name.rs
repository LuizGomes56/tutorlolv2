use crate::AbilityId;

macro_rules! ability_name {
    ($($field:ident),+$(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
        )]
        #[repr(u8)]
        pub enum AbilityName { $($field),+ }

        $(
            #[allow(non_upper_case_globals)]
            pub const $field: AbilityName = AbilityName::$field;
        )+

        #[derive(Clone, Copy)]
        pub enum P { $($field),+ }
        impl Into<AbilityId> for P { fn into(self) -> AbilityId { match self {
            $(P::$field => AbilityId::P(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum Q { $($field),+ }
        impl Into<AbilityId> for Q { fn into(self) -> AbilityId { match self {
            $(Q::$field => AbilityId::Q(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum W { $($field),+ }
        impl Into<AbilityId> for W { fn into(self) -> AbilityId { match self {
            $(W::$field => AbilityId::W(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum E { $($field),+ }
        impl Into<AbilityId> for E { fn into(self) -> AbilityId { match self {
            $(E::$field => AbilityId::E(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum R { $($field),+ }
        impl Into<AbilityId> for R { fn into(self) -> AbilityId { match self {
            $(R::$field => AbilityId::R(AbilityName::$field)),+ } }
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
