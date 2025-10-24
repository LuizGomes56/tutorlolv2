pub(self) use super::{gen_decl::decl_items::*, gen_factories::fac_items::ItemData, Generator};
pub(self) use crate::MayFail;
pub(self) use tutorlolv2_macros::item_generator;

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    paste::paste! {
        pub mod [<Name:snake>];
    }
});
