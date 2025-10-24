pub(self) use super::{Generator, MayFail, gen_decl::decl_items::*};
pub(self) use crate::items::Item;
pub(self) use tutorlolv2_macros::item_generator;

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    paste::paste! {
        pub mod [<Name:snake>];
    }
});
