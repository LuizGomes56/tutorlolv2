use crate::{
    ENV_CONFIG, JsonRead, MayFail, Progress, client::Tag, gen_factories::Parser,
    gen_runes::rune_gen_fn,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_types::DamageType;
use tutorlolv2_wiki::runes::WikiRune;

pub struct RuneParser {
    pub data: BTreeMap<String, WikiRune>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rune {
    pub data: WikiRune,
    pub damage_type: DamageType,
    pub minimum_damage: Option<String>,
    pub maximum_damage: Option<String>,
    progress: Progress,
    version: String,
}

impl Parser<Rune> for RuneParser {
    const TAG: Tag = Tag::Runes;

    fn run_fn(&self, id: &str) -> MayFail<Rune> {
        self.data
            .get(id)
            .and_then(|data| {
                let function = rune_gen_fn(id)?;
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

        for (i, (key, effect)) in data.effects.iter().enumerate() {
            if effect.formula.is_some() {
                result.push_str(&format!(".min({i}) /* {key} */"));
            }
        }

        for description in &data.descriptions {
            Self::infer_damage_type(result, &description)
        }
    }

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/runes/full.json")?,
        })
    }
}

impl Rune {
    pub fn new(data: WikiRune) -> Self {
        Self {
            data,
            version: ENV_CONFIG.lol_version.clone(),
            damage_type: Default::default(),
            minimum_damage: Default::default(),
            maximum_damage: Default::default(),
            progress: Default::default(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != ENV_CONFIG.lol_version
    }

    pub const fn progress(&mut self, progress: Progress) -> &mut Self {
        self.progress = progress;
        self
    }

    pub fn end(&self) -> MayFail {
        Ok(())
    }

    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn min(&mut self, index: usize) -> &mut Self {
        if let Some(effect) = self.data.effects.values().nth(index)
            && let Some(formula) = &effect.formula
        {
            self.minimum_damage = Some(formula.clone());
        }
        self
    }
}
