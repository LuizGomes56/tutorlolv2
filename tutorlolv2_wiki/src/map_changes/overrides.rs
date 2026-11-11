//! Which changes of a game mode can be applied without a generator, and the
//! effects they produce. The build applies them to the effects its generators
//! select, and the report runs the same rules on the effects the generator
//! sources name, so both agree on what is applied.
//!
//! A change names a value, as "AP ratio" or "Base damage", but not the effect
//! it belongs to. [`decide`] finds that effect among the selected ones:
//!
//! 1. A part that names something, as "Passive" in "Passive AP ratio", keeps the
//!    effects whose name has those words.
//! 2. One effect left takes the change. With several, the one whose current value
//!    is closest to the new one takes it: modes change values by a margin, not to
//!    a different magnitude.
//! 3. Other effects that are a constant multiple of it, as a total that is three
//!    hits, are multiplied the same way. Effects clearly farther away keep their
//!    value. Anything else is ambiguous, and left to a generator

use super::{
    ChangeTarget, MapChanges,
    change::{ChangeKind, MapChange, Values},
};
use crate::{
    champions::WikiChampion, items::item_parser::WikiItem, parser::Effect, parser::Scaling,
};
use std::{collections::BTreeMap, str::FromStr};
use tutorlolv2_types::{AttackType, CtxVar, Key, StatName};

/// An effect of a champion ability, as a generator selects it with
/// `.ability_nth(nth, key, [(name, …)])`
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SelectedEffect {
    pub key: Key,
    pub nth: usize,
    pub name: String,
}

