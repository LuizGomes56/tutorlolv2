//! Champion generators: they select effects of the wikitext model
//! (`cache/wiki/champions/model.json`) by label and edit their [`Formula`] trees, from
//! which both the generated code and the codec's source are written. The registry
//! covers every supported champion.
//!
//! ```ignore
//! impl Generator for Darius {
//!     fn generate(&mut self) -> MayFail {
//!         self.take_nth("Hemorrhage", 0, P(Min))?
//!             .derive(P(Max), P(Min), |p| f![5 * p])?
//!             .take("True Damage", R(Min))?
//!             .combos([&[Attack, Ability(R(Min))]])?
//!             .end()
//!     }
//! }
//! ```
//!
//! Calls that can fail return `MayFail<&mut Self>`, and the others `&mut Self`

use {
    crate::{
        MayFail,
        generators::{
            constants::{Constant, WithConstants},
            parser::champions::{Ability, Champion},
        },
    },
    lazy_regex::regex,
    std::{collections::BTreeMap, path::Path, sync::LazyLock},
    types::{AbilityId, Attrs, ComboElement, CtxVar, DamageType, Key, MapMode},
    wiki::{
        JsonRead,
        map_changes::change::MapChange,
        wikitext::{
            AbilityModel, ChampionModel, EffectModel, Formula, IntoFormula, Lookup, MapValue,
        },
    },
};

/// A damage of a champion, as it is taken from an effect and edited
#[derive(Clone, Debug)]
pub struct Damage {
    pub formula: Formula,
    /// Name of the ability
    pub name: String,
    pub damage_type: DamageType,
    /// The effect's label, unless a generator sets another one
    pub comment: String,
    pub attributes: Attrs,
}

/// Damages selected from the structured model, with the shared [`Champion`]
/// output for combos, constants, exceptions and final consistency checks.
pub struct ChampionGenerator {
    pub inner: Champion,
    pub model: ChampionModel,
    damages: BTreeMap<AbilityId, Damage>,
}

impl ChampionGenerator {
    pub fn new(inner: Champion, model: ChampionModel) -> Self {
        Self {
            inner,
            model,
            damages: BTreeMap::new(),
        }
    }

    /// Models of every champion in `path`, as `cache/wiki/champions/model.json`
    pub fn load_models(path: &Path) -> MayFail<BTreeMap<String, ChampionModel>> {
        BTreeMap::from_file(path).map_err(|e| {
            format!(
                "Can't read the champion models at {path:?}: {e}. Run `cargo run -- wiki cpm` in wiki"
            )
            .into()
        })
    }

    /// Models of every champion, read once from `cache/wiki/champions/model.json`
    pub fn models() -> MayFail<&'static BTreeMap<String, ChampionModel>> {
        static MODELS: LazyLock<Result<BTreeMap<String, ChampionModel>, String>> =
            LazyLock::new(|| {
                let path = crate::project_root().join("cache/wiki/champions/model.json");
                ChampionGenerator::load_models(&path).map_err(|e| e.to_string())
            });

