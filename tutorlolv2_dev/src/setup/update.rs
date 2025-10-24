use crate::{
    generators::{MayFail, gen_factories::fac_champions::ChampionFactory},
    model::{
        champions::CdnChampion,
        items::{CdnItem, Item},
        riot::RiotCdnItem,
    },
    riot::RiotCdnRune,
    setup::essentials::ext::FilePathExt,
};
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};
use tutorlolv2_gen::{Attrs, DamageType, GameMap, ItemId};

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

pub fn setup_internal_champions() {
    ChampionFactory::run_all();
}

/// Replaces the content found in the files to a shorter and adapted version,
/// initializes items as default, and Damaging stats must be added separately.
pub fn setup_internal_items() -> MayFail {
    let cdn_items = "cache/cdn/items".read_dir_json::<CdnItem>()?;
    let mut riot_items = "cache/riot/items".read_dir_json::<RiotCdnItem>()?;

    struct ItemCache {
        cdn_item: CdnItem,
        riot_item: RiotCdnItem,
    }

    let common_items = cdn_items
        .into_iter()
        .filter_map(|(riot_id, cdn_item)| {
            riot_items.remove(&riot_id).map(|riot_item| {
                (
                    riot_id,
                    ItemCache {
                        cdn_item,
                        riot_item,
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    for (_, item) in common_items {
        let item_id = ItemId::from_riot_id(item.cdn_item.id);

        let ItemCache {
            cdn_item,
            riot_item,
        } = item;

        let result = Item {
            item_id,
            maps: riot_item
                .maps
                .into_iter()
                .map(|(map_id, is_available)| (GameMap::from(map_id), is_available))
                .collect(),
            sell: riot_item.gold.sell,
            riot_id: item_id.to_riot_id(),
            builds_from_item_ids: cdn_item
                .builds_from
                .iter()
                .map(|v| ItemId::from_riot_id(*v))
                .collect(),
            builds_from_riot_ids: cdn_item.builds_from,
            builds_into_item_ids: cdn_item
                .builds_into
                .iter()
                .map(|v| ItemId::from_riot_id(*v))
                .collect(),
            builds_into_riot_ids: cdn_item.builds_into,
            prettified_stats: Vec::new(),
            name: cdn_item.name,
            price: cdn_item.shop.prices.total,
            damage_type: DamageType::Unknown,
            attributes: Attrs::None,
            stats: cdn_item.stats,
            tier: cdn_item.tier,
            ranged: Default::default(),
            melee: Default::default(),
            purchasable: cdn_item.shop.purchasable && riot_item.gold.purchasable,
        };
        let json = serde_json::to_string(&result).unwrap();
        format!("internal/items/{item_id:?}.json",)
            .write_to_file(json.as_bytes())
            .unwrap();
    }

    Ok(())
}

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
pub fn setup_damaging_items() -> MayFail {
    let re = Regex::new(r"\{\{[^}]*\}\}")?;

    let contains_damage_outside_template = |text: &str| -> bool {
        let cleaned = re.replace_all(text, "");
        cleaned.contains("damage")
    };

    let mut is_damaging = Vec::new();
    let cdn_items = "cache/cdn/items".read_dir_json::<CdnItem>()?;

    for (_, result) in cdn_items {
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

    let json = serde_json::to_string_pretty(&is_damaging)?;

    "internal/damaging_items.json".write_to_file(json.as_bytes())
}

/// Uses champion display name and converts to their respective ids, saving to internal
pub fn setup_champion_names() -> MayFail {
    let cdn_champions = "cache/cdn/champions".read_dir_json::<CdnChampion>()?;
    let mut map = HashMap::<String, String>::default();

    for (name, cdn_champion) in cdn_champions {
        map.insert(cdn_champion.name, name);
    }

    let json = serde_json::to_string(&map)?;
    "internal/champion_names.json".write_to_file(json.as_bytes())
}

pub async fn prettify_internal_items() -> MayFail {
    for (riot_id, riot_cdn_item) in "cache/riot/items.json".read_dir_json::<RiotCdnItem>()? {
        let internal_path = format!(
            "internal/items/{:?}.json",
            ItemId::from_riot_id(riot_id.parse()?)
        );
        let mut internal_item = internal_path.read_json::<Item>()?;

        let prettified_stats = pretiffy_items(&riot_cdn_item);

        internal_item.prettified_stats = prettified_stats;

        let json = serde_json::to_string(&internal_item)?;
        internal_path.write_to_file(json.as_bytes())?;
    }
    Ok(())
}

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(data: &RiotCdnItem) -> Vec<String> {
    let mut result = HashMap::<_, f64>::default();

    let tag_regex = Regex::new(
        r#"<(attention|buffedStat|nerfedStat|ornnBonus)>(.*?)<\/(attention|buffedStat|nerfedStat|ornnBonus)>"#,
    ).unwrap();
    let line_regex = Regex::new(r"(.*?)<br>").unwrap();
    let percent_prefix_regex = Regex::new(r"^\s*\d+\s*%?\s*").unwrap();
    let tag_strip_regex = Regex::new(r"<\/?[^>]+(>|$)").unwrap();

    let tags = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];

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
        if tags.contains(&t) {
            if let Some(n_val) = &n {
                let j = percent_prefix_regex.replace(n_val, "").trim().to_string();
                if !j.is_empty() {
                    match v.parse::<f64>() {
                        Ok(num) => result.insert(j, num),
                        Err(_) => continue,
                    };
                }
            }
        }
    }

    result
        .into_iter()
        .map(|(key, value)| {
            format!(
                "StatName::{}({value})",
                tutorlolv2_fmt::to_pascal_case(&key)
            )
        })
        .collect()
}
