use crate::AbilityLike;
use bincode::Encode;
use serde::{Deserialize, Serialize};

macro_rules! ability_name {
    ($($field:ident),+$(,)?) => {
        #[derive(Clone, Copy, Serialize, Deserialize, Encode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum AbilityName { $($field),+ }

        $(
            #[allow(non_upper_case_globals)]
            pub const $field: AbilityName = AbilityName::$field;
        )+

        #[derive(Clone, Copy)]
        pub enum P { $($field),+ }
        impl Into<AbilityLike> for P { fn into(self) -> AbilityLike { match self {
            $(P::$field => AbilityLike::P(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum Q { $($field),+ }
        impl Into<AbilityLike> for Q { fn into(self) -> AbilityLike { match self {
            $(Q::$field => AbilityLike::Q(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum W { $($field),+ }
        impl Into<AbilityLike> for W { fn into(self) -> AbilityLike { match self {
            $(W::$field => AbilityLike::W(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
        pub enum E { $($field),+ }
        impl Into<AbilityLike> for E { fn into(self) -> AbilityLike { match self {
            $(E::$field => AbilityLike::E(AbilityName::$field)),+ } }
        }

        #[derive(Clone, Copy)]
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
