use std::collections::BTreeMap;

use crate::{
    champions::{
        clean_text,
        template::{ChampionTemplate, SkillSet},
    },
    client::{MayFail, SyncMayFail, fetch},
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedAbilityPage {
    pub champion_id: String,
    pub slot: char,
    pub variant: Option<usize>,
    pub source_file: String,

    pub name: String,
    pub damage_type: Option<String>,
    pub spell_effects: Option<String>,

    pub effects: Vec<ParsedEffectBlock>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedEffectBlock {
    pub effect_index: usize,
    pub suffix: String,

    pub description: Option<String>,
    pub description_raw_html: Option<String>,

    pub leveling_raw_html: Option<String>,
    pub levelings: Vec<ParsedLevelingLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedLevelingLine {
    pub leveling_index: usize,
    pub attribute: String,
    pub data: String,
    pub raw_html: String,
}

pub async fn download() -> MayFail {
    for entry in std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false))
    {
        let path = entry.path();

        let champion_id = entry
            .file_name()
            .into_string()
            .map_err(|e| format!("[error] Failed to get file name for entry: {entry:?}: {e:?}"))?;

        println!("[dir] Processing {champion_id:?}");

        let bytes = std::fs::read(path.join("template").with_extension("json"))
            .map_err(|e| format!("[error] Failed to read entry: {entry:?}: {e:?}"))?;

        let data = serde_json::from_slice::<ChampionTemplate>(&bytes)
            .map_err(|e| format!("[error] Failed to deserialize entry: {entry:?}: {e:?}"))?;

        let SkillSet {
            skill_i,
            skill_q,
            skill_w,
            skill_e,
            skill_r,
        } = data.skills;

        let save_to = path.join("abilities");

        std::fs::create_dir_all(&save_to)
            .map_err(|e| format!("[error] Failed to create directory: {e:?}"))?;

        let mut futures = Vec::new();

        for (key, skills) in [
            ('P', skill_i),
            ('Q', skill_q),
            ('W', skill_w),
            ('E', skill_e),
            ('R', skill_r),
        ] {
            for (i, skill) in skills.into_iter().enumerate() {
                let save_to = save_to.clone();
                let champion_id = champion_id.clone();
                futures.push(tokio::spawn(async move {
                    let url = format!("Template:Data_{champion_id}/{skill}");
                    if let Err(e) = fetch(
                        save_to.join(format!("{key}{i}")).with_extension("html"),
                        &url,
                    )
                    .await
                    {
                        eprintln!("[error] Problem occurred with url: {url:?}: {e:?}")
                    }
                }));
            }
        }

        for future in futures {
            future
                .await
                .map_err(|e| format!("[future] Failed to join future: {e:?}"))?;
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}

pub fn parse() -> MayFail {
    let result: SyncMayFail = std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false))
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let path = entry.path();

            let champion_id = entry.file_name().into_string().map_err(|e| {
                format!("[error] Failed to get file name for entry: {entry:?}: {e:?}")
            })?;

            println!("[parallel] Processing {champion_id:?}");

            let result: SyncMayFail = std::fs::read_dir(path.join("abilities"))
                .map_err(|e| format!("[error] Failed to read abilities directory: {e:?}"))?
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().ok().map(|v| v.is_file()).unwrap_or(false)
                        && entry
                            .path()
                            .extension()
                            .map(|v| v == "html")
                            .unwrap_or(false)
                })
                .par_bridge()
                .into_par_iter()
                .try_for_each(|entry| {
                    let f = || -> MayFail {
                        let file_name = entry.file_name().into_string().map_err(|e| {
                            format!("[error] Failed to get file name for entry: {entry:?}: {e:?}")
                        })?;

                        let key = file_name.chars().next().ok_or_else(|| {
                            format!("[error] Failed to get key from file name: {file_name:?}")
                        })?;

                        let path = entry.path();

                        let html = std::fs::read_to_string(&path)?;
                        let parsed = parse_ability_html(&champion_id, key, &file_name, &html)?;

                        println!("[{champion_id:?}] Processing {file_name:?}");

                        std::fs::write(
                            path.with_extension("json"),
                            serde_json::to_string_pretty(&parsed)?,
                        )?;

                        Ok(())
                    };

                    f().map_err(|e| e.to_string().into())
                });

            result.map_err(|e| e.to_string().into())
        });

    result.map_err(|e| e.to_string().into())
}

