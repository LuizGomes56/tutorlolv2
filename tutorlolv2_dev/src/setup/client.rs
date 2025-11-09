use crate::{
    FileWrite, JsonRead, JsonWrite, MayFail,
    champions::MerakiChampion,
    get_file_names,
    init::ENV_CONFIG,
    items::MerakiItem,
    model::riot::{RiotCdnChampion, RiotCdnRune},
    read_file, resolve_path,
    riot::RiotCdnStandard,
};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    path::Path,
    sync::Arc,
};
use tokio::{sync::Semaphore, task::JoinHandle};

macro_rules! target_dir {
    () => {
        "raw_img"
    };
    ($first:literal) => {
        concat!(target_dir!(), "/", $first)
    };
}

#[derive(Clone)]
pub struct HttpClient(Client);

impl From<Client> for HttpClient {
    fn from(value: Client) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for HttpClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    pub async fn download(&self, url: impl AsRef<str>, save_to: impl AsRef<Path>) -> MayFail {
        let url = url.as_ref();
        let save_to = save_to.as_ref();
        if save_to.exists() {
            println!("[ALREADY_EXISTS] {save_to:?}");
            Ok(())
        } else {
            println!("[DOWNLOADING] {url}");
            match self.get(url).send().await {
                Ok(response) => {
                    let bytes = response.bytes().await?;
                    bytes.write_file(save_to)
                }
                Err(e) => {
                    println!("[ERROR] {e}");
                    Err(e.to_string().into())
                }
            }
        }
    }

    pub fn ddragon_url() -> String {
        format!(
            "{}/cdn/{}",
            ENV_CONFIG.dd_dragon_endpoint, ENV_CONFIG.lol_version
        )
    }

    pub async fn download_general_img(&self) -> MayFail {
        let riot_champions = RiotCdnChampion::from_dir("cache/riot/champions")?;
        let ddragon_url = Self::ddragon_url();
        let champion_dir = target_dir!("champions");
        let abilities_dir = target_dir!("abilities");
        for (champion_id, champion) in riot_champions {
            let mut futures = Vec::new();

            let _ = self.download(
                format!("{}/img/champion/{}", ddragon_url, champion.image.full),
                format!("{champion_dir}/{champion_id}.png"),
            );
            let _ = self.download(
                format!("{ddragon_url}/img/passive/{}", champion.passive.image.full),
                format!("{abilities_dir}/{champion_id}P.png"),
            );

            for (index, spell) in champion.spells.into_iter().enumerate() {
                let ddragon_url = ddragon_url.clone();
                let champion_id = champion_id.clone();
                let client = self.clone();
                futures.push(tokio::spawn(async move {
                    let _ = client.download(
                        format!("{ddragon_url}/img/spell/{}", spell.image.full),
                        format!(
                            "{abilities_dir}/{champion_id}{}.png",
                            ["Q", "W", "E", "R"][index]
                        ),
                    );
                }));
            }

            for future in futures {
                if let Err(e) = future.await {
                    println!("[ERROR] requesting {champion_id} images: {e}");
                };
            }
        }
        Ok(())
    }

    pub async fn download_items_img(&self) -> MayFail {
        let riot_items = get_file_names("cache/riot/items")?;
        let ddragon_url = Self::ddragon_url();
        let mut futures = Vec::new();
        for item_id in riot_items {
            let ddragon_url = ddragon_url.clone();
            let client = self.clone();
            futures.push(tokio::spawn(async move {
                let _ = client
                    .download(
                        format!("{ddragon_url}/img/item/{item_id}.png"),
                        format!("{}/{item_id}.png", target_dir!("items")),
                    )
                    .await;
            }));
        }
        for future in futures {
            let _ = future.await;
        }
        Ok(())
    }

    pub async fn download_arts_img(&self) -> MayFail {
        let riot_champions = RiotCdnChampion::from_dir("cache/riot/champions")?;
        let base_url = format!("{}/cdn", ENV_CONFIG.dd_dragon_endpoint);
        for (champion_id, champion) in riot_champions {
            let mut futures = Vec::new();
            for skin in champion.skins.into_iter() {
                let champion_id = champion_id.clone();
                let base_url = base_url.clone();
                let client = self.clone();
                futures.push(tokio::spawn(async move {
                    for label in ["centered", "splash"] {
                        let file_name = format!("{champion_id}_{}.jpg", skin.num);
                        let url = format!("{base_url}/img/champion/{label}/{file_name}",);
                        let label_dir = format!("{}/{label}", target_dir!());
                        let save_to = format!("{label_dir}/{file_name}");
                        let _ = client.download(url, save_to).await;
                    }
                }));
            }
            for future in futures {
                let _ = future.await;
            }
        }
        Ok(())
    }

