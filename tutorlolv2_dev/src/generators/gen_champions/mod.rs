pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_decl::decl_champions::*},
    model::champions::{Ability, Champion},
};
pub(self) use tutorlolv2_gen::{enums::*, eval::*};
pub(self) use tutorlolv2_types::{AbilityLike, AbilityName, E, P, Q, R, W, ability_name::*};

macro_rules! dynvec {
    ($($field:expr),+) => {
        [$({
            let value: AbilityLike = $field.into();
            value
        }),+]
    };
}

macro_rules! offset {
    ($($letter:ident),+$(,)?) => {
        $(
            pub const $letter: AbilityLike = AbilityLike::$letter(AbilityName::Void);
        )+
    }
}

offset![P, Q, W, E, R];

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    paste::paste! {
        pub mod [<Name:lower>];
    }
});
