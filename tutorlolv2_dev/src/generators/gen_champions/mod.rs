pub(self) use crate::{
    MayFail, Step,
    Step::*,
    generators::{Generator, gen_decl::decl_champions::*, gen_utils::RegExtractor},
    model::champions::{Ability, Champion},
};
pub(self) use tutorlolv2_gen::{
    AbilityId,
    AbilityId::*,
    AbilityName::*,
    ComboElement::*,
    Key,
    enums::{Attrs::*, DamageType::*},
    eval::CtxVar::*,
};

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pastey::paste! {
        pub mod [<Name:lower>];
    }
});
