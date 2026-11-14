//! Validates structured champion label selections and reports changes to the
//! named item/rune raw sources and shared wiki metadata.

use super::{LockedAbility, WikiLock, compare, entry_name};
use crate::{
    MayFail,
    champions::exceptions::{EXCEPTIONS, Exception},
    map_changes::change::ChangeKind,
};
use lazy_regex::regex;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};
use types::Key;

#[derive(Debug, Default)]
pub struct Report {
    pub warnings: Vec<String>,
    /// Effects used by generators that no longer exist
    pub errors: Vec<String>,
}

impl Report {
    pub fn print(&self) {
        if self.warnings.is_empty() && self.errors.is_empty() {
            println!("[lock] Generators are consistent with the lock file");
        }

        for (tag, lines) in [("warning", &self.warnings), ("error", &self.errors)] {
            for line in lines {
                println!("[lock] {tag}: {line}");
            }
        }
    }
}

/// A generator file, identified by the type it implements `Generator` for
#[derive(Debug)]
pub struct GeneratorFile {
    pub path: PathBuf,
    pub id: String,
    pub source: String,
}

impl GeneratorFile {
    /// Reads every generator in `dir`, sorted by id. Files that do not
    /// implement `Generator` are skipped
    pub fn read_dir(dir: impl AsRef<Path>) -> MayFail<Vec<Self>> {
        let dir = dir.as_ref();

        if !dir.try_exists()? {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();

        for entry in crate::read_dir(dir)? {
            let path = entry.path();

            if path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }

            let source = crate::read_to_string(&path)?;

            // Matches `impl Generator for Name`, which identifies the generator of a file
            if let Some(captures) =
                regex!(r"impl\s+Generator(?:V2)?\s+for\s+(\w+)").captures(&source)
            {
                files.push(Self {
                    id: captures[1].to_string(),
                    path,
                    source,
                });
            }
        }

        files.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(files)
    }
}

fn unquote(literal: &str) -> String {
    let inner = literal
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(literal);

    inner.replace("\\\"", "\"").replace("\\\\", "\\")
}

impl LockedAbility {
    /// Name and formula hash of every structured effect.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.effects.iter().map(|entry| entry_name(entry))
    }
}

/// Match stable structured identities, retaining separate occurrences of repeated labels.
fn champion_formulas(
    abilities: &BTreeMap<Key, Vec<LockedAbility>>,
) -> BTreeMap<(Key, String, String, usize), String> {
    let mut formulas = BTreeMap::new();
    let mut occurrences = BTreeMap::new();

    for (key, pages) in abilities {
        for ability in pages {
            for (label, hash) in ability.entries() {
                let identity = (*key, ability.name.clone(), label.to_string());
                let occurrence = occurrences.entry(identity.clone()).or_insert(0);

                formulas.insert(
                    (identity.0, identity.1, identity.2, *occurrence),
                    hash.to_string(),
                );

                *occurrence += 1;
            }
        }
    }

    formulas
}

fn parse_key(key: &str) -> Option<Key> {
    serde_json::from_value(Value::String(key.to_string())).ok()
}

impl WikiLock {
    /// Checks the generators in `generators` against `self`, the newest lock
    /// file. When `old` is given, formula changes and item/rune index changes
    /// are reported for entities with generators.
    pub fn check(&self, old: Option<&WikiLock>, generators: impl AsRef<Path>) -> MayFail<Report> {
        let generators = generators.as_ref();
        let mut report = Report::default();

        let v2 = if generators == Path::new(super::GENERATORS) {
            PathBuf::from(super::CHAMPION_GENERATORS)
        } else {
            generators.join("champions")
        };

        self.check_champions(old, &v2, &mut report)?;
        self.check_exceptions(old, &mut report);
        self.check_map_changes(old, &mut report);
        self.check_map_stats(old, &mut report);
        self.check_native_sources(old, &mut report);

        if let Some(old) = old {
            self.check_runes(old, &generators.join("runes"), &mut report)?;
            self.check_items(old, &generators.join("items"), &mut report)?;
        }

        Ok(report)
    }

    fn check_native_sources(&self, old: Option<&WikiLock>, report: &mut Report) {
        let Some(old) = old else { return };

        if !old.rune_sources.is_empty() {
            for id in old
                .rune_sources
                .keys()
                .chain(self.rune_sources.keys())
                .collect::<BTreeSet<_>>()
            {
                if old.rune_sources.get(id) != self.rune_sources.get(id) {
                    report
                        .warnings
                        .push(format!("[runes] {id}: raw source changed"));
                }
            }
        }

        if !old.item_sources.is_empty() {
            for id in old
                .item_sources
                .keys()
                .chain(self.item_sources.keys())
                .collect::<BTreeSet<_>>()
            {
                let before = old.item_sources.get(id);
                let after = self.item_sources.get(id);
                let names = before
                    .into_iter()
                    .chain(after)
                    .flat_map(BTreeMap::keys)
                    .collect::<BTreeSet<_>>();

                for name in names {
                    if before.and_then(|map| map.get(name)) != after.and_then(|map| map.get(name)) {
                        report
                            .warnings
                            .push(format!("[items] {id}/{name}: raw source changed"));
                    }
                }
            }
        }
    }

