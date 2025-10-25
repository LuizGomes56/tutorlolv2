pub(self) use super::{
    Generator, gen_decl::decl_items::*, gen_factories::fac_items::ItemData, gen_utils::RegExtractor,
};
pub(self) use crate::MayFail;
pub(self) use tutorlolv2_gen::{Attrs, DamageType};
pub(self) use tutorlolv2_macros::item_generator;

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    paste::paste! {
        pub mod [<Name:snake>];
    }
});
