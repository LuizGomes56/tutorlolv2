pub(self) use crate::{
    MayFail,
    generators::{
        Generator, gen_decl::decl_champions::*, gen_factories::fac_champions::Key,
        gen_utils::RegExtractor,
    },
    model::champions::{Ability, Champion},
};
pub(self) use tutorlolv2_gen::{
    AbilityId,
    AbilityId::*,
    AbilityName::*,
    enums::{Attrs::*, DamageType::*},
    eval::CtxVar::*,
};

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pastey::paste! {
        pub mod [<Name:lower>];
    }
});
