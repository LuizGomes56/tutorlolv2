use crate::{
    ENV_CONFIG, JsonRead, JsonWrite, MayFail,
    client::{SaveTo, Tag},
    gen_utils::RegExtractor,
    generators::{Generator, gen_decl::decl_items::*},
    items::{Effect, Item, ItemStats, MerakiItem},
    riot::RiotCdnItem,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::Value;
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_gen::{Attrs, DamageType, ItemId, StatName};

pub struct ItemData {
    pub meraki_data: Option<MerakiItem>,
    pub riot_data: RiotCdnItem,
    pub current_data: Item,
}

impl ItemData {
    /// Creates a new struct that serves as argument for the item generator functions
    pub const fn new(
        meraki_data: Option<MerakiItem>,
        riot_data: RiotCdnItem,
        current_data: Item,
    ) -> Self {
        Self {
            meraki_data,
            riot_data,
            current_data,
        }
    }

    pub fn infer_stats_ifdef(&mut self) {
        let data = &mut self.current_data;
        let ItemStats {
            ability_power,
            armor,
            armor_penetration,
            magic_penetration,
            attack_damage,
            attack_speed,
            critical_strike_chance,
            critical_strike_damage,
            health,
            lifesteal,
            magic_resistance,
            mana,
            movespeed,
            omnivamp,
        } = &mut data.stats;
        const fn assign(stat: &mut f64, value: u16) {
            *stat = value as _;
        }
        for (s, value) in &data.prettified_stats {
            let v = *value;
            match s {
                StatName::AdaptiveForce => { /* Unknown */ }
                StatName::AbilityPower => assign(&mut ability_power.flat, v),
                StatName::Armor => assign(&mut armor.flat, v),
                StatName::ArmorPenetration => assign(&mut armor_penetration.percent, v),
                StatName::AttackDamage => assign(&mut attack_damage.flat, v),
                StatName::AttackSpeed => assign(&mut attack_speed.flat, v),
                StatName::CriticalStrikeChance => assign(&mut critical_strike_chance.flat, v),
                StatName::CriticalStrikeDamage => assign(&mut critical_strike_damage.flat, v),
                StatName::Health => assign(&mut health.flat, v),
                StatName::Lethality => assign(&mut armor_penetration.flat, v),
                StatName::LifeSteal => assign(&mut lifesteal.flat, v),
                StatName::MagicPenetration => assign(&mut magic_penetration.flat, v),
                StatName::MagicResist => assign(&mut magic_resistance.flat, v),
                StatName::Mana => assign(&mut mana.flat, v),
                StatName::MoveSpeed => assign(&mut movespeed.flat, v),
                StatName::Omnivamp => assign(&mut omnivamp.flat, v),
                _ => {}
            }
        }
    }

    pub fn yield_to(&mut self, item_id: ItemId) -> MayFail {
        let data = ItemFactory::run(item_id)?;
        self.current_data.melee = data.current_data.melee;
        self.current_data.ranged = data.current_data.ranged;
        self.damage_type(data.current_data.damage_type);
        self.infer_stats_ifdef();
        Ok(())
    }

    fn meraki_data(&self) -> MayFail<&MerakiItem> {
        let riot_id = self.current_data.riot_id;
        let item_name = &self.current_data.name;
        self.meraki_data
            .as_ref()
            .ok_or(format!("No meraki_data for item {item_name} ({riot_id})").into())
    }

    /// Returns the damage of the `passive` or `active` effect in the field `field`
    fn effect_damage(&self, from: &[Effect], offset: usize, field: &str) -> MayFail<String> {
        let name = &self.meraki_data()?.name;
        Ok(from
            .get(offset)
            .ok_or(format!(
                "[{name}]: meraki_data.{field}[{offset}] does not exist"
            ))?
            .effects
            .get_damage())
    }

    /// Returns the damage of the `passive` effect in the field `passives`
    pub fn passive(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data()?.passives, offset, "passives")
    }

    /// Returns the damage of the `active` effect in the field `active`
    pub fn active(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data()?.active, offset, "active")
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
        ItemId::VARIANTS] = tutorlolv2_macros::expand_dir!("../internal/items", |[Name]| Name::new);

    /// Runs all item generators, stopping the execution if one of them fails
    pub fn run_all() -> MayFail {
        ItemId::VALUES
            .into_par_iter()
            .for_each(|item_id| match Self::run(item_id) {
                Ok(data) => data
                    .current_data
                    .into_file(SaveTo::Internal(Tag::Items, &format!("{item_id:?}")).path())
                    .unwrap(),
                Err(e) => panic!("Unable to run generator for {item_id:?}, Error: {e}"),
            });
        Ok(())
    }

    pub fn clean() -> MayFail {
        for item_id in ItemId::VALUES {
            let fname = format!("{item_id:?}");
            let generator_path = Self::generator_path(&fname);
            let path = SaveTo::Internal(Tag::Items, &fname).path();
            let json = Value::from_file(&path)?;

            if let Some(version) = json.get("version")
                && let Some(version) = version.as_str()
                && version != ENV_CONFIG.lol_version
            {
                if let Ok(data) = Self::read_gen(&fname)
                    && Self::is_stable(&data)
                {
                    println!("ItemId::{item_id:?} is stable but no longer available");
                    continue;
                }
            }

            std::fs::remove_file(path)?;
            std::fs::remove_file(generator_path)?;
        }
        Ok(())
    }

    /// Runs some item generator, taking its generated data and saving to an internal folder
    pub fn run(item_id: ItemId) -> MayFail<ItemData> {
        let riot_id = &item_id.to_riot_id().to_string();
        let meraki_data =
            MerakiItem::from_file(SaveTo::MerakiCache(Tag::Items, riot_id).path()).ok();
        let riot_data = RiotCdnItem::from_file(SaveTo::RiotCache(Tag::Items, riot_id).path())?;
        let current_data =
            Item::from_file(SaveTo::Internal(Tag::Items, &format!("{item_id:?}")).path())?;

        let function = Self::GENERATOR_FUNCTIONS[item_id as usize];
        let generator = function(ItemData::new(meraki_data, riot_data, current_data));
        Ok(generator.generate()?)
    }

    pub fn is_stable(data: &str) -> bool {
        data.contains("#![stable]") || data.contains("#![preserve]")
    }

    pub fn generator_path(entity_id: &str) -> String {
        SaveTo::Generator(Tag::Items, &to_ssnake(entity_id).to_lowercase()).path()
    }

    pub fn read_gen(entity_id: &str) -> std::io::Result<String> {
        std::fs::read_to_string(Self::generator_path(entity_id))
    }
}
