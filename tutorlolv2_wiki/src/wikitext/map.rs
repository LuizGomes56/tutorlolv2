//! Changes of the game modes applied to the model of a champion. Each bullet of
//! `Module:MapChanges/data/*` is read again with the wikitext module, in the context
//! of the ability it targets, and the value it changes becomes a
//! [`Formula::ByMap`] branch of that value only

use {
    super::{
        AbilityModel, ChampionModel, EffectModel, Formula, Lookup, Nodes, Progression, Scaling,
        Variables, build::stat_var,
    },
    crate::map_changes::{
        ChangeTarget, MapChanges,
        change::{ChangeKind, MapChange},
        overrides::DESCRIPTIVE,
    },
    serde::{Deserialize, Serialize},
    std::collections::{BTreeMap, BTreeSet},
    tutorlolv2_types::{CtxVar, Key, MapMode},
};

/// New value of a change that affects a damage formula
#[derive(Clone, Debug, PartialEq)]
pub enum MapValue {
    /// "AP ratio changed to {{as|65% AP}}": the new ratio of the scaling terms of `stat`
    Ratio { stat: CtxVar, ratio: Formula },
    /// "Base damage changed to {{ap|70 to 190}}": the new base values
    Base(Formula),
}

impl MapValue {
    /// Reads the new value of `change`, a change of an ability of `key`. `None` for
    /// changes that don't change a damage formula, or whose value can't be read, as
    /// a ratio that scales with another stat
    pub fn new(change: &MapChange, key: Key, lookup: &Lookup) -> Option<Self> {
        let nodes = Variables::default().expand(&lookup.expand(&Nodes::parse(&change.text)));
        let templates = nodes.templates();
        let first = |name: &str| templates.iter().copied().find(|t| t.name == name);

        match change.kind {
            ChangeKind::Ratio { .. } => {
                let scaling = Scaling::new(first("as")?, key)?;

                let plain = scaling.per.is_none()
                    && scaling.per_stat.is_none()
                    && scaling.nested.is_empty()
                    && scaling.power <= 1;

                if !plain {
                    return None;
                }

                let ratios = scaling.ratios.iter().map(|ratio| ratio / 100.0).collect();

                Some(Self::Ratio {
                    stat: stat_var(&scaling.stat),
                    ratio: Self::by_rank(key, ratios)?,
                })
            }
            ChangeKind::Base { .. } => {
                let progression = Progression::new(first("ap")?, key).ok()?;
                Some(Self::Base(Self::by_rank(key, progression.numbers()?)?))
            }
            _ => None,
        }
    }

    /// One value as a constant, several by the rank of `key`
    fn by_rank(key: Key, values: Vec<f64>) -> Option<Formula> {
        match values[..] {
            [] => None,
            [value] => Some(Formula::Const(value)),
            _ => Some(Formula::ByRank {
                rank: key.as_ctx_var(),
                values,
            }),
        }
    }
}

/// What happened to a change on a champion
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Placement {
    /// The effects it changed
    Applied(Vec<PlacedEffect>),
    /// Why no effect took it
    NotPlaced(String),
}

/// An effect that a change was applied to, with the values before and after it
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlacedEffect {
    pub page: String,
    pub label: String,
    pub before: Vec<f64>,
    pub after: Vec<f64>,
}

impl MapValue {
    /// Terms of `formula` that the change replaces
    fn targets<'f>(&self, formula: &'f mut Formula) -> Vec<&'f mut Formula> {
        match self {
            MapValue::Ratio { stat, .. } => formula.stat_ratio_mut(*stat),
            MapValue::Base(_) => formula.base_mut().into_iter().collect(),
        }
    }

    fn value(&self) -> &Formula {
        match self {
            MapValue::Ratio { ratio, .. } => ratio,
            MapValue::Base(base) => base,
        }
    }

    fn missing(&self) -> String {
        match self {
            MapValue::Ratio { stat, .. } => format!("no effect has a {stat:?} ratio"),
            MapValue::Base(_) => "no effect has a base".into(),
        }
    }

    /// The factor `values` is of `of`, when it is the same at every rank, within 1%.
    /// Ranks where both are 0 match any factor, as Gnar's W base that starts at 0
    fn multiple(values: &[f64], of: &[f64]) -> Option<f64> {
        if values.len() != of.len() {
            return None;
        }

        let pairs = values
            .iter()
            .zip(of)
            .filter(|(v, o)| **v != 0.0 || **o != 0.0)
            .collect::<Vec<_>>();

        let Some((first, first_of)) = pairs.first() else {
            return Some(1.0);
        };

        if pairs.iter().any(|(_, o)| **o == 0.0) {
            return None;
        }

        let factor = *first / *first_of;

        pairs
            .iter()
            .all(|(v, o)| (*v / *o - factor).abs() <= 0.01 * factor.abs())
            .then_some(factor)
    }
}

