use {
    crate::{
        DynError, MayFail,
        generators::{
            impls::items::item_gen_fn,
            parser::{
                DamageRange, GeneratorFnSig, Parser, ZERO, infer_damage_type, likely_damages,
                model::{RiotCdn, RiotCdnItem, RiotCdnItemGold},
            },
            utils::RegExtractor,
        },
    },
    heck::ToPascalCase,
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt::Write,
        ops::{Index, IndexMut},
    },
    tutorlolv2_types::{AttackType, Attrs, CtxVar, DamageIndex, DamageType, Tag},
    tutorlolv2_wiki::{
        JsonRead,
        items::item_parser::{ItemEffect, WikiItem},
        parser::Effect,
    },
};

pub use recipes::ItemRecipe;

mod recipes;

pub struct ItemParser {
    pub data: BTreeMap<String, WikiItem>,
    pub recipes: BTreeMap<String, ItemRecipe>,
}

impl Parser<WikiItem, Item> for ItemParser {
    const TAG: Tag = Tag::Items;
    const FN: fn(&str) -> Option<GeneratorFnSig<WikiItem, Item>> = item_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiItem> {
        &self.data
    }

    fn prepare(&self, id: &str, item: &mut Item) {
        if let Some(recipe) = self.recipes.get(id) {
            item.builds_from.clone_from(&recipe.builds_from);
            item.builds_into.clone_from(&recipe.builds_into);
        }
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

    fn from_root(root: &std::path::Path) -> MayFail<Self> {
        let mut data =
            BTreeMap::<String, WikiItem>::from_file(root.join("cache/wiki/items/full.json"))?;

        let riot_items =
            RiotCdn::<u32, RiotCdnItem>::from_file(root.join("cache/riot/items.json"))?;

        let riot_recipes_by_id: BTreeMap<_, _> = riot_items
            .data
            .iter()
            .map(|(&id, item)| {
                (
                    id,
                    recipes::RiotRecipe {
                        from: item.from.clone(),
                        into: item.into.clone(),
                    },
                )
            })
            .collect();

        let mut redirects = BTreeMap::new();
        let mut selected_riot_ids = BTreeMap::new();

        // `RiotCdn::data` is a `HashMap`, so entries are grouped by name in riot id order
        // to keep the generated code identical between builds.
        let mut riot_items = riot_items.data.into_iter().collect::<Vec<_>>();

        riot_items.sort_by_key(|(id, _)| *id);

        let mut groups = BTreeMap::<String, Vec<(u32, RiotCdnItem)>>::new();

        for (id, cdn_item) in riot_items {
            let key = cdn_item.name.to_pascal_case().to_lowercase();
            groups.entry(key).or_default().push((id, cdn_item));
        }

        let wiki_keys = data
            .keys()
            .map(|key| (key.to_lowercase(), key.clone()))
            .collect::<BTreeMap<_, _>>();

        for (lowercase_key, entries) in groups {
            if let Some(key) = wiki_keys.get(&lowercase_key) {
                let item = data.get_mut(key).unwrap();

                // The wiki already has this item. Prefer the riot entry with the same id
                // over Arena or other mode-specific copies that share its name.
                let (riot_id, cdn_item) = entries
                    .iter()
                    .find(|(id, _)| *id == item.id)
                    .unwrap_or(&entries[0]);

                item.purchasable = cdn_item.gold.purchasable;
                selected_riot_ids.insert(key.clone(), *riot_id);

                for (id, _) in &entries {
                    redirects.insert(*id, key.clone());
                }

                continue;
            }

            let suffix_id = entries.len() > 1;

            for (id, cdn_item) in entries {
                let stats = cdn_item.pretiffy_stats().unwrap_or_default();

                let RiotCdnItem {
                    name,
                    gold:
                        RiotCdnItemGold {
                            total, purchasable, ..
                        },
                    maps,
                    from: recipe,
                    ..
                } = cdn_item;

                if name.is_empty() || name.starts_with("<") {
                    continue;
                }

                // Different items sharing a name keep one variant each, such as
                // `JungleQuestReward1209`, instead of collapsing into one of them.
                let key = match suffix_id {
                    true => format!("{}{id}", name.to_pascal_case()),
                    false => name.to_pascal_case(),
                };

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

                if data.insert(key.clone(), value).is_some() {
                    return Err(format!("[ItemParser::new] Duplicated item key {key}").into());
                }

                redirects.insert(id, key);
            }
        }

        // Use the exact numeric match first, then the same-name Riot record
        // selected above. Wiki fallback applies only when neither is available.
        let riot_recipes = data
            .iter()
            .filter_map(|(key, item)| {
                let recipe = riot_recipes_by_id.get(&item.id).or_else(|| {
                    selected_riot_ids
                        .get(key)
                        .and_then(|id| riot_recipes_by_id.get(id))
                })?;
                Some((key.clone(), recipe.clone()))
            })
            .collect();

        let recipes = recipes::resolve(&data, &riot_recipes, &redirects);

        Ok(Self { data, recipes })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub data: WikiItem,
    #[serde(default)]
    pub builds_from: Vec<String>,
    #[serde(default)]
    pub builds_into: Vec<String>,
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
            builds_from: Vec::new(),
            builds_into: Vec::new(),
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
            attributes: Attrs::Undefined,
            data,
        })
    }
}

#[allow(dead_code)]
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
        let filter = indexes.into_iter().collect::<Vec<_>>();
        Ok(self
            .effect(source)?
            .scalings
            .iter()
            .enumerate()
            .filter(|(i, _)| filter.contains(i))
            .filter_map(|(_, scaling)| scaling.render(CtxVar::Level).ok())
            .collect::<Vec<_>>()
            .join(" + "))
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

    pub fn set_min(&mut self, damage: impl ToString) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, &damage)
    }

    pub fn set_max(&mut self, damage: impl ToString) -> &mut Self {
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