    pub async fn download_runes_img(&self) -> MayFail {
        let riot_runes = Vec::<RiotCdnRune>::from_file("cache/riot/runes.json")?;
        let mut futures = Vec::new();
        let mut icon_map = Vec::<(usize, String)>::new();
        for value in riot_runes {
            icon_map.push((value.id, value.icon));
            for slot in value.slots {
                for rune in slot.runes {
                    icon_map.push((rune.id, rune.icon));
                }
            }
        }
        for (rune_id, rune_icon) in icon_map {
            let url = format!("{}/{rune_icon}", ENV_CONFIG.riot_image_endpoint);
            let file_path = format!("{}/{rune_id}.png", target_dir!("runes"));
            let client = self.clone();
            futures.push(tokio::spawn(async move {
                let _ = client.download(url, file_path);
            }));
        }
        for future in futures {
            let _ = future.await;
        }
        Ok(())
    }

    pub async fn fetch_version(&self) -> MayFail<String> {
        let result = self
            .get(format!(
                "{}/api/versions.json",
                ENV_CONFIG.dd_dragon_endpoint
            ))
            .send()
            .await?;

        let data = result.json::<Vec<String>>().await?;
        Ok(data
            .get(0)
            .ok_or_else(|| String::from("Version not found"))?
            .clone())
    }

    pub fn riot_endpoint(endpoint: &str, language: &str) -> String {
        format!("{}/data/{language}/{endpoint}.json", Self::ddragon_url())
    }

    pub async unsafe fn update_env_version(&self) -> MayFail {
        let version = self.fetch_version().await?;
        Ok(unsafe { set_env_var("LOL_VERSION", &version)? })
    }

