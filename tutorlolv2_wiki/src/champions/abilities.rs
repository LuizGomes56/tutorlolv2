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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedAbilityPage {
    pub champion_id: String,
    pub slot: char,
    pub variant: Option<usize>,
    pub source_file: String,

    pub name: String,
    pub damage_type: DamageType,
    pub spell_effects: Option<String>,

    pub effects: Vec<ParsedEffectBlock>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedEffectBlock {
    pub index: usize,
    pub suffix: String,
    pub description: Option<String>,
    pub raw_description: Option<String>,
    pub raw_leveling: Option<String>,
    pub levelings: Vec<ParsedLevelingLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedLevelingLine {
    pub leveling_index: usize,
    pub attribute: String,
    pub data: String,
    pub formula_attempt: Vec<String>,
    pub raw_html: String,
    pub from_description: bool,
    pub level_ranges: Vec<(usize, usize)>,
}

pub async fn download() -> MayFail {
    println!("[fn] champions::abilities::download");

    for entry in crate::read_dir(cache())?.filter(is_dir) {
        let path = entry.path();

        let champion_id = file_name(&entry)?;

        println!("[dir] Processing {champion_id:?}");

        let bytes = crate::read(path.join("template").with_extension("json"))?;

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

        for (key, skills) in [
            ('P', skill_i),
            ('Q', skill_q),
            ('W', skill_w),
            ('E', skill_e),
            ('R', skill_r),
        ] {
            for (i, skill) in skills.into_iter().enumerate() {
                let url = format!("Template:Data_{champion_id}/{skill}");

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

                    std::fs::write(
                        path.with_extension("json"),
                        serde_json::to_string_pretty(&cells)?,
                    )?;

                    Ok(())
                })
        })
}
