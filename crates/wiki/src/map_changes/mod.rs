//! Changes that game modes make to champions, items and runes, read from
//! `Module:MapChanges/data/<mode>` with every patch. See
//! `docs/map-formula-overrides-spec.md`

pub mod change;
pub mod overrides;

use crate::{
    HTTP_CLIENT, JsonRead, MayFail,
    champions::WikiChampion,
    items::item_parser::WikiItem,
    lock::{self, check::GeneratorFile},
    runes::WikiRune,
};
use change::{ChangeKind, MapChange};
use lazy_regex::regex;
use mlua::{Lua, LuaSerdeExt, Value};
use overrides::{EffectOverrides, Outcome};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use types::{GameMap, Key};

/// A data module of `Module:MapChanges` and the maps it applies to
pub struct MapSource {
    /// Name of the module, as in `Module:MapChanges/data/aram`
    pub code: &'static str,
    pub maps: &'static [GameMap],
}

/// Modes whose changes are downloaded. ARAM: Mayhem has no module and uses
/// ARAM's. The event modes are left out for now
pub const MAPS: &[MapSource] = &[
    MapSource {
        code: "aram",
        maps: &[GameMap::Aram, GameMap::AramMayhem],
    },
    MapSource {
        code: "ar",
        maps: &[GameMap::Arena],
    },
];

/// Summoner spells, whose changes the engine doesn't model
const SUMMONER_SPELLS: &[&str] = &[
    "Barrier", "Cleanse", "Clarity", "Exhaust", "Flash", "Ghost", "Heal", "Ignite", "Mark",
    "Smite", "Teleport",
];

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/map_changes")
}

