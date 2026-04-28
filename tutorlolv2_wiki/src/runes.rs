use crate::{
    client::{MayFail, fetch},
    file_name, is_dir,
    parser::{SUFFIXES, get_cells, parse_description_effects},
    selector,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use scraper::Html;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::PathBuf};
use tutorlolv2_fmt::pascal_case;

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/runes")
}

#[derive(Deserialize, Serialize)]
pub struct RuneLink<'a> {
    pub name: &'a str,
    pub href: &'a str,
}

pub async fn links() -> MayFail {
    println!("[fn] runes::links");

    let save_to = cache().join("links").with_extension("html");

    let data = fetch(save_to, "Category:Rune_data_templates").await?;
    let html = Html::parse_document(&data);

    let sections_selector =
        selector("div.mw-category.mw-category-columns > div.mw-category-group a")?;

    let sections = html
        .select(&sections_selector)
        .filter_map(|a| {
            let text = a.text().collect::<String>();
            let href = a.attr("href")?.trim_start_matches("/en-us/");

            Some((text, href))
        })
        .collect::<BTreeMap<_, _>>();

    let get_links = |save_to, trim| -> MayFail {
        let links = sections
            .iter()
            .filter(|(k, _)| k.contains(trim) && !k.contains("Trait:"))
            .map(|(text, href)| {
                let name = text.trim_start_matches(trim).trim();
                (pascal_case(&name), RuneLink { name, href })
            })
            .collect::<BTreeMap<_, _>>();

        let json = serde_json::to_string_pretty(&links)?;
        crate::write(cache().join(save_to).with_extension("json"), &json)?;

        Ok(())
    };

    get_links("links_wr", "Template:WR Rune data")?;
    get_links("links_lol", "Template:Rune data")
}

pub async fn download() -> MayFail {
    println!("[fn] runes::download");

    let links_file = crate::read(cache().join("links_lol").with_extension("json"))?;
    let links = serde_json::from_slice::<BTreeMap<String, RuneLink>>(&links_file)?;

    for (id, link) in links {
        let save_to = cache().join(id);
        fetch(save_to.join("data").with_extension("html"), link.href).await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] runes::parse");

    crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .try_for_each(|entry| {
            let path = entry.path();

            let data = crate::read_to_string(path.join("data").with_extension("html"))?;
            let html = Html::parse_document(&data);

            let cells = get_cells(&html)?;

            let mut result = json!(cells);

            let effects = SUFFIXES
                .into_iter()
                .enumerate()
                .filter_map(|(i, suffix)| {
                    let key = format!("description{suffix}");

                    match cells.get(&key) {
                        Some(description)
                            if let Some(description_raw) = cells.get(&(key + "_raw")) =>
                        {
                            parse_description_effects(i, description, description_raw).ok()
                        }
                        _ => None,
                    }
                })
                .flatten()
                .collect::<BTreeMap<_, _>>();

            result
                .as_object_mut()
                .ok_or("[unlikely] Failed to transform cells in a map")?
                .insert("effects".into(), json!(effects));

            let json = serde_json::to_string_pretty(&result)?;

            crate::write(path.join("data").with_extension("json"), &json)?;

            Ok(())
        })
}

pub fn concat() -> MayFail {
    println!("[fn] runes::concat");

    let runes = crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();
            let key = file_name(&entry).ok()?;
            let bytes = crate::read(path.join("data").with_extension("json")).ok()?;
            let json = serde_json::from_slice::<Value>(&bytes).ok()?;
            Some((key, json))
        })
        .collect::<BTreeMap<_, _>>();

    let json = serde_json::to_string_pretty(&runes)?;
    crate::write(cache().join("data").with_extension("json"), &json)
}

pub async fn run() -> MayFail {
    links().await?;
    download().await?;
    parse()?;
    concat()
}
