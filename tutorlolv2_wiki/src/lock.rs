//! Tracks the effects that generators select by index. Effects are stored in maps
//! sorted by name, so whenever the wiki adds, removes or renames an effect, the
//! index of every effect after it changes. The lock file lists every effect with
//! its index and a hash of its formula, so these changes show up in its diff and
//! in the report printed when it is regenerated

use crate::{
    JsonRead, LOL_VERSION, MayFail, champions::WikiChampion, parser::Effect, runes::WikiRune,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};
use tutorlolv2_types::Key;

pub fn path() -> PathBuf {
    PathBuf::from("wiki.lock.json")
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WikiLock {
    pub version: String,
    pub champions: BTreeMap<String, BTreeMap<Key, Vec<LockedAbility>>>,
    pub runes: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LockedAbility {
    pub name: String,
    /// Effects as `"index: name (formula hash)"`
    pub effects: Vec<String>,
}

/// 64-bit FNV-1a, used because the standard library's hasher is not guaranteed
/// to produce the same output across Rust versions
fn fnv1a(data: &str) -> u64 {
    data.bytes().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x100000001b3)
    })
}

fn lock_effects<'a>(effects: impl Iterator<Item = (&'a String, &'a Effect)>) -> Vec<String> {
    effects
        .enumerate()
        .map(|(i, (name, effect))| {
            let formula = effect.formula.as_deref().unwrap_or_default();
            format!("{i}: {name} ({:08x})", fnv1a(formula) as u32)
        })
        .collect()
}

/// Splits `"index: name (hash)"` into `"index: name"` and its hash
fn split_entry(entry: &str) -> (&str, &str) {
    entry.rsplit_once(" (").unwrap_or((entry, ""))
}

pub fn build() -> MayFail<WikiLock> {
    let champions = BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?
        .into_iter()
        .map(|(champion_id, champion)| {
            let abilities = champion
                .wiki_abilities
                .into_iter()
                .map(|(key, abilities)| {
                    let abilities = abilities
                        .into_iter()
                        .map(|ability| LockedAbility {
                            name: ability.name,
                            effects: lock_effects(ability.effects.iter()),
                        })
                        .collect();

                    (key, abilities)
                })
                .collect();

            (champion_id, abilities)
        })
        .collect();

    let runes = BTreeMap::<String, WikiRune>::from_file("cache/wiki/runes/full.json")?
        .into_iter()
        .map(|(rune_id, rune)| (rune_id, lock_effects(rune.effects.iter())))
        .collect();

    Ok(WikiLock {
        version: LOL_VERSION.to_string(),
        champions,
        runes,
    })
}

/// Compares two lists of effects, returning the entries whose index or name
/// changed, and how many effects kept their index and name but not their formula
fn compare(old: &[String], new: &[String]) -> (Vec<String>, usize) {
    let names = |list: &[String]| -> Vec<String> {
        list.iter()
            .map(|entry| split_entry(entry).0.to_string())
            .collect()
    };

    let (old_names, new_names) = (names(old), names(new));
    let mut lines = Vec::new();

    if old_names != new_names {
        let old_set = old_names.iter().collect::<BTreeSet<_>>();
        let new_set = new_names.iter().collect::<BTreeSet<_>>();
        let index = |entry: &str| -> usize {
            entry
                .split_once(':')
                .and_then(|(i, _)| i.parse().ok())
                .unwrap_or(usize::MAX)
        };

        let mut removed = old_set.difference(&new_set).collect::<Vec<_>>();
        let mut added = new_set.difference(&old_set).collect::<Vec<_>>();

        removed.sort_by_key(|e| index(e));
        added.sort_by_key(|e| index(e));

        lines.extend(removed.into_iter().map(|e| format!("    - {e}")));
        lines.extend(added.into_iter().map(|e| format!("    + {e}")));
    }

    let formulas = old
        .iter()
        .zip(new)
        .filter(|(a, b)| {
            let (a, b) = (split_entry(a), split_entry(b));
            a.0 == b.0 && a.1 != b.1
        })
        .count();

    (lines, formulas)
}

fn report(old: &WikiLock, new: &WikiLock) -> Vec<String> {
    let mut out = Vec::new();
    let mut formulas = BTreeMap::<String, usize>::new();
    let empty = BTreeMap::new();

    for (champion_id, abilities) in &new.champions {
        let old_abilities = old.champions.get(champion_id).unwrap_or(&empty);

        for (key, list) in abilities {
            for (nth, ability) in list.iter().enumerate() {
                let old_effects = old_abilities
                    .get(key)
                    .and_then(|list| list.get(nth))
                    .map(|ability| ability.effects.as_slice())
                    .unwrap_or_default();

                let (lines, changed) = compare(old_effects, &ability.effects);

                if !lines.is_empty() {
                    out.push(format!(
                        "  {champion_id} {key:?}[{nth}] {}: effect indexes changed",
                        ability.name
                    ));

                    out.extend(lines);
                }

                *formulas.entry(champion_id.clone()).or_default() += changed;
            }
        }
    }

    for (rune_id, effects) in &new.runes {
        let old_effects = old
            .runes
            .get(rune_id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let (lines, changed) = compare(old_effects, effects);

        if !lines.is_empty() {
            out.push(format!("  {rune_id}: effect indexes changed"));
            out.extend(lines);
        }

        *formulas.entry(rune_id.clone()).or_default() += changed;
    }

    let changed = formulas
        .into_iter()
        .filter(|(_, n)| *n > 0)
        .map(|(id, n)| format!("{id} ({n})"))
        .collect::<Vec<_>>();

    if !changed.is_empty() {
        out.push(format!("  Formulas changed: {}", changed.join(", ")));
    }

    out
}

/// Regenerates the lock file from the parsed wiki data, printing what changed
/// since the previous one
pub fn write() -> MayFail {
    println!("[fn] lock::write");

    let new = build()?;

    if let Ok(old) = WikiLock::from_file(path()) {
        let report = report(&old, &new);

        match report.is_empty() {
            true => println!("[lock] No changes"),
            false => {
                if old.version != new.version {
                    println!("[lock] Version {} -> {}", old.version, new.version);
                }

                println!("[lock] Changes since the previous lock file:");
                report.iter().for_each(|line| println!("{line}"));
            }
        }
    }

    let json = serde_json::to_string_pretty(&new)?;
    crate::write(path(), json + "\n")
}

#[cfg(test)]
mod tests {
    use super::compare;

    fn entries(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn reports_shifted_indexes_and_formula_changes() {
        let old = entries(&["0: A (1)", "1: B (2)", "2: C (3)"]);
        let new = entries(&["0: A (9)", "1: C (3)"]);

        let (lines, formulas) = compare(&old, &new);

        assert_eq!(
            lines,
            ["    - 1: B", "    - 2: C", "    + 1: C"].map(String::from)
        );
        assert_eq!(formulas, 1);
    }

    #[test]
    fn identical_lists_report_nothing() {
        let list = entries(&["0: A (1)", "1: B (2)"]);
        assert_eq!(compare(&list, &list), (Vec::new(), 0));
    }
}
