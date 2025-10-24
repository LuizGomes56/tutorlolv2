use crate::{
    ext::FilePathExt,
    generators::{Generator, MayFail, gen_decl::decl_items::*},
    items::{Item, MerakiItem},
    riot::RiotCdnItem,
};
use tutorlolv2_gen::{INTERNAL_ITEMS, ItemId};

pub struct ItemData {
    pub meraki_data: MerakiItem,
    pub riot_data: RiotCdnItem,
    pub current_data: Item,
}

impl ItemData {
    pub fn new(meraki_data: MerakiItem, riot_data: RiotCdnItem, current_data: Item) -> Self {
        Self {
            meraki_data,
            riot_data,
            current_data,
        }
    }
}

pub struct ItemFactory;

impl ItemFactory {
    pub const NUMBER_OF_ITEMS: usize = INTERNAL_ITEMS.len();
    pub const GENERATOR_FUNCTIONS: [fn(ItemData) -> Box<dyn Generator<Item>>;
        Self::NUMBER_OF_ITEMS] =
        tutorlolv2_macros::expand_dir!("../internal/items", |[Name]| Name::new);

    pub fn run_all() {
        for i in 0..Self::NUMBER_OF_ITEMS {
            let item_id = unsafe { std::mem::transmute::<_, ItemId>(i as u16) };
            let result = Self::run(item_id);
            match result {
                Ok(item) => {
                    format!("internal/items/{item_id:?}.json")
                        .write_json(&item)
                        .unwrap();
                }
                Err(e) => {
                    println!("Error generating {item_id:?}: {e:?}.");
                }
            };
        }
    }

    pub fn run(item_id: ItemId) -> MayFail<Item> {
        let meraki_data = format!("cache/cdn/items/{item_id:?}.json").read_json::<MerakiItem>()?;
        let riot_data = format!("cache/riot/items/{item_id:?}.json").read_json::<RiotCdnItem>()?;
        let current_data = format!("internal/items/{item_id:?}.json").read_json::<Item>()?;

        let function = Self::GENERATOR_FUNCTIONS[item_id as usize];
        let generator = function(ItemData::new(meraki_data, riot_data, current_data));
        Ok(generator.generate()?)
    }
}
