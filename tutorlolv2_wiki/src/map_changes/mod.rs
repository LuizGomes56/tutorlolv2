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
use mlua::{Lua, LuaSerdeExt, Value};
use overrides::{ChampionOverrides, Outcome, SelectedEffect};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tutorlolv2_types::{GameMap, Key};

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

/// Effects that each champion generator selects by name, read from its source
/// as `lock-check` does. The build knows them exactly; the report uses this to
/// run the same rules without building
fn selected_effects() -> MayFail<BTreeMap<String, Vec<SelectedEffect>>> {
    let dir = Path::new(lock::GENERATORS).join("champions");

    Ok(GeneratorFile::read_dir(dir)?
        .into_iter()
        .map(|file| {
            let selected = file
                .effect_refs()
                .into_iter()
                .filter_map(|r| {
                    let key = match r.key.as_str() {
                        "P" => Key::P,
                        "Q" => Key::Q,
                        "W" => Key::W,
                        "E" => Key::E,
                        "R" => Key::R,
                        _ => return None,
                    };

                    Some(SelectedEffect {
                        key,
                        nth: r.nth,
                        name: r.name,
                    })
                })
                .collect();

            (file.id, selected)
        })
        .collect())
}

/// Outcome of every change of a mode, by key and index of the change
pub fn outcomes(
    changes: &MapChanges,
    code: &str,
    champions: &BTreeMap<String, WikiChampion>,
    items: &BTreeMap<String, WikiItem>,
    selected: &BTreeMap<String, Vec<SelectedEffect>>,
) -> BTreeMap<(String, usize), Outcome> {
    let mut outcomes = BTreeMap::new();

    for champion in champions.values() {
        let selected = selected
            .get(&champion.champion_id)
            .map(Vec::as_slice)
            .unwrap_or_default();

        for (key, i, outcome) in ChampionOverrides::new(champion, changes, selected).outcomes {
            outcomes.insert((key, i), outcome);
        }
    }

    for (key, entry) in &changes.entries {
        for (i, change) in entry.changes.iter().enumerate() {
            let relevant = ChangeKind::is_damage_relevant(change.kind.name());
            let outcome = match &entry.target {
                ChangeTarget::Item { item_id } if let Some(item) = items.get(item_id) => {
                    overrides::item_outcome(change, item, code)
                }
                ChangeTarget::Champion { .. }
                | ChangeTarget::Ability { .. }
                | ChangeTarget::NamedAbility { .. } => continue,
                _ if !relevant => Outcome::NotRelevant,
                ChangeTarget::Rune { .. } => Outcome::Manual("runes need a generator"),
                ChangeTarget::SummonerSpell => Outcome::NotRelevant,
                _ => Outcome::Manual("its key names nothing known"),
            };

            outcomes.insert((key.clone(), i), outcome);
        }
    }

    outcomes
}

/// Prints, for each mode, what its changes are, which ones are applied, and
/// why the others are not
pub fn report() -> MayFail {
    let full = BTreeMap::<String, MapChanges>::from_file(full_path())?;
    let champions = BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?;
    let items = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?;
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

        let outcomes = outcomes(changes, code, &champions, &items, &selected);
        let mut summary = BTreeMap::<String, usize>::new();
        let mut lines = BTreeMap::<String, Vec<String>>::new();

        for ((key, i), outcome) in &outcomes {
            let change = &changes.entries[key].changes[*i];
            let (group, detail) = match outcome {
                Outcome::Applied(effects) => (
                    "applied".to_string(),
                    effects
                        .iter()
                        .map(|e| format!("{:?}[{}] {:?}", e.key, e.nth, e.name))
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
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
