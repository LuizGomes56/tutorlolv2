use crate::{
    JsonRead, JsonWrite, MayFail,
    model::{
        items::{Item, MerakiItem},
        riot::RiotCdnItem,
    },
    parallel_read,
    riot::RiotCdnRune,
};
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};
use tutorlolv2_gen::{Attrs, DamageType, GameMap, ItemId, StatName};

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
            riot_items.remove(&riot_id).map(|riot_item| ItemCache {
                meraki_item,
                riot_item,
            })
        })
        .collect::<Vec<_>>();

    println!("Found {} common items", common_items.len());

    if common_items.is_empty() {
        panic!("No common items found");
    }

    common_items.into_par_iter().for_each(|item| {
        let id = item.meraki_item.id;
        let Some(item_id) = ItemId::from_riot_id(id) else {
            return println!("[fail] ItemId::from_riot_id({id})");
        };

        let ItemCache {
            meraki_item,
            riot_item,
        } = item;

        let result = Item {
            item_id,
            maps: riot_item
                .maps
                .into_iter()
                .map(|(map_id, is_available)| (GameMap::from_u8(map_id), is_available))
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
        result
            .into_file(format!("internal/items/{item_id:?}.json"))
            .unwrap();
    });

    Ok(())
}

/// Reads the cached runes json extracted from Riot's API and generates a new file containing
/// only the names of each rune, and their ids
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

static RE_DMGI_PARENS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{[^}]*\}\}").unwrap());

/// Not meant to be used frequently. Just a quick check for every
/// patch to identify if a new damaging item was added
pub fn setup_damaging_items() -> MayFail {
    let likely_damages = |text: &str| -> bool {
        let cleaned = RE_DMGI_PARENS.replace_all(text, "");
        cleaned.contains("damage")
    };

    let is_damaging = Arc::new(Mutex::new(Vec::new()));

    parallel_read("cache/meraki/items", {
        let is_damaging = is_damaging.clone();
        move |_, meraki_item: MerakiItem| {
            if !meraki_item.shop.purchasable {
                return Ok(());
            }

            let mut found_match = false;

            for passive in &meraki_item.passives {
                if likely_damages(&passive.effects) {
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                for active in &meraki_item.active {
                    if likely_damages(&active.effects) {
                        found_match = true;
                        break;
                    }
                }
            }

            if found_match {
                is_damaging.lock().unwrap().push(meraki_item.id);
            }

            Ok(())
        }
    })?;

    let mut is_damaging = Arc::try_unwrap(is_damaging)
        .map_err(|_| "[unknown] Unable to unwrap `is_damaging` from its Arc")?
        .into_inner()?;
    is_damaging.sort();
    is_damaging.into_file("internal/damaging_items.json")
}

pub fn prettify_internal_items() -> MayFail {
    parallel_read("cache/riot/items", |riot_id, riot_item| {
        if let Some(item_id) = ItemId::from_riot_id(riot_id.parse()?) {
            let internal_path = format!("internal/items/{item_id:?}.json");
            let mut internal_item = Item::from_file(&internal_path)?;
            internal_item.prettified_stats = pretiffy_items(&riot_item)?;
            internal_item.into_file(internal_path)?;
        }
        Ok(())
    })
}

static TAGS: [&str; 4] = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
static RE_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*?)<br>").unwrap());
static RE_TAG: Lazy<Regex> = Lazy::new(|| {
    let tags = TAGS.join("|");
    Regex::new(&format!(r#"<({tags})>(.*?)<\/({tags})>"#)).unwrap()
});
static RE_PERCENT_PREFIX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*\d+\s*%?\s*").unwrap());
static RE_TAG_STRIP: Lazy<Regex> = Lazy::new(|| Regex::new(r"<\/?[^>]+(>|$)").unwrap());

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(data: &RiotCdnItem) -> MayFail<Vec<StatName>> {
    let mut result = HashMap::<_, _>::default();

    let lines = RE_LINE.captures_iter(&data.description).collect::<Vec<_>>();
    let mut line_index = 0usize;

    for caps in RE_TAG.captures_iter(&data.description) {
        let t = &caps[1];
        let v = caps[2].replace('%', "");
        let mut n = None;
        if line_index < lines.len() {
            let cleaned = RE_TAG_STRIP
                .replace_all(&lines[line_index][1], "")
                .trim()
                .to_string();
            if !cleaned.is_empty() {
                n = Some(cleaned);
            }
            line_index += 1;
        }
        if TAGS.contains(&t) {
            if let Some(n_val) = &n {
                let j = RE_PERCENT_PREFIX.replace(n_val, "").trim().to_string();
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
