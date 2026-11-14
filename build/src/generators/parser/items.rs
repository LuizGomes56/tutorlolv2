use {
    crate::{
        DynError, MayFail,
        generators::{
            constants::{Constants, WithConstants},
            items::item_gen_fn,
            parser::{
                DamageRange, GeneratorFnSig, Parser, WithMapChanges, infer_damage_type,
                likely_damages,
                model::{RiotCdn, RiotCdnItem, RiotCdnItemGold},
            },
            utils::RegExtractor,
        },
    },
    heck::ToPascalCase,
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt::Write,
        ops::{Index, IndexMut},
    },
    tutorlolv2_types::{AttackType, Attrs, CtxVar, DamageIndex, DamageType, Key, MapMode, Tag},
    tutorlolv2_wiki::{
        JsonRead,
        items::item_parser::{ItemEffect, WikiItem},
        map_changes::{ChangeTarget, change::MapChange},
        wikitext::{
            Formula, IntoFormula, Lookup, MapValue, Nodes, PageContext, Sample, SourceDocument,
            SourceForm, SourceRole, SourceVariant, Variables,
        },
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

        let raw = tutorlolv2_wiki::items::parse_module(&std::fs::read_to_string(
            root.join("cache/wiki/items/data.lua"),
        )?)?;

        let mut data = raw
            .into_values()
            .map(|raw| {
                let item = raw.into_native();

                (item.item_id.clone(), item)
            })
            .collect::<BTreeMap<_, _>>();

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
    Named(&'static str),
    Event(&'static str, ItemEvent),
}

#[derive(Copy, Clone, Debug)]
pub enum ItemEvent {
    PrimaryTarget,
    AttackTarget,
    NonChampionDeath,
    InitialHit,
    Spellblade,
    FogDamage,
    CriticalBleed,
    BaseProc,
    MaximumProc,
}

impl WithMapChanges for Item {
    fn is_map_target(&self, target: &ChangeTarget) -> bool {
        matches!(target, ChangeTarget::Item { item_id } if *item_id == self.data.item_id)
    }
}

impl Item {
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
            Self::Named(name) | Self::Event(name, _) => name,
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

    fn checked_effect(&self, source: Source) -> MayFail<&ItemEffect> {
        if let Source::Named(name) | Source::Event(name, _) = source {
            let matches = [
                Source::Passive,
                Source::Passive2,
                Source::Passive3,
                Source::Passive4,
                Source::Active,
                Source::Consume,
            ]
            .into_iter()
            .filter_map(|slot| self[slot].as_ref())
            .filter(|effect| effect.name.as_deref() == Some(name))
            .collect::<Vec<_>>();

            return match matches.as_slice() {
                [effect] => Ok(effect),
                _ => Err(format!(
                    "{} expected exactly one effect {name:?}, found {}",
                    self.data.item_id,
                    matches.len()
                )
                .into()),
            };
        }

        self[source]
            .as_ref()
            .ok_or_else(|| format!("{} missing {source:?}", self.data.item_id).into())
    }

    /// Wikitext description of an effect, as written in the wiki
    pub fn description(&self, source: Source) -> MayFail<&str> {
        self.checked_effect(source)?
            .raw_description
            .as_deref()
            .ok_or_else(|| {
                let item_id = &self.data.item_id;
                format!("{item_id} has no description for {source:?}").into()
            })
    }

    pub fn native_document(
        &self,
        source: Source,
        attack_type: AttackType,
        alternate: bool,
    ) -> MayFail<SourceDocument> {
        let lookup = Lookup::new("return {}", "return {}")?;
        let context = PageContext {
            lookup: &lookup,
            four_rank_ultimate: false,
            ability_keys: Default::default(),
            crit_damage: 200.0,
        };

        let variant = SourceVariant {
            ranged: matches!(attack_type, AttackType::Ranged),
            alternate,
        };

        let selected_effect = self.checked_effect(source)?;
        let mut variables = Variables::default();

        for slot in [
            Source::Passive,
            Source::Passive2,
            Source::Passive3,
            Source::Passive4,
            Source::Active,
            Source::Consume,
        ] {
            if let Some(effect) = &self[slot] {
                let nodes = Nodes::parse(
                    effect
                        .raw_description
                        .as_deref()
                        .ok_or("Missing raw item description")?,
                )
                .for_variant(variant)?;

                let nodes = variables.expand(&context.lookup.expand(&nodes));

                if std::ptr::eq(effect, selected_effect) {
                    return SourceDocument::from_nodes(
                        effect.name.as_deref().unwrap_or(slot.slot()),
                        &nodes,
                        &context,
                        variant,
                    )
                    .map_err(Into::into);
                }
            }
        }

        Err("Missing native effect".into())
    }

    pub fn native_formula(
        &self,
        source: Source,
        attack_type: AttackType,
        alternate: bool,
        form: SourceForm,
    ) -> MayFail<Formula> {
        let document = self.native_document(source, attack_type, alternate)?;

        let value = document.select_where(SourceRole::Damage, form, |value| {
            let context = value.context.to_ascii_lowercase();

            if context.contains("against monsters") {
                return false;
            }

            match source {
                Source::Event(_, ItemEvent::PrimaryTarget) => value
                    .following
                    .trim_start()
                    .starts_with("bonus physical damage to the"),
                Source::Event(_, ItemEvent::AttackTarget) => value
                    .following
                    .trim_start()
                    .starts_with("bonus physical damage to the attack target"),
                Source::Event(_, ItemEvent::NonChampionDeath) => {
                    !context.contains("scoring a takedown")
                }
                Source::Event(_, ItemEvent::Spellblade) => {
                    !context.ends_with("for a minimum of ") && !context.contains("slow")
                }
                Source::Event(_, ItemEvent::InitialHit) if form == SourceForm::Base => {
                    !context.contains("deal an additional")
                }
                Source::Event(_, ItemEvent::CriticalBleed) => {
                    !value.following.starts_with("% of the")
                }
                Source::Event(_, ItemEvent::BaseProc) => context.ends_with("to deal "),
                Source::Event(_, ItemEvent::MaximumProc) => context.ends_with("for up to "),
                Source::Event(_, ItemEvent::FogDamage) => {
                    !value.raw.contains("ultimate's damage instance")
                }
                _ => true,
            }
        })?;

        let mut options = super::native_maps::MapOptions::default();

        if self.data.item_id == "KrakenSlayer" && document.name == "Bring It Down" {
            options.amplification_subject = Some("Bring It Down bonus damage");
        }

        options.ignore_secondary = self.data.item_id == "LudensEcho" && document.name == "Echo";

        if matches!(
            document.name.as_str(),
            "Baleful Blaze" | "Hatefog" | "Torment"
        ) {
            let (interval, duration) = self.damage_timing(source, attack_type)?;
            options.timing = Some((
                if alternate || form == SourceForm::Total {
                    duration
                } else {
                    interval
                },
                interval,
            ));
        }

        if self.data.item_id == "LiandrysTorment" && document.name == "Torment" {
            options.alias = Some("Suffering");
        }

        if self.data.item_id == "Heartsteel" && document.name == "Colossal Consumption" {
            options.replace_stat = Some((CtxVar::MaxHealth, CtxVar::BonusHealth));
        }

        self.mapped_formula_with(
            &document.name,
            attack_type,
            value.formula()?.clone(),
            options,
        )
    }

    pub fn echo_total(&self) -> MayFail<Formula> {
        use crate::formula as f;

        let source = Source::Event("Echo", ItemEvent::InitialHit);
        let primary = self.formula(source)?;
        let document = self.native_document(source, AttackType::Melee, true)?;
        let [stacks] = document
            .source
            .capture(regex!(r"Gain ([\d.]+) ''Echo'' stacks"))?;

        let [percent] = document
            .source
            .capture(regex!(r"deal an additional ([\d.]+)% damage"))?;

        let remaining = stacks - 1.0;
        let factor = 1.0 + remaining * percent / 100.0;
        let mut total = f![primary * factor];

        for (mode, text) in self.map_texts(regex!(r"Echo secondary damage changed to"))? {
            let [percent] = text.capture(regex!(r"changed to ([\d.]+)% of primary damage"))?;

            let primary = self.formula(source)?.on(mode);
            let factor = 1.0 + remaining * percent / 100.0;

            total = total.with_map(mode, f![primary * factor]);
        }

        Ok(total)
    }

    /// Read the source's primary presentation so totals and ticks share one clock.
    fn damage_timing(&self, source: Source, attack: AttackType) -> MayFail<(f64, f64)> {
        let document = self.native_document(source, attack, false)?;
        let interval = regex!(r"every (?:\{\{fd\|)?([\d.]+)(?:\}\})? seconds")
            .captures_iter(&document.source)
            .map(|capture| capture[1].parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;

        let durations = regex!(r"(?:over|for) ([\d.]+) seconds")
            .captures_iter(&document.source)
            .map(|capture| capture[1].parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;

        match (interval.as_slice(), durations.as_slice()) {
            ([interval], [duration]) if *interval > 0.0 && *duration >= *interval => {
                Ok((*interval, *duration))
            }
            _ => Err(format!(
                "Expected one damage interval and duration for {}: {:?}, {:?}",
                document.name, interval, durations
            )
            .into()),
        }
    }

    pub fn formula(&self, source: Source) -> MayFail<Formula> {
        self.formula_for(source, AttackType::Melee)
    }

    pub fn formula_for(&self, source: Source, attack_type: AttackType) -> MayFail<Formula> {
        self.native_formula(source, attack_type, false, SourceForm::Base)
    }
    /// Resolve a supported maximum presentation using the source's own caps.
    pub fn maximum_damage(&mut self, source: Source) -> MayFail<&mut Self> {
        for attack_type in [AttackType::Melee, AttackType::Ranged] {
            let formula = Self::maximum_axes(self.formula_for(source, attack_type)?);
            self.assign(attack_type, DamageIndex::Min, formula);
        }

        Ok(self)
    }
    fn maximum_axes(formula: Formula) -> Formula {
        match formula {
            Formula::Table {
                var: CtxVar::Stacks,
                values,
                ..
            }
            | Formula::NumericTable { values, .. } => {
                Formula::Const(*values.last().expect("nonempty source progression"))
            }
            Formula::Sum(values) => {
                Formula::Sum(values.into_iter().map(Self::maximum_axes).collect())
            }
            Formula::Product(values) => {
                Formula::Product(values.into_iter().map(Self::maximum_axes).collect())
            }
            Formula::Div(a, b) => Formula::Div(
                Box::new(Self::maximum_axes(*a)),
                Box::new(Self::maximum_axes(*b)),
            ),
            Formula::ByMap { default, maps } => Formula::ByMap {
                default: Box::new(Self::maximum_axes(*default)),
                maps: maps
                    .into_iter()
                    .map(|(mode, formula)| (mode, Self::maximum_axes(formula)))
                    .collect(),
            },
            other => other,
        }
    }

    pub fn maximum_amplification(&self, source: Source) -> MayFail<f64> {
        let document = self.native_document(source, AttackType::Melee, false)?;
        let values = document
            .values
            .iter()
            .filter(|value| value.context.ends_with("increased by "))
            .collect::<Vec<_>>();

        let [value] = values.as_slice() else {
            return Err("Expected one increased damage progression".into());
        };

        let [progression] = value.progressions.as_slice() else {
            return Err("Expected one amplification progression".into());
        };

        progression
            .values
            .last()
            .map(|value| value / 100.0)
            .ok_or_else(|| "Empty amplification progression".into())
    }

    /// Percent amplification is a distinct selection from the proc's flat base.
    pub fn maximum_amplification_formula(&self, source: Source) -> MayFail<Formula> {
        let mut result = Formula::Const(self.maximum_amplification(source)?);
        let document = self.native_document(source, AttackType::Melee, false)?;
        let subject = format!("{} bonus damage", document.name);
        let lookup = Lookup::new("return {}", "return {}")?;

        for changes in super::map_changes() {
            for entry in changes
                .entries
                .values()
                .filter(|entry| self.is_map_target(&entry.target))
            {
                for change in entry.changes.iter().flat_map(|change| {
                    std::iter::once(change.clone()).chain(
                        change
                            .notes
                            .iter()
                            .map(|text| MapChange::parse(text, false)),
                    )
                }) {
                    if change.subject() != Some(subject.as_str()) {
                        continue;
                    }

                    // This annotation selects the bounded percentage progression
                    // by target missing-health percentage, not damage points.
                    if !change.text.contains("key=%")
                        || !change.text.contains("missing")
                        || !change.text.contains("health")
                    {
                        return Err(format!(
                            "Expected percent missing-health amplification: {}",
                            change.text
                        )
                        .into());
                    }

                    let Some(MapValue::Base(Formula::NumericTable { keys, values, .. })) =
                        MapValue::new(&change, Key::P, &lookup)
                    else {
                        return Err(format!(
                            "Expected bounded amplification progression: {}",
                            change.text
                        )
                        .into());
                    };

                    if keys.last() != Some(&100.0) || values.is_empty() {
                        return Err("Expected amplification cap at 100% missing health".into());
                    }

                    let maximum = values.last().unwrap() / 100.0;

                    result = result.with_map(
                        super::single_mode(&changes.maps, &subject)?,
                        Formula::Const(maximum),
                    );
                }
            }
        }

        Ok(result)
    }

    pub fn critical_instance_ratio(&self, source: Source) -> MayFail<f64> {
        let document = self.native_document(source, AttackType::Melee, true)?;
        let [ratio] = document.source.capture(regex!(r"plus ([\d.]+)% of the"))?;

        Ok(ratio / 100.0)
    }

    pub fn critical_bonus(&self, source: Source) -> MayFail<Formula> {
        let document = self.native_document(source, AttackType::Melee, false)?;
        let value = document
            .values
            .iter()
            .filter(|value| value.context.ends_with("critical strike for"))
            .collect::<Vec<_>>();

        match value.as_slice() {
            [value] => Ok(value.formula()?.clone()),
            _ => Err("Expected one critical strike bonus".into()),
        }
    }

    pub fn alternate_formula(&self, source: Source) -> MayFail<Formula> {
        self.native_formula(source, AttackType::Melee, true, SourceForm::Base)
    }

    pub fn total_formula(&self, source: Source) -> MayFail<Formula> {
        self.native_formula(source, AttackType::Melee, false, SourceForm::Total)
            .map(Self::maximum_axes)
    }

    pub fn map_stat_ratios(&self, source: Source) -> MayFail<Vec<(MapMode, f64)>> {
        let document = self.native_document(source, AttackType::Melee, false)?;
        let formula = document
            .select(SourceRole::Stat, SourceForm::Base)?
            .formula()?
            .clone();

        let formula = self.mapped_formula(&document.name, AttackType::Melee, formula)?;

        formula
            .modes()
            .into_iter()
            .map(|mode| Ok((mode, Self::ratio_value(&formula.on(mode))?)))
            .collect()
    }

    fn ratio_value(formula: &Formula) -> MayFail<f64> {
        let vars = [CtxVar::BonusMana, CtxVar::MaxMana, CtxVar::BonusHealth]
            .into_iter()
            .map(|var| (var, 1.0));

        formula
            .eval(&Sample::new(vars))
            .ok_or_else(|| "Expected stat ratio".into())
    }

    pub fn stat_ratio(&self, source: Source) -> MayFail<f64> {
        let document = self.native_document(source, AttackType::Melee, false)?;
        let formula = document
            .select(SourceRole::Stat, SourceForm::Base)?
            .formula()?;

        Self::ratio_value(formula)
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
            Source::Named(name) | Source::Event(name, _) => [
                &self.data.effects.pass,
                &self.data.effects.pass2,
                &self.data.effects.pass3,
                &self.data.effects.pass4,
                &self.data.effects.act,
                &self.data.effects.consume,
            ]
            .into_iter()
            .find(|effect| {
                effect
                    .as_ref()
                    .is_some_and(|effect| effect.name.as_deref() == Some(name))
            })
            .unwrap_or(&None),
        }
    }
}

impl WithConstants for Item {
    fn constants_mut(&mut self) -> &mut Constants {
        &mut self.constants
    }
}