    /// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
    /// Runs a maximum of 32 tokio threads at the same time
    pub async fn update_riot_cache(&self) -> MayFail {
        self.download(
            Self::riot_endpoint("champion", &ENV_CONFIG.lol_language),
            "cache/riot/champions.json",
        )
        .await?;

        let champions_json = RiotCdnStandard::<Value>::from_file("cache/riot/champions.json")?;

        let champion_ids = champions_json
            .data
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let mut champions_futures = Vec::<JoinHandle<_>>::new();
        let semaphore = Arc::new(Semaphore::new(1 << 5));

        for champion_id in champion_ids.clone() {
            let client = self.clone();
            let semaphore = semaphore.clone();

            champions_futures.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let save_to = format!("cache/riot/raw_champions/{champion_id}.json");

                client
                    .download(
                        Self::riot_endpoint(
                            &format!("champion/{champion_id}"),
                            &ENV_CONFIG.lol_language,
                        ),
                        &save_to,
                    )
                    .await
                    .unwrap();

                let champion_data = RiotCdnStandard::<Value>::from_file(save_to).unwrap();

                champion_data
                    .data
                    .get(&champion_id)
                    .unwrap()
                    .into_file(format!("cache/riot/champions/{champion_id}.json"))
                    .unwrap();
            }));
        }

        for future in champions_futures {
            if let Err(e) = future.await {
                println!("[CHAMPIONS] Task join error: {e:?}");
            }
        }

        let items_path = "cache/riot/items.json";
        self.download(
            Self::riot_endpoint("item", &ENV_CONFIG.lol_language),
            items_path,
        )
        .await?;

        let items_json = RiotCdnStandard::<Value>::from_file(items_path)?;

        let mut items_futures = Vec::<_>::new();

        for (item_id, item_data) in items_json.data.clone() {
            items_futures.push(tokio::task::spawn_blocking(move || {
                item_data
                    .into_file(format!("cache/riot/items/{item_id}.json"))
                    .unwrap();
            }));
        }

        for future in items_futures {
            if let Err(e) = future.await {
                println!("[ITEMS] Task join error: {e:?}");
            }
        }

        self.download(
            Self::riot_endpoint("runesReforged", &ENV_CONFIG.lol_language),
            "cache/riot/runes.json",
        )
        .await?;

        self.update_language_cache().await?;
        let languages = Vec::<String>::from_file("internal/languages.json")?;

        let mut languages_data = HashMap::<String, Vec<String>>::from_iter(
            champion_ids
                .into_iter()
                .map(|champion_id| (champion_id, Vec::new())),
        );

        let mut languages_future = Vec::new();

        #[derive(Deserialize)]
        struct NameField {
            name: String,
        }

        for language in languages {
            let language = language.clone();
            let client = self.clone();
            languages_future.push(tokio::spawn(async move {
                let champion_file = format!("cache/riot/champions_lang/{language}.json");
                client
                    .download(Self::riot_endpoint("champion", &language), &champion_file)
                    .await
                    .unwrap();

                let champion_lang = RiotCdnStandard::<NameField>::from_file(champion_file).unwrap();

                let mut result = HashMap::new();
                for (champion_id, name_field) in champion_lang.data {
                    result.insert(champion_id, name_field.name);
                }
                result
            }))
        }

        for future in languages_future {
            if let Ok(data) = future.await {
                for (champion_id, champion_name) in data.into_iter() {
                    match languages_data.get_mut(&champion_id) {
                        Some(v) => {
                            if !v.contains(&champion_name) {
                                v.push(champion_name)
                            }
                        }
                        None => {
                            let _ = languages_data.insert(champion_id, vec![champion_name]);
                        }
                    };
                }
            }
        }

        languages_data.into_file("internal/champion_languages.json")
    }

    pub async fn update_language_cache(&self) -> MayFail {
        self.download(
            format!("{}/cdn/languages.json", ENV_CONFIG.dd_dragon_endpoint),
            "internal/languages.json",
        )
        .await
    }

    pub async fn update_meraki_cache(&self, endpoint: &str) -> MayFail {
        let save_to = format!("cache/meraki/{endpoint}.json");
        self.download(
            format!("{}/{endpoint}.json", ENV_CONFIG.meraki_endpoint),
            &save_to,
        )
        .await?;

        fn save_and_order<T: DeserializeOwned + Serialize>(
            path: impl AsRef<Path>,
            endpoint: &str,
        ) -> MayFail
        where
            HashMap<String, T>: OrderJson<T>,
        {
            let data = HashMap::<String, T>::from_file(path)?;
            save_cache(data, endpoint)
        }

        match endpoint {
            "champions" => save_and_order::<MerakiChampion>(save_to, endpoint),
            "items" => save_and_order::<MerakiItem>(save_to, endpoint),
            _ => panic!("Called update_meraki_cache with invalid endpoint"),
        }
    }

    pub async fn combo_scraper(&self) -> MayFail {
        let champion_ids = get_file_names("cache/riot/champions")?;

        for champion_id in champion_ids {
            let path = format!("cache/scraper/combos/{champion_id}.html");
            self.download(
                format!("{}/{champion_id}/combos", ENV_CONFIG.meta_endpoint),
                &path,
            )
            .await?;

            tokio::task::spawn_blocking(move || {
                let run_task = || -> MayFail {
                    let bytes = read_file(path)?;
                    let html = Html::parse_document(&String::from_utf8(bytes)?);

                    let mut result = Vec::<Vec<String>>::new();

                    let combo_section = Selector::parse("div.m-1o7d3sk")?;
                    let combo_span = Selector::parse("span.m-1pm4585.e1o1aytf0")?;

                    for combo_div in html.select(&combo_section) {
                        let mut combo_strings = Vec::new();

                        for combo_span in combo_div.select(&combo_span) {
                            if let Some(text) = combo_span.text().next() {
                                combo_strings.push(text.to_string());
                            };
                        }
                        result.push(combo_strings);
                    }

                    result.into_file(format!("internal/scraper/combos/{champion_id}.json"))
                };
                match run_task() {
                    Ok(_) => (),
                    Err(e) => println!("Error scraping combo for {champion_id:?}: {e:?}."),
                }
            });
        }
        Ok(())
    }

    pub async fn call_scraper(&self) -> MayFail {
        let champion_ids = get_file_names("cache/riot/champions")?;

        for champion_id in champion_ids.iter() {
            let mut futures_vec = Vec::new();

            for position in ["top", "jungle", "mid", "adc", "support"] {
                let champion_id = champion_id.clone();
                let name = champion_id.to_lowercase();
                let client = self.clone();

                futures_vec.push(tokio::spawn(async move {
                    let path = format!("scraper/builds/{position}/{champion_id}");
                    let cache_path = format!("cache/{path}.html");
                    let internal_path = format!("internal/{path}.json");

                    let _ = client
                        .download(
                            format!("{}/{name}/build/{position}", ENV_CONFIG.meta_endpoint),
                            &cache_path,
                        )
                        .await;

                    tokio::task::spawn_blocking(move || {
                        let run_task = || -> MayFail {
                            let mut result = HashMap::<String, _>::default();

                            let html = String::from_utf8(read_file(&cache_path)?)?;

                            let document = Html::parse_document(&html);
                            let full_build =
                                Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")?;
                            let situational_build = Selector::parse(".m-s76v8c > div > div img")?;
                            let rune_selector = Selector::parse("img.m-1nx2cdb")?;
                            let legend_selector = Selector::parse("img.m-1u3ui07")?;
                            let mut runes = Vec::<String>::new();

                            let push_alt_attr = |array: &mut Vec<String>, selector: &Selector| {
                                for img in document.select(selector) {
                                    if let Some(alt) = img.value().attr("alt") {
                                        array.push(
                                            alt.replace(" ", "")
                                                .replace("-", "")
                                                .replace(")", "")
                                                .replace("(", "")
                                                .replace("'", "")
                                                .replace(".", "")
                                                .replace(",", "")
                                                .replace(":", "")
                                                .replace(
                                                    "BladeofTheRuinedKing",
                                                    "BladeoftheRuinedKing",
                                                ),
                                        );
                                    }
                                }
                            };

                            let mut items = Vec::<String>::new();

                            push_alt_attr(&mut runes, &rune_selector);
                            push_alt_attr(&mut runes, &legend_selector);
                            push_alt_attr(&mut items, &full_build);
                            push_alt_attr(&mut items, &situational_build);

                            result.insert(String::from(position), (items, runes));
                            result.into_file(internal_path)
                        };
                        match run_task() {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error processing HTML from {champion_id:?}: {e:#?}")
                            }
                        }
                    });
                }));
            }

            for future in futures_vec {
                let _ = future.await;
            }
        }

        let mut results = HashMap::<String, HashMap<String, (Vec<String>, Vec<String>)>>::new();

        fn wait_ready(
            raw_path: impl AsRef<Path>,
        ) -> MayFail<HashMap<String, (Vec<String>, Vec<String>)>> {
            let mut attempts = 0;
            loop {
                let resolved_path = resolve_path(&raw_path)?;
                let path = resolved_path.as_ref();
                match HashMap::<String, (Vec<String>, Vec<String>)>::from_file(path) {
                    Ok(m) => return Ok(m),
                    Err(e) => {
                        if !path.exists() {
                            eprintln!("File for {path:?} might not yet have been created");
                        } else if attempts > 12 {
                            eprintln!(
                                "Failed to load {path:?} after 12 attempts. Throwing error: {e:?}."
                            );
                            return Err(e);
                        } else {
                            eprintln!("Retrying {path:?} (error: {e:?})");
                            attempts += 1;
                        }
                        std::thread::sleep(std::time::Duration::from_secs(5));
                    }
                }
            }
        }

        for champion_id in champion_ids {
            for position in ["top", "jungle", "mid", "adc", "support"] {
                let data = wait_ready(format!(
                    "internal/scraper/builds/{position}/{champion_id}.json"
                ))?;
                results.insert(champion_id.clone(), data);
            }
        }

        results.into_file("internal/scraper/data.json")
    }
}