/// What happens to one change
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// Replaces the formula of these effects, by the rule that chose them
    Applied {
        effects: Vec<String>,
        rule: &'static str,
    },
    /// The game data already has it, so there is nothing to override
    AlreadyApplied(&'static str),
    /// A generator has to apply it, for this reason
    Manual(&'static str),
    /// No computed damage depends on it
    NotRelevant,
}

/// What [`decide`] does with one change over the effects `K` it may apply to
#[derive(Debug)]
pub enum Decision<K> {
    /// New effects, with their formula simplified again
    Apply {
        effects: Vec<(K, Effect)>,
        rule: &'static str,
    },
    AlreadyApplied(&'static str),
    Manual(&'static str),
}

/// A selected effect a change may apply to, with its name and current effect
pub struct Candidate<K> {
    pub id: K,
    pub name: String,
    pub effect: Effect,
}

/// Words of a part that only describe the value, as "AP" or "bonus", rather
/// than naming a piece of the ability
const DESCRIPTIVE: &[&str] = &[
    "ap",
    "ad",
    "bonus",
    "base",
    "damage",
    "ratio",
    "attack",
    "ability",
    "power",
    "health",
    "armor",
    "magic",
    "resistance",
    "mana",
    "of",
    "the",
    "target's",
    "missing",
    "current",
];

impl ChangeKind {
    /// Applies a Ratio or Base change to the right ones of `candidates`, the effects
    /// of what the change targets. `key` simplifies the formulas, and `ranged` reads
    /// the values that `{{rd}}` gives ranged champions
    pub fn decide<K: Clone>(
        &self,
        key: Key,
        ranged: bool,
        candidates: Vec<Candidate<K>>,
    ) -> Decision<K> {
        let (part, plan) = match self {
            ChangeKind::Ratio {
                part,
                scaling,
                ranged: ranged_scaling,
            } => {
                let scaling = match (ranged, ranged_scaling) {
                    (true, Some(ranged)) => ranged,
                    _ => scaling,
                };

                let Some(ctx_var) = scaling.ctx_var().filter(|v| *v != CtxVar::Unknown) else {
                    return Decision::Manual("its scaling has no known variable");
                };

                let Some(new) = scaling.values() else {
                    return Decision::Manual("its scaling is not a ratio");
                };

                let mut found = Vec::new();

                for candidate in candidates {
                    let indexes = candidate
                        .effect
                        .scalings
                        .iter()
                        .enumerate()
                        .filter(|(_, s)| s.ctx_var() == Some(ctx_var))
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    match indexes.as_slice() {
                        [] => {}
                        [index] => match candidate.effect.scalings[*index].values() {
                            Some(old) => found.push((candidate, Target::Scaling(*index), old)),
                            None => return Decision::Manual("an effect scales with it by level"),
                        },
                        _ => return Decision::Manual("an effect scales with it more than once"),
                    }
                }

                if found.is_empty() {
                    return Decision::Manual("no selected effect scales with it");
                }

                (
                    part,
                    Plan {
                        found,
                        new,
                        scaling: Some(scaling.clone()),
                    },
                )
            }
            ChangeKind::Base { part, amount } => {
                if amount.percent {
                    return Decision::Manual("its values are not a plain base damage");
                }

                let values = match (ranged, &amount.ranged) {
                    (true, Some(ranged)) => ranged,
                    _ => &amount.values,
                };

                let new = match values {
                    Values::Scalar(value) => vec![*value],
                    Values::ByRank(values) => values.clone(),
                    Values::ByLevel { .. } => return Decision::Manual("its values are by level"),
                };

                let found = candidates
                    .into_iter()
                    .filter_map(|c| {
                        let old = c.effect.base.clone()?;
                        Some((c, Target::Base, old))
                    })
                    .collect::<Vec<_>>();

                if found.is_empty() {
                    return Decision::Manual("no selected effect has a base damage");
                }

                (
                    part,
                    Plan {
                        found,
                        new,
                        scaling: None,
                    },
                )
            }
            _ => return Decision::Manual("it needs a generator"),
        };

        plan.apply(part, key)
    }
}

/// Which value of an effect a change replaces
#[derive(Clone, Copy)]
enum Target {
    Scaling(usize),
    Base,
}

struct Plan<K> {
    /// Candidates that have the value, with its current values
    found: Vec<(Candidate<K>, Target, Vec<f64>)>,
    new: Vec<f64>,
    /// The new scaling, for ratio changes
    scaling: Option<Scaling>,
}

impl<K: Clone> Plan<K> {
    fn apply(self, part: &str, key: Key) -> Decision<K> {
        let mut rule = "the only selected effect";

        // 1. A part that names a piece of the ability keeps the effects named like it
        let words = part
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !DESCRIPTIVE.contains(w))
            .map(String::from)
            .collect::<Vec<_>>();

        // Effects that may take the change. The others may still follow it
        let pool = (0..self.found.len())
            .filter(|&i| {
                let name = self.found[i].0.name.to_lowercase();
                words.iter().all(|w| name.contains(w.as_str()))
            })
            .collect::<Vec<_>>();

        if pool.is_empty() {
            return Decision::Manual("it names a part that no selected effect is named after");
        }

        if !words.is_empty() {
            rule = "the effects named like it";
        }

        // 2. The closest value takes the change
        let best = pool
            .iter()
            .copied()
            .min_by(|&a, &b| {
                let (da, db) = (
                    distance(&self.found[a].2, &self.new),
                    distance(&self.found[b].2, &self.new),
                );
                da.total_cmp(&db)
            })
            .unwrap_or_default();

        let best_distance = distance(&self.found[best].2, &self.new);

        if best_distance > 0.75 {
            return Decision::Manual("no selected effect has a value close to it");
        }

        let all_same = pool.iter().all(|&i| self.found[i].2 == self.found[best].2);

        // 3. Constant multiples follow it, and values far from it stay. A part that
        // names a piece of the ability only changes that piece, since other pieces
        // may be multiples of it by coincidence, as Gwen's snips and final snip
        let mut factors = Vec::new();

        for (i, (_, _, old)) in self.found.iter().enumerate() {
            if i == best || (all_same && pool.contains(&i)) {
                factors.push(Some(1.0));
                continue;
            }

            if !pool.contains(&i) {
                factors.push(None);
                continue;
            }

            match follows(&self.found[i], &self.found[best]) {
                Some(factor) => factors.push(Some(factor)),
                None if distance(old, &self.new) >= 2.0 * best_distance + 0.05 => {
                    factors.push(None)
                }
                None => return Decision::Manual("the selected effects are ambiguous"),
            }
        }

        if pool.len() > 1 && !words.is_empty() {
            rule = "the effects named like it, by the closest value";
        } else if pool.len() > 1 && !all_same {
            rule = "the closest value";
        } else if pool.len() > 1 {
            rule = "the effects that share the value";
        }

        let mut effects = Vec::new();
        let mut unchanged = true;

        for ((candidate, target, old), factor) in self.found.into_iter().zip(factors) {
            let Some(factor) = factor else {
                continue;
            };

            let new = self.new.iter().map(|v| v * factor).collect::<Vec<_>>();
            let mut effect = candidate.effect.clone();

            match target {
                Target::Base => {
                    let Some(values) = fit(&new, old.len(), key, true) else {
                        return Decision::Manual("it has another number of ranks");
                    };

                    unchanged &= values == old;
                    effect.base = Some(values);
                }
                Target::Scaling(index) => {
                    let Some(values) = fit(&new, old.len(), key, false) else {
                        return Decision::Manual("it has another number of ranks");
                    };

                    let Some(scaling) = self
                        .scaling
                        .as_ref()
                        .and_then(|s| s.with_values(values.clone()))
                    else {
                        return Decision::Manual("its scaling can't be scaled");
                    };

                    unchanged &= values == old;
                    effect.scalings[index] = scaling;
                }
            }

            let formula = effect.formula.clone();
            effect.formula = effect.simplify_formula(key).ok().flatten();

            // Formulas given by `use_formula` don't come from the scalings or base
            if effect.formula == formula && !unchanged {
                return Decision::Manual("its formula doesn't come from what it changes");
            }

            effects.push((candidate.id, effect));
        }

        if unchanged {
            return Decision::AlreadyApplied("the value is the same");
        }

        Decision::Apply { effects, rule }
    }
}

impl Scaling {
    /// Values of a scaling that a change can replace
    fn values(&self) -> Option<Vec<f64>> {
        match self {
            Scaling::Simple { value, .. }
            | Scaling::Per100 { value, .. }
            | Scaling::PercentAttr { value, .. } => Some(vec![*value]),
            Scaling::Ranked { values, .. } | Scaling::RankedPer100 { values, .. } => {
                Some(values.clone())
            }
            _ => None,
        }
    }

    /// This scaling with other values, by rank when there are several
    fn with_values(&self, values: Vec<f64>) -> Option<Self> {
        let ctx_var = self.ctx_var()?;

        Some(match (self, values.as_slice()) {
            (Scaling::Simple { .. } | Scaling::Ranked { .. }, [value]) => Scaling::Simple {
                value: *value,
                ctx_var,
            },
            (Scaling::Simple { .. } | Scaling::Ranked { .. }, _) => {
                Scaling::Ranked { values, ctx_var }
            }
            (Scaling::Per100 { .. } | Scaling::RankedPer100 { .. }, [value]) => Scaling::Per100 {
                value: *value,
                ctx_var,
            },
            (Scaling::Per100 { .. } | Scaling::RankedPer100 { .. }, _) => {
                Scaling::RankedPer100 { values, ctx_var }
            }
            (Scaling::PercentAttr { debug, .. }, [value]) => Scaling::PercentAttr {
                value: *value,
                debug: debug.clone(),
                ctx_var,
            },
            _ => return None,
        })
    }
}

/// Values of a change for an effect with `len` values. A single value applies
/// to every rank of a base, and ultimates keep their ranks at indexes 0, 2 and 4
/// of 5 values, so 3 values fill those and the ones between
fn fit(new: &[f64], len: usize, key: Key, repeat_single: bool) -> Option<Vec<f64>> {
    match new.len() {
        n if n == len => Some(new.to_vec()),
        1 if repeat_single => Some(vec![new[0]; len]),
        1 => Some(new.to_vec()),
        3 if key == Key::R && len == 5 => Some(vec![
            new[0],
            (new[0] + new[1]) / 2.0,
            new[1],
            (new[1] + new[2]) / 2.0,
            new[2],
        ]),
        _ => None,
    }
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len().max(1) as f64
}

/// How far `old` is from `new`, relative to `new`
fn distance(old: &[f64], new: &[f64]) -> f64 {
    let (old, new) = (mean(old), mean(new));
    (old - new).abs() / new.abs().max(f64::EPSILON)
}

/// The factor that `other` is of `best`, when every value of it is that multiple.
/// A single value is a multiple of any other, so the other values of the effects
/// must have the same factor: the base for a ratio, and the ratios for a base
fn follows<K>(
    other: &(Candidate<K>, Target, Vec<f64>),
    best: &(Candidate<K>, Target, Vec<f64>),
) -> Option<f64> {
    let factor = multiple(&other.2, &best.2)?;

    if other.2.len() > 1 {
        return Some(factor);
    }

    let (a, b) = (&other.0.effect, &best.0.effect);
    let confirms = match other.1 {
        Target::Scaling(_) => match (&a.base, &b.base) {
            (Some(a), Some(b)) => multiple(a, b),
            _ => None,
        },
        Target::Base => {
            let values = |e: &Effect| {
                e.scalings
                    .iter()
                    .filter_map(Scaling::values)
                    .flatten()
                    .collect::<Vec<_>>()
            };

            let (a, b) = (values(a), values(b));

            (!a.is_empty()).then(|| multiple(&a, &b)).flatten()
        }
    };

    confirms
        .filter(|other| (other - factor).abs() <= 0.01 * factor.abs())
        .map(|_| factor)
}

/// The factor `values` is of `of`, when it is the same at every rank
fn multiple(values: &[f64], of: &[f64]) -> Option<f64> {
    if values.len() != of.len() || of.contains(&0.0) {
        return None;
    }

    let factor = values[0] / of[0];

    values
        .iter()
        .zip(of)
        .all(|(v, o)| (v / o - factor).abs() <= 0.01 * factor.abs())
        .then_some(factor)
}

/// Effects of a champion as changed by one game mode, and what happened to
/// each of its changes, in the order of the module
#[derive(Debug, Default)]
pub struct ChampionOverrides {
    /// New effects, with their formula simplified again
    pub effects: BTreeMap<SelectedEffect, Effect>,
    /// Outcome of each change of the champion, by key and index of the change
    pub outcomes: Vec<(String, usize, Outcome)>,
}

impl ChampionOverrides {
    /// Applies the changes of `changes` that target `champion`, one after
    /// another, to the effects in `selected`
    pub fn new(champion: &WikiChampion, changes: &MapChanges, selected: &[SelectedEffect]) -> Self {
        let mut this = Self::default();

        for (key, entry) in &changes.entries {
            let champion_id = match &entry.target {
                ChangeTarget::Champion { champion_id }
                | ChangeTarget::Ability { champion_id, .. }
                | ChangeTarget::NamedAbility { champion_id, .. } => champion_id,
                _ => continue,
            };

            if *champion_id != champion.champion_id {
                continue;
            }

            for (i, change) in entry.changes.iter().enumerate() {
                let outcome = this.apply(champion, &entry.target, change, selected);
                this.outcomes.push((key.clone(), i, outcome));
            }
        }

        this
    }

    /// Current effect of `selected`, with the changes applied so far
    fn effect(&self, champion: &WikiChampion, selected: &SelectedEffect) -> Option<Effect> {
        if let Some(effect) = self.effects.get(selected) {
            return Some(effect.clone());
        }

        champion
            .wiki_abilities
            .get(&selected.key)?
            .get(selected.nth)?
            .effects
            .get(&selected.name)
            .cloned()
    }

    fn apply(
        &mut self,
        champion: &WikiChampion,
        target: &ChangeTarget,
        change: &MapChange,
        selected: &[SelectedEffect],
    ) -> Outcome {
        if !ChangeKind::is_damage_relevant(change.kind.name()) {
            return Outcome::NotRelevant;
        }

        // Base stats are applied with the stats of the champion, see `WikiMapStats`
        if let ChangeKind::BaseStat { .. } = change.kind {
            return Outcome::Applied {
                effects: vec!["base stats".into()],
                rule: "the stats of the champion",
            };
        }

        let key = match target {
            ChangeTarget::Ability { key, .. }
            | ChangeTarget::NamedAbility { key: Some(key), .. } => *key,
            ChangeTarget::NamedAbility { key: None, .. } => {
                return Outcome::Manual("its ability name matches no ability");
            }
            _ => return Outcome::Manual("it changes the champion, not an ability"),
        };

        match &change.kind {
            ChangeKind::Named { .. } => {
                return Outcome::Manual("only a generator can tell its value");
            }
            ChangeKind::Modifier { .. } => return Outcome::Manual("it is a damage modifier"),
            _ => {}
        }

        let candidates = selected
            .iter()
            .filter(|s| s.key == key)
            .filter_map(|s| {
                Some(Candidate {
                    id: s.clone(),
                    name: s.name.clone(),
                    effect: self.effect(champion, s)?,
                })
            })
            .collect();

        let ranged = matches!(champion.attack_type, AttackType::Ranged);

        match change.kind.decide(key, ranged, candidates) {
            Decision::Apply { effects, rule } => {
                let labels = effects
                    .iter()
                    .map(|(s, _)| format!("{:?}[{}] {:?}", s.key, s.nth, s.name))
                    .collect();

                self.effects.extend(effects);

                Outcome::Applied {
                    effects: labels,
                    rule,
                }
            }
            Decision::AlreadyApplied(reason) => Outcome::AlreadyApplied(reason),
            Decision::Manual(reason) => Outcome::Manual(reason),
        }
    }
}

impl MapChange {
    /// Outcome of a change of an item that the item data may already have, as
    /// disabled items or stats with the same value. `code` is the code of the module,
    /// which is also the key of the item modes
    pub fn item_outcome(&self, item: &WikiItem, code: &str) -> Outcome {
        match &self.kind {
            ChangeKind::Disabled => match item.modes.get(code) {
                Some(false) => Outcome::AlreadyApplied("the item data doesn't sell it on this map"),
                _ => Outcome::Manual("the item data sells it on this map"),
            },
            ChangeKind::ItemStat { stat, amount } => {
                let current = item
                    .stats
                    .iter()
                    .find(|(name, _)| StatName::from_str(name).ok() == Some(*stat))
                    .map(|(_, value)| *value);

                match (&amount.values, current) {
                    (Values::Scalar(value), Some(current)) if *value == current => {
                        Outcome::AlreadyApplied("the item has the same stat")
                    }
                    (Values::Scalar(_), _) if amount.ranged.is_none() => Outcome::Applied {
                        effects: vec!["item stats".into()],
                        rule: "the stats of the item",
                    },
                    _ => Outcome::Manual("its stat is not a single value"),
                }
            }
            kind if ChangeKind::is_damage_relevant(kind.name()) => {
                Outcome::Manual("items need a generator")
            }
            _ => Outcome::NotRelevant,
        }
    }
}

/// Effects of an item or rune as changed by one game mode, by the key their
/// generator selects them with, as the slot `pass2` of an item or the name of a
/// rune effect
#[derive(Debug, Default)]
pub struct EffectOverrides {
    pub effects: BTreeMap<String, Effect>,
    /// Outcome of each change of the item or rune, by key and index of the change
    pub outcomes: Vec<(String, usize, Outcome)>,
}

impl EffectOverrides {
    /// Applies the changes of `changes` that target `target`, one after another, to
    /// `effects`, the selected effects by key with their name. `ranged` reads the
    /// values that `{{rd}}` gives ranged champions
    pub fn new(
        target: &ChangeTarget,
        changes: &MapChanges,
        effects: &BTreeMap<String, (String, Effect)>,
        ranged: bool,
    ) -> Self {
        let mut this = Self::default();

        for (key, entry) in changes.entries.iter().filter(|(_, e)| e.target == *target) {
            for (i, change) in entry.changes.iter().enumerate() {
                let outcome = this.apply(change, effects, ranged);
                this.outcomes.push((key.clone(), i, outcome));
            }
        }

        this
    }

    fn apply(
        &mut self,
        change: &MapChange,
        effects: &BTreeMap<String, (String, Effect)>,
        ranged: bool,
    ) -> Outcome {
        match &change.kind {
            kind if !ChangeKind::is_damage_relevant(kind.name()) => return Outcome::NotRelevant,
            ChangeKind::Ratio { .. } | ChangeKind::Base { .. } => {}
            ChangeKind::Named { .. } => {
                return Outcome::Manual("only a generator can tell its value");
            }
            _ => return Outcome::Manual("it needs a generator"),
        }

        let candidates = effects
            .iter()
            .map(|(id, (name, effect))| Candidate {
                id: id.clone(),
                name: name.clone(),
                effect: self.effects.get(id).unwrap_or(effect).clone(),
            })
            .collect();

        // Items and runes are simplified as innates
        match change.kind.decide(Key::P, ranged, candidates) {
            Decision::Apply { effects, rule } => {
                let labels = effects.iter().map(|(id, _)| id.clone()).collect();

                self.effects.extend(effects);
                Outcome::Applied {
                    effects: labels,
                    rule,
                }
            }
            Decision::AlreadyApplied(reason) => Outcome::AlreadyApplied(reason),
            Decision::Manual(reason) => Outcome::Manual(reason),
        }
    }
}
