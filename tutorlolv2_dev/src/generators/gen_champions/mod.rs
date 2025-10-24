pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_decl::decl_champions::*},
    model::champions::{Ability, Champion},
};
use tutorlolv2_gen::EvalIdent;
pub(self) use tutorlolv2_gen::{AbilityLike, AbilityName, Attrs, DamageType};
pub(self) use tutorlolv2_macros::champion_generator;

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    paste::paste! {
        pub mod [<Name:lower>];
    }
});
