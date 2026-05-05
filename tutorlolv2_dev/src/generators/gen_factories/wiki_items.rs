use crate::{
    ENV_CONFIG, JsonRead, MayFail, client::Tag, gen_factories::Parser, gen_items::item_gen_fn,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_gen::DamageType;
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
                result.push_str(&format!(".min({tag})"));
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub data: WikiItem,
    pub damage_type: DamageType,
}

pub enum Source {
    Active,
    Passive,
}

impl Item {
    pub fn new(data: WikiItem) -> Self {
        Self {
            data,
            damage_type: Default::default(),
        }
    }

    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn min(&mut self, source: Source) -> &mut Self {
        self
    }

    pub fn end(&self) -> MayFail {
        todo!()
    }
}
