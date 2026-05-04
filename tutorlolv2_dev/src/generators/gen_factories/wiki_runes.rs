use crate::{ENV_CONFIG, JsonRead, MayFail, Progress, client::Tag, gen_factories::Parser};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use tutorlolv2_types::DamageType;
use tutorlolv2_wiki::runes::WikiRune;

pub struct RuneParser {
    pub data: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
        todo!()
        // self.data
        //     .get(rune_id)
        //     .and_then(|data| {
        //         let function = rune_gen_fn(rune_id)?;
        //         Some(function(data.clone()))
        //     })
        //     .ok_or_else(|| format!("[WikiFactory::run] {rune_id} not found"))?
        //     .call()
    }

    fn keys(&self) -> Vec<&str> {
        self.data.keys().map(String::as_str).collect()
    }

    fn create_methods(&self, result: &mut String, id: &str) {
        let data = &self.data[id];

        todo!()
    }

    fn new() -> MayFail<Self> {
        let data = BTreeMap::<String, _>::from_file("cache/wiki/runes/full.json")?;
        Ok(Self { data })
    }
}

impl Rune {
    pub fn new(data: WikiRune) -> Self {
        Self {
            data,
            version: ENV_CONFIG.lol_version.clone(),
            ..Default::default()
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != ENV_CONFIG.lol_version
    }

    pub const fn progress(&mut self, progress: Progress) -> &mut Self {
        self.progress = progress;
        self
    }

    pub fn end(&mut self) -> MayFail {
        Ok(())
    }

    pub fn damage_type(&mut self, damage_type: DamageType) -> MayFail<&mut Self> {
        self.damage_type = damage_type;
        Ok(self)
    }
}
