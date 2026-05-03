use crate::{
    champions::{
        cache,
        template::{ChampionTemplate, SkillSet},
    },
    client::{MayFail, fetch},
    file_name, is_dir,
    parser::{
        Effect, EffectInner, SUFFIXES, get_cells, parse_base_damage, parse_description_effects,
        parse_scalings,
    },
    selector,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_types::{DamageType, Key};

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

                    ability
                        .effects
                        .values_mut()
                        .enumerate()
                        .try_for_each(|(i, effect)| {
                            effect.load_formula(ability.skill, &champion_id, i)
                        })?;

                    ability.champion_id = champion_id.clone();

                    crate::write(
                        path.with_extension("json"),
                        serde_json::to_string_pretty(&ability)?,
                    )?;

                    Ok(())
                })
        })
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WikiAbility {
    pub champion_id: String,
    pub damage_type: DamageType,
    pub name: String,
    pub skill: Key,
    pub spell_effects: Option<String>,
    pub effects: BTreeMap<String, Effect>,
}

pub fn parse_abilities(cells: &BTreeMap<String, String>) -> MayFail<WikiAbility> {
    let mut ability = WikiAbility::default();

    ability.name = cells["1"].clone();
    ability.skill = cells["skill"].parse::<char>()?.try_into()?;

    ability.damage_type = cells
        .get("damagetype")
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(DamageType::default);

    ability.spell_effects = cells.get("spelleffects").cloned();

    for (i, suffix) in SUFFIXES.into_iter().enumerate() {
        let get_key = |k: &str| format!("{k}{suffix}");

        let description_key = get_key("description");
        let leveling_key = get_key("leveling");

        let description = cells.get(&description_key);
        let description_raw = cells.get(&(description_key + "_raw"));
        let leveling = cells.get(&leveling_key);
        let leveling_raw = cells.get(&(leveling_key + "_raw"));

        if let Some(description) = description {
            if let Some(leveling) = leveling {
                let raw = leveling_raw.map(String::as_str).unwrap_or(leveling);

                if !raw.trim().is_empty() {
                    ability.effects.extend(parse_effects(i, description, raw)?);
                    continue;
                }
            }

            if ability.skill == Key::P {
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

    assert!(
        !dts.is_empty() && dts.len() == dds.len(),
        "No structured dd/dt tags found"
    );

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
                        .map(str::trim)
                        .filter(|v| !v.is_empty())
                        .filter_map(|v| v.parse().ok())
                        .collect()
                });

            Some((
                key,
                Effect {
                    index,
                    formula: None,
                    base: parse_base_damage(&dd_leveling),
                    inner: EffectInner {
                        description: description.to_string(),
                        leveling: dd_leveling,
                    },
                    use_formula,
                    use_values,
                    scalings: parse_scalings(&dd_leveling_raw),
                },
            ))
        })
        .collect())
}