/// Words of the part of a change that name something, as "first" and "dash" of
/// "First dash AP", without the ones that only describe the value
fn qualifier(part: &str) -> Vec<String> {
    part.to_lowercase()
        .split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .filter(|word| !word.is_empty() && !DESCRIPTIVE.contains(word))
        .map(str::to_string)
        .collect()
}

/// A term a change may apply to: its ability and effect, its index among the
/// targets of the effect, and its current values
struct Candidate {
    ability: usize,
    effect: usize,
    target: usize,
    values: Vec<f64>,
    /// Whether the effect's label names a damage
    damage: bool,
}

impl AbilityModel {
    /// Effects a change of a game mode may apply to: the `{{st}}` effects, or the
    /// values of the description of a page without them, as Akali's passive
    fn placeable_mut(&mut self) -> &mut Vec<EffectModel> {
        match self.effects.is_empty() {
            true => &mut self.extras,
            false => &mut self.effects,
        }
    }
}

impl ChampionModel {
    /// Applies the change of `part`, as "First dash AP", to the effects of the
    /// abilities of `key` on `mode`, or only of the one named `ability` when the change
    /// names it. Words of `part` that name something keep only the effects whose label
    /// has them. The candidate whose value is closest to `value` takes it, since modes
    /// change values by a margin, not to another magnitude, and the
    /// candidates that are a constant multiple of it take the same multiple of `value`.
    /// The others keep their value, and so do the values an earlier change of `mode`
    /// already changed
    pub fn apply_map_change(
        &mut self,
        key: Key,
        ability: Option<&str>,
        part: &str,
        mode: MapMode,
        value: &MapValue,
    ) -> Placement {
        let Some(abilities) = self.abilities.get_mut(&key) else {
            return Placement::NotPlaced(format!("the champion has no {key:?}"));
        };

        let words = qualifier(part);

        let named = |label: &str| {
            let label = label.to_lowercase();
            words.iter().all(|word| label.contains(word))
        };

        let mut candidates = Vec::new();
        let mut any_named = false;
        let mut changed = false;

        for (a, model) in abilities.iter_mut().enumerate() {
            if ability.is_some_and(|name| !model.name.eq_ignore_ascii_case(name)) {
                continue;
            }

            for (e, effect) in model.placeable_mut().iter_mut().enumerate() {
                if !named(&effect.label) {
                    continue;
                }

                any_named = true;

                for (t, target) in value.targets(&mut effect.formula).into_iter().enumerate() {
                    // Changed by an earlier change of this mode, which isn't undone
                    if let Formula::ByMap { maps, .. } = target
                        && maps.iter().any(|(arm, _)| *arm == mode)
                    {
                        changed = true;
                        continue;
                    }

                    if let Some(values) = target.default_values() {
                        candidates.push(Candidate {
                            ability: a,
                            effect: e,
                            target: t,
                            values,
                            damage: effect.label.to_lowercase().contains("damage"),
                        });
                    }
                }
            }
        }

        // Without words, every effect is named, so none means there is no effect
        if !any_named && !words.is_empty() {
            return Placement::NotPlaced(format!("no effect is named {}", words.join(" ")));
        }

        if candidates.is_empty() && changed {
            return Placement::NotPlaced(format!(
                "an earlier change of {mode:?} already changed its values"
            ));
        }

        let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len().max(1) as f64;
        let new = mean(&value.value().default_values().unwrap_or_default());
        // How far a candidate is from the new value
        let distance = |values: &[f64]| (mean(values) - new).abs();

        // A change is a damage change unless it names another effect, so a damage
        // takes it before a closer shield or heal, as Kassadin's Q
        let Some(primary) = candidates
            .iter()
            .min_by(|a, b| {
                (!a.damage, distance(&a.values))
                    .partial_cmp(&(!b.damage, distance(&b.values)))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|candidate| candidate.values.clone())
        else {
            return Placement::NotPlaced(value.missing());
        };

        let mut placed = Vec::new();

        for candidate in candidates {
            // The primary takes the value as it is, even when its values have zeros
            let factor = match candidate.values == primary {
                true => Some(1.0),
                false => MapValue::multiple(&candidate.values, &primary),
            };

            let Some(factor) = factor else {
                continue;
            };

            let ability = &mut abilities[candidate.ability];
            let page = ability.page.clone();
            let effect = &mut ability.placeable_mut()[candidate.effect];
            let new = value.value().clone().times(factor);

            if let Some(target) = value
                .targets(&mut effect.formula)
                .into_iter()
                .nth(candidate.target)
            {
                let current = std::mem::replace(target, Formula::Const(0.0));
                *target = current.with_map(mode, new.clone());
            }

            placed.push(PlacedEffect {
                page,
                label: effect.label.clone(),
                before: candidate.values,
                after: new.default_values().unwrap_or_default(),
            });
        }

        match placed.is_empty() {
            true => Placement::NotPlaced(value.missing()),
            false => Placement::Applied(placed),
        }
    }
}

/// What a change targets: the abilities of `key` of a champion, or only the one named
/// `ability`, on `mode`
#[derive(Clone, Copy)]
struct Target<'a> {
    champion_id: &'a str,
    key: Key,
    ability: Option<&'a str>,
    mode: MapMode,
}

