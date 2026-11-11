//! Which changes of a game mode can be applied without a generator, and the
//! effects they produce. The build applies them to the effects its generators
//! select, and the report runs the same rules on the effects the generator
//! sources name, so both agree on what is applied

use super::{
    ChangeTarget, MapChanges,
    change::{ChangeKind, MapChange, Values},
};
use crate::{champions::WikiChampion, items::item_parser::WikiItem, parser::Effect};
use std::{collections::BTreeMap, str::FromStr};
use tutorlolv2_types::{CtxVar, Key, StatName};

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
    /// Replaces the formula of these effects
    Applied(Vec<SelectedEffect>),
    /// The game data already has it, so there is nothing to override
    AlreadyApplied(&'static str),
    /// A generator has to apply it, for this reason
    Manual(&'static str),
    /// No computed damage depends on it
    NotRelevant,
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

        let key = match target {
            ChangeTarget::Ability { key, .. }
            | ChangeTarget::NamedAbility { key: Some(key), .. } => *key,
            ChangeTarget::NamedAbility { key: None, .. } => {
                return Outcome::Manual("its ability name matches no ability");
            }
            _ => return Outcome::Manual("it changes the champion, not an ability"),
        };

        let effects = selected
            .iter()
            .filter(|s| s.key == key)
            .filter_map(|s| Some((s.clone(), self.effect(champion, s)?)))
            .collect::<Vec<_>>();

        let replaced = match &change.kind {
            ChangeKind::Ratio {
                part,
                scaling,
                ranged,
            } => {
                if ranged.is_some() {
                    return Outcome::Manual("it differs for ranged champions");
                }

                if !is_plain_part(part) {
                    return Outcome::Manual("it names a part of the ability");
                }

                let Some(ctx_var) = scaling.ctx_var().filter(|v| *v != CtxVar::Unknown) else {
                    return Outcome::Manual("its scaling has no known variable");
                };

                let mut found = Vec::new();

                for (selected, effect) in effects {
                    let indexes = effect
                        .scalings
                        .iter()
                        .enumerate()
                        .filter(|(_, s)| s.ctx_var() == Some(ctx_var))
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    match indexes.as_slice() {
                        [] => {}
                        [index] => found.push((selected, effect, *index)),
                        _ => return Outcome::Manual("an effect scales with it more than once"),
                    }
                }

                let Some((_, first, first_index)) = found.first() else {
                    return Outcome::Manual("no selected effect scales with it");
                };

                // Effects that repeat the same scaling, as minimum and maximum
                // damages, change together. Different ones are ambiguous
                let old = first.scalings[*first_index].clone();

                if found.iter().any(|(_, e, i)| e.scalings[*i] != old) {
                    return Outcome::Manual("the selected effects scale with it differently");
                }

                if old == *scaling {
                    return Outcome::AlreadyApplied("the scaling is the same");
                }

                found
                    .into_iter()
                    .map(|(selected, mut effect, index)| {
                        effect.scalings[index] = scaling.clone();
                        (selected, effect)
                    })
                    .collect::<Vec<_>>()
            }
            ChangeKind::Base { part, amount } => {
                if !matches!(part.to_lowercase().as_str(), "" | "base") {
                    return Outcome::Manual("it names a part of the ability");
                }

                if amount.ranged.is_some() || amount.percent {
                    return Outcome::Manual("its values are not a plain base damage");
                }

                let values = match &amount.values {
                    Values::Scalar(value) => vec![*value],
                    Values::ByRank(values) => values.clone(),
                    Values::ByLevel { .. } => {
                        return Outcome::Manual("its values are by level");
                    }
                };

                let found = effects
                    .into_iter()
                    .filter(|(_, e)| e.base.is_some())
                    .collect::<Vec<_>>();

                let Some((_, first)) = found.first() else {
                    return Outcome::Manual("no selected effect has a base damage");
                };

                let old = first.base.clone().unwrap_or_default();

                if found.iter().any(|(_, e)| e.base.as_ref() != Some(&old)) {
                    return Outcome::Manual("the selected effects have different base damages");
                }

                // A single value applies to every rank
                let values = match values.len() {
                    1 => vec![values[0]; old.len()],
                    n if n == old.len() => values,
                    _ => return Outcome::Manual("it has another number of ranks"),
                };

                if values == old {
                    return Outcome::AlreadyApplied("the base damage is the same");
                }

                found
                    .into_iter()
                    .map(|(selected, mut effect)| {
                        effect.base = Some(values.clone());
                        (selected, effect)
                    })
                    .collect::<Vec<_>>()
            }
            ChangeKind::Named { .. } => {
                return Outcome::Manual("only a generator can tell its value");
            }
            ChangeKind::Modifier { .. } => return Outcome::Manual("it is a damage modifier"),
            _ => return Outcome::Manual("it needs a generator"),
        };

        let mut applied = Vec::new();

        for (selected, mut effect) in replaced {
            let old = effect.formula.clone();

            effect.formula = effect.simplify_formula(key).ok().flatten();

            // Formulas given by `use_formula` don't come from the scalings or base
            if effect.formula == old {
                return Outcome::Manual("its formula doesn't come from what it changes");
            }

            applied.push((selected, effect));
        }

        let selected = applied.iter().map(|(s, _)| s.clone()).collect();
        self.effects.extend(applied);

        Outcome::Applied(selected)
    }
}

/// Outcome of a change of an item that the item data may already have, as
/// disabled items or stats with the same value. `code` is the code of the module,
/// which is also the key of the item modes
pub fn item_outcome(change: &MapChange, item: &WikiItem, code: &str) -> Outcome {
    match &change.kind {
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
                _ => Outcome::Manual("the item has another stat"),
            }
        }
        kind if ChangeKind::is_damage_relevant(kind.name()) => {
            Outcome::Manual("items need a generator")
        }
        _ => Outcome::NotRelevant,
    }
}

/// Whether the text before "ratio" only names the ratio, as "AP" or "Bonus AD",
/// rather than a part of the ability, as "First dash AP"
fn is_plain_part(part: &str) -> bool {
    part.to_lowercase().split_whitespace().all(|word| {
        matches!(
            word,
            "ap" | "ad"
                | "bonus"
                | "base"
                | "attack"
                | "damage"
                | "ability"
                | "power"
                | "health"
                | "maximum"
                | "missing"
                | "current"
                | "target's"
                | "of"
                | "the"
                | "armor"
                | "magic"
                | "resistance"
                | "mana"
        )
    })
}
