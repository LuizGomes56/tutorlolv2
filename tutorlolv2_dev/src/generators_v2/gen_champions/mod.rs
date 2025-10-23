pub(self) use crate::{
    generators_v2::{Generator, MayFail, gen_decl::decl_champions::*},
    model::champions::{Ability, Champion},
};
use tutorlolv2_gen::EvalIdent;
pub(self) use tutorlolv2_gen::{AbilityLike, AbilityName, Attrs, DamageType};
pub(self) use tutorlolv2_macros::generator_v2;

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    paste::paste! {
        pub mod [<Name:lower>];
    }
});