fn parse_ability_html(
    champion_id: &str,
    slot: char,
    file_name: &str,
    html: &str,
) -> MayFail<ParsedAbilityPage> {
    let doc = Html::parse_document(html);

    let table_selector = Selector::parse("table")?;
    let tr_selector = Selector::parse("tr")?;
    let cell_selector = Selector::parse("th, td")?;

    let mut value_texts = BTreeMap::<String, String>::new();
    let mut value_htmls = BTreeMap::<String, String>::new();

    for table in doc.select(&table_selector) {
        let rows = table.select(&tr_selector).collect::<Vec<_>>();

        if rows.is_empty() {
            continue;
        }

        let header = rows[0].text().collect::<String>().to_ascii_lowercase();
        if !(header.contains("parameter") && header.contains("value")) {
            continue;
        }

        let mut first_data_row = true;

        for row in rows.into_iter().skip(1) {
            let cells = row.select(&cell_selector).collect::<Vec<_>>();
            if cells.len() < 2 {
                continue;
            }

            let mut key = clean_text(&cells[0].text().collect::<String>());
            if key.is_empty() {
                continue;
            }

            if first_data_row && key == "1" {
                key = "name".to_string();
            }
            first_data_row = false;

            let value_text = clean_text(&cells[1].text().collect::<String>());
            let value_html = cells[1].inner_html();

            value_texts.insert(key.clone(), value_text);
            value_htmls.insert(key, value_html);
        }
    }

    let effects = build_effect_blocks(&value_texts, &value_htmls);

    Ok(ParsedAbilityPage {
        champion_id: champion_id.to_string(),
        slot,
        variant: parse_variant(file_name),
        source_file: file_name.to_string(),

        name: value_texts.get("name").cloned().unwrap_or_default(),
        damage_type: value_texts
            .get("damagetype")
            .cloned()
            .filter(|v| !v.is_empty()),
        spell_effects: value_texts
            .get("spelleffects")
            .cloned()
            .filter(|v| !v.is_empty()),

        effects,
    })
}

fn build_effect_blocks(
    value_texts: &BTreeMap<String, String>,
    value_htmls: &BTreeMap<String, String>,
) -> Vec<ParsedEffectBlock> {
    const ENDINGS: [&str; 12] = ["", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    let mut out = Vec::new();

    for suffix in ENDINGS {
        let desc_key = format!("description{suffix}");
        let lvl_key = format!("leveling{suffix}");

        let description = value_texts
            .get(&desc_key)
            .cloned()
            .filter(|v| !v.is_empty());
        let description_raw_html = value_htmls
            .get(&desc_key)
            .cloned()
            .filter(|v| !v.is_empty());

        let leveling_raw_html = value_htmls.get(&lvl_key).cloned().filter(|v| !v.is_empty());

        if description.is_none() && leveling_raw_html.is_none() {
            continue;
        }

        let effect_index = out.len();

        let levelings = leveling_raw_html
            .as_deref()
            .map(parse_leveling_lines)
            .unwrap_or_default();

        out.push(ParsedEffectBlock {
            effect_index,
            suffix: suffix.to_string(),
            description,
            description_raw_html,
            leveling_raw_html,
            levelings,
        });
    }

    out
}

fn parse_leveling_lines(raw_html: &str) -> Vec<ParsedLevelingLine> {
    let fragment = Html::parse_fragment(raw_html);
    let selector = Selector::parse("dt, dd").unwrap();

    let mut parts = Vec::<(String, String, String)>::new();

    for element in fragment.select(&selector) {
        let tag = element.value().name().to_string();
        let text = clean_text(&element.text().collect::<String>());
        let inner = element.inner_html();
        parts.push((tag, text, inner));
    }

    if parts.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 1 < parts.len() {
        let (tag_a, attr, _) = &parts[i];
        let (tag_b, data, raw) = &parts[i + 1];

        if tag_a == "dt" && tag_b == "dd" {
            out.push(ParsedLevelingLine {
                leveling_index: out.len(),
                attribute: attr.trim_end_matches(':').trim().to_string(),
                data: data.clone(),
                raw_html: raw.clone(),
            });
            i += 2;
        } else {
            i += 1;
        }
    }

    out
}

fn parse_variant(file_name: &str) -> Option<usize> {
    let stem = file_name.strip_suffix(".html").unwrap_or(file_name);
    let digits = stem
        .chars()
        .skip(1)
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    digits.parse::<usize>().ok()
}
