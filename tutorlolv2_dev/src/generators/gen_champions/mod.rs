pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_decl::decl_champions::*, gen_utils::RegExtractor},
    model::champions::{Ability, Champion},
};
pub(self) use tutorlolv2_gen::{AbilityId, ability_name::*, enums::*, eval::*};

macro_rules! dynarr {
    ($($field:expr),+) => {
        [$($field.cast()),+]
    };
}

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pastey::paste! {
        pub mod [<Name:lower>];
    }
});
