use crate::{
    JsonRead, JsonWrite, MayFail,
    model::{
        items::{Item, MerakiItem},
        riot::RiotCdnItem,
    },
    riot::RiotCdnRune,
};
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};
use tutorlolv2_gen::{Attrs, DamageType, GameMap, ItemId};
use tutorlolv2_types::StatName;

/// Creates basic folders necessary to run the program. If one of these folders are not found,
/// The program is likely to panic when an update is called.
pub fn setup_project_folders() -> MayFail {
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
        "cache/scraper/combos",
        "cache/scraper/builds",
        "cache/scraper/builds/top",
        "cache/scraper/builds/jungle",
        "cache/scraper/builds/mid",
        "cache/scraper/builds/adc",
        "cache/scraper/builds/support",
        "cache/meraki",
        "cache/meraki/champions",
        "cache/meraki/items",
        "cache/riot",
        "cache/riot/champions",
        "cache/riot/champions_lang",
        "cache/riot/raw_champions",
        "cache/riot/items",
        "internal",
        "internal/items",
        "internal/champions",
        "internal/scraper",
        "internal/scraper/combos",
        "internal/scraper/builds",
        "internal/scraper/builds/top",
        "internal/scraper/builds/jungle",
        "internal/scraper/builds/mid",
        "internal/scraper/builds/adc",
        "internal/scraper/builds/support",
    ] {
        let path = Path::new(dir);

        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

/// Replaces the content found in the files to a shorter and adapted version,
/// initializes items as default, and Damaging stats must be added separately.
pub fn setup_internal_items() -> MayFail {
    let meraki_items = MerakiItem::from_dir("cache/meraki/items")?;
    let mut riot_items = RiotCdnItem::from_dir("cache/riot/items")?;

    println!("[ok] Found {} riot items", riot_items.len());
    println!("[ok] Found {} meraki items", meraki_items.len());

    struct ItemCache {
        meraki_item: MerakiItem,
        riot_item: RiotCdnItem,
    }

    let common_items = meraki_items
        .into_iter()
        .filter_map(|(riot_id, meraki_item)| {
            riot_items.remove(&riot_id).map(|riot_item| {
                (
                    riot_id,
                    ItemCache {
                        meraki_item,
                        riot_item,
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    println!("Found {} common items", common_items.len());

    if common_items.is_empty() {
        panic!("No common items found");
    }

    for (_, item) in common_items {
        let item_id = ItemId::from_riot_id(item.meraki_item.id)
            .ok_or("[fail] ItemId::from_riot_id(item.meraki_item.id)")?;

        let ItemCache {
            meraki_item,
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
            builds_from_item_ids: meraki_item
                .builds_from
                .iter()
                .filter_map(|v| ItemId::from_riot_id(*v))
                .collect(),
            builds_from_riot_ids: meraki_item.builds_from,
            builds_into_item_ids: meraki_item
                .builds_into
                .iter()
                .filter_map(|v| ItemId::from_riot_id(*v))
                .collect(),
            builds_into_riot_ids: meraki_item.builds_into,
            prettified_stats: Vec::new(),
            name: meraki_item.name,
            price: meraki_item.shop.prices.total,
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
            stats: meraki_item.stats,
            tier: meraki_item.tier,
            ranged: Default::default(),
            melee: Default::default(),
            purchasable: meraki_item.shop.purchasable && riot_item.gold.purchasable,
        };
        result.into_file(format!("internal/items/{item_id:?}.json"))?;
    }

    Ok(())
}

pub fn setup_runes_json() -> MayFail {
    let map = Vec::<RiotCdnRune>::from_file("cache/riot/runes.json")?;
    let mut result = HashMap::<String, usize>::new();

    for tree in map.into_iter() {
        for slot in tree.slots.into_iter() {
            for riot_rune in slot.runes.into_iter() {
                result.insert(riot_rune.name, riot_rune.id);
            }
        }
    }
    result.into_file("internal/rune_names.json")
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
    let meraki_items = MerakiItem::from_dir("cache/meraki/items")?;

    for (_, result) in meraki_items {
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
    is_damaging.into_file("internal/damaging_items.json")
}

pub async fn prettify_internal_items() -> MayFail {
    for (riot_id, riot_item) in RiotCdnItem::from_dir("cache/riot/items")? {
        if let Some(item_id) = ItemId::from_riot_id(riot_id.parse()?) {
            let internal_path = format!("internal/items/{item_id:?}.json",);
            let mut internal_item = Item::from_file(&internal_path)?;
            internal_item.prettified_stats = pretiffy_items(&riot_item)?;
            internal_item.into_file(internal_path)?;
        }
    }
    Ok(())
}

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(data: &RiotCdnItem) -> MayFail<Vec<StatName>> {
    let mut result = HashMap::<_, _>::default();

    let tag_regex = Regex::new(
        r#"<(attention|buffedStat|nerfedStat|ornnBonus)>(.*?)<\/(attention|buffedStat|nerfedStat|ornnBonus)>"#,
    )?;
    let line_regex = Regex::new(r"(.*?)<br>")?;
    let percent_prefix_regex = Regex::new(r"^\s*\d+\s*%?\s*")?;
    let tag_strip_regex = Regex::new(r"<\/?[^>]+(>|$)")?;

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
                    match v.parse::<usize>() {
                        Ok(num) => result.insert(j, num),
                        Err(_) => continue,
                    };
                }
            }
        }
    }

    let json = result
        .into_iter()
        .map(|(key, value)| {
            let name = tutorlolv2_fmt::pascal_case(&key);
            format!(r#"{{ "name": {name:?}, "value": {value} }}"#)
        })
        .collect::<Vec<_>>()
        .join(",");

    Ok(serde_json::from_str::<Vec<StatName>>(&format!("[{json}]"))?)
}
