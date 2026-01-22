use crate::{
    JsonRead, JsonWrite, MayFail,
    client::{SaveTo, Tag},
    gen_utils::RegExtractor,
    generators::{Generator, gen_decl::decl_items::*},
    items::{Effect, Item, MerakiItem},
    parallel_read,
    riot::RiotCdnItem,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::Value;
use tutorlolv2_fmt::{rustfmt, to_ssnake};
use tutorlolv2_gen::{Attrs, DamageType, ItemId};

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
    pub const GENERATOR_FOLDER: &str = "tutorlolv2_dev/src/generators/gen_items";
    pub const GENERATOR_FUNCTIONS: [fn(ItemData) -> Box<dyn Generator<ItemData>>;
        ItemId::VARIANTS] =
        tutorlolv2_macros::expand_dir!("../internal/items", |[File]| %pascal(File.name)%::new);

    /// Runs all item generators, stopping the execution if one of them fails
    pub fn run_all() -> MayFail {
        ItemId::ARRAY.into_par_iter().for_each(|item_id| {
            Self::run(item_id)
                .unwrap()
                .current_data
                .into_file(format!("internal/items/{item_id:?}.json"))
                .unwrap();
        });
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

    pub fn create_from_raw(entity_id: &str) -> MayFail<String> {
        if let Ok(data) = std::fs::read_to_string(SaveTo::Generator(Tag::Items, entity_id).path()) {
            if data.contains("#![stable]") || data.contains("#![preserve]") {
                return Ok(data);
            }
        }

        let generated_content = format!(
            "use super::*;

            impl Generator<ItemData> for {entity_id} {{
                fn generate(self: Box<Self>) -> MayFail<ItemData> {{
                    /* No implementation */
                    self.end()
                }}
            }}"
        );

        let formatted = rustfmt(&generated_content);
        Ok(match formatted.is_empty() {
            true => generated_content,
            false => formatted,
        })
    }

    pub fn create(item_id: ItemId) -> MayFail<String> {
        Self::create_from_raw(&format!("{item_id:?}"))
    }

    pub fn create_all_raw() -> MayFail {
        parallel_read(SaveTo::InternalDir(Tag::Items).path(), |name, _: Value| {
            let entity_id = to_ssnake(name).to_lowercase();
            match Self::create_from_raw(name) {
                Ok(data) => Ok(std::fs::write(
                    SaveTo::Generator(Tag::Items, &entity_id).path(),
                    data,
                )?),
                Err(e) => Err(format!(
                    "Error trying to create generator file for entity ItemId::{name:?}, {e:?}"
                )
                .into()),
            }
        })
    }

    pub fn create_all() -> MayFail {
        let dir = SaveTo::GeneratorDir(Tag::Items).path();
        if !std::fs::exists(&dir)? {
            std::fs::create_dir(dir)?;
        }

        ItemId::ARRAY.into_par_iter().for_each(|champion_id| {
            let Ok(data) = Self::create(champion_id) else {
                return println!("Unable to create generator file for {champion_id:?}");
            };
            let file_name = format!("{champion_id:?}").to_lowercase();
            std::fs::write(
                SaveTo::Generator(Tag::Items, &file_name).path(),
                data.as_bytes(),
            )
            .unwrap();
        });

        Ok(())
    }
}
