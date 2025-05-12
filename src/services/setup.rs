use crate::model::champions::{Ability, CdnAbility, CdnChampion, Champion};
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, io::Write, path::Path};

async fn fetch_api(path: &str) -> HashMap<String, Value> {
    let url = std::env::var("CDN_ENDPOINT").expect("CDN_ENDPOINT is not set");
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/{}", url.trim_end_matches('/'), path))
        .send()
        .await
        .expect("Failed to send request");

    let data: Value = res.json().await.expect("Failed to parse JSON");

    let result = data.as_object().expect("Failed to convert JSON to object");

    result.clone().into_iter().collect()
}

pub async fn write_champions() {
    let result = fetch_api("champions.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/champions/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

pub async fn write_items() {
    let result = fetch_api("items.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/items/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

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

fn write_to_file(path_name: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(path_name).expect("Unable to create file");
    file.write_all(bytes).expect("Unable to write data");
}

fn read_from_file<T: serde::de::DeserializeOwned>(path_name: &str) -> T {
    let data = fs::read_to_string(path_name).expect("Unable to read file");
    serde_json::from_str(&data).expect("Failed to parse JSON")
}

pub fn setup_champion_cache() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    for file in files {
        let path = file.unwrap().path();
        generate_champion_file(path.to_str().expect("Failed to convert path to string"));
    }
}

fn transform_ability(key: &str, map: &mut HashMap<String, Ability>, abilities: &Vec<CdnAbility>) {
    for (ability_index, ability) in abilities.iter().enumerate() {
        let damages_in_area = ability
            .spell_effects
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains("aoe");

        let damage_type = ability.damage_type.as_deref().unwrap_or_default();

        let mut effect_index = 1;

        for effect in &ability.effects {
            for leveling in &effect.leveling {
                let attr = leveling
                    .attribute
                    .as_deref()
                    .unwrap_or_default()
                    .to_lowercase();

                if !attr.contains("damage") {
                    continue;
                }

                let suffix = if attr.contains("monster") {
                    "_MONSTER"
                } else if attr.contains("bonus") {
                    "_BONUS"
                } else if attr.contains("maximum")
                    || attr.contains("total")
                    || attr.contains("increased")
                {
                    "_MAXIMUM"
                } else if attr.contains("minimum") {
                    "_MINIMUM"
                } else {
                    ""
                };

                let mut final_values = Vec::<String>::new();
                let mut size_list = Vec::<usize>::new();

                for modifier in &leveling.modifiers {
                    let mut row_values = Vec::<String>::new();

                    size_list.push(modifier.values.len());

                    for i in 0..modifier.values.len() {
                        let value = modifier.values[i];
                        let unit = modifier.units[i].trim();

                        let formatted = if unit.contains('%') {
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

                        row_values.push(formatted);
                    }

                    final_values.extend(row_values);
                }

                let base_key = format!("{}_{}_{}", key, ability_index, effect_index);
                let final_key = format!("{}{}", base_key, suffix);

                let chunk_size = size_list.get(0).copied().unwrap_or(match key {
                    "P" => 18,
                    "R" => 3,
                    _ => 5,
                });
                let split_vec = chunk_vector(&final_values, chunk_size);
                let result_vec = join_by_index(&split_vec);

                let entry = map.entry(final_key).or_insert(Ability {
                    damage_type: damage_type.to_string(),
                    damages_in_area,
                    minimum_damage: vec![],
                    maximum_damage: vec![],
                });

                if suffix.contains("MAX") {
                    entry.maximum_damage = result_vec;
                } else {
                    entry.minimum_damage = result_vec;
                }
            }

            effect_index += 1;
        }
    }
}

fn trim_f64(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{:.0}", val)
    } else {
        format!("{}", val)
    }
}

fn join_by_index(chunks: &Vec<Vec<String>>) -> Vec<String> {
    if chunks.is_empty() {
        return vec![];
    }

    let len = chunks[0].len();

    (0..len)
        .map(|i| {
            chunks
                .iter()
                .map(|v| v[i].clone())
                .collect::<Vec<_>>()
                .join(" + ")
        })
        .collect()
}

fn chunk_vector<T: Clone>(input: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    let chunks = input
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    if chunks.iter().all(|c| c.len() == chunks[0].len()) {
        return chunks;
    }

    let chunks_plus = input
        .chunks(chunk_size + 1)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    if chunks_plus.iter().all(|c| c.len() == chunks_plus[0].len()) {
        return chunks_plus;
    }

    eprintln!(
        "chunk_size[..{}] e ++chunk_size[..{}] are not equal in size. [T::{}]",
        chunk_size,
        chunk_size + 1,
        input.len(),
    );

    Vec::new()
}

fn generate_champion_file(path: &str) {
    let result = read_from_file::<CdnChampion>(path);

    let mut refined_abilities = HashMap::new();

    transform_ability("P", &mut refined_abilities, &result.abilities.p);
    transform_ability("Q", &mut refined_abilities, &result.abilities.q);
    transform_ability("W", &mut refined_abilities, &result.abilities.w);
    transform_ability("E", &mut refined_abilities, &result.abilities.e);
    transform_ability("R", &mut refined_abilities, &result.abilities.r);

    let champion = Champion {
        name: result.name.clone(),
        adaptative_type: result.adaptive_type,
        attack_type: result.attack_type,
        positions: result.positions,
        stats: result.stats,
        abilities: refined_abilities,
    };

    let bytes = serde_json::to_string(&champion).unwrap();

    write_to_file(
        &format!("src/internal/{}.json", result.name),
        bytes.as_bytes(),
    );
}
