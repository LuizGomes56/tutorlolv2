use crate::{
    JsonWrite, MayFail, client::SaveTo, parallel_read, riot::RiotCdnRune, setup::riot::RiotCdnItem,
};
use regex::Regex;
use std::{collections::BTreeMap, fs, path::Path, sync::LazyLock};
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

/// Reads the cached runes json extracted from Riot's API and generates a new file containing
/// only the names of each rune, and their ids
pub fn setup_runes_names() -> MayFail {
    let result: Vec<Vec<(String, usize)>> =
        parallel_read(SaveTo::RiotRunes.path(), |_, rune: RiotCdnRune| {
            let mut runes = Vec::new();

            for slot in rune.slots.into_iter() {
                for riot_rune in slot.runes.into_iter() {
                    runes.push((riot_rune.name, riot_rune.id));
                }
            }

            Ok(runes)
        })?;

    result
        .into_iter()
        .flatten()
        .collect::<BTreeMap<String, usize>>()
        .into_file(SaveTo::InternalRuneNames.path())
}

/// Returns the value that will be added to key `prettified_stats` for each item.
/// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
fn pretiffy_items(data: &RiotCdnItem) -> MayFail<BTreeMap<StatName, u16>> {
    static TAGS: [&str; 4] = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
    static RE_LINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)<br>").unwrap());
    static RE_TAG: LazyLock<Regex> = LazyLock::new(|| {
        let tags = TAGS.join("|");
        Regex::new(&format!(r#"<({tags})>(.*?)<\/({tags})>"#)).unwrap()
    });
    static RE_PERCENT_PREFIX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\s*\d+\s*%?\s*").unwrap());
    static RE_TAG_STRIP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<\/?[^>]+(>|$)").unwrap());

    let mut result = BTreeMap::<_, _>::default();

    let lines = RE_LINE.captures_iter(&data.description).collect::<Vec<_>>();
    let mut line_index = 0usize;

    for caps in RE_TAG.captures_iter(&data.description) {
        let t = &caps[1];
        let v = caps[2].replace('%', "");
        let mut n = None;
        if line_index < lines.len() {
            let tag_strip = RE_TAG_STRIP.replace_all(&lines[line_index][1], "");
            let cleaned = tag_strip.trim();
            if !cleaned.is_empty() {
                n = Some(cleaned.to_string());
            }
            line_index += 1;
        }
        if TAGS.contains(&t) {
            if let Some(n_val) = &n {
                let percent_prefix = RE_PERCENT_PREFIX.replace(n_val, "");
                let j = percent_prefix.trim();
                if !j.is_empty() {
                    match v.parse::<u16>() {
                        Ok(num) => result.insert(tutorlolv2_fmt::pascal_case(&j), num),
                        Err(_) => continue,
                    };
                }
            }
        }
    }

    let json = result
        .into_iter()
        .map(|(stat, value)| {
            let variant = format!("{stat:?}")
                .replace("CriticalStrikeChance", "CritChance")
                .replace("CriticalStrikeDamage", "CritDamage");
            let key = serde_json::from_str(&variant).unwrap();
            (key, value)
        })
        .collect();

    Ok(json)
}
