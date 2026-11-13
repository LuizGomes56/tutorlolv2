use {
    crate::{
        DynError, MayFail,
        generators::{
            constants::{Constants, WithConstants},
            impls::items::item_gen_fn,
            parser::{
                DamageRange, GeneratorFnSig, Parser, WithMapChanges, infer_damage_type,
                likely_damages, map_changes,
                model::{RiotCdn, RiotCdnItem, RiotCdnItemGold},
                single_mode,
            },
        },
    },
    heck::ToPascalCase,
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt::Write,
        ops::{Index, IndexMut},
    },
    tutorlolv2_types::{AttackType, Attrs, CtxVar, DamageIndex, DamageType, MapMode, Tag},
    tutorlolv2_wiki::{
        JsonRead,
        items::item_parser::{ItemEffect, WikiItem},
        map_changes::{ChangeTarget, overrides::EffectOverrides},
        parser::Effect,
        wikitext::{Formula, IntoFormula},
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
        super::load_map_changes(root)?;

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
    #[serde(default, skip_serializing_if = "Constants::is_empty")]
    pub constants: Constants,
    /// Formulas on each mode that changes them, for melee minimum and maximum,
    /// then ranged. Serialized, since the build reads the generator's output
    #[serde(default, skip_serializing_if = "super::no_map_damages")]
    pub map_damages: [Vec<(MapMode, String)>; 4],
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Active,
    Passive,
    Passive2,
    Passive3,
    Passive4,
    Consume,
}

impl WithMapChanges for Item {
    fn is_map_target(&self, target: &ChangeTarget) -> bool {
        matches!(target, ChangeTarget::Item { item_id } if *item_id == self.data.item_id)
    }
}

impl Item {
    /// Effect of `source` for `attack_type` on each mode that changes it, by the
    /// same rules as damages, for generators that build their damage from it
    pub fn map_effects_for(
        &self,
        source: Source,
        attack_type: AttackType,
    ) -> MayFail<Vec<(MapMode, Effect)>> {
        let effect = self.effect_for(source, attack_type)?.clone();
        let name = self[source]
            .as_ref()
            .and_then(|ie| ie.name.clone())
            .unwrap_or_else(|| source.slot().into());

        let candidates = BTreeMap::from([(source.slot().to_string(), (name, effect))]);
        let target = ChangeTarget::Item {
            item_id: self.data.item_id.clone(),
        };

        let ranged = matches!(attack_type, AttackType::Ranged);
        let mut result = Vec::new();

        for changes in map_changes() {
            let overrides = EffectOverrides::new(&target, changes, &candidates, ranged);

            if let Some(effect) = overrides.effects.get(source.slot()) {
                let mode = single_mode(&changes.maps, &self.data.item_id)?;
                result.push((mode, effect.clone()));
            }
        }

        Ok(result)
    }

    /// Formula of the effect of `source` on each mode that changes it, for
    /// constants captured from the formula
    pub fn map_formulas(&self, source: Source) -> MayFail<Vec<(MapMode, Formula)>> {
        self.map_effects_for(source, AttackType::Melee)?
            .into_iter()
            .map(|(mode, effect)| {
                Ok((
                    mode,
                    effect
                        .to_formula(CtxVar::Level)?
                        .ok_or("Item effect has no formula")?,
                ))
            })
            .collect()
    }

    /// Sets the damage `var` of `attack_type` on `mode`, for generators that build
    /// it themselves. The damages a generator sets aren't changed automatically
    pub fn map_assign(
        &mut self,
        mode: MapMode,
        attack_type: AttackType,
        var: DamageIndex,
        damage: impl IntoFormula,
    ) -> &mut Self {
        let index = match (attack_type, var) {
            (AttackType::Melee, DamageIndex::Min) => 0,
            (AttackType::Melee, DamageIndex::Max) => 1,
            (AttackType::Ranged, DamageIndex::Min) => 2,
            (AttackType::Ranged, DamageIndex::Max) => 3,
        };

        self.map_damages[index].push((mode, damage.to_formula().rust()));
        self
    }
}

