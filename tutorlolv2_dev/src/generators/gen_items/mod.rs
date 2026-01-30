pub(self) use super::{
    Generator, gen_decl::decl_items::*, gen_factories::fac_items::ItemData, gen_utils::RegExtractor,
};
pub(self) use crate::{MayFail, gen_factories::fac_items::ItemFactory};
pub(self) use tutorlolv2_gen::{ItemId, enums::*, eval::*};

tutorlolv2_macros::expand_dir!("../internal/items", |File| {
    pub mod %snake(File)%;
});
