use crate::{
    champions::{
        cache, clean_text,
        template::{ChampionTemplate, SkillSet},
    },
    client::{MayFail, fetch},
    file_name, is_dir,
    parser::get_cells,
    selector,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use regex::{Regex, RegexBuilder};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, sync::LazyLock};
use tutorlolv2_types::{CtxVar::*, DamageType};

pub async fn download() -> MayFail {
    println!("[fn] champions::abilities::download");

    for entry in crate::read_dir(cache())?.filter(is_dir) {
        let path = entry.path();

        let bytes = crate::read(path.join("template").with_extension("json"))?;

        let ChampionTemplate {
            name,
            skills:
                SkillSet {
                    skill_i,
                    skill_q,
                    skill_w,
                    skill_e,
                    skill_r,
                },
            ..
        } = serde_json::from_slice(&bytes)
            .map_err(|e| format!("[error] Failed to deserialize entry: {entry:?}: {e:?}"))?;

        println!("[download] Processing {name:?}");

        let save_to = path.join("abilities");

        std::fs::create_dir_all(&save_to)
            .map_err(|e| format!("[error] Failed to create directory: {e:?}"))?;

        for (key, skills) in [
            ('P', skill_i),
            ('Q', skill_q),
            ('W', skill_w),
            ('E', skill_e),
            ('R', skill_r),
        ] {
            for (i, skill) in skills.into_iter().enumerate() {
                let url = format!("Template:Data_{name}/{skill}");

                fetch(
                    save_to.join(format!("{key}{i}")).with_extension("html"),
                    &url,
                )
                .await?;
            }
        }
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] champions::abilities::parse");

    crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let path = entry.path();

            let champion_id = file_name(&entry)?;

            println!("[{champion_id:?}] Processing abilities");

            crate::read_dir(path.join("abilities"))?
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|v| v == "html")
                        .unwrap_or(false)
                })
                .par_bridge()
                .into_par_iter()
                .try_for_each(|entry| {
                    let file_name = file_name(&entry)?;

                    let key = file_name.chars().next().ok_or_else(|| {
                        format!("[error] Failed to get key from file name: {file_name:?}")
                    })?;

                    let path = entry.path();

                    let bytes = crate::read_to_string(&path)?;
                    let html = Html::parse_document(&bytes);

                    if let Some(element) = html.select(&selector("div.noarticletext > p")?).next()
                        && element
                            .text()
                            .any(|text| text.contains("There is currently no text in this page"))
                    {
                        return Ok(());
                    }

                    let cells = get_cells(&html)?;

                    let mut ability = parse_abilities(&cells)?;
                    ability.champion_id = &champion_id;

                    std::fs::write(
                        path.with_extension("json"),
                        serde_json::to_string_pretty(&ability)?,
                    )?;

                    Ok(())
                })
        })
}

#[derive(Default, Deserialize, Serialize)]
pub struct Ability<'a> {
    pub champion_id: &'a str,
    pub damage_type: DamageType,
    pub name: &'a str,
    pub skill: char,
    pub spell_effects: Option<&'a str>,
    pub effects: BTreeMap<String, Effect>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct AbilityInner {
    pub description: String,
    pub leveling: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Effect {
    pub index: usize,
    pub inner: AbilityInner,
    pub scalings: Vec<String>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<String>>,
    pub base: Option<Vec<String>>,
}

pub fn parse_abilities(cells: &BTreeMap<String, String>) -> MayFail<Ability<'_>> {
    let mut ability = Ability::default();

    ability.name = &cells["1"];

    ability.skill = match cells["skill"].parse::<char>()? {
        'I' => 'P',
        v => v,
    };

    ability.damage_type = cells
        .get("damagetype")
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(DamageType::default);

    ability.spell_effects = cells.get("spelleffects").map(String::as_str);

    for i in 0..10 {
        let suffix: &dyn Display = match i {
            0 => &"",
            _ => &i,
        };

        let get_key = |k: &str| format!("{k}{suffix}");

        let description = cells.get(&get_key("description"));
        let description_raw = cells.get(&(get_key("description") + "_raw"));
        let leveling = cells.get(&get_key("leveling"));
        let leveling_raw = cells.get(&(get_key("leveling") + "_raw"));

        if let Some(description) = cells.get(&get_key("description"))
            && let Some(leveling) = cells.get(&get_key("leveling"))
        {
            let raw = cells
                .get(&(get_key("leveling") + "_raw"))
                .map(String::as_str)
                .unwrap_or(leveling);

            if !raw.trim().is_empty() {
                ability.effects.extend(parse_effects(i, description, raw)?);
                continue;
            }
        }

        if ability.skill == 'P' {
            if let Some(description) = description {
                let raw = description_raw.map(String::as_str).unwrap_or(description);
                ability
                    .effects
                    .extend(parse_description_effects(i, description, raw)?);
            }
        }
    }

    Ok(ability)
}

