use std::str::FromStr;

use crate::{
    ENV_CONFIG, JsonRead, JsonWrite, MayFail,
    client::{SaveTo, Tag},
    gen_utils::RegExtractor,
    generators::gen_items::*,
    items::{Effect, Item, ItemStats, MerakiItem},
    riot::RiotCdnItem,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::Value;
use tutorlolv2_gen::{Attrs, DamageType, GameMap, ItemId, StatName};

pub struct ItemData {
    pub meraki_data: Option<MerakiItem>,
    pub riot_data: RiotCdnItem,
    pub current_data: Item,
}

pub enum Capture {
    Passive(usize),
    Active(usize),
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
            adaptive_force,
            ability_power,
            armor,
            armor_penetration,
            magic_penetration,
            attack_damage,
            attack_speed,
            crit_chance,
            crit_damage,
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
                StatName::AdaptiveForce => *adaptive_force = v as _,
                StatName::AbilityPower => assign(&mut ability_power.flat, v),
                StatName::Armor => assign(&mut armor.flat, v),
                StatName::ArmorPenetration => assign(&mut armor_penetration.percent, v),
                StatName::AttackDamage => assign(&mut attack_damage.flat, v),
                StatName::AttackSpeed => assign(&mut attack_speed.flat, v),
                StatName::CritChance => assign(&mut crit_chance.flat, v),
                StatName::CritDamage => assign(&mut crit_damage.flat, v),
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

    pub fn try_yield(&mut self, name: &str) -> MayFail {
        println!("[try] Attempting to yield generator of ItemId::{name}");
        let base = name.trim_end_matches("Arena");
        let Ok(item_id) = ItemId::from_str(base) else {
            println!("[warn] Failed to yield generator of ItemId::{name}");
            return Ok(());
        };
        println!("[ok] Yielding generator of ItemId::{name} to ItemId::{item_id:?}");
        self.yield_to(item_id)
    }

    pub fn yield_to(&mut self, item_id: ItemId) -> MayFail {
        let data = ItemFactory::run_fn(item_id.debug(), item_id.to_riot_id())?;
        self.current_data.melee = data.current_data.melee;
        self.current_data.ranged = data.current_data.ranged;
        self.damage_type(data.current_data.damage_type);
        self.attr(data.current_data.attributes);
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

    pub fn has_map(&self, game_map: GameMap) -> bool {
        self.current_data
            .maps
            .iter()
            .find_map(|(map, v)| (*map == game_map).then_some(*v))
            .unwrap_or(false)
    }

    /// Returns the damage of the `passive` effect in the field `passives`
    pub fn passive(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data()?.passives, offset, "passives")
    }

    pub fn scaling(&self, from: Capture) -> MayFail<[f64; 2]> {
        match from {
            Capture::Passive(i) => self.passive(i),
            Capture::Active(i) => self.active(i),
        }?
        .capture_numbers::<f64>()
        .get(0..2)
        .ok_or("Failed to get melee/ranged scaling at Self::scalings")?
        .try_into()
        .map_err(|v| format!("Self::scalings failed, error: {v:?}").into())
    }

    pub fn capture(&self, from: Capture, f: impl Fn(f64) -> String) -> MayFail<[String; 2]> {
        Ok(self.scaling(from)?.map(f))
    }

    /// Returns the damage of the `active` effect in the field `active`
    pub fn active(&self, offset: usize) -> MayFail<String> {
        self.effect_damage(&self.meraki_data()?.active, offset, "active")
    }

    /// Sets some attribute for the current item
    pub const fn attr(&mut self, attr: Attrs) -> &mut Self {
        self.current_data.attributes = attr;
        self
    }

    pub fn end(&mut self) -> MayFail {
        Ok(self.infer_stats_ifdef())
    }

    /// Sets the damage type of the current item
    pub const fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.current_data.damage_type = damage_type;
        self
    }

    /// Sets the damage only for the `melee` and `minimum_damage` fields
    pub fn melee_min_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.current_data.melee.minimum_damage = dmg.to_string().trim().to_string();
        self
    }

    /// Sets the damage only for the `melee` and `maximum_damage` fields
    pub fn melee_max_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.current_data.melee.maximum_damage = dmg.to_string().trim().to_string();
        self
    }

    /// Sets the damage only for the `ranged` and `minimum_damage` fields
    pub fn ranged_min_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.current_data.ranged.minimum_damage = dmg.to_string().trim().to_string();
        self
    }

    /// Sets the damage only for the `ranged` and `maximum_damage` fields
    pub fn ranged_max_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.current_data.ranged.maximum_damage = dmg.to_string().trim().to_string();
        self
    }

    /// Sets only the `minimum_damage` field for both `melee` and `ranged` fields
    pub fn const_min_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.melee_min_dmg(dmg.to_string());
        self.ranged_min_dmg(dmg)
    }

    /// Sets only the `maximum_damage` field for both `melee` and `ranged` fields
    pub fn const_max_dmg<S: ToString>(&mut self, dmg: S) -> &mut Self {
        self.melee_max_dmg(dmg.to_string());
        self.ranged_max_dmg(dmg)
    }

    /// Adds a `::rust` marker at the beginning of all damage defined damage
    /// fields to indicate that they are rust code, not regular math expressions.
    /// In other words, they may use any rust code that is valid within a const context
    pub fn nonstandard(&mut self) -> &mut Self {
        let data = &mut self.current_data;
        let add_marker = |field: &mut String| {
            if !field.is_empty() && field != "zero" {
                field.insert_str(0, "::rust ");
            }
        };
        add_marker(&mut data.melee.minimum_damage);
        add_marker(&mut data.melee.maximum_damage);
        add_marker(&mut data.ranged.minimum_damage);
        add_marker(&mut data.ranged.maximum_damage);
        self
    }

    /// Sets the `minimum_damage` and `maximum_damage` to both `melee` and `ranged`
    /// fields according to the provided values
    pub fn const_dmg<T: ToString, U: ToString>(&mut self, min_dmg: T, max_dmg: U) -> &mut Self {
        self.const_min_dmg(min_dmg);
        self.const_max_dmg(max_dmg)
    }
}

