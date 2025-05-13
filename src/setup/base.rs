use crate::model::champions::{CdnChampion, Modifiers};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, io::Write, path::Path};

use super::*;

// Helper function to fetch data from the CDN. ReturnTypes are not strongly typed.
async fn fetch_api(path_name: &str) -> HashMap<String, Value> {
    let url = std::env::var("CDN_ENDPOINT").expect("CDN_ENDPOINT is not set");
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/{}", url.trim_end_matches('/'), path_name))
        .send()
        .await
        .expect("Failed to send request");

    let data: Value = res.json().await.expect("Failed to parse JSON");

    let result = data.as_object().expect("Failed to convert JSON to object");

    result.clone().into_iter().collect()
}

// Recovers all champions from CDN and create a separate file for each of them.
pub async fn write_champions() {
    let result = fetch_api("champions.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/champions/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

// Recovers all items from CDN and create a separate file for each of them.
pub async fn write_items() {
    let result = fetch_api("items.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/items/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

// Creates basic folders necessary to run the program. If one of these folders are not found,
// The program is likely to panic when an update is called.
pub fn setup_folders() {
    for dir in &[
        "cache",
        "cache/cdn",
        "cache/cdn/champions",
        "cache/cdn/items",
        "internal",
    ] {
        let path = Path::new(dir);
        if !path.exists() {
            let error_msg = format!("Unable to create directory '{}'", dir);

            fs::create_dir_all(path).expect(&error_msg);
        }
    }
}

// Helper function to write files
pub(super) fn write_to_file(path_name: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(path_name).expect("Unable to create file");
    file.write_all(bytes).expect("Unable to write data");
}

// Helper to read from files and parse the value to a struct
pub(super) fn read_from_file<T: DeserializeOwned>(path_name: &str) -> T {
    let data = fs::read_to_string(path_name).expect("Unable to read file");
    serde_json::from_str(&data).expect("Failed to parse JSON")
}

// Read every file in cache/cdn/champions folder and delegates the processing to generate_champion_file
pub fn setup_champion_cache() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    for file in files {
        let path = file.unwrap().path();
        generate_champion_file(path.to_str().expect("Failed to convert path to string"));
    }
}

// Automatically updates every champion in the game. New champions, or big updates to existing
// champions will need to be rewritten over time. If an error occurs while trying to update a
// champion, it will be skipped. Writes the resulting json to internal/{champion_name}.json
fn generate_champion_file(path_name: &str) {
    let result = read_from_file::<CdnChampion>(path_name);

    let name = Path::new(path_name)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default()
        .trim_end_matches(".json");

    let champion: Option<Champion> = match name {
        "Aatrox" => Some(aatrox::transform(result)),
        "Ahri" => Some(ahri::transform(result)),
        _ => None,
    };

    if champion.is_none() {
        return;
    }

    let bytes = serde_json::to_string(&champion).unwrap();

    write_to_file(&format!("src/internal/{}.json", name), bytes.as_bytes());
}

// Replaces common keys found in the API with the corresponding ones used internally
pub(super) fn replace_keys(s: &str) -> String {
    let mut new_str = s.to_string();

    let replacement_tuples = vec![
        ("target's maximum health", "ENEMY_MAX_HEALTH"),
        ("target's current health", "ENEMY_CURRENT_HEALTH"),
        ("target's missing health", "ENEMY_MISSING_HEALTH"),
        ("bonus ad", "BONUS_AD"),
    ];
    for (old, new) in replacement_tuples {
        new_str = s.replace(old, new)
    }
    new_str
}

// Takes a string with the match "{number}% : {number}%" and returns the numeric values
// Might return nothing if no values are found, or a tuple is malformed
pub(super) fn extract_percentage_range(input: &str) -> Option<(f64, f64)> {
    let re = Regex::new(r"(\d+(?:\.\d+)?)%\s*[:\--]?\s*(\d+(?:\.\d+)?)%").ok()?;

    let caps = re.captures(input)?;

    let first = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
    let second = caps.get(2)?.as_str().parse::<f64>().ok()? / 100.0;

    Some((first, second))
}

// Useful for passives where scalling is linear over all 18 levels.
// Returns the array with the values for each level adjusted
pub(super) fn extract_as_linear_range(
    bounds: (f64, f64),
    size: usize,
    postfix: &str,
) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let (start, end) = bounds;

    for i in 0..size {
        let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
        result.push(format!("({} + {})", value, postfix));
    }

    result
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
            let unit = modifier.units[i].trim();
            let formatted_string = if unit.contains('%') {
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
            parts.push(formatted_string);
        }
        target_vec.push(parts.join(" + "));
    }
}

pub(super) enum IterationTarget {
    MINIMUM,
    MAXIMUM,
}

type IteratorExtractor = HashMap<usize, HashMap<usize, (String, IterationTarget)>>;

// Helper function to remove the decimal point if it's not needed, or expand floats.
fn trim_f64(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{:.0}", val)
    } else {
        format!("{}", val)
    }
}

// Takes a pattern of [Index on Vec<Effect>], [Index on Vec<Leveling>], [(Keyname, Max/Min)]
// And assigns to the map the correct format that will be used internally.
pub(super) fn get_from_pattern(
    data: &CdnAbility,
    indexes: IteratorExtractor,
    map: &mut HashMap<String, Ability>,
) {
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