fn parse_effects(
    index: usize,
    description: &str,
    leveling_raw: &str,
) -> MayFail<Vec<(String, Effect)>> {
    let fragment = Html::parse_fragment(leveling_raw);

    let tooltip_selector = selector("span.pp-tooltip")?;

    let dts = fragment.select(&selector("dt")?).collect::<Vec<_>>();
    let dds = fragment.select(&selector("dd")?).collect::<Vec<_>>();

    if !dts.is_empty() && dts.len() == dds.len() {
        Ok(dts
            .into_iter()
            .zip(dds.into_iter())
            .filter_map(|(dt, dd)| {
                let key = dt
                    .select(&selector("span.template_lc").ok()?)
                    .next()
                    .map(|e| e.text())
                    .unwrap_or(dt.text())
                    .collect::<String>()
                    .trim()
                    .trim_end_matches(':')
                    .trim()
                    .to_string();

                if key.is_empty() {
                    return None;
                }

                let dd_leveling = dd.text().collect::<String>().trim().to_string();
                let dd_leveling_raw = dd.inner_html();
                let tooltip = dd.select(&tooltip_selector).next();

                let use_formula = tooltip
                    .and_then(|el| el.value().attr("data-useformula"))
                    .map(str::to_string);

                let use_values = tooltip
                    .and_then(|el| el.value().attr("data-bot-values"))
                    .map(|s| {
                        s.split(';')
                            .map(|v| v.trim().to_string())
                            .filter(|v| !v.is_empty())
                            .collect()
                    });

                Some((
                    key,
                    Effect {
                        index,
                        base: parse_base_damage(&dd_leveling),
                        inner: AbilityInner {
                            description: description.to_string(),
                            leveling: dd_leveling,
                        },
                        use_formula,
                        use_values,
                        scalings: parse_scalings(&clean_text(&dd_leveling_raw)),
                    },
                ))
            })
            .collect())
    } else {
        panic!("No structured dd/dt tags found")
    }
}

fn parse_description_effects(
    index: usize,
    description: &str,
    description_raw: &str,
) -> MayFail<BTreeMap<String, Effect>> {
    let fragment = Html::parse_fragment(description_raw);

    let tooltip = fragment.select(&selector("span.pp-tooltip")?).next();

    let use_formula = tooltip
        .and_then(|el| el.value().attr("data-useformula"))
        .map(str::to_string);

    let use_values = tooltip
        .and_then(|el| el.value().attr("data-bot-values"))
        .map(|s| {
            s.split(';')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>()
        });

    let key = fragment
        .select(&selector("span.template_sbc")?)
        .next()
        .and_then(|node| {
            node.select(&selector("b").ok()?)
                .next()
                .map(|b| b.text().collect::<String>())
        })
        .map(|s| s.trim().trim_end_matches(':').trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(format!("Description {index}"));

    let mut map = BTreeMap::new();

    map.insert(
        key,
        Effect {
            index,
            inner: AbilityInner {
                description: description.to_string(),
                leveling: String::new(),
            },
            scalings: parse_scalings(&clean_text(description_raw)),
            use_formula,
            use_values,
            base: parse_base_damage(description),
        },
    );

    Ok(map)
}

fn parse_base_damage(text: &str) -> Option<Vec<String>> {
    static RANKED_VALUES_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"-?\d+(?:\.\d+)?%?(?:\s*/\s*-?\d+(?:\.\d+)?%?)+").unwrap());

    fn get_base_text(s: &str) -> &str {
        for (i, char) in s.char_indices() {
            if char == '(' {
                return &s[..i];
            }
        }
        s
    }

    let base = get_base_text(text).trim();

    let values = RANKED_VALUES_RE
        .find(base)?
        .as_str()
        .split('/')
        .map(|v| v.trim().trim_end_matches('%').trim())
        .filter(|v| !v.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    match values.is_empty() {
        true => None,
        false => Some(values),
    }
}

fn parse_scalings(input: &str) -> Vec<String> {
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 1 < chars.len() {
        let (_, c1) = chars[i];
        let (_, c2) = chars[i + 1];

        if c1 == '(' && c2 == '+' {
            let start = chars[i].0;
            let mut depth = 1usize;
            let mut j = i + 1;

            while j + 1 < chars.len() {
                j += 1;
                let (byte, ch) = chars[j];

                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end = byte + ch.len_utf8();
                            out.push(input[start..end].trim().to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }

            i = j;
        } else {
            i += 1;
        }
    }

    fn dedup_strings(list: &mut Vec<String>) {
        list.retain(|s| !s.trim().is_empty());

        let mut temp = Vec::new();
        for item in core::mem::take(list) {
            if !temp.contains(&item) {
                temp.push(item);
            }
        }

        *list = temp;
    }

    dedup_strings(&mut out);
    out
}
