use crate::{
    JsonRead, JsonWrite, MayFail,
    gen_utils::RegExtractor,
    generators::{Generator, gen_decl::decl_items::*},
    items::{Effect, Item, MerakiItem},
    riot::RiotCdnItem,
};
use tutorlolv2_gen::{Attrs, DamageType, ITEM_CACHE, ItemId};

pub struct ItemData {
    pub meraki_data: MerakiItem,
    pub riot_data: RiotCdnItem,
    pub current_data: Item,
}

impl ItemData {
    /// Creates a new struct that serves as argument for the item generator functions
    pub const fn new(meraki_data: MerakiItem, riot_data: RiotCdnItem, current_data: Item) -> Self {
        Self {
            meraki_data,
            riot_data,
            current_data,
        }
    }

    /// Returns the damage of the `passive` or `active` effect in the field `field`
    fn effect_damage(&self, from: &[Effect], offset: usize, field: &str) -> MayFail<String> {
        Ok(from
            .get(offset)
            .ok_or(format!(
                "[{name}]: meraki_data.{field}[{offset}] does not exist",
                name = self.meraki_data.name,
            ))?
            .effects
            .get_damage())
    }

    /// Returns the damage of the `passive` effect in the field `passives`
    pub fn passive(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data.passives, offset, "passives")
    }

    /// Returns the damage of the `active` effect in the field `active`
    pub fn active(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data.active, offset, "active")
    }

    /// Sets some attribute for the current item
    pub const fn attr(&mut self, attr: Attrs) {
        self.current_data.attributes = attr;
    }

    /// Sets the damage type of the current item
    pub const fn damage_type(&mut self, damage_type: DamageType) {
        self.current_data.damage_type = damage_type;
    }

    /// Sets the damage only for the `melee` and `minimum_damage` fields
    pub fn melee_min_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.current_data.melee.minimum_damage = dmg.as_ref().to_string();
    }

    /// Sets the damage only for the `melee` and `maximum_damage` fields
    pub fn melee_max_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.current_data.melee.maximum_damage = dmg.as_ref().to_string();
    }

    /// Sets the damage only for the `ranged` and `minimum_damage` fields
    pub fn ranged_min_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.current_data.ranged.minimum_damage = dmg.as_ref().to_string();
    }

    /// Sets the damage only for the `ranged` and `maximum_damage` fields
    pub fn ranged_max_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.current_data.ranged.maximum_damage = dmg.as_ref().to_string();
    }

    /// Sets only the `minimum_damage` field for both `melee` and `ranged` fields
    pub fn const_min_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.melee_min_dmg(&dmg);
        self.ranged_min_dmg(dmg);
    }

    /// Sets only the `maximum_damage` field for both `melee` and `ranged` fields
    pub fn const_max_dmg<S: AsRef<str>>(&mut self, dmg: S) {
        self.melee_max_dmg(&dmg);
        self.ranged_max_dmg(dmg);
    }

    /// Sets the `minimum_damage` and `maximum_damage` to both `melee` and `ranged`
    /// fields according to the provided values
    pub fn const_dmg<S: AsRef<str>>(&mut self, min_dmg: S, max_dmg: S) {
        self.const_min_dmg(min_dmg);
        self.const_max_dmg(max_dmg);
    }
}

pub struct ItemFactory;

impl ItemFactory {
    pub const NUMBER_OF_ITEMS: usize = ITEM_CACHE.len();
    pub const GENERATOR_FUNCTIONS: [fn(ItemData) -> Box<dyn Generator<ItemData>>;
        Self::NUMBER_OF_ITEMS] =
        tutorlolv2_macros::expand_dir!("../internal/items", |[Name]| Name::new);

    /// Runs all item generators, stopping the execution if one of them fails
    pub fn run_all() -> MayFail {
        for i in 0..Self::NUMBER_OF_ITEMS as u16 {
            let item_id = unsafe { ItemId::from_u16_unchecked(i) };
            Self::run(item_id)?
                .current_data
                .into_file(format!("internal/items/{item_id:?}.json"))?;
        }
        Ok(())
    }

    /// Runs some item generator, taking its generated data and saving to an internal folder
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