        MODELS.as_ref().map_err(|e| e.clone().into())
    }

    fn champion_id(&self) -> &str {
        &self.inner.champion_id
    }

    /// Every effect of `key`, in page and tab order, then the values of the pages'
    /// descriptions, labeled by their heading as Darius's `"Hemorrhage"`
    fn effects(&self, key: Key) -> Vec<(&AbilityModel, &EffectModel)> {
        let pages = || self.model.abilities.get(&key).into_iter().flatten();

        pages()
            .flat_map(|ability| ability.effects.iter().map(move |effect| (ability, effect)))
            .chain(
                pages()
                    .flat_map(|ability| ability.extras.iter().map(move |extra| (ability, extra))),
            )
            .collect()
    }

    /// Every effect of `key` as `page · ability · label · occurrence`, for errors
    fn candidates(&self, key: Key) -> String {
        let mut seen = BTreeMap::<&str, usize>::new();

        self.effects(key)
            .into_iter()
            .map(|(ability, effect)| {
                let occurrence = seen.entry(effect.label.as_str()).or_default();
                let line = format!(
                    "  {} · {} · {} · {}",
                    ability.page, ability.name, effect.label, occurrence
                );
                *occurrence += 1;
                line
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Takes the effect labeled `label` as `id`: the only one of `id`'s key, or of the
    /// ability named `ability`, or the `nth` one of the key
    fn select(
        &mut self,
        id: AbilityId,
        label: &str,
        ability: Option<&str>,
        nth: Option<usize>,
    ) -> MayFail<&mut Self> {
        let key = id.as_key();
        let champion_id = self.champion_id().to_string();
        let error = |this: &Self, problem: String| -> MayFail<&mut Self> {
            Err(format!(
                "[{champion_id}] {id:?}: {problem}. Effects of {key:?}:\n{}",
                this.candidates(key)
            )
            .into())
        };

        if self.damages.contains_key(&id) {
            return error(self, "taken already".into());
        }

        let matches = self
            .effects(key)
            .into_iter()
            .filter(|(model, effect)| {
                effect.label == label && ability.is_none_or(|name| model.name == name)
            })
            .collect::<Vec<_>>();

        let selected = match (nth, matches.len()) {
            (None, 1) => matches.first(),
            (None, 0) => None,
            (None, n) => {
                let problem = format!("{n} effects \"{label}\", use take_nth or take_in");
                return error(self, problem);
            }
            (Some(n), _) => matches.get(n),
        };

        let Some((model, effect)) = selected else {
            return error(self, format!("no effect \"{label}\""));
        };

        if !effect.issues.is_empty() {
            println!(
                "cargo:warning=[{champion_id}] {id:?} \"{label}\" has parts the model can't read: {:?}",
                effect.issues
            );
        }

        let damage = Damage {
            formula: effect.formula.clone(),
            name: model.name.clone(),
            damage_type: model.damage_type,
            comment: effect.label.clone(),
            attributes: Attrs::Undefined,
        };

        self.damages.insert(id, damage);
        Ok(self)
    }

    /// Formula of the only effect or description value labeled `label` among the pages
    /// of `key`, without taking it, as Caitlyn R's increase by critical strike chance
    pub fn value(&self, label: &str, key: Key) -> MayFail<Formula> {
        self.find_value(label, key, None)
    }

    /// Formula of the `nth` effect or description value labeled `label` among the pages
    /// of `key`, counted as [`Self::take_nth`], without taking it, as the damage per
    /// Plasma stack of Kai'Sa's passive
    pub fn value_nth(&self, label: &str, nth: usize, key: Key) -> MayFail<Formula> {
        self.find_value(label, key, Some(nth))
    }

    /// Formula of the only value labeled `label` of `key`, or of the `nth` one
    fn find_value(&self, label: &str, key: Key, nth: Option<usize>) -> MayFail<Formula> {
        let matches = self
            .effects(key)
            .into_iter()
            .filter(|(_, effect)| effect.label == label)
            .collect::<Vec<_>>();

        let selected = match (nth, &matches[..]) {
            (None, [(_, effect)]) => Some(effect),
            (None, _) => None,
            (Some(n), _) => matches.get(n).map(|(_, effect)| effect),
        };

        match selected {
            Some(effect) => Ok(effect.formula.clone()),
            None => Err(format!(
                "[{}] {key:?}: {} values \"{label}\"{}. Effects of {key:?}:\n{}",
                self.champion_id(),
                matches.len(),
                nth.map(|n| format!(", none at {n}")).unwrap_or_default(),
                self.candidates(key)
            )
            .into()),
        }
    }

    /// Takes the only effect labeled `label` among every page of `id`'s key
    pub fn take(&mut self, label: &str, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, None, None)
    }

    /// Takes the only effect labeled `label` of the ability named `ability`, as
    /// Gnar's `"Boulder Toss"`
    pub fn take_in(&mut self, ability: &str, label: &str, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, Some(ability), None)
    }

    /// Takes the `nth` effect labeled `label` of `id`'s key, counted from 0 in page and
    /// tab order, as the cast and recast of Vex's R
    pub fn take_nth(&mut self, label: &str, nth: usize, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, None, Some(nth))
    }

    /// Adds `id` with `damage`, written by the generator: a damage no effect has, as
    /// the attack of Annie's Tibbers
    pub fn insert(&mut self, id: AbilityId, damage: Damage) -> MayFail<&mut Self> {
        if self.damages.contains_key(&id) {
            return Err(format!("[{}] {id:?}: taken already", self.champion_id()).into());
        }

        self.damages.insert(id, damage);
        Ok(self)
    }

    /// Damage `id`, which a generator must have taken before
    fn damage_mut(&mut self, id: AbilityId) -> &mut Damage {
        let champion_id = self.inner.champion_id.clone();

        self.damages
            .get_mut(&id)
            .unwrap_or_else(|| panic!("[{champion_id}] {id:?} was never taken"))
    }

    /// Applies one explicitly named map change to a selected damage. Matching the
    /// ability key prevents identical subjects on other abilities from leaking in.
    pub fn map_change(&mut self, id: AbilityId, subject: &str) -> MayFail<&mut Self> {
        let formula = self.map_changed(id.as_key(), subject, self.damage(id)?)?;
        Ok(self.set(id, formula))
    }

    /// Mode and bullet of each map change of `key` whose subject is `subject`. The
    /// notes of a change are read as changes too, as Naafiri Q's "Maximum bonus damage
    /// changed to …" under its minimum bonus damage. Fails when there is none
    fn map_bullets(&self, key: Key, subject: &str) -> MayFail<Vec<(MapMode, MapChange)>> {
        use crate::generators::parser::{map_changes, single_mode};
        use wiki::map_changes::ChangeTarget;

        let champion_id = self.champion_id();
        let mut bullets = Vec::new();

        // Changes of the key, or of one of its abilities by name, as Ambessa Q's sweeps
        let of_key = |target: &ChangeTarget| match target {
            ChangeTarget::Ability {
                champion_id: id,
                key: of,
            } => id == champion_id && *of == key,
            ChangeTarget::NamedAbility {
                champion_id: id,
                key: of,
                ..
            } => id == champion_id && *of == Some(key),
            _ => false,
        };

        for module in map_changes() {
            let changes = module
                .entries
                .values()
                .filter(|entry| of_key(&entry.target))
                .flat_map(|entry| &entry.changes)
                .flat_map(|change| {
                    std::iter::once(change.clone()).chain(
                        change
                            .notes
                            .iter()
                            .map(|note| MapChange::parse(note, false)),
                    )
                });

            for change in changes {
                if change
                    .subject()
                    .is_some_and(|part| part.eq_ignore_ascii_case(subject))
                {
                    bullets.push((single_mode(&module.maps, subject)?, change));
                }
            }
        }

        match bullets.is_empty() {
            true => Err(format!(
                "[{}] {key:?} has no map change named {subject:?}",
                self.champion_id()
            )
            .into()),
            false => Ok(bullets),
        }
    }

    /// New value and mode of each map change of `key` whose subject is `subject`.
    /// Fails when there is none, or one can't be read
    fn map_values(&self, key: Key, subject: &str) -> MayFail<Vec<(MapMode, MapValue)>> {
        let lookup = Lookup::new("return {}", "return {}")?;

        self.map_bullets(key, subject)?
            .into_iter()
            .map(|(mode, change)| {
                MapValue::new(&change, key, &lookup)
                    .map(|value| (mode, value))
                    .ok_or_else(|| format!("Unreadable map change: {}", change.text).into())
            })
            .collect()
    }

    /// Mode and numbers, in order, of what the only map change of `key` named
    /// `subject` changes to, for one written in a shape [`Self::map_value`] can't
    /// read, as Zoe Q's "AP ratio changed to 70-175% AP": `[70, 175]`. Reading them
    /// by position from the wiki follows its updates, where typed values wouldn't
    pub fn map_numbers(&self, key: Key, subject: &str) -> MayFail<(MapMode, Vec<f64>)> {
        let mut bullets = self.map_bullets(key, subject)?;

        bullets.dedup_by(|a, b| a.0 == b.0 && a.1.text == b.1.text);

        let [(mode, change)] = <[_; 1]>::try_from(bullets).map_err(|bullets| {
            format!(
                "[{}] {key:?} has {} map changes named {subject:?}",
                self.champion_id(),
                bullets.len()
            )
        })?;

        let value = change
            .text
            .split_once("changed to")
            .map_or(change.text.as_str(), |(_, value)| value);

        let numbers = regex!(r"\d+(?:\.\d+)?")
            .find_iter(value)
            .filter_map(|number| number.as_str().parse().ok())
            .collect();

        Ok((mode, numbers))
    }

    /// Mode and new value of the only map change of `key` named `subject`, for a
    /// generator to place itself, as Udyr R's base damage per second, which his
    /// damage per tick has half of
    pub fn map_value(&self, key: Key, subject: &str) -> MayFail<(MapMode, Formula)> {
        let mut values = self.map_values(key, subject)?;

        // The same change written twice, as the notes of both of LeBlanc R's mimics
        values.dedup();

        match &values[..] {
            [(mode, MapValue::Base(value) | MapValue::Ratio { ratio: value, .. })] => {
                Ok((*mode, value.clone()))
            }
            values => Err(format!(
                "[{}] {key:?} has {} map changes named {subject:?}",
                self.champion_id(),
                values.len()
            )
            .into()),
        }
    }

    /// `formula` with the map changes of `key` whose subject is `subject`, as the
    /// spikes of Evelynn's Q, a value that no damage holds alone. A ratio of a stat
    /// that also scales with another, as `3% (+ 1.5% per 100 AP)` of the target's
    /// maximum health, has only its constant part replaced. Fails when no change of
    /// `key` is named `subject`
    pub fn map_changed(&self, key: Key, subject: &str, mut formula: Formula) -> MayFail<Formula> {
        for (mode, value) in self.map_values(key, subject)? {
            let (targets, replacement) = match value {
                MapValue::Base(value) => {
                    (formula.base_mut().into_iter().collect::<Vec<_>>(), value)
                }
                MapValue::Ratio { stat, ratio } => (
                    formula
                        .stat_ratio_mut(stat)
                        .into_iter()
                        .filter_map(|ratio| match ratio {
                            Formula::Sum(_) => ratio.base_mut(),
                            ratio => Some(ratio),
                        })
                        .collect(),
                    ratio,
                ),
            };

            let [target] = <[_; 1]>::try_from(targets).map_err(|targets| {
                format!(
                    "[{}] {key:?}: {} terms for map change {subject:?}",
                    self.champion_id(),
                    targets.len()
                )
            })?;

            Self::place(target, mode, replacement);
        }

        Ok(formula)
    }

    /// Sets `ratio` on `mode` for the ratio reached through `stats` in `id`, for map
    /// changes whose value the wiki writes in a shape [`Self::map_change`] can't read,
    /// as Camille W's `{{fd|3}}% per 100 bonus AD` of the target's maximum health,
    /// reached through `[EnemyMaxHealth, BonusAd]`. A ratio that is a sum has only its
    /// constant part replaced
    pub fn map_ratio(
        &mut self,
        id: AbilityId,
        mode: MapMode,
        stats: &[CtxVar],
        ratio: impl IntoFormula,
    ) -> MayFail<&mut Self> {
        let champion_id = self.champion_id().to_string();
        let mut target = &mut self.damage_mut(id).formula;

        for stat in stats {
            let [ratio] = <[_; 1]>::try_from(target.stat_ratio_mut(*stat)).map_err(|ratios| {
                format!(
                    "[{champion_id}] {id:?}: {} ratios of {stat:?}",
                    ratios.len()
                )
            })?;

            target = ratio;
        }

        if let Formula::Sum(_) = target {
            target = target
                .base_mut()
                .ok_or_else(|| format!("[{champion_id}] {id:?}: ratio without a constant"))?;
        }

        Self::place(target, mode, ratio.to_formula());

        Ok(self)
    }

    /// Sets `base` on `mode` for the base of `id`, for a map change of another value
    /// it is derived from, as Udyr R's damage per tick from its damage per second
    pub fn map_base(&mut self, id: AbilityId, mode: MapMode, base: Formula) -> MayFail<&mut Self> {
        let champion_id = self.champion_id().to_string();
        let target = self
            .damage_mut(id)
            .formula
            .base_mut()
            .ok_or_else(|| format!("[{champion_id}] {id:?} has no base"))?;

        Self::place(target, mode, base);

        Ok(self)
    }

    /// Gives `target` the value `value` on `mode`. A value it already has there, as
    /// Viktor Q's discharge AP ratio on Arena, or Twitch E's 6 times 35% AP written
    /// as 2.0999…, needs no branch
    fn place(target: &mut Formula, mode: MapMode, value: Formula) {
        let close = |a: &f64, b: &f64| (a - b).abs() <= 1e-9 * a.abs().max(1.0);

        let same = match (target.on(mode), &value) {
            (Formula::Const(a), Formula::Const(b)) => close(&a, b),
            (
                Formula::ByRank { rank, values },
                Formula::ByRank {
                    rank: other,
                    values: others,
                },
            ) => {
                rank == *other
                    && values.len() == others.len()
                    && values.iter().zip(others).all(|(a, b)| close(a, b))
            }
            (current, value) => current == *value,
        };

        if !same {
            *target = target.clone().with_map(mode, value);
        } else if target.on(MapMode::Default) == target.on(mode)
            && let Formula::ByMap { default, maps } = target
            && maps.iter().any(|(arm, _)| *arm == mode)
        {
            // An arm of the model with the default's value, as Irelia R's AP ratio
            maps.retain(|(arm, _)| *arm != mode);

            if maps.is_empty() {
                *target = default.as_ref().clone();
            }
        }
    }

    /// Replaces the formula of `id`
    pub fn set(&mut self, id: AbilityId, formula: Formula) -> &mut Self {
        self.damage_mut(id).formula = formula;
        self
    }

    /// Replaces the formula of `id` with `f` of it, applied to its value everywhere
    /// and on each mode that changes all of it
    pub fn edit(&mut self, id: AbilityId, f: impl Fn(Formula) -> Formula) -> &mut Self {
        let damage = self.damage_mut(id);
        damage.formula = damage.formula.clone().edit_arms(f);
        self
    }

    /// Adds `new` with `f` of the formula of `from`, and its name and damage type
    pub fn derive(
        &mut self,
        new: AbilityId,
        from: AbilityId,
        f: impl FnOnce(Formula) -> Formula,
    ) -> MayFail<&mut Self> {
        let champion_id = self.champion_id().to_string();

        if self.damages.contains_key(&new) {
            return Err(format!("[{champion_id}] {new:?}: taken already").into());
        }

        let mut damage = self
            .damages
            .get(&from)
            .ok_or_else(|| format!("[{champion_id}] {from:?} was never taken"))?
            .clone();

        damage.formula = f(damage.formula);
        self.damages.insert(new, damage);
        Ok(self)
    }

    pub fn comment(&mut self, id: AbilityId, comment: &str) -> &mut Self {
        self.damage_mut(id).comment = comment.to_string();
        self
    }

    pub fn damage_type(&mut self, id: AbilityId, damage_type: DamageType) -> &mut Self {
        self.damage_mut(id).damage_type = damage_type;
        self
    }

    pub fn attributes(&mut self, id: AbilityId, attributes: Attrs) -> &mut Self {
        self.damage_mut(id).attributes = attributes;
        self
    }

    /// A copy of the formula of `id`, for use in `f![…]`
    pub fn damage(&self, id: AbilityId) -> MayFail<Formula> {
        self.damages
            .get(&id)
            .map(|damage| damage.formula.clone())
            .ok_or_else(|| format!("[{}] {id:?} was never taken", self.champion_id()).into())
    }

    /// Text of a description section, grouped by its wiki heading.
    pub fn description(&self, key: Key, heading: &str) -> MayFail<&str> {
        let pages = self
            .model
            .abilities
            .get(&key)
            .ok_or_else(|| format!("[{}] has no {key:?} pages", self.champion_id()))?;

        let matches = pages
            .iter()
            .filter_map(|page| page.descriptions.get(heading))
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [text] => Ok(text.as_str()),
            _ => Err(format!(
                "[{}] {key:?} has {} descriptions named {heading:?}",
                self.champion_id(),
                matches.len()
            )
            .into()),
        }
    }

    /// Text of one of the champion's wiki exceptions, from the cached metadata
    pub fn exception(&self, name: &str) -> MayFail<&str> {
        self.inner.exception(name)
    }

    /// Adds `combos`, Every ability of a combo must have been taken before
    pub fn combos<const N: usize>(&mut self, combos: [&[ComboElement]; N]) -> MayFail<&mut Self> {
        for combo in combos {
            for element in combo {
                if let ComboElement::Ability(id) = element
                    && !self.damages.contains_key(id)
                {
                    let champion_id = self.champion_id();
                    return Err(format!("[{champion_id}] combo of {id:?}, never taken").into());
                }
            }

            self.inner.combo.push(combo.to_vec());
        }

        Ok(self)
    }

    pub fn constant(&mut self, name: &str, value: impl Into<Constant>) -> MayFail<&mut Self> {
        self.inner.constant(name, value)?;
        Ok(self)
    }

    /// Writes each damage into the abilities of [`Self::inner`], then checks the output consistency
    /// of `end`: zero damages are removed, unknown damage types warned and merges made.
    /// Writes Rust code and documentation from the same structured formula tree.
    pub fn end(&mut self) -> MayFail {
        for (id, damage) in std::mem::take(&mut self.damages) {
            let Damage {
                formula,
                name,
                damage_type,
                comment,
                attributes,
            } = damage;

            // Structured zero damage has no generated ability.
            if formula.is_zero() {
                println!("[{}] Removing [{id:?}]", self.inner.champion_id);
                continue;
            }

            self.inner.docs.insert(id, formula.to_string());

            let rust = formula.rust();
            let ability = Ability {
                name,
                damage_type,
                attributes,
                comment,
                damage: rust,
            };

            self.inner.abilities.insert(id, ability);
        }

        self.inner.end()?;

        // Damages that `end` removed have no source either
        let Champion {
            abilities, docs, ..
        } = &mut self.inner;
        docs.retain(|id, _| abilities.contains_key(id));

        Ok(())
    }

    /// The champion the build writes. Fails when damages were taken but never written,
    /// as by a generator that does not call [`Self::end`]
    pub fn into_champion(self) -> MayFail<Champion> {
        match self.damages.is_empty() {
            true => Ok(self.inner),
            false => Err(format!(
                "[{}] {:?} were taken but never written: the generator must call end()",
                self.champion_id(),
                self.damages.keys().collect::<Vec<_>>()
            )
            .into()),
        }
    }

    /// [`Self::end`], returning the champion for callers that need the output directly.
    pub fn finish(mut self) -> MayFail<Champion> {
        self.end()?;
        self.into_champion()
    }
}
