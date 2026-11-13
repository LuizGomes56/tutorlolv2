use {
    crate::{
        HTTP_CLIENT, MayFail, file_name, is_dir,
        parser::{Effect, SUFFIXES},
        selector,
    },
    heck::ToPascalCase,
    rayon::iter::{ParallelBridge, ParallelIterator},
    scraper::Html,
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    serde_json::{Map, Value, json},
    std::{collections::BTreeMap, path::PathBuf},
};

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

    let data = HTTP_CLIENT
        .fetch(save_to, "Category:Rune_data_templates")
        .await?;
    let html = Html::parse_document(&data);

    let sections = html
        .select(selector!(
            "div.mw-category.mw-category-columns > div.mw-category-group a"
        ))
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
                (name.to_pascal_case(), RuneLink { name, href })
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

        HTTP_CLIENT
            .fetch_raw(
                &save_to.join("data.lua"),
                &format!("{}?action=raw", link.href),
            )
            .await?;

        std::thread::sleep(std::time::Duration::from_millis(200));
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
            let data = crate::read_to_string(path.join("data.lua"))?;
            let nodes = crate::wikitext::Nodes::parse(&data);
            let template = nodes
                .templates()
                .into_iter()
                .find(|template| template.named("description").is_some())
                .ok_or("Missing raw Rune data template")?;

            let mut cells = BTreeMap::new();

            cells.insert(
                "1".to_string(),
                template.positional(0).ok_or("Missing rune name")?.plain(),
            );

            for arg in &template.args {
                if let Some(name) = &arg.name {
                    cells.insert(name.clone(), arg.value.plain());
                }
            }

            let result = json!(cells);
            let json = serde_json::to_string_pretty(&result)?;

            crate::write(path.join("data").with_extension("json"), &json)?;

            Ok(())
        })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RuneSlot {
    Keystone,
    Position(usize),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RuneKeystone {
    Precision,
    Domination,
    Sorcery,
    Resolve,
    Inspiration,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiRune {
    #[serde(default)]
    pub raw_source: String,
    pub name: String,
    pub rune_id: String,
    pub path: RuneKeystone,
    pub slot: RuneSlot,
    pub effects: BTreeMap<String, Effect>,
    pub descriptions: Vec<String>,
    pub riot_id: usize,
    pub custom: bool,
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
            let object = json.as_object()?;
            let mut descriptions = Vec::new();

            for suffix in SUFFIXES {
                if let Some(desc) = object.get(&format!("description{suffix}"))
                    && let Some(description) = desc.as_str()
                {
                    descriptions.push(description.to_string());
                }
            }

            fn get_value<T: DeserializeOwned>(object: &Map<String, Value>, key: &str) -> Option<T> {
                serde_json::from_value(object.get(key)?.clone()).ok()
            }

            let get_str = |key: &str| object.get(key)?.as_str();
            let slot = match get_str("slot")? {
                "Keystone" => RuneSlot::Keystone,
                n => RuneSlot::Position(n.parse().ok()?),
            };

            let rune = WikiRune {
                raw_source: crate::read_to_string(path.join("data.lua")).ok()?,
                name: get_str("1")?.to_string(),
                rune_id: key.clone(),
                path: get_value(object, "path")?,
                slot,
                effects: BTreeMap::new(),
                descriptions,
                riot_id: 0,
                custom: get_str("removed").map(|s| s == "true").unwrap_or(false),
            };

            Some((key, rune))
        })
        .collect::<BTreeMap<_, _>>();

    let json = serde_json::to_string_pretty(&runes)?;

    crate::write(cache().join("full").with_extension("json"), &json)
}

pub async fn run() -> MayFail {
    links().await?;
    download().await?;
    parse()?;
    concat()
}
