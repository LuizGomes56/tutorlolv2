pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_decl::decl_champions::*, gen_utils::RegExtractor},
    model::champions::{Ability, Champion},
};
pub(self) use tutorlolv2_gen::{AbilityId, AbilityName, ability_name::*, enums::*, eval::*};

macro_rules! dynarr {
    ($($field:expr),+) => {
        [$({
            let value: AbilityId = $field.into();
            value
        }),+]
    };
}

macro_rules! offset {
    ($($letter:ident),+$(,)?) => {
        $(
            pub const $letter: AbilityId = AbilityId::$letter(AbilityName::Void);
        )+
    }
}

offset![P, Q, W, E, R];

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pastey::paste! {
        pub mod [<Name:lower>];
    }
});
