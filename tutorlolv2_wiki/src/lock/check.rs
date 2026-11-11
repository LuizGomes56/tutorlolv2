//! Checks the generators against the changes between two lock files. Champion
//! generators select effects by name, so an effect renamed in the wiki is fixed
//! in their source when exactly one new effect has the same formula, and an
//! effect that no longer exists is reported as an error. Rune and item
//! generators select effects and scalings by index, so their changes are only
//! reported as warnings

use super::{LockedAbility, WikiLock, compare, entry_name, hash};
use crate::{
    MayFail,
    champions::exceptions::{EXCEPTIONS, Exception},
};
use regex::Regex;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use tutorlolv2_types::Key;

/// Matches `impl Generator for Name`, which identifies the generator of a file
static IMPL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"impl\s+Generator\s+for\s+(\w+)").unwrap());

/// Matches the start of `.ability(Key::Q, [` and `.ability_nth(1, Key::Q, [`
static CALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\.ability(?:_nth\(\s*(\d+)\s*,|\()\s*Key::([PQWER])\s*,\s*\[").unwrap()
});

/// Matches `.formula(Key::Q, "name"`, as well as `effect` and `scaling`, and
/// `.effect_nth(Key::Q, 1, "name"` as well as `scaling_nth`
static DIRECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"\.(?:formula|scaling|effect|effect_nth|scaling_nth)\(\s*Key::([PQWER])\s*,\s*(?:(\d+)\s*,\s*)?("(?:[^"\\]|\\.)*")"#,
    )
    .unwrap()
});

static STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap());

#[derive(Debug, Default)]
pub struct Report {
    /// Generators changed to use the new name of an effect
    pub fixes: Vec<String>,
    pub warnings: Vec<String>,
    /// Effects used by generators that no longer exist
    pub errors: Vec<String>,
}

