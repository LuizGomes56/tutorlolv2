use crate::{
    ENV_CONFIG, JsonRead, JsonWrite, MayFail,
    client::{SaveTo, Tag},
    gen_factories::fac_items::ItemFactory,
    model::{
        items::{Item, MerakiItem},
        riot::RiotCdnItem,
    },
    parallel_read,
    riot::RiotCdnRune,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::Path,
};
use tutorlolv2_fmt::{pascal_case, to_ssnake};
use tutorlolv2_gen::{GameMap, ItemId, StatName};

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
        "cache/scraper/builds/Top",
        "cache/scraper/builds/Jungle",
        "cache/scraper/builds/Middle",
        "cache/scraper/builds/Bottom",
        "cache/scraper/builds/Support",
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
        "internal/scraper/builds/Top",
        "internal/scraper/builds/Jungle",
        "internal/scraper/builds/Middle",
        "internal/scraper/builds/Bottom",
        "internal/scraper/builds/Support",
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
    parallel_read(
        SaveTo::RiotItemsDir.path(),
        move |fname, riot_cdn_item: RiotCdnItem| {
            let meraki_item =
                MerakiItem::from_file(SaveTo::MerakiCache(Tag::Items, &fname).path()).ok();
            let riot_id = fname.parse()?;

            let (stats, tier, builds_from_riot_ids, builds_into_riot_ids) = meraki_item
                .and_then(|item| Some((item.stats, item.tier, item.builds_from, item.builds_into)))
                .unwrap_or_default();

            let name = {
                let rname = riot_cdn_item.name;
                match rname.is_empty() || rname.starts_with("<") {
                    true => format!("Unknown_{riot_id}"),
                    false => match rname.as_str() {
                        "Reinforced Armor" | "Structure Bounty" | "Kalista's Black Spear" => {
                            format!("{rname}_{riot_id}")
                        }
                        _ => rname,
                    },
                }
            };

            let mut item = Item {
                version: ENV_CONFIG.lol_version.clone(),
                maps: riot_cdn_item
                    .maps
                    .into_iter()
                    .map(|(map_id, is_available)| (GameMap::from_u8(map_id), is_available))
                    .collect::<BTreeMap<_, _>>()
                    .into_iter()
                    .collect(),
                sell: riot_cdn_item.gold.sell,
                purchasable: riot_cdn_item.gold.purchasable,
                price: riot_cdn_item.gold.total,
                riot_id,
                name,
                stats,
                tier,
                builds_from_riot_ids,
                builds_into_riot_ids,
                ..Default::default()
            };

            match riot_id {
                220000..230000 => item.name += " [Arena]",
                320000..330000 => item.name += " [U-32]",
                440000..450000 => item.name += " [U-44]",
                660000..670000 => item.name += " [U-66]",
                990000..1000000 => item.name += " [U-99]",
                _ => {}
            }

            let internal_fname = pascal_case(&item.name);
            let generator_fname = to_ssnake(&internal_fname).to_lowercase();

            item.into_file(SaveTo::Internal(Tag::Items, &internal_fname).path())
        },
    )
}

/// Reads the cached runes json extracted from Riot's API and generates a new file containing
/// only the names of each rune, and their ids
pub fn setup_runes_json() -> MayFail {
    let map = Vec::<RiotCdnRune>::from_file("cache/riot/runes.json")?;
    let mut result = BTreeMap::<String, usize>::new();

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

    let is_damaging: Vec<_> = parallel_read("cache/meraki/items", {
        move |_, meraki_item: MerakiItem| {
            if !meraki_item.shop.purchasable {
                return Ok(None);
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

            Ok(found_match.then_some(meraki_item.id))
        }
    })?;

    let mut is_damaging = is_damaging.into_iter().flatten().collect::<Vec<_>>();
    is_damaging.sort();
    is_damaging.into_file("internal/damaging_items.json")
}

pub fn prettify_internal_items() -> MayFail {
    parallel_read("cache/riot/items", |riot_id, riot_item| {
        if let Some(item_id) = ItemId::from_riot_id(riot_id.parse()?) {
            let internal_path = SaveTo::Internal(Tag::Items, &format!("{item_id:?}")).path();
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
            format!("{{{name:?}:{value}}}")
        })
        .collect::<Vec<_>>()
        .join(",");

    Ok(
        serde_json::from_str::<BTreeSet<StatName>>(&format!("[{json}]"))?
            .into_iter()
            .collect(),
    )
}
