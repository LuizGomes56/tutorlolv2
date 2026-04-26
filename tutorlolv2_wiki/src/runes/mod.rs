// https://wiki.leagueoflegends.com/en-us/Template:Rune%20data%20Electrocute
// https://wiki.leagueoflegends.com/en-us/Template:Rune%20data%20<name>

use crate::{
    client::{MayFail, fetch},
    is_dir,
    parser::get_cells,
    selector,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use scraper::Html;
use serde::{Deserialize, Serialize};
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
            let json = serde_json::to_string_pretty(&cells)?;

            crate::write(path.join("data").with_extension("json"), &json)?;

            Ok(())
        })
}

pub async fn run() -> MayFail {
    links().await?;
    download().await?;
    parse()
}