/// Changes of every mode in [`MAPS`], by the code of its module
pub fn full_path() -> PathBuf {
    cache().join("full").with_extension("json")
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MapChanges {
    pub maps: Vec<GameMap>,
    /// Changes by the key of the module, as `"Akali Q"` or `"Hubris"`
    pub entries: BTreeMap<String, MapEntry>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MapEntry {
    pub target: ChangeTarget,
    pub changes: Vec<MapChange>,
}

/// What the key of an entry names
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChangeTarget {
    /// `"Azir"`
    Champion {
        champion_id: String,
    },
    /// `"Akali Q"`, where `I` is [`Key::P`]
    Ability {
        champion_id: String,
        key: Key,
    },
    /// `"Nidalee Takedown"`, with its key when the name is one of its abilities
    NamedAbility {
        champion_id: String,
        name: String,
        key: Option<Key>,
    },
    Item {
        item_id: String,
    },
    Rune {
        rune_id: String,
    },
    SummonerSpell,
    /// Didn't match a champion, item, rune or summoner spell
    Unresolved,
}

pub async fn run() -> MayFail {
    download().await?;
    parse()
}

pub async fn download() -> MayFail {
    println!("[fn] map_changes::download");

    // These modules are rendered with syntax highlighting, unlike `Module:ItemData/data`,
    // so their source is downloaded as is
    for MapSource { code, .. } in MAPS {
        HTTP_CLIENT
            .fetch(
                cache().join(code).with_extension("lua"),
                format!("Module:MapChanges/data/{code}?action=raw"),
            )
            .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] map_changes::parse");

    let targets = Targets::load()?;
    let mut full = BTreeMap::new();

    for MapSource { code, maps } in MAPS {
        let source = crate::read_to_string(cache().join(code).with_extension("lua"))?;
        let lua = Lua::new();
        let value = lua
            .load(&source)
            .eval::<Value>()
            .map_err(|e| format!("[map_changes] Failed to eval {code}: {e:?}"))?;

        let raw = lua
            .from_value::<BTreeMap<String, String>>(value)
            .map_err(|e| format!("[map_changes] Failed to read {code}: {e:?}"))?;

        let entries = raw
            .into_iter()
            .map(|(key, body)| {
                let target = targets.resolve(&key);
                let is_item = matches!(target, ChangeTarget::Item { .. });
                let changes = MapChange::parse_list(&body, is_item);

                (key, MapEntry { target, changes })
            })
            .collect();

        let changes = MapChanges {
            maps: maps.to_vec(),
            entries,
        };

        crate::write(
            cache().join(code).with_extension("json"),
            serde_json::to_string_pretty(&changes)?,
        )?;
        full.insert(code.to_string(), changes);
    }

    crate::write(full_path(), serde_json::to_string_pretty(&full)?)
}

/// Names of the champions, items and runes, to resolve the keys of the modules
struct Targets {
    /// Longest names first, so "Aurelion Sol" is tried before shorter names
    champions: Vec<(String, WikiChampion)>,
    models: BTreeMap<String, crate::wikitext::ChampionModel>,
    items: BTreeMap<String, String>,
    runes: BTreeMap<String, String>,
}

impl Targets {
    fn load() -> MayFail<Self> {
        let mut champions =
            BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?
                .into_values()
                .map(|champion| (champion.name.clone(), champion))
                .collect::<Vec<_>>();

        champions.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));

        let items = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?
            .into_values()
            .map(|item| (item.name, item.item_id))
            .collect();

        let runes = BTreeMap::<String, WikiRune>::from_file("cache/wiki/runes/full.json")?
            .into_values()
            .map(|rune| (rune.name, rune.rune_id))
            .collect();

        let models = BTreeMap::from_file("cache/wiki/champions/model.json")?;

        Ok(Self {
            models,
            champions,
            items,
            runes,
        })
    }

    fn resolve(&self, key: &str) -> ChangeTarget {
        if let Some(item_id) = self.items.get(key) {
            return ChangeTarget::Item {
                item_id: item_id.clone(),
            };
        }

        if let Some(rune_id) = self.runes.get(key) {
            return ChangeTarget::Rune {
                rune_id: rune_id.clone(),
            };
        }

        if SUMMONER_SPELLS.contains(&key) {
            return ChangeTarget::SummonerSpell;
        }

        for (name, champion) in &self.champions {
            let champion_id = champion.champion_id.clone();

            if key == name {
                return ChangeTarget::Champion { champion_id };
            }

            let Some(rest) = key
                .strip_prefix(name.as_str())
                .and_then(|r| r.strip_prefix(' '))
            else {
                continue;
            };

            // The last word is the key, also when the module uses a longer name of
            // the champion, as "Nunu & Willump W" for Nunu
            let key = match rest.rsplit(' ').next().unwrap_or(rest) {
                "I" | "P" => Some(Key::P),
                "Q" => Some(Key::Q),
                "W" => Some(Key::W),
                "E" => Some(Key::E),
                "R" => Some(Key::R),
                _ => None,
            };

            return match key {
                Some(key) => ChangeTarget::Ability { champion_id, key },
                None => ChangeTarget::NamedAbility {
                    key: self.models.get(&champion_id).and_then(|model| {
                        model.abilities.iter().find_map(|(key, abilities)| {
                            abilities
                                .iter()
                                .any(|a| a.name.eq_ignore_ascii_case(rest))
                                .then_some(*key)
                        })
                    }),
                    name: rest.to_string(),
                    champion_id,
                },
            };
        }

        ChangeTarget::Unresolved
    }
}

/// Effects that generators select, read from their sources as `lock-check`
/// does. The build knows them exactly; the report uses these to run the same
/// rules without building
#[derive(Debug, Default)]
pub struct Selected {
    /// Slots of each item, as `pass2`
    pub items: BTreeMap<String, Vec<&'static str>>,
    /// What each generator reads of the changes, by the id of its entity
    pub generators: BTreeMap<String, GeneratorReads>,
}

/// Changes that a generator reads with `map_values`, `map_texts` and
/// `map_formulas`, from its source
#[derive(Debug, Default)]
pub struct GeneratorReads {
    /// Parts of `map_values("…")`
    pub parts: Vec<String>,
    /// Every `regex!` of the generator. Its description patterns don't match
    /// changes, so the ones that do are the ones `map_texts` reads
    pub patterns: Vec<lazy_regex::Regex>,
    /// Item slots of `map_formulas(…)`, as `pass`
    pub formula_slots: Vec<&'static str>,
    /// Whether it assigns damages, rather than only declaring constants
    pub assigns_damage: bool,
}

