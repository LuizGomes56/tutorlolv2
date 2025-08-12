use crate::{
    model::dev::{
        champions::CdnChampion,
        items::{CdnItem, Item, PartialStats},
        riot::RiotCdnItem,
    },
    setup::{
        essentials::helpers::{extract_file_name, read_json_file, write_to_file},
        generators::champions::run_generator_file,
    },
};
use internal_comptime::Attrs;
use regex::Regex;
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

type MetaItemValue<T> = HashMap<String, HashMap<String, Vec<T>>>;

/// Creates basic folders necessary to run the program. If one of these folders are not found,
/// The program is likely to panic when an update is called.
pub fn setup_project_folders() {
    for dir in [
        "img",
        "img/champions",
        "img/runes",
        "img/centered",
        "img/splash",
        "img/abilities",
        "img/items",
        "img/other",
        "img/stats",
        "raw_img",
        "raw_img/champions",
        "raw_img/runes",
        "raw_img/centered",
        "raw_img/splash",
        "raw_img/abilities",
        "raw_img/items",
        "cache",
        "cache/cdn",
        "cache/cdn/champions",
        "cache/cdn/items",
        "cache/riot",
        "cache/riot/champions",
        "cache/riot/items",
        "internal",
        "internal/items",
        "internal/champions",
    ] {
        let path = Path::new(dir);

        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
    }
}

/// Read every file in cache/cdn/champions folder and delegates
/// the processing to generate_champion_file
#[generator_macros::trace_time]
pub fn setup_internal_champions() {
    let files = fs::read_dir("cache/cdn/champions").unwrap();

    for file in files {
        let path_name = file.unwrap().path();

        match path_name.to_str() {
            Some(strpath) => {
                if let Err(_) = std::panic::catch_unwind(|| {
                    let _ = run_generator_file(strpath);
                }) {
                    println!("Adjustments need to be made to {}", strpath);
                }
            }
            None => {
                eprintln!(
                    "Failed to convert path to string at [setup_champion_cache]: {:?}",
                    path_name
                );
            }
        };
    }
}

/// Replaces the content found in the files to a shorter and adapted version,
/// initializes items as default, and Damaging stats must be added separately.
#[generator_macros::trace_time]
pub fn setup_internal_items() {
    let files = fs::read_dir("cache/cdn/items").unwrap();
    for file in files {
        let entry = file.unwrap();
        let path_buf = entry.path();
        let path_str = path_buf.to_str().unwrap();
        let cdn_item: CdnItem = read_json_file(path_str).unwrap();
        let stats = &cdn_item.stats;
        let mut item_stats = PartialStats::default();

        macro_rules! insert_non_zero {
            ($field:ident) => {
                item_stats.$field = stats.$field.flat;
            };
        }

        insert_non_zero!(ability_power);
        insert_non_zero!(ability_power);
        insert_non_zero!(armor);
        insert_non_zero!(attack_damage);
        insert_non_zero!(attack_speed);
        insert_non_zero!(critical_strike_chance);
        insert_non_zero!(critical_strike_damage);
        insert_non_zero!(health);
        insert_non_zero!(lifesteal);
        insert_non_zero!(magic_resistance);
        insert_non_zero!(mana);
        insert_non_zero!(movespeed);
        insert_non_zero!(omnivamp);

        item_stats.armor_penetration_flat = stats.armor_penetration.flat;
        item_stats.armor_penetration_percent = stats.armor_penetration.percent;
        item_stats.magic_penetration_flat = stats.magic_penetration.flat;
        item_stats.magic_penetration_percent = stats.magic_penetration.percent;

        let result: Item = Item {
            prettified_stats: HashMap::default(),
            name: cdn_item.name,
            gold: cdn_item.shop.prices.total,
            damage_type: None,
            attributes: Attrs::None,
            stats: item_stats,
            tier: cdn_item.tier,
            builds_from: cdn_item.builds_from,
            ranged: None,
            melee: None,
            purchasable: cdn_item.shop.purchasable,
        };
        let json = serde_json::to_string(&result).unwrap();
        write_to_file(
            &format!("internal/items/{}.json", cdn_item.id),
            json.as_bytes(),
        )
        .unwrap();
    }
}