impl Source {
    /// Key of the slot in `Module:ItemData`, as `pass2`
    pub fn slot(&self) -> &'static str {
        match self {
            Self::Active => "act",
            Self::Passive => "pass",
            Self::Passive2 => "pass2",
            Self::Passive3 => "pass3",
            Self::Passive4 => "pass4",
            Self::Consume => "consume",
        }
    }
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
            constants: Default::default(),
            map_damages: Default::default(),
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

    /// Wikitext description of an effect, as written in the wiki
    pub fn description(&self, source: Source) -> MayFail<&str> {
        self[source]
            .as_ref()
            .and_then(|effect| effect.raw_description.as_deref())
            .ok_or_else(|| {
                let item_id = &self.data.item_id;
                format!("{item_id} has no description for {source:?}").into()
            })
    }

    pub fn effect(&self, source: Source) -> MayFail<&Effect> {
        match &self[source] {
            Some(ie) => Ok(&ie.effect),
            _ => Err(format!("No source for its {source:?}").into()),
        }
    }

    /// Effect for the given attack type. Ranged champions use the ranged effect
    /// when the wiki gives them different values with `{{rd|melee|ranged}}`
    pub fn effect_for(&self, source: Source, attack_type: AttackType) -> MayFail<&Effect> {
        match (&self[source], attack_type) {
            (
                Some(ItemEffect {
                    ranged: Some(effect),
                    ..
                }),
                AttackType::Ranged,
            ) => Ok(effect),
            _ => self.effect(source),
        }
    }

    /// Effect for the alternate text of `{{ft|text|alternate}}`, such as the
    /// total damage of a burn that the main text describes per tick
    pub fn flip(&self, source: Source) -> MayFail<&Effect> {
        self[source]
            .as_ref()
            .and_then(|ie| ie.flip.as_ref())
            .ok_or(format!("No alternate text for its {source:?}").into())
    }

    /// Effect described after "for a total of"
    pub fn total(&self, source: Source) -> MayFail<&Effect> {
        self[source]
            .as_ref()
            .and_then(|ie| ie.total.as_ref())
            .ok_or(format!("No total for its {source:?}").into())
    }

    /// Formula of any effect of this item, such as [`Self::flip`] or [`Self::total`]
    pub fn render(&self, effect: &Effect) -> MayFail<Formula> {
        effect
            .to_formula(CtxVar::Level)?
            .ok_or_else(|| "Item effect has no formula".into())
    }

    /// Formula with the base value and only the given scalings of an effect, for
    /// descriptions that also mention healing, shields or other bonuses
    pub fn render_with(
        &self,
        effect: &Effect,
        scalings: impl IntoIterator<Item = usize>,
    ) -> MayFail<Formula> {
        let indexes = scalings.into_iter().collect::<Vec<_>>();
        let mut formula = self.render_selected(effect, &indexes)?;

        for source in [
            Source::Active,
            Source::Passive,
            Source::Passive2,
            Source::Passive3,
            Source::Passive4,
            Source::Consume,
        ] {
            for attack_type in [AttackType::Melee, AttackType::Ranged] {
                if self
                    .effect_for(source, attack_type)
                    .is_ok_and(|candidate| std::ptr::eq(candidate, effect))
                {
                    for (mode, effect) in self.map_effects_for(source, attack_type)? {
                        formula = formula.with_map(mode, self.render_selected(&effect, &indexes)?);
                    }
                    return Ok(formula);
                }
            }
        }
        Ok(formula)
    }

    fn render_selected(&self, effect: &Effect, indexes: &[usize]) -> MayFail<Formula> {
        let mut effect = effect.clone();

        for &index in indexes {
            if index >= effect.scalings.len() {
                return Err(format!(
                    "[{}] Scaling {index} out of bounds for {:?}",
                    self.data.name, effect.scalings
                )
                .into());
            }
        }

        effect.scalings = indexes
            .iter()
            .map(|&index| effect.scalings[index].clone())
            .collect();

        effect.formula = None;
        self.render(&effect)
    }

    /// Same as [`Self::render_with`] for each attack type, assigned to `var`
    pub fn assign_with(
        &mut self,
        var: DamageIndex,
        source: Source,
        scalings: impl IntoIterator<Item = usize>,
    ) -> MayFail<&mut Self> {
        let scalings = scalings.into_iter().collect::<Vec<_>>();

        for attack_type in [AttackType::Melee, AttackType::Ranged] {
            let effect = self.effect_for(source, attack_type)?;
            let mut formula = self.render_selected(effect, &scalings)?;

            for (mode, effect) in self.map_effects_for(source, attack_type)? {
                formula = formula.with_map(mode, self.render_selected(&effect, &scalings)?);
            }

            self.assign(attack_type, var, formula);
        }

        Ok(self)
    }

    pub fn min_with(
        &mut self,
        source: Source,
        scalings: impl IntoIterator<Item = usize>,
    ) -> MayFail<&mut Self> {
        self.assign_with(DamageIndex::Min, source, scalings)
    }

    pub fn max_with(
        &mut self,
        source: Source,
        scalings: impl IntoIterator<Item = usize>,
    ) -> MayFail<&mut Self> {
        self.assign_with(DamageIndex::Max, source, scalings)
    }

    pub fn base(&self, source: Source) -> MayFail<&Vec<f64>> {
        self.effect(source)?
            .base
            .as_ref()
            .ok_or(format!("No base damage for its {source:?}").into())
    }

    pub fn formula(&self, source: Source) -> MayFail<Formula> {
        self.formula_for(source, AttackType::Melee)
    }

    pub fn formula_for(&self, source: Source, attack_type: AttackType) -> MayFail<Formula> {
        let mut formula = self
            .effect_for(source, attack_type)?
            .to_formula(CtxVar::Level)?
            .unwrap_or(Formula::Const(0.0));

        for (mode, effect) in self.map_effects_for(source, attack_type)? {
            formula = formula.with_map(
                mode,
                effect
                    .to_formula(CtxVar::Level)?
                    .ok_or("Item effect has no formula")?,
            );
        }

        Ok(formula)
    }

    pub fn assign(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        damage: impl IntoFormula,
    ) -> &mut Self {
        let damage = damage.to_formula();

        for mode in damage.modes() {
            self.map_assign(mode, attack_type, var, damage.on(mode));
        }

        self[attack_type][var] = damage.on(MapMode::Default).rust();
        self
    }

    pub fn scaling(
        &self,
        source: Source,
        indexes: impl IntoIterator<Item = usize>,
    ) -> MayFail<Formula> {
        self.scaling_for(source, AttackType::Melee, indexes)
    }

    pub fn scaling_for(
        &self,
        source: Source,
        attack_type: AttackType,
        indexes: impl IntoIterator<Item = usize>,
    ) -> MayFail<Formula> {
        let filter = indexes.into_iter().collect::<Vec<_>>();
        let selected = |effect: &Effect| -> MayFail<Formula> {
            Ok(Formula::Sum(
                effect
                    .scalings
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| filter.contains(i))
                    .map(|(_, scaling)| scaling.to_formula(CtxVar::Level))
                    .collect::<MayFail<Vec<_>>>()?,
            ))
        };

        let mut formula = selected(self.effect_for(source, attack_type)?)?;

        for (mode, effect) in self.map_effects_for(source, attack_type)? {
            formula = formula.with_map(mode, selected(&effect)?);
        }

        Ok(formula)
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: Source,
    ) -> MayFail<&mut Self> {
        let formula = self.formula_for(index, attack_type)?;
        Ok(self.assign(attack_type, var, &formula))
    }

    pub fn set_min(&mut self, damage: impl IntoFormula) -> &mut Self {
        let damage = damage.to_formula();

        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn set_max(&mut self, damage: impl IntoFormula) -> &mut Self {
        let damage = damage.to_formula();

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
            Source::Passive2 => &self.data.effects.pass2,
            Source::Passive3 => &self.data.effects.pass3,
            Source::Passive4 => &self.data.effects.pass4,
            Source::Consume => &self.data.effects.consume,
        }
    }
}

impl IndexMut<Source> for Item {
    fn index_mut(&mut self, index: Source) -> &mut Self::Output {
        match index {
            Source::Active => &mut self.data.effects.act,
            Source::Passive => &mut self.data.effects.pass,
            Source::Passive2 => &mut self.data.effects.pass2,
            Source::Passive3 => &mut self.data.effects.pass3,
            Source::Passive4 => &mut self.data.effects.pass4,
            Source::Consume => &mut self.data.effects.consume,
        }
    }
}

impl WithConstants for Item {
    fn constants_mut(&mut self) -> &mut Constants {
        &mut self.constants
    }
}
