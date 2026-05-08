use crate::{
    GeneratorExt, JsonRead, MayFail,
    client::Tag,
    gen_factories::{DamageIndex, DamageRange, Parser, infer_damage_type, likely_damages},
    gen_items::item_gen_fn,
    gen_utils::RegExtractor,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};
use tutorlolv2_types::{AttackType, DamageType};
use tutorlolv2_wiki::items::item_parser::{ItemEffect, WikiItem};

pub struct ItemParser {
    pub data: BTreeMap<String, WikiItem>,
}

impl Parser<WikiItem, Item> for ItemParser {
    const TAG: Tag = Tag::Items;
    const FN: fn(&str) -> Option<fn(WikiItem) -> Box<dyn GeneratorExt<Item>>> = item_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiItem> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> bool {
        let data = &self.data[id];

        match data.effects.act.as_ref().or(data.effects.pass.as_ref()) {
            Some(ie) => {
                let description = &ie.effect.inner.description;

                match likely_damages(description) {
                    true => infer_damage_type(result, description),
                    false => return false,
                }
            }
            None => return false,
        }

        let mut new_method = |field: &Option<ItemEffect>, tag| {
            if let Some(ie) = &field
                && ie.effect.formula.is_some()
            {
                result.push_str(&format!(".min({tag})?"));
            }
        };

        new_method(&data.effects.act, "Active");
        new_method(&data.effects.pass, "Passive");

        true
    }

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/items/full.json")?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(flatten)]
    pub data: WikiItem,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Active,
    Passive,
}

impl From<WikiItem> for Item {
    fn from(data: WikiItem) -> Self {
        Self {
            data,
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
        }
    }
}

impl Item {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, source: Source) -> MayFail<String> {
        match &self[source] {
            Some(ie) if let Some(ref formula) = ie.effect.formula => Ok(formula.parenthesize()),
            _ => Err(format!(
                "[{name}] No formula for its {source:?}",
                name = self.data.name
            )
            .into()),
        }
    }

    pub fn assign(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        damage: impl AsRef<str>,
    ) -> &mut Self {
        self[attack_type][var] = damage.as_ref().to_string();
        self
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: Source,
    ) -> MayFail<&mut Self> {
        let formula = self.formula(index)?;
        Ok(self.assign(attack_type, var, formula))
    }

    pub fn asgn_min(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn asgn_max(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Max, &damage)
            .assign(AttackType::Ranged, DamageIndex::Max, damage)
    }

    pub fn min(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Min, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Min, index)
    }

    pub fn max(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Max, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Max, index)
    }

    pub fn end(&self) -> MayFail {
        Ok(())
    }
}

impl Index<AttackType> for Item {
    type Output = DamageRange;

    fn index(&self, index: AttackType) -> &Self::Output {
        match index {
            AttackType::Melee => &self.melee,
            AttackType::Ranged => &self.ranged,
        }
    }
}

impl IndexMut<AttackType> for Item {
    fn index_mut(&mut self, index: AttackType) -> &mut Self::Output {
        match index {
            AttackType::Melee => &mut self.melee,
            AttackType::Ranged => &mut self.ranged,
        }
    }
}

impl Index<Source> for Item {
    type Output = Option<ItemEffect>;

    fn index(&self, index: Source) -> &Self::Output {
        match index {
            Source::Active => &self.data.effects.act,
            Source::Passive => &self.data.effects.pass,
        }
    }
}

impl IndexMut<Source> for Item {
    fn index_mut(&mut self, index: Source) -> &mut Self::Output {
        match index {
            Source::Active => &mut self.data.effects.act,
            Source::Passive => &mut self.data.effects.pass,
        }
    }
}
