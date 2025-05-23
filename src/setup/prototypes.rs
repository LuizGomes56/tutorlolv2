use crate::{model::{items::Item, riot::RiotCDNItem}, setup::extract_file_name};

use super::{CdnAbility, CdnChampion, read_from_file, write_to_file};
use std::{collections::HashMap, fs, sync::Arc};

use regex::Regex;
use scraper::{Html, Selector};
use serde_json::Value;

type _MetaItemValue<T> = HashMap<String, HashMap<String, Vec<T>>>;

fn _replace_item_names_with_ids() {
    let mut meta_items = read_from_file::<_MetaItemValue<Value>>("src/internal/meta_items.json");
    let items_folder = fs::read_dir("src/internal/items").expect("Failed to read items folder");

    for entry in items_folder {
        let entry = entry.expect("Invalid DirEntry");
        let path = entry.path();
        let file_name = extract_file_name(&path);
        let item_id = file_name.parse::<usize>().unwrap_or(0);

        let internal_item =
            read_from_file::<Item>(path.to_str().expect("Failed to convert path to string"));

        for (_, positions) in meta_items.iter_mut() {
            for (_, items) in positions.iter_mut() {
                for item in items.iter_mut() {
                    if let Value::String(s) = item {
                        if s == &internal_item.name {
                            *item = Value::Number(item_id.into());
                        }
                    }
                }
            }
        }
    }
    write_to_file(
        "src/internal/meta_items.json",
        serde_json::to_string_pretty(&meta_items)
            .unwrap()
            .as_bytes(),
    );
}

// Website can't handle too many requests at once, making this function worse than a complete synchronous one
// Even with a semafor of 2, it still gets rate limited and requests start to timeout
async fn _get_meta_items() {
    let client_arc = reqwest::Client::new();

    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    let positions = ["top", "jungle", "mid", "adc", "support"];
    let mut final_map = _MetaItemValue::<usize>::new();
    let mut outer_future = Vec::new();

    let endpoint = std::env::var("META_ENDPOINT").expect("META_ENDPOINT is not set");
    let replacement_str = std::env::var("META_REPLACEMENT").expect("META_REPLACEMENT is not set");
    let semaphore = Arc::new(tokio::sync::Semaphore::new(2));

    for (_, name) in champion_names {
        let outer_client_clone = client_arc.clone();
        let endpoint = endpoint.clone();
        let replacement_str = replacement_str.clone();
        let semaphore = semaphore.clone();

        outer_future.push(tokio::spawn(async move {
            let mut inner_future = Vec::new();
            let mut temp_map = HashMap::<String, Vec<usize>>::new();

            for position in positions {
                let champion_name = name.to_lowercase().clone();
                let inner_client_clone = outer_client_clone.clone();
                let endpoint = endpoint.clone();
                let replacement_str = replacement_str.clone();
                let semaphore = semaphore.clone();
                let _permit = semaphore.acquire_owned().await.unwrap();

                inner_future.push(tokio::spawn(async move {
                    println!(
                        "Fetching meta items for {}, position: {}",
                        champion_name, position
                    );

                    let url = format!("{}/{}/build/{}", endpoint.clone(), champion_name, position);

                    let res = inner_client_clone
                        .get(url)
                        .send()
                        .await
                        .expect("Failed to send request");

                    let mut result = HashMap::<String, Vec<usize>>::new();

                    let html = res.text().await.expect("Could not read response text");
                    let document = Html::parse_document(&html);
                    let full_build = Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")
                        .expect("Failed to parse nested selector");
                    let situational_build = Selector::parse(".m-s76v8c > div > div img")
                        .expect("Failed to parse selector .m-s76v8c");

                    let mut items = Vec::<usize>::new();
                    let mut push_items = |selector: &Selector| {
                        for img in document.select(selector) {
                            if let Some(src) = img.value().attr("src") {
                                let item_str = src.replace(&replacement_str, "");
                                if let Some(id_str) = item_str.split(".png").next() {
                                    if let Ok(id) = id_str.parse::<usize>() {
                                        items.push(id);
                                    }
                                }
                            }
                        }
                    };
                    push_items(&full_build);
                    push_items(&situational_build);
                    result.insert(String::from(position), items);
                    result
                }));
            }
            for inner_result in inner_future {
                let inner_task_result = inner_result.await;
                match inner_task_result {
                    Ok(inner_task) => temp_map.extend(inner_task),
                    Err(e) => println!("Failed inner task result: {}", e),
                }
            }
            (name, temp_map)
        }));
    }
    for outer_result in outer_future {
        let outer_task_result = outer_result.await;
        match outer_task_result {
            Ok((name, map)) => {
                final_map.insert(name, map);
            }
            Err(e) => println!("Failed outer task result: {}", e),
        }
    }

    write_to_file(
        "src/internal/meta_items.json",
        serde_json::to_string(&final_map).unwrap().as_bytes(),
    );
}

