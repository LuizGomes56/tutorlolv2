use std::collections::HashMap;

use regex::Regex;

use crate::model::champions::Modifiers;

use super::{Ability, CdnAbility, CdnChampion};

// Takes the reference of the description of one ability, the reference vector
// where data will be written at, and adds the tuples of scalling found.
fn assign_scalings(description: &String, ref_vec: &mut Vec<String>) {
    if description.is_empty() {
        return;
    }
    let scalings = extract_scaled_values(&description);
    ref_vec.iter_mut().for_each(|dmg| {
        *dmg = format!("{} + {}", dmg, scalings);
    });
}

// Easier way to get passive damage when the standard format matches.
// data -> the reference to the data passed to the caller function.
// indexes -> the (ability_index, effect_index) of the description string to be extracted.
// postfix -> optional hardcoded string to be added after each matching in final Vec<String>
// scalings -> optional index where the description string can be found to get passive's scallings
// target_vec -> determines if final ocurrence will be written in Minimum or Maximum vector
// keyname -> name of the key to be added in the map after final Vec<String> is created
// map -> reference to the map created internally by the caller function. (Must be created)
pub(super) fn get_passive_damage(
    data: &CdnChampion,
    indexes: (usize, usize),
    postfix: Option<&str>,
    scalings: Option<usize>,
    target_vec: &IterationTarget,
    keyname: &str,
    map: &mut HashMap<String, Ability>,
) {
    let mut minimum_damage = Vec::<String>::new();
    let mut maximum_damage = Vec::<String>::new();

    let (passive, passive_bounds) = extract_passive_bounds(&data, indexes);

    let mut description = &String::new();

    if let Some(scalings) = scalings {
        description = &passive.effects[scalings].description;
    }

    match target_vec {
        IterationTarget::MINIMUM => {
            minimum_damage = assign_as_linear_range(passive_bounds, 18, postfix);
            assign_scalings(&description, &mut minimum_damage);
        }
        IterationTarget::MAXIMUM => {
            maximum_damage = assign_as_linear_range(passive_bounds, 18, postfix);
            assign_scalings(&description, &mut minimum_damage);
        }
    };

    map.insert(
        String::from(keyname),
        passive.format(minimum_damage, maximum_damage),
    );
}

// Helper function to remove the decimal point if it's not needed, or expand floats.
fn trim_f64(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{:.0}", val)
    } else {
        format!("{}", val)
    }
}

// Takes the default format of the API and assigns to target_vec the correct format
// Used internally.
fn extract_ability(modifiers: &Vec<Modifiers>, target_vec: &mut Vec<String>) {
    if modifiers.is_empty() {
        return;
    }
    let length = modifiers[0].values.len();
    for i in 0..length {
        let mut parts = Vec::new();
        for modifier in modifiers {
            let value = modifier.values[i];
            let raw_unit = modifier.units[i].trim();
            let scallings = extract_scaled_values(&raw_unit);
            let unit = remove_parenthesized_additions(&raw_unit);
            let cleaned_string = if unit.contains('%') {
                let parts: Vec<&str> = unit.split('%').collect();
                let suffix = parts
                    .get(1)
                    .map_or("".to_string(), |s| s.trim().to_string());
                let coef = value / 100.0;
                if coef == 1.0 && !suffix.is_empty() {
                    suffix
                } else if !suffix.is_empty() {
                    format!("({} * {})", trim_f64(coef), suffix)
                } else {
                    format!("{}", trim_f64(coef))
                }
            } else if unit.is_empty() {
                trim_f64(value)
            } else {
                format!("{}{}", trim_f64(value), unit)
            };
            let formatted_string = replace_keys(&cleaned_string);
            let final_string = if scallings.is_empty() {
                formatted_string
            } else {
                format!("{} + {}", formatted_string, scallings)
            };
            parts.push(final_string);
        }
        target_vec.push(parts.join(" + "));
    }
}

// Takes a string with the match "{number} : {number}" and returns the numeric values
// Might return nothing if no values are found, or a tuple is malformed
fn extract_range_values(input: &str) -> Option<(f64, f64)> {
    let re = Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").ok()?;
    let caps = re.captures(input)?;

    let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
    let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

    let first_is_percent = caps.get(2).is_some();
    let second_is_percent = caps.get(4).is_some();

    let denom1 = if first_is_percent { 100.0 } else { 1.0 };
    let denom2 = if second_is_percent { 100.0 } else { 1.0 };

    Some((first / denom1, second / denom2))
}

