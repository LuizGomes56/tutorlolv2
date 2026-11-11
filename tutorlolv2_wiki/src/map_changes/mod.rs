//! Changes that game modes make to champions, items and runes, read from
//! `Module:MapChanges/data/<mode>` with every patch. See
//! `docs/map-formula-overrides-spec.md`

pub mod change;

use crate::{
    HTTP_CLIENT, JsonRead, MayFail, champions::WikiChampion, items::item_parser::WikiItem,
    parser::Effect, runes::WikiRune,
};
use change::{ChangeKind, MapChange};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use tutorlolv2_types::{CtxVar, GameMap, Key};

/// A data module of `Module:MapChanges` and the maps it applies to
pub struct MapSource {
    /// Name of the module, as in `Module:MapChanges/data/aram`
    pub code: &'static str,
    pub maps: &'static [GameMap],
}

/// Modes whose changes are downloaded. ARAM: Mayhem has no module and uses
/// ARAM's. Arena comes next, and the event modes are left out for now
pub const MAPS: &[MapSource] = &[MapSource {
    code: "aram",
    maps: &[GameMap::Aram, GameMap::AramMayhem],
}];

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

        Ok(Self {
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

            let key = match rest {
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
                    key: champion.wiki_abilities.iter().find_map(|(key, abilities)| {
                        abilities
                            .iter()
                            .any(|a| a.name.eq_ignore_ascii_case(rest))
                            .then_some(*key)
                    }),
                    name: rest.to_string(),
                    champion_id,
                },
            };
        }

        ChangeTarget::Unresolved
    }
}

/// Prints, for each mode, what its changes are and which ones may change a
/// damage. Nothing is applied yet, so every damage-relevant change is listed
pub fn report() -> MayFail {
    let full = BTreeMap::<String, MapChanges>::from_file(full_path())?;
    let effects = Effects::load()?;

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

        let relevant = all
            .iter()
            .filter(|(_, _, c)| ChangeKind::is_damage_relevant(c.kind.name()))
            .collect::<Vec<_>>();

        let mut candidates = BTreeMap::<String, usize>::new();

        println!(
            "   -- {} damage-relevant changes, none applied yet",
            relevant.len()
        );

        for (key, entry, change) in &relevant {
            let label = match effects.candidates(&entry.target, &change.kind) {
                Some(0) => "no candidate".to_string(),
                Some(1) => "1 candidate".to_string(),
                Some(n) => format!("{n} candidates"),
                None => "review".to_string(),
            };

            *candidates.entry(label.clone()).or_default() += 1;
            println!(
                "      {key} [{}] {} ({label})",
                change.kind.name(),
                change.text
            );
        }

        println!("   by candidates: {candidates:?}");

        for (key, _, change) in all.iter().filter(|(_, _, c)| c.kind == ChangeKind::Unknown) {
            println!("   unknown: {key}: {}", change.text);
        }
    }

    Ok(())
}

/// Effects of every champion ability, item and rune, to count which effects a
/// ratio or base damage change could apply to
struct Effects {
    champions: BTreeMap<String, WikiChampion>,
    items: BTreeMap<String, WikiItem>,
    runes: BTreeMap<String, WikiRune>,
}

impl Effects {
    fn load() -> MayFail<Self> {
        Ok(Self {
            champions: BTreeMap::from_file("cache/wiki/champions/full.json")?,
            items: BTreeMap::from_file("cache/wiki/items/full.json")?,
            runes: BTreeMap::from_file("cache/wiki/runes/full.json")?,
        })
    }

    fn of(&self, target: &ChangeTarget) -> Vec<&Effect> {
        let abilities = |champion_id: &str, key: Option<Key>| {
            self.champions
                .get(champion_id)
                .into_iter()
                .flat_map(|c| &c.wiki_abilities)
                .filter(move |(k, _)| key.is_none_or(|key| **k == key))
                .flat_map(|(_, abilities)| abilities)
                .flat_map(|a| a.effects.values())
                .collect::<Vec<_>>()
        };

        match target {
            ChangeTarget::Ability { champion_id, key } => abilities(champion_id, Some(*key)),
            ChangeTarget::NamedAbility {
                champion_id,
                key: Some(key),
                ..
            } => abilities(champion_id, Some(*key)),
            ChangeTarget::Item { item_id } => self
                .items
                .get(item_id)
                .into_iter()
                .flat_map(|item| {
                    let e = &item.effects;
                    [&e.pass, &e.pass2, &e.pass3, &e.pass4, &e.act, &e.consume]
                })
                .flatten()
                .flat_map(|ie| {
                    [
                        Some(&ie.effect),
                        ie.ranged.as_ref(),
                        ie.flip.as_ref(),
                        ie.total.as_ref(),
                    ]
                })
                .flatten()
                .collect(),
            ChangeTarget::Rune { rune_id } => self
                .runes
                .get(rune_id)
                .into_iter()
                .flat_map(|rune| rune.effects.values())
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Effects that a ratio could replace a scaling of, or a base damage change
    /// the base of. `None` for kinds that need a generator to tell
    fn candidates(&self, target: &ChangeTarget, kind: &ChangeKind) -> Option<usize> {
        let effects = self.of(target);

        match kind {
            ChangeKind::Ratio { scaling, .. } => {
                let ctx_var = scaling.ctx_var().filter(|v| *v != CtxVar::Unknown)?;

                Some(
                    effects
                        .iter()
                        .filter(|e| e.scalings.iter().any(|s| s.ctx_var() == Some(ctx_var)))
                        .count(),
                )
            }
            // Effects that repeat the same base count once
            ChangeKind::Base { .. } => {
                let mut bases = Vec::new();

                for base in effects.iter().filter_map(|e| e.base.as_ref()) {
                    if !bases.contains(&base) {
                        bases.push(base);
                    }
                }

                Some(bases.len())
            }
            _ => None,
        }
    }
}
