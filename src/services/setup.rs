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

pub fn setup_champion_cache() {
    let files = fs::read_dir("cache/cdn/champions").expect("Unable to read directory");

    for file in files {
        let path = file.unwrap().path();
        generate_champion_file(path.to_str().expect("Failed to convert path to string"));
    }
}

fn normalize_percentiles(s: &str) -> String {
    let split: Vec<&str> = s.split("% ").collect();
    if split.len() == 2 {
        let num = split[0].parse::<f64>().unwrap_or(1.0) / 100.0;

        if num == 1.0 {
            return format!("{}", split[1].trim_end_matches(" +"));
        }
        format!("({} * {})", num, split[1].trim_end_matches(" +"))
    } else {
        s.to_string()
    }
}

fn join_common_values(v: &mut Vec<String>) {
    let half = v.len() / 2;
    if half > 0 && v.len() % 2 == 0 {
        let (first_half, second_half) = v.split_at(half);

        let combined: Vec<String> = first_half
            .iter()
            .zip(second_half.iter())
            .map(|(a, b)| format!("{} {}", normalize_percentiles(a), normalize_percentiles(b)))
            .collect();

        *v = combined;
    }
}

fn transform_ability(key: &str, map: &mut HashMap<String, Ability>, abilities: &Vec<CdnAbility>) {
    for (_i, ability) in abilities.iter().enumerate() {
        if !ability
            .damage_type
            .clone()
            .unwrap_or_default()
            .to_lowercase()
            .contains("damage")
        {
            continue;
        }

        if ability
            .damage_type
            .clone()
            .unwrap_or_default()
            .to_lowercase()
            .contains("mixed")
        {
            println!("{}, {} contains mixed", key, ability.name);
            continue;
        }

        for (j, effect) in ability.effects.iter().enumerate() {
            let mut minimum_damage: Vec<String> = vec![];
            let mut maximum_damage: Vec<String> = vec![];

            for modifier_like in &effect.leveling {
                let ability_attr = modifier_like
                    .attribute
                    .clone()
                    .unwrap_or_default()
                    .to_lowercase();

                if modifier_like.attribute.is_none()
                    || !ability_attr.contains("damage")
                    || ability_attr.contains("monster")
                {
                    continue;
                }

                for modifier in &modifier_like.modifiers {
                    for k in 0..modifier.units.len() {
                        let raw_string =
                            format!("{}{} +", modifier.values[k].to_string(), modifier.units[k]);

                        if ability_attr.contains("maximum") {
                            maximum_damage.push(raw_string);
                            continue;
                        }

                        minimum_damage.push(raw_string);
                    }
                }
            }

            if minimum_damage.is_empty() {
                continue;
            }

            join_common_values(&mut minimum_damage);
            join_common_values(&mut maximum_damage);

            let is_max = !maximum_damage.is_empty();
            let base_key = format!("{}{}", key, j + 1);
            let entry_key = if is_max {
                format!("{}_MAX", key)
            } else {
                base_key.clone()
            };

            let ability_struct = Ability {
                damage_type: ability.damage_type.clone().unwrap_or_default(),
                damages_in_area: ability
                    .spell_effects
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains("aoe"),
                minimum_damage: minimum_damage.clone(),
                maximum_damage: if is_max { maximum_damage } else { vec![] },
            };

            if is_max {
                map.insert(base_key, ability_struct.clone());
                map.insert(
                    entry_key,
                    Ability {
                        maximum_damage: vec![],
                        ..ability_struct
                    },
                );
            } else {
                map.insert(entry_key, ability_struct);
            }
        }
    }
}

pub fn generate_champion_file(path: &str) {
    let error_msg = format!("Unable to parse JSON for file '{}'", path);

    let data = fs::read_to_string(path).expect("Unable to read file");
    let result: CdnChampion = serde_json::from_str(&data).expect(&error_msg);

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