    fn check_champions(&self, old: Option<&WikiLock>, dir: &Path, report: &mut Report) -> MayFail {
        for file in GeneratorFile::read_dir(dir)? {
            let Some(abilities) = self.champions.get(&file.id) else {
                report.errors.push(format!(
                    "{}: the generator has no champion in the lock file",
                    file.id
                ));
                continue;
            };

            if let Some(previous) = old.and_then(|old| old.champions.get(&file.id)) {
                let previous = champion_formulas(previous);
                let current = champion_formulas(abilities);
                let changed = current
                    .iter()
                    .filter(|(id, hash)| {
                        previous.get(*id).is_some_and(|old_hash| old_hash != *hash)
                    })
                    .count();

                if changed > 0 {
                    report
                        .warnings
                        .push(format!("{}: {changed} champion formulas changed", file.id));
                }
            }

            // take/value select across all pages by label, with an optional occurrence
            // or ability name, rather than the old ability-vector index.
            for caps in regex!(
                r#"\.(take(?:_nth|_in)?|value(?:_nth)?)\(\s*("(?:[^"\\]|\\.)*")\s*,\s*(?:("(?:[^"\\]|\\.)*")\s*,\s*|(\d+)\s*,\s*)?(?:Key::|AbilityId::)?([PQWER])\s*(?:\(|[,\)])"#
            ).captures_iter(&file.source) {
                let literal = caps.get(2).unwrap();
                let line_start = file.source[..literal.start()]
                    .rfind('\n')
                    .map_or(0, |i| i + 1);

                if file.source[line_start..literal.start()].contains("//") {
                    continue;
                }

                let in_ability = (&caps[1] == "take_in").then(|| unquote(&caps[2]));
                let label = unquote(caps.get(3).map_or(&caps[2], |m| m.as_str()));
                let nth = caps.get(4).and_then(|m| m.as_str().parse::<usize>().ok());
                let key = parse_key(&caps[5]).unwrap();
                let count = abilities
                    .get(&key)
                    .into_iter()
                    .flatten()
                    .filter(|ability| in_ability.as_ref().is_none_or(|name| *name == ability.name))
                    .flat_map(|ability| ability.entries())
                    .filter(|(name, _)| *name == label)
                    .count();

                if !match nth {
                    Some(n) => n < count,
                    None => count == 1,
                } {
                    report.errors.push(format!(
                        "{} {key:?}: selection {label:?} matches {count} effects{}",
                        file.id,
                        nth.map(|n| format!(", requested occurrence {n}"))
                            .unwrap_or_default()
                    ));
                }
            }
        }

        Ok(())
    }

    fn check_map_stats(&self, old: Option<&WikiLock>, report: &mut Report) {
        // Lock files written before map stats were tracked have none to compare
        let Some(old) = old.map(|old| &old.map_stats).filter(|old| !old.is_empty()) else {
            return;
        };

        for champion_id in old
            .keys()
            .chain(self.map_stats.keys())
            .collect::<BTreeSet<_>>()
        {
            match (old.get(champion_id), self.map_stats.get(champion_id)) {
                (Some(a), Some(b)) if a != b => report.warnings.push(format!(
                    "{champion_id}: the stats that game modes change changed"
                )),
                (Some(_), None) => report.warnings.push(format!(
                    "{champion_id}: game modes no longer change its stats"
                )),
                (None, Some(_)) => report
                    .warnings
                    .push(format!("{champion_id}: game modes now change its stats")),
                _ => {}
            }
        }
    }

