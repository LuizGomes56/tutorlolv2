//! Tracks the wiki data that generators depend on. Champion generators select
//! effects by name, while rune and item generators select effects and scalings
//! by index. Effects are stored in maps sorted by name, so whenever the wiki
//! adds, removes or renames an effect, the index of every effect after it
//! changes. The lock file lists every effect with its index and a hash of its
//! formula, and the scalings of every item effect, so these changes show up in
//! its diff and in the report of [`check`] printed when it is regenerated

pub mod check;

use crate::{
    JsonRead, MayFail,
    champions::WikiChampion,
    items::item_parser::WikiItem,
    map_changes::{self, MapChanges},
    parser::{Effect, Scaling},
    runes::WikiRune,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};
use tutorlolv2_types::Key;

pub fn path() -> PathBuf {
    PathBuf::from("wiki.lock.json")
}

/// Name of the lock file inside the archive of a previous version
pub const FILE_NAME: &str = "wiki.lock.json";

/// Directory with the generators of every champion, rune and item
pub const CHAMPION_GENERATORS: &str = "build/src/generators_v2/impls";

pub const GENERATORS: &str = "build/src/generators/impls";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WikiLock {
    pub version: String,
    pub champions: BTreeMap<String, BTreeMap<Key, Vec<LockedAbility>>>,
    pub runes: BTreeMap<String, Vec<String>>,
    /// Effects of every item by slot, such as `pass`, `pass2.ranged` or `act`
    #[serde(default)]
    pub items: BTreeMap<String, BTreeMap<String, LockedItemEffect>>,
    /// Hash of the text of every champion exception by its name, see
    /// [`crate::champions::exceptions`]
    #[serde(default)]
    pub exceptions: BTreeMap<String, BTreeMap<String, String>>,
    /// Changes of every game mode by the code of its module and its key, as
    /// `"index: kind (hash)"`, see [`crate::map_changes`]
    #[serde(default)]
    pub map_changes: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    /// Hash of the stats that game modes change, by champion, see
    /// [`crate::champions::WikiChampion::map_stats`]
    #[serde(default)]
    pub map_stats: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LockedAbility {
    pub name: String,
    /// Effects as `"index: name (formula hash)"`
    pub effects: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LockedItemEffect {
    /// Hash of the formula, empty if the effect has none
    pub formula: String,
    /// Scalings as `"index: kind variable (hash)"`
    pub scalings: Vec<String>,
}

/// 64-bit FNV-1a, used because the standard library's hasher is not guaranteed
/// to produce the same output across Rust versions
fn fnv1a(data: &str) -> u64 {
    data.bytes().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x100000001b3)
    })
}

pub(crate) fn hash(data: &str) -> String {
    format!("{:08x}", fnv1a(data) as u32)
}

fn lock_effects<'a>(effects: impl Iterator<Item = (&'a String, &'a Effect)>) -> Vec<String> {
    effects
        .enumerate()
        .map(|(i, (name, effect))| {
            let formula = effect.formula.as_deref().unwrap_or_default();
            format!("{i}: {name} ({})", hash(formula))
        })
        .collect()
}

/// Describes each scaling by its kind and variable, such as `Simple BaseAd`,
/// followed by a hash of its values
fn lock_scalings(scalings: &[Scaling]) -> Vec<String> {
    scalings
        .iter()
        .enumerate()
        .map(|(i, scaling)| {
            let value = serde_json::to_value(scaling).unwrap_or_default();
            let (kind, fields) = match &value {
                Value::Object(map) => map
                    .iter()
                    .next()
                    .map_or(("Unknown", None), |(k, v)| (k.as_str(), Some(v))),
                Value::String(kind) => (kind.as_str(), None),
                _ => ("Unknown", None),
            };

            let var = fields
                .and_then(|f| f.get("ctx_var").or_else(|| f.get("level_var")))
                .and_then(Value::as_str)
                .map(|v| format!(" {v}"))
                .unwrap_or_default();

            format!("{i}: {kind}{var} ({})", hash(&value.to_string()))
        })
        .collect()
}

impl LockedItemEffect {
    fn new(effect: &Effect) -> Option<Self> {
        if effect.formula.is_none() && effect.scalings.is_empty() {
            return None;
        }

        Some(Self {
            formula: effect.formula.as_deref().map(hash).unwrap_or_default(),
            scalings: lock_scalings(&effect.scalings),
        })
    }
}

fn lock_item(item: &WikiItem) -> BTreeMap<String, LockedItemEffect> {
    let effects = &item.effects;
    let slots = [
        ("pass", &effects.pass),
        ("pass2", &effects.pass2),
        ("pass3", &effects.pass3),
        ("pass4", &effects.pass4),
        ("act", &effects.act),
        ("consume", &effects.consume),
    ];

    let mut result = BTreeMap::new();

    for (slot, item_effect) in slots {
        let Some(item_effect) = item_effect else {
            continue;
        };

        let variants = [
            ("", Some(&item_effect.effect)),
            (".ranged", item_effect.ranged.as_ref()),
            (".flip", item_effect.flip.as_ref()),
            (".total", item_effect.total.as_ref()),
        ];

        for (suffix, effect) in variants {
            if let Some(locked) = effect.and_then(LockedItemEffect::new) {
                result.insert(format!("{slot}{suffix}"), locked);
            }
        }
    }

    result
}