// Lots of passive strings match with a pattern of (number) : (number) ... (+ Scalings)
// This function returns the first two values it found, assuming there will always be two.
fn extract_passive_bounds(
    data: &CdnChampion,
    indexes: (usize, usize),
) -> (&CdnAbility, (f64, f64)) {
    let (ability_index, effect_index) = indexes;

    let passive = data
        .abilities
        .p
        .get(ability_index)
        .expect("ability_index is invalid.");

    let passive_effects = passive
        .effects
        .get(effect_index)
        .expect("effect_index is invalid.")
        .description
        .clone();

    let passive_bounds = extract_range_values(&passive_effects)
        .expect("Couldn't extract numeric values for passive.");

    (passive, passive_bounds)
}

// Gets the tuples that are in pattern (+ Scalling) and formats the string to the internal format.
fn extract_scaled_values(input: &str) -> String {
    let re = Regex::new(r"\(([^)]+)\)").unwrap();
    let mut result = Vec::new();
    for cap in re.captures_iter(input) {
        let content = cap[1].trim();
        if content.to_lowercase().contains("based on level") {
            continue;
        }
        let cleaned = content.trim_start_matches('+').trim();
        let parts: Vec<&str> = cleaned.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].ends_with('%') {
            if let Ok(percent) = parts[0].trim_end_matches('%').parse::<f64>() {
                let decimal = percent / 100.0;
                let rest = parts[1..].join(" ");
                result.push(format!("({} * {})", decimal, rest));
                continue;
            }
        }
        result.push(format!("({})", cleaned));
    }
    result
        .iter()
        .map(|value| replace_keys(value))
        .collect::<Vec<String>>()
        .join(" + ")
}

// Useful for passives where scalling is linear over all 18 levels.
// Returns the array with the values for each level adjusted
fn assign_as_linear_range(bounds: (f64, f64), size: usize, postfix: Option<&str>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let (start, end) = bounds;
    for i in 0..size {
        let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
        if let Some(postfix) = postfix {
            result.push(format!("({} + {})", value, postfix));
            continue;
        }
        result.push(format!("{}", value));
    }
    result
}

fn remove_parenthesized_additions(input: &str) -> String {
    let re = Regex::new(r"\(\+\s*[^)]*\)").unwrap();
    re.replace_all(input, "").to_string()
}

pub(super) enum IterationTarget {
    MINIMUM,
    MAXIMUM,
}

type IteratorExtractor<'a> = HashMap<usize, HashMap<usize, (String, &'a IterationTarget)>>;

// Takes a pattern of [Index on Vec<Effect>], [Index on Vec<Leveling>], [(Keyname, Max/Min)]
// And assigns to the map the correct format that will be used internally.
pub(super) fn get_from_pattern(
    data: &CdnAbility,
    map: &mut HashMap<String, Ability>,
    pattern: &[(usize, usize, &str, IterationTarget)],
) {
    let mut indexes: IteratorExtractor = HashMap::new();

    for (effect_index, leveling_index, keyname, target_vector) in pattern.into_iter() {
        indexes
            .entry(*effect_index)
            .or_insert(HashMap::new())
            .insert(*leveling_index, (keyname.to_string(), target_vector));
    }

    for (effect_index, leveling) in indexes {
        for (leveling_index, (keyname, target_vector)) in leveling {
            let mut minimum_damage = Vec::<String>::new();
            let mut maximum_damage = Vec::<String>::new();

            let effects = data
                .effects
                .get(effect_index)
                .expect("Effect index passed is wrong.");

            let modifiers = &effects
                .leveling
                .get(leveling_index)
                .expect("Leveling index passed is wrong.")
                .modifiers;

            match target_vector {
                IterationTarget::MINIMUM => extract_ability(modifiers, &mut minimum_damage),
                IterationTarget::MAXIMUM => extract_ability(modifiers, &mut maximum_damage),
            }

            map.insert(keyname, data.format(minimum_damage, maximum_damage));
        }
    }
}

// Replaces common keys found in the API with the corresponding ones used internally
pub(super) fn replace_keys(s: &str) -> String {
    let replacements = [
        ("of target's maximum health", "ENEMY_MAX_HEALTH"),
        ("target's current health", "ENEMY_CURRENT_HEALTH"),
        ("target's missing health", "ENEMY_MISSING_HEALTH"),
        ("per Feast stack", "CHOGATH_STACKS"),
        ("bonus AD", "BONUS_AD"),
        ("bonus health", "BONUS_HEALTH"),
    ];

    replacements
        .iter()
        .fold(s.to_string(), |acc, (old, new)| acc.replace(old, new))
}
