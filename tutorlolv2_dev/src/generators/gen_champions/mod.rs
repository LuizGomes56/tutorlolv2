pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_decl::decl_champions::*},
    model::champions::{Ability, Champion},
};
use tutorlolv2_gen::EvalIdent;
pub(self) use tutorlolv2_gen::{
    _1, _1Max, _1Min, _2, _2Max, _2Min, _3, _3Max, _3Min, _4, _4Max, _4Min, _5, _5Max, _5Min, _6,
    _6Max, _6Min, _7, _7Max, _7Min, _8, _8Max, _8Min, Max, Mega, Min, Minion, Minion1, Minion2,
    Minion3, MinionMax, Monster, Monster1, Monster2, Monster3, Monster4, MonsterMax, Void,
};
pub(self) use tutorlolv2_gen::{AbilityLike, AbilityName, Attrs, DamageType, E, P, Q, R, W};
pub(self) use tutorlolv2_macros::champion_generator;

macro_rules! offset {
    ($(($letter:ident, $value:literal)),+$(,)?) => {
        $(
            pub const $letter: usize = $value;
        )+
    }
}

offset![(P, 0), (Q, 1), (W, 2), (E, 3), (R, 4)];

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    paste::paste! {
        pub mod [<Name:lower>];
    }
});
