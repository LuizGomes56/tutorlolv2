//! Native map changes replace named terms in structured formulas.
use super::{WithMapChanges, map_changes, single_mode};
use crate::MayFail;
use types::{AttackType, CtxVar, Key};
use wiki::{
    map_changes::change::{ChangeKind, MapChange},
    wikitext::{Formula, Lookup, MapValue, Nodes, SourceVariant},
};

pub(super) fn map_formula(
    entity: &impl WithMapChanges,
    effect: &str,
    attack: AttackType,
    formula: Formula,
) -> MayFail<Formula> {
    map_formula_with(entity, effect, attack, formula, MapOptions::default())
}

#[derive(Default)]
pub struct MapOptions<'a> {
    pub ignore_secondary: bool,
    /// A separately selected percent amplification, never a flat damage base.
    pub amplification_subject: Option<&'a str>,
    pub alias: Option<&'a str>,
    /// Seconds represented by this output, and the source tick interval.
    pub timing: Option<(f64, f64)>,
    pub replace_stat: Option<(CtxVar, CtxVar)>,
}

pub(super) fn map_formula_with(
    entity: &impl WithMapChanges,
    effect: &str,
    attack: AttackType,
    formula: Formula,
    options: MapOptions<'_>,
) -> MayFail<Formula> {
    let lookup = Lookup::new("return {}", "return {}")?;
    let mut result = formula.clone();

    for changes in map_changes() {
        let mode = single_mode(&changes.maps, effect)?;
        let mut mapped = formula.clone();
        let mut applied = false;

        for entry in changes
            .entries
            .values()
            .filter(|entry| entity.is_map_target(&entry.target))
        {
            for change in entry.changes.iter().flat_map(|change| {
                std::iter::once(change.clone()).chain(
                    change
                        .notes
                        .iter()
                        .map(|text| MapChange::parse(text, false)),
                )
            }) {
                if !matches!(
                    change.kind,
                    ChangeKind::Base { .. } | ChangeKind::Ratio { .. }
                ) {
                    continue;
                }

                let subject = change.subject().unwrap_or("");

                if !qualified(subject, effect)
                    && !options.alias.is_some_and(|alias| qualified(subject, alias))
                {
                    continue;
                }

                if options.amplification_subject == Some(subject) {
                    continue;
                }

                if options.ignore_secondary && subject.contains("secondary damage") {
                    continue;
                }

                // The nested Shipwrecker ratio note expands the maximum-ratio
                // bullet to a Momentum progression. This generator selects full
                // Momentum, so the parent already supplies this same value.
                if effect == "Shipwrecker" && subject == "Shipwrecker damage AD ratio" {
                    continue;
                }

                let mut raw = change.clone();

                raw.text = Nodes::parse(&change.text)
                    .for_variant(SourceVariant {
                        ranged: attack == AttackType::Ranged,
                        alternate: false,
                    })?
                    .to_string();

                let Some(mut value) = MapValue::new(&raw, Key::P, &lookup) else {
                    return Err(format!("Unreadable map value for {effect}: {}", raw.text).into());
                };

                if raw.text.contains("target")
                    && let MapValue::Ratio { stat, .. } = &mut value
                    && *stat == CtxVar::MaxHealth
                {
                    *stat = CtxVar::EnemyMaxHealth;
                }

                if let Some((seconds, interval)) = options.timing {
                    let factor = if subject.contains("per second") {
                        seconds
                    } else if subject.contains("per tick") {
                        seconds / interval
                    } else {
                        1.0
                    };

                    let term = match &mut value {
                        MapValue::Base(v) | MapValue::Ratio { ratio: v, .. } => v,
                    };

                    *term = term.clone().times(factor);
                }

                if let (Some((old, new)), MapValue::Ratio { stat, .. }) =
                    (options.replace_stat, &value)
                    && *stat == new
                {
                    replace_variable(&mut mapped, old, new);
                }

                let debug_value = format!("{value:?} on {mapped:?}");
                let changed = apply_value(&mut mapped, subject, value)?;

                if !changed {
                    return Err(format!(
                        "Unplaced map value for {effect}: {} {debug_value}",
                        raw.text
                    )
                    .into());
                }

                applied = true;
            }
        }

        if applied {
            result = result.with_map(mode, mapped)
        }
    }

    Ok(result)
}

