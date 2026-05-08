use crate::{
    GeneratorExt, JsonRead, MayFail,
    client::Tag,
    gen_factories::{DamageObject, Parser, infer_damage_type, likely_damages},
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
    #[serde(flatten)]
    pub data: WikiRune,
    pub damage_type: DamageType,
    #[serde(flatten)]
    pub damage: DamageObject,
}

impl Parser<WikiRune, Rune> for RuneParser {
    const TAG: Tag = Tag::Runes;
    const FN: fn(&str) -> Option<fn(WikiRune) -> Box<dyn GeneratorExt<Rune>>> = rune_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiRune> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> bool {
        let data = &self.data[id];

        for description in &data.descriptions {
            match likely_damages(description) {
                true => infer_damage_type(result, &description),
                false => return false,
            }
        }

        for (i, (key, effect)) in data.effects.iter().enumerate() {
            if effect.formula.is_some() {
                result.push_str(&format!(".min({i})? /* {key} */"));
            }
        }

        true
    }

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/runes/full.json")?,
        })
    }
}

impl From<WikiRune> for Rune {
    fn from(data: WikiRune) -> Self {
        Self {
            data,
            damage_type: Default::default(),
            damage: Default::default(),
        }
    }
}

impl Rune {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, index: usize) -> MayFail<String> {
        match self.data.effects.values().nth(index) {
            Some(effect) if let Some(formula) = &effect.formula => Ok(formula.clone()),
            _ => Err(format!(
                "[{name}] No formula found in effect[{index}]",
                name = self.data.name
            )
            .into()),
        }
    }

    pub fn min(&mut self, index: usize) -> MayFail<&mut Self> {
        self.damage.minimum_damage = self.formula(index)?;
        Ok(self)
    }

    pub fn max(&mut self, index: usize) -> MayFail<&mut Self> {
        self.damage.maximum_damage = self.formula(index)?;
        Ok(self)
    }

    pub fn end(&self) -> MayFail {
        Ok(())
    }
}