/// Pending. Must read CDN API files and interpret the damages of runes
// #![manual_impl]
// #![unstable] "05/24/2024" | "14.5"
pub fn setup_internal_runes() {
    write_to_file("internal/runes.json", 
br#"{
    "8005": {
        "name": "Press The Attack",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(40 + ((120 / 17) * (LEVEL - 1))) * ADAPTATIVE_DAMAGE",
        "ranged": "(40 + ((120 / 17) * (LEVEL - 1))) * ADAPTATIVE_DAMAGE"
    },
    "8112": {
        "name": "Electrocute",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(30 + ((190 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(30 + ((190 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8124": {
        "name": "Predator",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(20 + ((160 / 17) * (LEVEL - 1)) + ((0.25 * BONUS_AD + 0.15 * AP))) * ADAPTATIVE_DAMAGE",
        "ranged": "(20 + ((160 / 17) * (LEVEL - 1)) + ((0.25 * BONUS_AD + 0.15 * AP))) * ADAPTATIVE_DAMAGE"
    },
    "8126": {
        "name": "Cheap Shot",
        "damage_type": "TRUE_DAMAGE",
        "melee": "10 + ((35 / 17) * (LEVEL - 1))",
        "ranged": "10 + ((35 / 17) * (LEVEL - 1))"
    },
    "8128": {
        "name": "Dark Harvest",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(20 + ((60 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(20 + ((60 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8143": {
        "name": "Sudden Impact",
        "damage_type": "TRUE_DAMAGE",
        "melee": "20 + (60 / 17 * (LEVEL - 1))",
        "ranged": "20 + (60 / 17 * (LEVEL - 1))"
    },
    "8214": {
        "name": "Aery",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(10 + ((40 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE",
        "ranged": "(10 + ((40 / 17) * (LEVEL - 1)) + (0.1 * BONUS_AD) + (0.05 * AP)) * ADAPTATIVE_DAMAGE"
    },
    "8229": {
        "name": "Comet",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(30 + ((100 / 17) * (LEVEL - 1)) + ((0.1 * BONUS_AD) + (0.05 * AP))) * ADAPTATIVE_DAMAGE",
        "ranged": "(30 + ((100 / 17) * (LEVEL - 1)) + ((0.1 * BONUS_AD) + (0.05 * AP))) * ADAPTATIVE_DAMAGE"
    },
    "8237": {
        "name": "Scorch",
        "damage_type": "MAGIC_DAMAGE",
        "melee": "(20 + ((20 / 17) * (LEVEL - 1))) * MAGIC_MULTIPLIER",
        "ranged": "(20 + ((20 / 17) * (LEVEL - 1))) * MAGIC_MULTIPLIER"
    },
    "8437": {
        "name": "Grasp",
        "damage_type": "MAGIC_DAMAGE",
        "melee": "(0.035 * MAX_HEALTH) * MAGIC_MULTIPLIER",
        "ranged": "(0.021 * MAX_HEALTH) * MAGIC_MULTIPLIER"
    },
    "8439": {
        "name": "Aftershock",
        "damage_type": "ADAPTATIVE_DAMAGE",
        "melee": "(25 + ((95 / 17) * (LEVEL - 1)) + (0.08 * BONUS_HEALTH)) * MAGIC_MULTIPLIER",
        "ranged": "(25 + ((95 / 17) * (LEVEL - 1)) + (0.08 * BONUS_HEALTH)) * MAGIC_MULTIPLIER"
    }
}"#).unwrap()
}

/// Not meant to be used frequently. Just a quick check for every
/// patch to identify if a new damaging item was added
#[generator_macros::trace_time]
pub fn setup_damaging_items() {
    let re: Regex = Regex::new(r"\{\{[^}]*\}\}").unwrap();

    let contains_damage_outside_template = |text: &str| -> bool {
        let cleaned = re.replace_all(text, "");
        cleaned.contains("damage")
    };

    let files = fs::read_dir("cache/cdn/items").unwrap();

    let mut is_damaging = Vec::new();

    for entry in files {
        let entry = entry.unwrap();

        let path_buf = entry.path();
        let path_str = path_buf.to_str().unwrap();

        let result: CdnItem = read_json_file(path_str).unwrap();

        if !result.shop.purchasable {
            continue;
        }

        let mut found_match: bool = false;

        for passive in &result.passives {
            if contains_damage_outside_template(&passive.effects) {
                found_match = true;
                break;
            }
        }

        if !found_match {
            for active in &result.active {
                if contains_damage_outside_template(&active.effects) {
                    found_match = true;
                    break;
                }
            }
        }

        if found_match {
            is_damaging.push(result.id);
        }
    }

    is_damaging.sort();

    let json = serde_json::to_string_pretty(&is_damaging).unwrap();

    write_to_file("internal/damaging_items.json", json.as_bytes()).unwrap();
}

/// Uses champion display name and converts to their respective ids, saving to internal
#[generator_macros::trace_time]
pub fn setup_champion_names() {
    let files = fs::read_dir("cache/cdn/champions").unwrap();

    let mut map: HashMap<String, String> = HashMap::<String, String>::default();

    for file in files {
        let path_buf = file.unwrap().path();

        let path_str = path_buf.to_str().unwrap();

        let result: CdnChampion = read_json_file(path_str).unwrap();

        let name = extract_file_name(&path_buf);
        map.insert(result.name, name.to_string());
    }

    let json = serde_json::to_string(&map).unwrap();

    write_to_file("internal/champion_names.json", json.as_bytes()).unwrap();
}

/// When MetaItems are recovered, each item is written in the array with its name instead of ID
/// This function replaces those names with IDs without changing the rest of the content.
/// If one's ID is not found, it will remain unchanged
#[generator_macros::trace_time]
pub fn setup_meta_items() {
    let mut meta_items: MetaItemValue<Value> = read_json_file("internal/meta_items.json").unwrap();
    let items_folder = fs::read_dir("internal/items").unwrap();

    for entry in items_folder {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = extract_file_name(&path);
        let item_id = file_name.parse::<usize>().unwrap();

        let path_str = path.to_str().unwrap();

        let internal_item: Item = read_json_file(path_str).unwrap();

        for (_, positions) in meta_items.iter_mut() {
            for (_, items) in positions.iter_mut() {
                for item in items.iter_mut() {
                    if let Value::String(s) = item {
                        if s.to_lowercase() == internal_item.name.to_lowercase() {
                            *item = Value::Number(item_id.into());
                        }
                    }
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&meta_items).unwrap();

    write_to_file("internal/meta_items.json", json.as_bytes()).unwrap();
}

/// `internal/items` folder must exist, as well as dir `cache/riot/items`. Takes every file
/// and reads the "description" value from Riot `item.json` and parses its XML into a HashMap
/// only updates the key `prettified_stats`. All the remaining content remains the same
#[generator_macros::trace_time]
pub async fn prettify_internal_items() {
    let files = fs::read_dir("cache/riot/items").unwrap();

    for file in files {
        let file_entry = file.unwrap();
        let path_buf = file_entry.path();
        let name = extract_file_name(&path_buf);
        let path_name = format!("cache/riot/items/{}.json", name);
        let internal_path = format!("internal/items/{}.json", name);

        let prettified_stats = pretiffy_items(&path_name);

        if !Path::new(&internal_path).exists() {
            println!("Item {} does not exist", name);
            continue;
        }

        let mut current_content: Item = read_json_file(&internal_path).unwrap();

        current_content.prettified_stats = prettified_stats;

        let json = serde_json::to_string(&current_content).unwrap();

        write_to_file(&internal_path, json.as_bytes()).unwrap();
    }
}

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(path_name: &str) -> HashMap<String, Value> {
    let data: RiotCdnItem = match read_json_file(path_name) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to read {}: {:#?}", path_name, e);
            return HashMap::default();
        }
    };
    let mut result: HashMap<_, Value> = HashMap::default();

    // #![manual_impl]
    let tag_regex = match Regex::new(
        r#"<(attention|buffedStat|nerfedStat|ornnBonus)>(.*?)<\/(attention|buffedStat|nerfedStat|ornnBonus)>"#,
    ) {
        Ok(value) => value,
        Err(e) => {
            println!("[tag_regex] Error on Regex Creation: {:#?}", e);
            return HashMap::default();
        }
    };
    let line_regex = match Regex::new(r"(.*?)<br>") {
        Ok(value) => value,
        Err(e) => {
            println!("[line_regex] Error on Regex Creation: {:#?}", e);
            return HashMap::default();
        }
    };
    let percent_prefix_regex = match Regex::new(r"^\s*\d+\s*%?\s*") {
        Ok(value) => value,
        Err(e) => {
            println!("[percent_prefix_regex] Error on Regex Creation: {:#?}", e);
            return HashMap::default();
        }
    };
    let tag_strip_regex = match Regex::new(r"<\/?[^>]+(>|$)") {
        Ok(value) => value,
        Err(e) => {
            println!("[tag_strip_regex] Error on Regex Creation: {:#?}", e);
            return HashMap::default();
        }
    };

    let u: [&'static str; 4] = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
    let k: [&'static str; 2] = ["Cooldown", "Healing"];

    let lines: Vec<_> = line_regex.captures_iter(&data.description).collect();
    let mut line_index: usize = 0;

    for caps in tag_regex.captures_iter(&data.description) {
        let t = &caps[1];
        let v = caps[2].replace('%', "");
        let mut n = None;
        if line_index < lines.len() {
            let cleaned = tag_strip_regex
                .replace_all(&lines[line_index][1], "")
                .trim()
                .to_string();
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
                    let value = if k.iter().any(|&keyword| n_val.contains(keyword)) && is_percent {
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