impl Report {
    pub fn print(&self) {
        if self.fixes.is_empty() && self.warnings.is_empty() && self.errors.is_empty() {
            println!("[lock] Generators are consistent with the lock file");
        }

        for (tag, lines) in [
            ("fixed", &self.fixes),
            ("warning", &self.warnings),
            ("error", &self.errors),
        ] {
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

/// An effect selected by name in a champion generator
#[derive(Debug, PartialEq)]
pub struct EffectRef {
    pub key: String,
    pub nth: usize,
    pub name: String,
    /// Byte range of the string literal in the source, quotes included
    pub span: Range<usize>,
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

            if let Some(captures) = IMPL.captures(&source) {
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

    /// Every effect selected by name, skipping the ones in line comments
    pub fn effect_refs(&self) -> Vec<EffectRef> {
        let source = &self.source;
        let mut refs = Vec::new();

        for captures in CALL.captures_iter(source) {
            let nth = captures
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let key = &captures[2];
            let start = captures.get(0).map_or(0, |m| m.end());
            let end = self.closing_bracket(start);

            for literal in STRING.find_iter(&source[start..end]) {
                let span = start + literal.start()..start + literal.end();
                self.push_ref(&mut refs, key, nth, span);
            }
        }

        for captures in DIRECT.captures_iter(source) {
            let nth = captures
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if let Some(literal) = captures.get(3) {
                self.push_ref(&mut refs, &captures[1], nth, literal.range());
            }
        }

        refs.sort_by_key(|r| r.span.start);
        refs
    }

    fn push_ref(&self, refs: &mut Vec<EffectRef>, key: &str, nth: usize, span: Range<usize>) {
        let line_start = self.source[..span.start].rfind('\n').map_or(0, |i| i + 1);

        if self.source[line_start..span.start].contains("//") {
            return;
        }

        refs.push(EffectRef {
            key: key.to_string(),
            nth,
            name: unquote(&self.source[span.clone()]),
            span,
        });
    }

    /// Position of the `]` closing the array that starts at `start`, skipping
    /// brackets inside strings, such as in `"Innate [4]"`
    fn closing_bracket(&self, start: usize) -> usize {
        let bytes = self.source.as_bytes();
        let (mut depth, mut in_string, mut i) = (1, false, start);

        while i < bytes.len() {
            match (in_string, bytes[i]) {
                (true, b'\\') => i += 1,
                (_, b'"') => in_string = !in_string,
                (false, b'[') => depth += 1,
                (false, b']') => {
                    depth -= 1;
                    if depth == 0 {
                        return i;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        bytes.len()
    }

    /// Replaces each range of the source with its text and saves the file
    fn apply(&mut self, mut edits: Vec<(Range<usize>, String)>) -> MayFail {
        if edits.is_empty() {
            return Ok(());
        }

        edits.sort_by_key(|(span, _)| std::cmp::Reverse(span.start));

        for (span, text) in edits {
            self.source.replace_range(span, &text);
        }

        println!("[write] {:?}", self.path);
        crate::write(&self.path, &self.source)
    }
}

fn unquote(literal: &str) -> String {
    let inner = literal
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(literal);

    inner.replace("\\\"", "\"").replace("\\\\", "\\")
}

/// Changes between the effects of an ability in two lock files
#[derive(Debug, Default, PartialEq)]
pub struct AbilityDiff {
    /// Effects whose name changed, matched by having the same formula
    pub renames: BTreeMap<String, String>,
    pub removed: Vec<String>,
    pub added: Vec<String>,
    /// Effects that kept their name but not their formula
    pub formulas: usize,
}

impl LockedAbility {
    /// Name and formula hash of every effect
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.effects.iter().map(|entry| entry_name(entry))
    }

    pub fn has(&self, name: &str) -> bool {
        self.entries().any(|(n, _)| n == name)
    }

    /// Changes from `self` to `new`. A removed effect is renamed to an added
    /// one only if they are the only ones with that formula, and the formula
    /// is not empty, since many effects without formulas share its hash
    pub fn diff(&self, new: &LockedAbility) -> AbilityDiff {
        let old = self.entries().collect::<BTreeMap<_, _>>();
        let new = new.entries().collect::<BTreeMap<_, _>>();
        let empty = hash("");

        let removed = old
            .iter()
            .filter(|(name, _)| !new.contains_key(*name))
            .collect::<Vec<_>>();

        let added = new
            .iter()
            .filter(|(name, _)| !old.contains_key(*name))
            .collect::<Vec<_>>();

        let mut diff = AbilityDiff {
            formulas: old
                .iter()
                .filter(|(name, h)| new.get(*name).is_some_and(|n| n != *h))
                .count(),
            ..Default::default()
        };

        let count =
            |list: &[(&&str, &&str)], h: &str| list.iter().filter(|(_, x)| **x == h).count();

        for (name, h) in &removed {
            let target = added.iter().find(|(_, x)| *x == *h);

            match target {
                Some((target, _))
                    if **h != empty && count(&removed, h) == 1 && count(&added, h) == 1 =>
                {
                    diff.renames.insert(name.to_string(), target.to_string());
                }
                _ => diff.removed.push(name.to_string()),
            }
        }

        let targets = diff.renames.values().cloned().collect::<BTreeSet<_>>();

        diff.added = added
            .iter()
            .map(|(name, _)| name.to_string())
            .filter(|name| !targets.contains(name))
            .collect();

        diff
    }
}

fn parse_key(key: &str) -> Option<Key> {
    serde_json::from_value(Value::String(key.to_string())).ok()
}

impl WikiLock {
    /// Checks the generators in `generators` against `self`, the newest lock
    /// file. When `old` is given, renamed effects are fixed in champion
    /// generators, and changes to runes and items with generators are reported
    pub fn check(&self, old: Option<&WikiLock>, generators: impl AsRef<Path>) -> MayFail<Report> {
        let generators = generators.as_ref();
        let mut report = Report::default();

        self.check_champions(old, &generators.join("champions"), &mut report)?;
        self.check_exceptions(old, &mut report);

        if let Some(old) = old {
            self.check_runes(old, &generators.join("runes"), &mut report)?;
            self.check_items(old, &generators.join("items"), &mut report)?;
        }

        Ok(report)
    }

    fn check_champions(&self, old: Option<&WikiLock>, dir: &Path, report: &mut Report) -> MayFail {
        let empty = Vec::new();
        let mut formulas = Vec::new();

        if let Some(old) = old {
            for champion_id in self.champions.keys() {
                if !old.champions.contains_key(champion_id) {
                    report.warnings.push(format!("{champion_id}: new champion"));
                }
            }
        }

        for mut file in GeneratorFile::read_dir(dir)? {
            let id = file.id.clone();

            let Some(abilities) = self.champions.get(&id) else {
                report.warnings.push(format!(
                    "{id}: the generator has no champion in the lock file"
                ));
                continue;
            };

            let old_abilities = old.and_then(|old| old.champions.get(&id));
            let old_ability = |key: Key, nth: usize| {
                old_abilities
                    .and_then(|abilities| abilities.get(&key))
                    .and_then(|list| list.get(nth))
            };

            let refs = file.effect_refs();
            let mut used = BTreeSet::new();
            let mut edits = Vec::new();

            for effect in &refs {
                let label = format!("{id} {}[{}]", effect.key, effect.nth);
                let key = parse_key(&effect.key);

                let Some((key, ability)) = key.and_then(|key| {
                    let ability = abilities.get(&key)?.get(effect.nth)?;
                    Some((key, ability))
                }) else {
                    report.errors.push(format!(
                        "{label}: the ability no longer exists, used for effect {:?}",
                        effect.name
                    ));
                    continue;
                };

                used.insert((key, effect.nth, effect.name.clone()));

                if ability.has(&effect.name) {
                    continue;
                }

                let renamed = old_ability(key, effect.nth)
                    .and_then(|old| old.diff(ability).renames.remove(&effect.name));

                match renamed {
                    Some(new_name) => {
                        report.fixes.push(format!(
                            "{label}: {:?} renamed to {new_name:?}",
                            effect.name
                        ));
                        edits.push((effect.span.clone(), format!("{new_name:?}")));
                    }
                    None => {
                        let available = ability
                            .entries()
                            .map(|(name, _)| format!("{name:?}"))
                            .collect::<Vec<_>>()
                            .join(", ");

                        report.errors.push(format!(
                            "{label}: effect {:?} no longer exists. Available: [{available}]",
                            effect.name
                        ));
                    }
                }
            }

            file.apply(edits)?;

            let Some(old_abilities) = old_abilities else {
                continue;
            };

            let mut changed = 0;

            for (key, list) in abilities {
                let old_list = old_abilities.get(key).unwrap_or(&empty);

                if list.len() != old_list.len() {
                    report.warnings.push(format!(
                        "{id} {key:?}: {} abilities, previously {}",
                        list.len(),
                        old_list.len()
                    ));
                }

                for (nth, (ability, old)) in list.iter().zip(old_list).enumerate() {
                    let label = format!("{id} {key:?}[{nth}]");
                    let diff = old.diff(ability);
                    let is_used = |name: &str| used.contains(&(*key, nth, name.to_string()));

                    for (from, to) in &diff.renames {
                        if !is_used(from) && !is_used(to) {
                            report
                                .warnings
                                .push(format!("{label}: unused effect {from:?} renamed to {to:?}"));
                        }
                    }

                    for name in diff.removed.iter().filter(|name| !is_used(name)) {
                        report
                            .warnings
                            .push(format!("{label}: unused effect {name:?} was removed"));
                    }

                    for name in &diff.added {
                        report
                            .warnings
                            .push(format!("{label}: new effect {name:?}"));
                    }

                    changed += diff.formulas;
                }
            }

            for key in old_abilities
                .keys()
                .filter(|key| !abilities.contains_key(*key))
            {
                report
                    .warnings
                    .push(format!("{id} {key:?}: the abilities were removed"));
            }

            if changed > 0 {
                formulas.push(format!("{id} ({changed})"));
            }
        }

        if !formulas.is_empty() {
            report.warnings.push(format!(
                "Champion formulas changed: {}",
                formulas.join(", ")
            ));
        }

        Ok(())
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