impl GeneratorReads {
    fn new(source: &str) -> Self {
        Self {
            parts: regex!(r#"map_values\(\s*"([^"]*)""#)
                .captures_iter(source)
                .map(|c| c[1].to_string())
                .collect(),
            patterns: regex!(r#"regex!\(\s*r"([^"]*)""#)
                .captures_iter(source)
                .filter_map(|c| lazy_regex::Regex::new(&c[1]).ok())
                .collect(),
            formula_slots: regex!(r"map_(?:formulas|effects_for)\(\s*(Active|Passive[234]?|Consume)\b")
                .captures_iter(source)
                .map(|c| slot_of(&c[1]))
                .collect(),
            assigns_damage: regex!(r"\.(?:min|max|damage|assign|set_min|set_max|min_with|max_with|assign_with|assign_min|assign_max|compose)\(")
                .is_match(source),
        }
    }

    /// Whether the generator reads `change` of its entity
    fn reads(&self, change: &MapChange) -> bool {
        let part = change.kind.part_amount().map(|(part, _)| part);

        part.is_some_and(|part| self.parts.iter().any(|p| p.eq_ignore_ascii_case(part)))
            || self.patterns.iter().any(|p| p.is_match(&change.text))
    }
}

/// Key in `Module:ItemData` of a `Source` of the build, as `pass2`
fn slot_of(source: &str) -> &'static str {
    match source {
        "Active" => "act",
        "Passive" => "pass",
        "Passive2" => "pass2",
        "Passive3" => "pass3",
        "Passive4" => "pass4",
        _ => "consume",
    }
}

fn selected_effects() -> MayFail<Selected> {
    let mut generators = BTreeMap::new();

    for tag in ["champions", "items", "runes"] {
        for file in GeneratorFile::read_dir(Path::new(lock::GENERATORS).join(tag))? {
            generators.insert(file.id.clone(), GeneratorReads::new(&file.source));
        }
    }

    let items = GeneratorFile::read_dir(Path::new(lock::GENERATORS).join("items"))?
        .into_iter()
        .map(|file| {
            let mut slots = regex!(r"\b(Active|Passive[234]?|Consume)\b")
                .find_iter(&file.source)
                .map(|m| slot_of(m.as_str()))
                .collect::<Vec<_>>();

            slots.sort_unstable();
            slots.dedup();

            (file.id, slots)
        })
        .collect();

    Ok(Selected { items, generators })
}

impl MapChanges {
    /// Outcome of every change of a mode, by key and index of the change
    pub fn outcomes(
        &self,
        code: &str,
        _champions: &BTreeMap<String, WikiChampion>,
        items: &BTreeMap<String, WikiItem>,
        runes: &BTreeMap<String, WikiRune>,
        selected: &Selected,
    ) -> BTreeMap<(String, usize), Outcome> {
        let mut outcomes = BTreeMap::new();

        for (key, entry) in &self.entries {
            // Effects of the item slots its generator names, or every rune effect
            let effects = match &entry.target {
                ChangeTarget::Item { item_id } if let Some(item) = items.get(item_id) => {
                    let slots = selected
                        .items
                        .get(item_id)
                        .map(Vec::as_slice)
                        .unwrap_or_default();

                    item.slot_effects()
                        .filter(|(slot, ..)| slots.contains(slot))
                        .map(|(slot, name, effect)| (slot.to_string(), (name, effect.clone())))
                        .collect()
                }
                ChangeTarget::Rune { rune_id } if let Some(rune) = runes.get(rune_id) => rune
                    .effects
                    .iter()
                    .filter(|(_, e)| e.formula.as_deref().is_some_and(|f| f != crate::ZERO))
                    .map(|(name, e)| (name.clone(), (name.clone(), e.clone())))
                    .collect(),
                _ => BTreeMap::new(),
            };

            let effect_outcomes = EffectOverrides::new(&entry.target, self, &effects, false)
                .outcomes
                .into_iter()
                .map(|(_, i, outcome)| (i, outcome))
                .collect::<BTreeMap<_, _>>();

            for (i, change) in entry.changes.iter().enumerate() {
                let relevant = ChangeKind::is_damage_relevant(change.kind.name());
                let outcome = match &entry.target {
                    ChangeTarget::Item { item_id }
                        if let Some(item) = items.get(item_id)
                            && matches!(
                                change.kind,
                                ChangeKind::ItemStat { .. } | ChangeKind::Disabled
                            ) =>
                    {
                        change.item_outcome(item, code)
                    }
                    ChangeTarget::Champion { .. }
                    | ChangeTarget::Ability { .. }
                    | ChangeTarget::NamedAbility { .. } => {
                        Outcome::Manual("see structured model map changes report")
                    }
                    _ if !relevant => Outcome::NotRelevant,
                    ChangeTarget::Item { .. } | ChangeTarget::Rune { .. } => effect_outcomes
                        .get(&i)
                        .cloned()
                        .unwrap_or(Outcome::Manual("it needs a generator")),
                    ChangeTarget::SummonerSpell => Outcome::NotRelevant,
                    _ => Outcome::Manual("its key names nothing known"),
                };

                outcomes.insert((key.clone(), i), outcome);
            }
        }

        // Changes that generators read for their constants
        for (key, entry) in &self.entries {
            let id = match &entry.target {
                ChangeTarget::Champion { champion_id }
                | ChangeTarget::Ability { champion_id, .. }
                | ChangeTarget::NamedAbility { champion_id, .. } => champion_id,
                ChangeTarget::Item { item_id } => item_id,
                ChangeTarget::Rune { rune_id } => rune_id,
                _ => continue,
            };

            let Some(reads) = selected.generators.get(id) else {
                continue;
            };

            for (i, change) in entry.changes.iter().enumerate() {
                let Some(outcome) = outcomes.get_mut(&(key.clone(), i)) else {
                    continue;
                };

                if reads.reads(change) {
                    *outcome = Outcome::Applied {
                        effects: vec!["constants".into()],
                        rule: "its generator",
                    };
                    continue;
                }

                // An item that only declares constants applies a slot's changes only
                // through its constants, read with `map_formulas`
                if let (ChangeTarget::Item { .. }, Outcome::Applied { effects, .. }) =
                    (&entry.target, &*outcome)
                    && !reads.assigns_damage
                    && !matches!(change.kind, ChangeKind::ItemStat { .. })
                {
                    *outcome = match effects
                        .iter()
                        .all(|slot| reads.formula_slots.contains(&slot.as_str()))
                    {
                        true => Outcome::Applied {
                            effects: vec!["constants".into()],
                            rule: "its generator",
                        },
                        false => {
                            Outcome::Manual("its generator reads the slot for a constant only")
                        }
                    };
                }
            }
        }

        outcomes
    }
}

/// Prints, for each mode, what its changes are, which ones are applied, and
/// why the others are not
pub fn report() -> MayFail {
    let full = BTreeMap::<String, MapChanges>::from_file(full_path())?;
    let champions = BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?;
    let items = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?;
    let runes = BTreeMap::<String, WikiRune>::from_file("cache/wiki/runes/full.json")?;
    let selected = selected_effects()?;

    for (code, changes) in &full {
        let all = changes
            .entries
            .iter()
            .flat_map(|(key, entry)| entry.changes.iter().map(move |c| (key, entry, c)))
            .collect::<Vec<_>>();

        println!("== {code} {:?}", changes.maps);
        println!(
            "   {} entries, {} changes, {} marked as bugs",
            changes.entries.len(),
            all.len(),
            all.iter().filter(|(_, _, c)| c.has_bug).count()
        );

        let mut kinds = BTreeMap::<&str, usize>::new();
        let mut ignored = BTreeMap::<&str, usize>::new();

        for (_, _, change) in &all {
            *kinds.entry(change.kind.name()).or_default() += 1;

            if let ChangeKind::Ignored { keyword } = &change.kind {
                *ignored.entry(keyword).or_default() += 1;
            }
        }

        println!("   kinds: {kinds:?}");
        println!("   ignored by keyword: {ignored:?}");

        let unresolved = changes
            .entries
            .iter()
            .filter(|(_, e)| e.target == ChangeTarget::Unresolved)
            .map(|(key, _)| key.as_str())
            .collect::<Vec<_>>();

        if !unresolved.is_empty() {
            println!("   unresolved keys: {unresolved:?}");
        }

        let outcomes = changes.outcomes(code, &champions, &items, &runes, &selected);
        let mut summary = BTreeMap::<String, usize>::new();
        let mut lines = BTreeMap::<String, Vec<String>>::new();

        for ((key, i), outcome) in &outcomes {
            let change = &changes.entries[key].changes[*i];
            let (group, detail) = match outcome {
                Outcome::Applied { effects, rule } => {
                    (format!("applied: by {rule}"), effects.join(", "))
                }
                Outcome::AlreadyApplied(reason) => {
                    ("already applied".to_string(), reason.to_string())
                }
                Outcome::Manual(reason) => (format!("not applied: {reason}"), String::new()),
                Outcome::NotRelevant => {
                    *summary.entry("not relevant".into()).or_default() += 1;
                    continue;
                }
            };

            *summary.entry(group.clone()).or_default() += 1;
            lines.entry(group).or_default().push(format!(
                "{key} [{}] {}{}",
                change.kind.name(),
                change.text,
                match detail.is_empty() {
                    true => String::new(),
                    false => format!(" -> {detail}"),
                }
            ));
        }

        println!("   outcomes: {summary:?}");

        for (group, lines) in lines {
            println!("   -- {group} ({})", lines.len());

            for line in lines {
                println!("      {line}");
            }
        }

        for (key, _, change) in all.iter().filter(|(_, _, c)| c.kind == ChangeKind::Unknown) {
            println!("   unknown: {key}: {}", change.text);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::champions::{WikiModifiers, WikiStats};
    use crate::wikitext::{AbilityModel, ChampionModel};

    #[test]
    fn named_targets_use_structured_ability_names_and_old_metadata_still_loads() {
        let champion = WikiChampion {
            name: "Nidalee".into(),
            champion_id: "Nidalee".into(),
            adaptive_type: Default::default(),
            attack_type: Default::default(),
            positions: vec![],
            stats: WikiStats::default(),
            modifiers: WikiModifiers::default(),
            exceptions: BTreeMap::new(),
            map_stats: vec![],
        };
        let mut cached = serde_json::to_value(&champion).unwrap();
        // Old cached ability payloads must be ignored, even when they cannot be parsed.
        cached["wiki_abilities"] = serde_json::json!({"Q": "retired payload"});
        let champion: WikiChampion = serde_json::from_value(cached).unwrap();
        assert!(
            serde_json::to_value(&champion)
                .unwrap()
                .get("wiki_abilities")
                .is_none()
        );
        let targets = Targets {
            champions: vec![(champion.name.clone(), champion)],
            models: BTreeMap::from([(
                "Nidalee".into(),
                ChampionModel {
                    champion_id: "Nidalee".into(),
                    abilities: BTreeMap::from([(
                        Key::Q,
                        vec![AbilityModel {
                            page: "Q1".into(),
                            descriptions: BTreeMap::new(),
                            name: "Takedown".into(),
                            damage_type: types::DamageType::Magic,
                            effects: vec![],
                            extras: vec![],
                        }],
                    )]),
                },
            )]),
            items: BTreeMap::new(),
            runes: BTreeMap::new(),
        };
        assert_eq!(
            targets.resolve("Nidalee takedown"),
            ChangeTarget::NamedAbility {
                champion_id: "Nidalee".into(),
                name: "takedown".into(),
                key: Some(Key::Q),
            }
        );
        assert_eq!(
            targets.resolve("Nidalee Unrecognized"),
            ChangeTarget::NamedAbility {
                champion_id: "Nidalee".into(),
                name: "Unrecognized".into(),
                key: None,
            }
        );
    }
}
