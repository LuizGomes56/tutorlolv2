use crate::{
    ENV_CONFIG, JsonRead, MayFail, Progress, client::Tag, gen_factories::Parser,
    gen_items::item_gen_fn,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_types::DamageType;
use tutorlolv2_wiki::items::item_parser::{ItemEffect, WikiItem};

pub struct ItemParser {
    pub data: BTreeMap<String, WikiItem>,
}

impl Parser<Item> for ItemParser {
    const TAG: Tag = Tag::Items;

    fn run_fn(&self, id: &str) -> MayFail<Item> {
        self.data
            .get(id)
            .and_then(|data| {
                let function = item_gen_fn(id)?;
                Some(function(data.clone()))
            })
            .ok_or_else(|| format!("[WikiFactory::run] {id} not found"))?
            .call()
    }

    fn keys(&self) -> Vec<&str> {
        self.data.keys().map(String::as_str).collect()
    }

    fn create_methods(&self, result: &mut String, id: &str) {
        let data = &self.data[id];

        let mut new_method = |field: &Option<ItemEffect>, tag| {
            if let Some(ie) = &field
                && ie.effect.formula.is_some()
            {
                result.push_str(&format!(".min({tag})?"));
            }
        };

        new_method(&data.effects.act, "Active");
        new_method(&data.effects.pass, "Passive");

        if let Some(ie) = data.effects.act.as_ref().or(data.effects.pass.as_ref()) {
            Self::infer_damage_type(result, &ie.effect.inner.description);
        }
    }

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/items/full.json")?,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemDamage {
    pub minimum_damage: String,
    pub maximum_damage: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub data: WikiItem,
    pub damage_type: DamageType,
    pub ranged: ItemDamage,
    pub melee: ItemDamage,
    version: String,
    progress: Progress,
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Active,
    Passive,
}

impl Item {
    pub fn new(data: WikiItem) -> Self {
        Self {
            data,
            version: ENV_CONFIG.lol_version.clone(),
            progress: Default::default(),
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != ENV_CONFIG.lol_version
    }

    pub const fn progress(&mut self, progress: Progress) -> &mut Self {
        self.progress = progress;
        self
    }

    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, source: Source) -> MayFail<String> {
        let effects = &self.data.effects;

        let item_effect = match source {
            Source::Active => &effects.act,
            Source::Passive => &effects.pass,
        };

        match item_effect {
            Some(ie) if let Some(ref formula) = ie.effect.formula => Ok(formula.clone()),
            _ => Err(format!(
                "[{name}] No formula for its {source:?}",
                name = self.data.name
            )
            .into()),
        }
    }

    pub fn min(&mut self, source: Source) -> MayFail<&mut Self> {
        self.ranged_min(source)?;
        self.melee_min(source)?;
        Ok(self)
    }

    pub fn max(&mut self, source: Source) -> MayFail<&mut Self> {
        self.ranged_max(source)?;
        self.melee_max(source)?;
        Ok(self)
    }

    pub fn melee_min(&mut self, source: Source) -> MayFail<&mut Self> {
        self.melee.minimum_damage = self.formula(source)?;
        Ok(self)
    }

    pub fn melee_max(&mut self, source: Source) -> MayFail<&mut Self> {
        self.melee.maximum_damage = self.formula(source)?;
        Ok(self)
    }

    pub fn ranged_min(&mut self, source: Source) -> MayFail<&mut Self> {
        self.ranged.minimum_damage = self.formula(source)?;
        Ok(self)
    }

    pub fn ranged_max(&mut self, source: Source) -> MayFail<&mut Self> {
        self.ranged.maximum_damage = self.formula(source)?;
        Ok(self)
    }

    pub fn end(&self) -> MayFail {
        Ok(())
    }
}
