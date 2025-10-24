use crate::{
    JsonRead, JsonWrite, MayFail,
    generators::{Generator, gen_decl::decl_items::*},
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
    pub const GENERATOR_FUNCTIONS: [fn(ItemData) -> Box<dyn Generator<ItemData>>;
        Self::NUMBER_OF_ITEMS] =
        tutorlolv2_macros::expand_dir!("../internal/items", |[Name]| Name::new);

    pub fn run_all() -> MayFail {
        for i in 0..Self::NUMBER_OF_ITEMS {
            let item_id = unsafe { std::mem::transmute::<_, ItemId>(i as u16) };
            Self::run(item_id)?
                .current_data
                .into_file(format!("internal/items/{item_id:?}.json"))?;
        }
        Ok(())
    }

    pub fn run(item_id: ItemId) -> MayFail<ItemData> {
        let riot_id = item_id.to_riot_id();
        let meraki_data = MerakiItem::from_file(format!("cache/meraki/items/{riot_id}.json"))?;
        let riot_data = RiotCdnItem::from_file(format!("cache/riot/items/{riot_id}.json"))?;
        let current_data = Item::from_file(format!("internal/items/{item_id:?}.json"))?;

        let function = Self::GENERATOR_FUNCTIONS[item_id as usize];
        let generator = function(ItemData::new(meraki_data, riot_data, current_data));
        Ok(generator.generate()?)
    }
}
