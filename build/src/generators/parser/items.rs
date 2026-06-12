use crate::{
    DynError, JsonRead, MayFail,
    generators::{
        GeneratorExt,
        impls::items::item_gen_fn,
        parser::{
            DamageRange, Parser, ZERO, infer_damage_type, is_zero, likely_damages,
            model::{RiotCdn, RiotCdnItem, RiotCdnItemGold},
        },
        utils::{RegExtractor, SaveTo, Tag},
    },
    model::{
        Effect,
        items::{ItemEffect, WikiItem},
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Write,
    ops::{Index, IndexMut},
};
use tutorlolv2_types::{AttackType, Attrs, DamageIndex, DamageType};

pub struct ItemParser {
    pub data: BTreeMap<String, WikiItem>,
}

impl Parser<WikiItem, Item> for ItemParser {
    const TAG: Tag = Tag::Items;
    const FN: fn(&str) -> Option<fn(WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>>> =
        item_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiItem> {
        &self.data
    }

    fn from_map(data: BTreeMap<String, WikiItem>) -> Self {
        Self { data }
    }

    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool> {
        let data = &self.data[id];

        match data.effects.act.as_ref().or(data.effects.pass.as_ref()) {
            Some(ie) => {
                let description = &ie.effect.inner.description;

                match likely_damages(description) {
                    true => infer_damage_type(result, description),
                    false => return Ok(false),
                }
            }
            None => return Ok(false),
        }

        let mut new_method = |field: &Option<ItemEffect>, tag| -> MayFail {
            if let Some(ie) = &field
                && ie.effect.formula.is_some()
            {
                write!(result, ".min({tag})?")?;
            }

            Ok(())
        };

        new_method(&data.effects.act, "Active")?;
        new_method(&data.effects.pass, "Passive")?;

        Ok(true)
    }

    fn new() -> MayFail<Self> {
        let mut data = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?;
        let riot_items = RiotCdn::<u32, RiotCdnItem>::from_file(SaveTo::RiotItems.path())?;

        for (id, cdn_item) in riot_items.data {
            let stats = cdn_item.pretiffy_stats().unwrap_or_default();

            let RiotCdnItem {
                name,
                gold: RiotCdnItemGold {
                    total, purchasable, ..
                },
                maps,
                from: recipe,
                ..
            } = cdn_item;

            let key = tutorlolv2_fmt::pascal_case(&name);

            match data.get_mut(&key) {
                Some(item) => item.purchasable = purchasable,
                None => {
                    if !name.is_empty() && !name.starts_with("<") {
                        let value = WikiItem {
                            id,
                            name,
                            item_id: key.clone(),
                            tier: Some(match total {
                                0..750 => 1,
                                750..2000 => 2,
                                _ => 3,
                            }),
                            modes: maps.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
                            stats: stats
                                .into_iter()
                                .map(|(k, v)| ((k as u8).to_string(), v as _))
                                .collect(),
                            effects: Default::default(),
                            recipe,
                            buy: Some(total as _),
                            purchasable,
                            custom: true,
                        };

                        data.insert(key, value);
                    }
                }
            }
        }

        Ok(Self { data })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub data: WikiItem,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
    pub attributes: Attrs,
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Active,
    Passive,
}

impl TryFrom<WikiItem> for Item {
    type Error = DynError;

    fn try_from(data: WikiItem) -> Result<Self, Self::Error> {
        Ok(Self {
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
            attributes: Attrs::Undefined,
            data,
        })
    }
}

impl Item {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn effect(&self, source: Source) -> MayFail<&Effect> {
        match &self[source] {
            Some(ie) => Ok(&ie.effect),
            _ => Err(format!("No source for its {source:?}").into()),
        }
    }

    pub fn base(&self, source: Source) -> MayFail<&Vec<f64>> {
        self.effect(source)?
            .base
            .as_ref()
            .ok_or(format!("No base damage for its {source:?}").into())
    }

    pub fn formula(&self, source: Source) -> MayFail<String> {
        match self.effect(source).ok() {
            Some(effect) if let Some(ref formula) = effect.formula => Ok(formula.parenthesize()),
            _ => {
                println!(
                    "[{name}] No formula for its {source:?}",
                    name = self.data.name
                );
                Ok(ZERO.into())
            }
        }
    }

    pub fn assign(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        damage: &impl ToString,
    ) -> &mut Self {
        self[attack_type][var] = damage.to_string();
        self
    }

    pub fn scaling(
        &self,
        source: Source,
        indexes: impl IntoIterator<Item = usize>,
    ) -> MayFail<String> {
        Ok("0".into())
        // let filter = indexes.into_iter().collect::<Vec<_>>();
        // Ok(self
        //     .effect(source)?
        //     .scalings
        //     .iter()
        //     .enumerate()
        //     .filter(|(i, _)| filter.contains(i))
        //     .filter_map(|(_, scaling)| scaling.render(CtxVar::Level).ok())
        //     .collect::<Vec<_>>()
        //     .join(" + "))
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: Source,
    ) -> MayFail<&mut Self> {
        let formula = self.formula(index)?;
        Ok(self.assign(attack_type, var, &formula))
    }

    pub fn asgn_min(&mut self, damage: impl ToString) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, &damage)
    }

    pub fn asgn_max(&mut self, damage: impl ToString) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Max, &damage)
            .assign(AttackType::Ranged, DamageIndex::Max, &damage)
    }

    pub fn min(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Min, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Min, index)
    }

    pub fn max(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Max, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Max, index)
    }

    pub fn attr(&mut self, attrs: Attrs) -> &mut Self {
        self.attributes = attrs;
        self
    }

    pub fn end(&mut self) -> MayFail {
        if matches!(self.damage_type, DamageType::Unspecified) {
            println!(
                "[warn] {item_id} has unknown damage type",
                item_id = self.data.item_id
            )
            // return Err("Unknown damage type for this item".into());
        }

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