/// Splits `"index: name (hash)"` into `"index: name"` and its hash
pub(crate) fn split_entry(entry: &str) -> (&str, &str) {
    entry.rsplit_once(" (").unwrap_or((entry, ""))
}

/// Name and formula hash of an entry `"index: name (hash)"`
pub(crate) fn entry_name(entry: &str) -> (&str, &str) {
    let (left, hash) = split_entry(entry);
    let name = left.split_once(": ").map_or(left, |(_, name)| name);
    (name, hash.trim_end_matches(')'))
}

pub fn build() -> MayFail<WikiLock> {
    let champions = BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?;

    let map_stats = champions
        .iter()
        .filter(|(_, champion)| !champion.map_stats.is_empty())
        .map(|(champion_id, champion)| {
            let json = serde_json::to_string(&champion.map_stats)?;

            Ok((champion_id.clone(), hash(&json)))
        })
        .collect::<MayFail<_>>()?;

    let exceptions = champions
        .iter()
        .filter(|(_, champion)| !champion.exceptions.is_empty())
        .map(|(champion_id, champion)| {
            let texts = champion
                .exceptions
                .iter()
                .map(|(name, text)| (name.clone(), hash(text)))
                .collect();

            (champion_id.clone(), texts)
        })
        .collect();

    let champions = BTreeMap::<String, crate::wikitext::ChampionModel>::from_file(
        "cache/wiki/champions/model.json",
    )?
    .into_iter()
    .map(|(id, model)| {
        let abilities = model
            .abilities
            .into_iter()
            .map(|(key, abilities)| {
                let abilities = abilities
                    .into_iter()
                    .map(|ability| LockedAbility {
                        name: ability.name,
                        effects: ability
                            .effects
                            .iter()
                            .chain(ability.extras.iter())
                            .enumerate()
                            .map(|(i, effect)| {
                                format!(
                                    "{i}: {} ({})",
                                    effect.label,
                                    hash(&effect.formula.to_string())
                                )
                            })
                            .collect(),
                    })
                    .collect();

                (key, abilities)
            })
            .collect();

        (id, abilities)
    })
    .collect();

    let runes = BTreeMap::<String, WikiRune>::from_file("cache/wiki/runes/full.json")?
        .into_iter()
        .map(|(rune_id, rune)| (rune_id, lock_effects(rune.effects.iter())))
        .collect();

    let items = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?
        .into_iter()
        .map(|(item_id, item)| (item_id, lock_item(&item)))
        .filter(|(_, slots)| !slots.is_empty())
        .collect();

    let map_changes = BTreeMap::<String, MapChanges>::from_file(map_changes::full_path())?
        .into_iter()
        .map(|(code, changes)| {
            let entries = changes
                .entries
                .into_iter()
                .map(|(key, entry)| {
                    let changes = entry
                        .changes
                        .iter()
                        .enumerate()
                        .map(|(i, change)| {
                            format!("{i}: {} ({})", change.kind.name(), hash(&change.text))
                        })
                        .collect();

                    (key, changes)
                })
                .collect();

            (code, entries)
        })
        .collect();

    Ok(WikiLock {
        version: crate::lol_version().to_string(),
        champions,
        runes,
        items,
        exceptions,
        map_changes,
        map_stats,
    })
}

/// Compares two lists of entries, returning the ones whose index or name
/// changed, and how many kept their index and name but not their hash
pub fn compare(old: &[String], new: &[String]) -> (Vec<String>, usize) {
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

/// Lock file of the most recent version archived in `cache`, other than
/// `current`, which is what [`check`] compares against when it runs on its own
pub fn latest_archive(cache: impl AsRef<Path>, current: &str) -> Option<PathBuf> {
    let parse = |v: &str| -> Vec<u32> { v.split('.').filter_map(|p| p.parse().ok()).collect() };

    crate::read_dir(cache.as_ref())
        .ok()?
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| crate::version::is_version(name) && name != current)
        .map(|name| cache.as_ref().join(&name).join(FILE_NAME))
        .filter(|path| path.is_file())
        .max_by_key(|path| {
            path.parent()
                .and_then(Path::file_name)
                .and_then(|name| name.to_str())
                .map(parse)
                .unwrap_or_default()
        })
}

/// Regenerates the lock file from the parsed wiki data, then checks the
/// generators against what changed since the previous one
pub fn write() -> MayFail {
    println!("[fn] lock::write");

    let new = build()?;
    let old = WikiLock::from_file(path()).ok();

    let json = serde_json::to_string_pretty(&new)?;
    crate::write(path(), json + "\n")?;

    new.check(old.as_ref(), GENERATORS)?.print();
    Ok(())
}

/// Checks the generators against the current lock file, comparing it with the
/// lock file of the most recent archived version, if there is one
pub fn check() -> MayFail {
    println!("[fn] lock::check");

    let new = WikiLock::from_file(path())?;
    let old = match latest_archive("cache", &new.version) {
        Some(path) => {
            println!("[lock] Comparing with {path:?}");
            Some(WikiLock::from_file(path)?)
        }
        None => {
            println!("[lock] No archived lock file, only checking effect names");
            None
        }
    };

    new.check(old.as_ref(), GENERATORS)?.print();

    Ok(())
}
