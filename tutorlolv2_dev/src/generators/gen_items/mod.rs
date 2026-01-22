pub(self) use super::{
    Generator, gen_decl::decl_items::*, gen_factories::fac_items::ItemData, gen_utils::RegExtractor,
};
pub(self) use crate::MayFail;
pub(self) use tutorlolv2_gen::{enums::*, eval::*};

tutorlolv2_macros::expand_dir!("../internal/items", |File| {
    pub mod %snake(pascal(File.name))%;
});