/// A change of a champion's ability on one mode, and what happened to it
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MapChangeEntry {
    pub champion_id: String,
    pub key: Key,
    pub mode: MapMode,
    /// The bullet as written in the wiki
    pub text: String,
    /// Name of its [`ChangeKind`]
    pub kind: String,
    pub placement: Placement,
}

/// What happened to the changes of every mode on the models of the champions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MapChangesReport {
    pub applied: usize,
    pub not_placed: usize,
    /// Changes that don't change a damage formula, by kind
    pub ignored: BTreeMap<String, usize>,
    /// Every change that may change a damage formula
    pub entries: Vec<MapChangeEntry>,
}

impl MapChangesReport {
    /// Applies the changes of `changes`, `full.json` of `map_changes` by the code of
    /// its module, to the champions of `models` they target
    pub fn apply(
        models: &mut BTreeMap<String, ChampionModel>,
        changes: &BTreeMap<String, MapChanges>,
        lookup: &Lookup,
    ) -> Self {
        let mut report = Self::default();

        for (code, module) in changes {
            let modes = module
                .maps
                .iter()
                .map(|map| map.mode())
                .collect::<BTreeSet<_>>();

            let [mode] = modes.into_iter().collect::<Vec<_>>()[..] else {
                eprintln!("[map] Module {code:?} has no single mode, its changes are skipped");
                continue;
            };

            for entry in module.entries.values() {
                let (champion_id, key, ability) = match &entry.target {
                    ChangeTarget::Ability { champion_id, key } => (champion_id, *key, None),
                    ChangeTarget::NamedAbility {
                        champion_id,
                        key: Some(key),
                        name,
                    } => (champion_id, *key, Some(name.as_str())),
                    _ => continue,
                };

                // Changes that name a part of the ability go first, so a change of the
                // whole ability takes what is left, as Kassadin's R base after the base
                // per stack
                let mut changes = entry.changes.iter().collect::<Vec<_>>();
                changes.sort_by_key(|change| match change.subject() {
                    Some(part) => qualifier(part).is_empty(),
                    None => true,
                });

                for change in changes {
                    let target = Target {
                        champion_id,
                        key,
                        ability,
                        mode,
                    };
                    report.push(models, &target, change, lookup);
                }
            }
        }

        report
    }

    fn push(
        &mut self,
        models: &mut BTreeMap<String, ChampionModel>,
        target: &Target,
        change: &MapChange,
        lookup: &Lookup,
    ) {
        let Target {
            champion_id,
            key,
            ability,
            mode,
        } = *target;

        let kind = change.kind.name();

        if !ChangeKind::is_damage_relevant(kind) {
            *self.ignored.entry(kind.to_string()).or_default() += 1;
            return;
        }

        let part = change.subject();

        let placement = match (part, models.get_mut(champion_id)) {
            (_, None) => Placement::NotPlaced("the champion has no model".into()),
            (None, _) => Placement::NotPlaced(format!("a {kind} change is not read")),
            (Some(part), Some(model)) => match MapValue::new(change, key, lookup) {
                Some(value) => model.apply_map_change(key, ability, part, mode, &value),
                None => Placement::NotPlaced("its value can't be read".into()),
            },
        };

        match placement {
            Placement::Applied(_) => self.applied += 1,
            Placement::NotPlaced(_) => self.not_placed += 1,
        }

        self.entries.push(MapChangeEntry {
            champion_id: champion_id.to_string(),
            key,
            mode,
            text: change.text.clone(),
            kind: kind.to_string(),
            placement,
        });
    }
}
