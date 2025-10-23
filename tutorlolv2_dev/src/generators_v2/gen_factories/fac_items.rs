use crate::{
    essentials::ext::FilePathExt,
    generators_v2::{Generator, MayFail, gen_decl::decl_items::*},
    items::{CdnItem, Item},
};
use tutorlolv2_gen::{INTERNAL_ITEMS, ItemId};

pub struct ItemGeneratorFactory;

impl ItemGeneratorFactory {
    pub const NUMBER_OF_ITEMS: usize = INTERNAL_ITEMS.len();
    pub const GENERATOR_FUNCTIONS: [fn(CdnItem) -> Box<dyn Generator<Item>>;
        Self::NUMBER_OF_ITEMS] =
        tutorlolv2_macros::expand_dir!("../internal/items", |[Name]| Name::new);

    pub fn run_all() {
        for i in 0..Self::NUMBER_OF_ITEMS {
            let item_id = unsafe { std::mem::transmute::<_, ItemId>(i as u16) };
            let result = Self::run(item_id);
            match result {
                Ok(item) => {
                    let json_string = serde_json::to_string_pretty(&item).unwrap();
                    format!("internal/items/{item_id:?}.json")
                        .write_to_file(json_string.as_bytes())
                        .unwrap();
                }
                Err(e) => {
                    println!("Error generating {item_id:?}: {e:?}.");
                }
            };
        }
    }

    pub fn run(item_id: ItemId) -> MayFail<Item> {
        let data = format!("cache/cdn/items/{item_id:?}.json").read_json::<CdnItem>()?;
        let function = Self::GENERATOR_FUNCTIONS[item_id as usize];
        let generator = function(data);
        Ok(generator.generate()?)
    }
}