unsafe fn set_env_var(key: &str, value: &str) -> std::io::Result<()> {
    let path = ".env";
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut found = false;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with(&format!("{}=", key)) {
            lines.push(format!("{}={}", key, value));
            found = true;
        } else {
            lines.push(line);
        }
    }
    if !found {
        lines.push(format!("{}={}", key, value));
    }
    let mut out = std::fs::File::create(path)?;
    for l in lines {
        writeln!(out, "{}", l)?;
    }
    Ok(())
}

pub trait OrderJson<T: Serialize> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, T)>;
}

impl OrderJson<MerakiChampion> for HashMap<String, MerakiChampion> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, MerakiChampion)> {
        let mut vec_self = self.into_iter().collect::<Vec<_>>();
        for (_, champion) in vec_self.iter_mut() {
            for ability_list in [
                &mut champion.abilities.q,
                &mut champion.abilities.w,
                &mut champion.abilities.e,
                &mut champion.abilities.r,
                &mut champion.abilities.p,
            ] {
                for ability in ability_list {
                    ability
                        .effects
                        .sort_by(|a, b| a.description.cmp(&b.description));
                    for effect in &mut ability.effects {
                        effect.leveling.sort_by(|a, b| {
                            a.attribute
                                .as_deref()
                                .unwrap_or("")
                                .cmp(b.attribute.as_deref().unwrap_or(""))
                        });
                    }
                }
            }
        }
        vec_self.into_iter()
    }
}

impl OrderJson<MerakiItem> for HashMap<String, MerakiItem> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, MerakiItem)> {
        let mut vec_self = self.into_iter().collect::<Vec<_>>();
        for (_, item) in vec_self.iter_mut() {
            item.active.sort_by(|a, b| a.effects.cmp(&b.effects));
            item.passives.sort_by(|a, b| a.effects.cmp(&b.effects));
        }
        vec_self.into_iter()
    }
}

pub fn save_cache<T: Serialize>(result: impl OrderJson<T>, endpoint: &str) -> MayFail {
    for (key, value) in result.into_iter_ord() {
        value.into_file(format!("cache/meraki/{endpoint}/{key}.json"))?;
    }
    Ok(())
}