pub struct ItemFactory;

impl ItemFactory {
    /// Runs all item generators, stopping the execution if one of them fails
    pub fn run_all() -> MayFail {
        ItemId::VALUES
            .into_par_iter()
            .try_for_each(|item_id| Self::run(item_id.debug(), item_id.to_riot_id()))
    }

    pub fn run(name: &str, riot_id: u32) -> MayFail {
        match Self::run_fn(name, riot_id) {
            Ok(data) => data
                .current_data
                .into_file(SaveTo::InternalRaw(Tag::Items, name).path()),
            Err(e) => panic!("Unable to run generator for {name:?}, Error: {e}"),
        }
    }

    pub fn clean() -> MayFail {
        for item_id in ItemId::VALUES {
            let name = item_id.debug();
            let generator_path = SaveTo::GeneratorRaw(Tag::Items, name).path();
            let path = SaveTo::InternalRaw(Tag::Items, name).path();
            let json = Value::from_file(&path)?;

            if let Some(version) = json.get("version")
                && let Some(version) = version.as_str()
                && version != ENV_CONFIG.lol_version
            {
                if let Ok(true) = std::fs::exists(&generator_path) {
                    println!("ItemId::{name} is stable but no longer available");
                    continue;
                }
            }

            std::fs::remove_file(path)?;
            std::fs::remove_file(generator_path)?;
        }
        Ok(())
    }

    /// Runs some item generator, taking its generated data and saving to an internal folder
    pub fn run_fn(name: &str, riot_id: u32) -> MayFail<ItemData> {
        let meraki_data =
            MerakiItem::from_file(SaveTo::MerakiCache(Tag::Items, &riot_id).path()).ok();
        let riot_data = RiotCdnItem::from_file(SaveTo::RiotCache(Tag::Items, &riot_id).path())?;
        let current_data = Item::from_file(SaveTo::InternalRaw(Tag::Items, name).path())?;

        let mut args = ItemData::new(meraki_data, riot_data, current_data);

        match item_gen_fn(name) {
            Some(f) => f(args).call(),
            None => {
                if args.has_map(GameMap::Arena) && name.contains("Arena") {
                    args.try_yield(name)?;
                }
                args.infer_stats_ifdef();
                Ok(args)
            }
        }
        .map(|mut r| {
            r.current_data.version = ENV_CONFIG.lol_version.clone();
            r
        })
    }
}