fn replace_variable(formula: &mut Formula, old: CtxVar, new: CtxVar) {
    match formula {
        Formula::Var(var) if *var == old => *var = new,
        Formula::Sum(values) | Formula::Product(values) => {
            for value in values {
                replace_variable(value, old, new);
            }
        }
        Formula::Div(a, b) => {
            replace_variable(a, old, new);
            replace_variable(b, old, new);
        }
        _ => {}
    }
}

fn qualified(subject: &str, effect: &str) -> bool {
    let subject = subject.to_ascii_lowercase();

    if !subject.contains("damage")
        && !subject.contains("ratio")
        && !subject.contains("per soul")
        && !subject.contains("per stack")
    {
        return false;
    }

    let effect = effect.to_ascii_lowercase();

    if subject.starts_with(&effect) {
        return true;
    }

    let descriptive = [
        "base", "bonus", "total", "damage", "attack", "speed", "health", "ap", "ad", "ratio",
        "ability", "power", "on", "hit", "on-hit", "per", "second", "tick", "maximum", "current",
        "target's", "of", "the", "mana", "soul", "stack", "stacks", "physical", "magic",
    ];

    subject
        .split_whitespace()
        .filter(|word| !descriptive.contains(word))
        .all(|word| effect.contains(word))
}

fn apply_value(formula: &mut Formula, subject: &str, value: MapValue) -> MayFail<bool> {
    match value {
        MapValue::Ratio { stat, ratio } => {
            let targets = formula.stat_ratio_mut(stat);

            match targets.len() {
                0 => Ok(false),
                1 => {
                    *targets.into_iter().next().unwrap() = ratio;
                    Ok(true)
                }
                _ => Err(format!("Ambiguous map scaling {subject:?} for {stat:?}").into()),
            }
        }
        MapValue::Base(value) => {
            if subject.contains("per soul") || subject.contains("per stack") {
                let targets = formula.stat_ratio_mut(CtxVar::Stacks);

                return match targets.len() {
                    0 => Ok(false),
                    1 => {
                        *targets.into_iter().next().unwrap() = value;
                        Ok(true)
                    }
                    _ => Err(format!("Ambiguous map per-stack value {subject:?}").into()),
                };
            }

            // A changed damage described as a stat ratio replaces that damage
            // expression, rather than pretending the ratio is a flat base.
            if matches!(
                value,
                Formula::Product(_) | Formula::Div(..) | Formula::Var(_)
            ) {
                *formula = value;
                return Ok(true);
            }

            if let Some(base) = formula.base_mut() {
                *base = value;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula as f;
    use types::CtxVar::{BonusAd, Stacks};
    use wiki::wikitext::Sample;
    #[test]
    fn semantic_map_placement_preserves_other_terms() {
        let mut damage = f![30 + 11 * Stacks + 0.1 * BonusAd];
        apply_value(&mut damage, "Damage per soul", MapValue::Base(f![5])).unwrap();
        apply_value(
            &mut damage,
            "Bonus AD ratio",
            MapValue::Ratio {
                stat: CtxVar::BonusAd,
                ratio: f![0.15],
            },
        )
        .unwrap();
        assert_eq!(
            damage.eval(&Sample::new([
                (CtxVar::Stacks, 10.0),
                (CtxVar::BonusAd, 100.0)
            ])),
            Some(95.0)
        );
        assert!(!qualified("Immolate health ratio", "Desolate"));
        assert!(qualified("Immolate health ratio", "Immolate"));
    }
}