fn _transform_ability(key: &str, abilities: &Vec<CdnAbility>) -> Vec<String> {
    let mut writer_keybinds = Vec::<String>::new();

    for (ability_index, ability) in abilities.iter().enumerate() {
        for (effect_index, effect) in ability.effects.iter().enumerate() {
            for (leveling_index, leveling) in effect.leveling.iter().enumerate() {
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

                let base_key = format!("{}_{}_{}", key, ability_index, effect_index);
                let final_key = format!("{}{}", base_key, suffix);

                writer_keybinds.push(format!(
                    "({},{},\"{}\",{})",
                    effect_index,
                    leveling_index,
                    final_key,
                    if suffix.contains("MAX") {
                        "IterationTarget::MAXIMUM"
                    } else {
                        "IterationTarget::MINIMUM"
                    }
                ));
            }
        }
    }
    writer_keybinds
}

pub async fn _generate_writer_files() {
    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    let mut futures = Vec::new();

    let bind_function = |ability_name: &str, coords: Vec<String>| {
        "get_from_pattern(&data.abilities.$1[0],&mut abilities,&[$2]);"
            .replace("$1", ability_name)
            .replace("$2", &coords.join(","))
    };

    for (_, champion_id) in champion_names {
        futures.push(tokio::spawn(async move {
            let path_name = format!("src/setup/writers/{}.rs", champion_id.to_lowercase());
            if let Ok(data) = fs::read_to_string(&path_name) {
                if data.get(0..25).map_or(false, |s| s.contains("#![mark_checked]")) {
                    return;
                }
            }

            let mut autogenerated_content = String::from("#![allow(dead_code)]use super::{Ability,CdnChampion,Champion,HashMap,IterationTarget,get_from_pattern};pub fn transform(data:CdnChampion)->Champion{let mut abilities=HashMap::<String,Ability>::new();");

            let champion_data =
                read_from_file::<CdnChampion>(&format!("src/cache/cdn/champions/{}.json", champion_id));

            for (key, val) in champion_data.abilities.iter() {
                let coords = _transform_ability(&key.to_uppercase(), val);
                if coords.len() > 0 {
                    autogenerated_content.push_str(&bind_function(&key, coords)); 
                }
            }

            autogenerated_content.push_str("data.format(abilities)}");

            write_to_file(&path_name, autogenerated_content.trim().as_bytes());
        }));
    }

    for future in futures {
        _ = future.await;
    }
}

pub fn _pretiffy_item_stats(path_name: &str) -> HashMap<String, Value> {
    let data = read_from_file::<RiotCDNItem>(path_name);
    let mut result = HashMap::new();

    let tag_regex = Regex::new(r#"<(attention|buffedStat|nerfedStat|ornnBonus)>(.*?)<\/(attention|buffedStat|nerfedStat|ornnBonus)>"#).unwrap();
    let line_regex = Regex::new(r"(.*?)<br>").unwrap();
    let percent_prefix_regex = Regex::new(r"^\s*\d+\s*%?\s*").unwrap();
    let tag_strip_regex = Regex::new(r"<\/?[^>]+(>|$)").unwrap();

    let u = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
    let k = ["Cooldown", "Healing"];

    let lines: Vec<_> = line_regex.captures_iter(&data.description).collect();
    let mut line_index = 0;

    for caps in tag_regex.captures_iter(&data.description) {
        let t = &caps[1];
        let v = caps[2].replace('%', "");
        let mut n: Option<String> = None;
        if line_index < lines.len() {
            let cleaned = tag_strip_regex.replace_all(&lines[line_index][1], "").trim().to_string();
            if !cleaned.is_empty() {
                n = Some(cleaned);
            }
            line_index += 1;
        }
        if u.contains(&t) {
            if let Some(n_val) = &n {
                let j = percent_prefix_regex.replace(n_val, "").trim().to_string();
                if !j.is_empty() {
                    let is_percent = caps[2].contains('%');
                    let value: Value = if k.iter().any(|&keyword| n_val.contains(keyword)) && is_percent {
                        Value::String(format!("{}%", v))
                    } else {
                        match v.parse::<f64>() {
                            Ok(num) => Value::from(num),
                            Err(_) => continue,
                        }
                    };
                    result.insert(j, value);
                }
            }
        }
    }
    result
}