    /// Changes of the game modes are only warnings: the damage-relevant ones
    /// are counted, and the ones that were added, edited or removed since the
    /// old lock file are listed. Which ones are applied depends on the
    /// generators, so the report lists them
    fn check_map_changes(&self, old: Option<&WikiLock>, report: &mut Report) {
        let empty = Vec::new();

        for (code, entries) in &self.map_changes {
            let relevant = entries
                .values()
                .flatten()
                .filter(|entry| ChangeKind::is_damage_relevant(entry_name(entry).0))
                .count();

            if relevant > 0 {
                report.warnings.push(format!(
                    "{code}: {relevant} damage-relevant changes, see `wiki map-changes-report` for the ones that are not applied"
                ));
            }

            // Lock files written before map changes were tracked have none to compare
            let Some(old_entries) = old.and_then(|old| old.map_changes.get(code)) else {
                continue;
            };

            for key in old_entries
                .keys()
                .chain(entries.keys())
                .collect::<BTreeSet<_>>()
            {
                let label = format!("{code} {key}");

                match (old_entries.get(key), entries.get(key)) {
                    (Some(_), None) => report.warnings.push(format!("{label}: changes removed")),
                    (None, Some(_)) => report.warnings.push(format!("{label}: new changes")),
                    (old, new) => {
                        let (lines, edited) = compare(old.unwrap_or(&empty), new.unwrap_or(&empty));

                        if !lines.is_empty() {
                            report.warnings.push(format!(
                                "{label}: changes added or removed\n{}",
                                lines.join("\n")
                            ));
                        }

                        if edited > 0 {
                            report
                                .warnings
                                .push(format!("{label}: {edited} changes edited"));
                        }
                    }
                }
            }
        }
    }

    /// Exceptions are only read by text patterns in their generators, so a changed
    /// text is a warning to review them, and a missing one breaks the build
    fn check_exceptions(&self, old: Option<&WikiLock>, report: &mut Report) {
        let empty = BTreeMap::new();

        for Exception {
            champion_id, name, ..
        } in EXCEPTIONS
        {
            // Champions that are not in the lock file have no generator to read it
            if !self.champions.contains_key(*champion_id) {
                continue;
            }

            let label = format!("{champion_id} exception {name:?}");
            let texts = self.exceptions.get(*champion_id).unwrap_or(&empty);

            let Some(new) = texts.get(*name) else {
                report.errors.push(format!(
                    "{label}: missing from the lock file, its generator can't read it"
                ));
                continue;
            };

            // Lock files written before exceptions were tracked have none to compare
            let Some(old) = old.and_then(|old| old.exceptions.get(*champion_id)) else {
                continue;
            };

            match old.get(*name) {
                Some(old) if old != new => report.warnings.push(format!(
                    "{label}: the text changed, check the values its generator captures"
                )),
                Some(_) => {}
                None => report.warnings.push(format!("{label}: new exception")),
            }
        }
    }

    fn check_runes(&self, old: &WikiLock, dir: &Path, report: &mut Report) -> MayFail {
        for file in GeneratorFile::read_dir(dir)? {
            let id = &file.id;

            let (Some(old_effects), new_effects) = (old.runes.get(id), self.runes.get(id)) else {
                continue;
            };

            let Some(new_effects) = new_effects else {
                report
                    .warnings
                    .push(format!("{id}: the rune is no longer in the lock file"));
                continue;
            };

            let (lines, formulas) = compare(old_effects, new_effects);

            if !lines.is_empty() {
                report.warnings.push(format!(
                    "{id}: rune effect indexes changed, update the indexes its generator selects\n{}",
                    lines.join("\n")
                ));
            }

            if formulas > 0 {
                report
                    .warnings
                    .push(format!("{id}: {formulas} rune formulas changed"));
            }
        }

        Ok(())
    }

    fn check_items(&self, old: &WikiLock, dir: &Path, report: &mut Report) -> MayFail {
        let empty = BTreeMap::new();

        for file in GeneratorFile::read_dir(dir)? {
            let id = &file.id;

            // Lock files written before items were tracked have none to compare
            let Some(old_slots) = old.items.get(id) else {
                continue;
            };

            let new_slots = self.items.get(id).unwrap_or(&empty);
            let slots = old_slots
                .keys()
                .chain(new_slots.keys())
                .collect::<BTreeSet<_>>();

            for slot in slots {
                let label = format!("{id} {slot}");

                let (old, new) = match (old_slots.get(slot), new_slots.get(slot)) {
                    (Some(old), Some(new)) => (old, new),
                    (Some(_), None) => {
                        report
                            .warnings
                            .push(format!("{label}: the effect was removed"));
                        continue;
                    }
                    (None, _) => {
                        report.warnings.push(format!("{label}: new effect"));
                        continue;
                    }
                };

                if old.formula != new.formula {
                    report
                        .warnings
                        .push(format!("{label}: the formula changed"));
                }

                let (lines, values) = compare(&old.scalings, &new.scalings);

                if !lines.is_empty() {
                    report.warnings.push(format!(
                        "{label}: scalings changed, update the indexes its generator selects\n{}",
                        lines.join("\n")
                    ));
                }

                if values > 0 {
                    report
                        .warnings
                        .push(format!("{label}: {values} scaling values changed"));
                }
            }
        }

        Ok(())
    }
}
