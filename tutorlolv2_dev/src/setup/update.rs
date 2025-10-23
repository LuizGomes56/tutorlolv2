use crate::{
    model::{
        champions::CdnChampion,
        items::{CdnItem, Item, PartialStats},
        riot::RiotCdnItem,
    },
    riot::RiotCdnRune,
    setup::{essentials::ext::FilePathExt, generators::champions::run_generator_file},
};
use regex::Regex;
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};
use tutorlolv2_gen::{Attrs, DamageType, ItemId};

/// Creates basic folders necessary to run the program. If one of these folders are not found,
/// The program is likely to panic when an update is called.
pub fn setup_project_folders() {
    for dir in [
        "html",
        "html/brotli/champions",
        "html/brotli/items",
        "html/brotli/runes",
        "html/zstd/champions",
        "html/zstd/items",
        "html/zstd/runes",
        "html/raw/champions",
        "html/raw/items",
        "html/raw/runes",
        "sprite",
        "sprite/abilities",
        "sprite/champions",
        "sprite/runes",
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
        "cache/scraper",
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
#[tutorlolv2_macros::trace_time]
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
#[tutorlolv2_macros::trace_time]
pub fn setup_internal_items() {
    let files = fs::read_dir("cache/cdn/items").unwrap();
    for file in files {
        let entry = file.unwrap();
        let path_buf = entry.path();
        let path_str = path_buf.to_str().unwrap();
        let cdn_item = path_str.read_json::<CdnItem>().unwrap();
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

        let result = Item {
            id: cdn_item.id,
            prettified_stats: HashMap::default(),
            name: cdn_item.name,
            gold: cdn_item.shop.prices.total,
            damage_type: DamageType::Unknown,
            attributes: Attrs::None,
            stats: item_stats,
            tier: cdn_item.tier,
            builds_from: cdn_item.builds_from,
            ranged: None,
            melee: None,
            purchasable: cdn_item.shop.purchasable,
        };
        let json = serde_json::to_string(&result).unwrap();
        format!(
            "internal/items/{:?}.json",
            ItemId::from_riot_id(cdn_item.id)
        )
        .write_to_file(json.as_bytes())
        .unwrap();
    }
}

#[tutorlolv2_macros::trace_time]
pub fn setup_runes_json() {
    let map = "cache/riot/runes.json"
        .read_json::<Vec<RiotCdnRune>>()
        .unwrap();
    let mut result = HashMap::<String, usize>::new();

    for tree in map.into_iter() {
        for slot in tree.slots.into_iter() {
            for riot_rune in slot.runes.into_iter() {
                result.insert(riot_rune.name, riot_rune.id);
            }
        }
    }

    "internal/rune_names.json"
        .write_to_file(serde_json::to_string(&result).unwrap().as_bytes())
        .unwrap();
}

/// Not meant to be used frequently. Just a quick check for every
/// patch to identify if a new damaging item was added
#[tutorlolv2_macros::trace_time]
pub fn setup_damaging_items() {
    let re = Regex::new(r"\{\{[^}]*\}\}").unwrap();

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

        let result = path_str.read_json::<CdnItem>().unwrap();

        if !result.shop.purchasable {
            continue;
        }

        let mut found_match = false;

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

    "internal/damaging_items.json"
        .write_to_file(json.as_bytes())
        .unwrap();
}

/// Uses champion display name and converts to their respective ids, saving to internal
#[tutorlolv2_macros::trace_time]
pub fn setup_champion_names() {
    let files = fs::read_dir("cache/cdn/champions").unwrap();

    let mut map = HashMap::<String, String>::default();

    for file in files {
        let path_buf = file.unwrap().path();
        let path_str = path_buf.to_str().unwrap();
        let result = path_str.read_json::<CdnChampion>().unwrap();
        let name = path_buf.json_file_name();
        map.insert(result.name, name.to_string());
    }

    let json = serde_json::to_string(&map).unwrap();
    "internal/champion_names.json"
        .write_to_file(json.as_bytes())
        .unwrap();
}

/// `internal/items` folder must exist, as well as dir `cache/riot/items`. Takes every file
/// and reads the "description" value from Riot `item.json` and parses its XML into a HashMap
/// only updates the key `prettified_stats`. All the remaining content remains the same
#[tutorlolv2_macros::trace_time]
pub async fn prettify_internal_items() {
    let files = fs::read_dir("cache/riot/items").unwrap();

    for file in files {
        let file_entry = file.unwrap();
        let path_buf = file_entry.path();
        let name = path_buf.json_file_name();
        let path_name = format!("cache/riot/items/{name}.json");
        let internal_path = format!(
            "internal/items/{:?}.json",
            ItemId::from_riot_id(name.parse().unwrap())
        );

        let prettified_stats = pretiffy_items(&path_name);

        if !Path::new(&internal_path).exists() {
            println!("Item {} does not exist", name);
            continue;
        }

        let mut current_content = internal_path.read_json::<Item>().unwrap();
        current_content.prettified_stats = prettified_stats;

        let json = serde_json::to_string(&current_content).unwrap();

        internal_path.write_to_file(json.as_bytes()).unwrap();
    }
}

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(path_name: &str) -> HashMap<String, Value> {
    let data = match path_name.read_json::<RiotCdnItem>() {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to read {}: {:#?}", path_name, e);
            return HashMap::default();
        }
    };
    let mut result = HashMap::<_, Value>::default();

    // #![manual_impl]
    let tag_regex = Regex::new(
        r#"<(attention|buffedStat|nerfedStat|ornnBonus)>(.*?)<\/(attention|buffedStat|nerfedStat|ornnBonus)>"#,
    ).unwrap();
    let line_regex = Regex::new(r"(.*?)<br>").unwrap();
    let percent_prefix_regex = Regex::new(r"^\s*\d+\s*%?\s*").unwrap();
    let tag_strip_regex = Regex::new(r"<\/?[^>]+(>|$)").unwrap();

    let u = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
    let k = ["Cooldown", "Healing"];

    let lines = line_regex
        .captures_iter(&data.description)
        .collect::<Vec<_>>();
    let mut line_index = 0usize;

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
