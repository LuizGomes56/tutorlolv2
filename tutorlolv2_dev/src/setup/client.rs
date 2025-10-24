use crate::{
    FileWrite, JsonRead, JsonWrite, MayFail,
    champions::MerakiChampion,
    get_file_names,
    init::ENV_CONFIG,
    items::MerakiItem,
    model::riot::{RiotCdnChampion, RiotCdnRune},
    read_file,
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

    pub async fn fetch_meraki_api<T: DeserializeOwned>(&self, endpoint: &str) -> MayFail<T> {
        let url = format!("{}/{endpoint}.json", ENV_CONFIG.meraki_endpoint);
        println!("fetch_meraki_api: {}", url);
        let result = self.get(&url).send().await?;
        let data = result.json::<T>().await?;
        Ok(data)
    }

    pub async fn fetch_riot_api<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        language: &str,
    ) -> MayFail<T> {
        let url = format!("{}/data/{language}/{endpoint}.json", Self::ddragon_url());
        println!("fetch_riot_api: {url}");
        let result = self.get(&url).send().await?;
        let data = result.json::<T>().await?;
        Ok(data)
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

    pub async fn fetch_languages(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let result = self
            .get(format!(
                "{}/cdn/languages.json",
                ENV_CONFIG.dd_dragon_endpoint
            ))
            .send()
            .await?;
        let data = result.json::<Vec<String>>().await?;
        Ok(data)
    }

    pub async unsafe fn update_env_version(&self) -> MayFail {
        let version = self.fetch_version().await?;
        Ok(unsafe { set_env_var("LOL_VERSION", &version)? })
    }

    /// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
    /// Runs a maximum of 32 tokio threads at the same time
    pub async fn update_riot_cache(&self) -> MayFail {
        let champions_json = self
            .fetch_riot_api::<RiotCdnStandard>("champion", &ENV_CONFIG.lol_language)
            .await?;

        let mut champions_futures = Vec::<JoinHandle<_>>::new();
        let semaphore = Arc::new(Semaphore::new(1 << 5));

        for (champion_id, _) in champions_json.data.clone() {
            let client = self.clone();
            let semaphore = semaphore.clone();

            champions_futures.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let champion_data = client
                    .fetch_riot_api::<RiotCdnStandard>(
                        &format!("champion/{champion_id}"),
                        &ENV_CONFIG.lol_language,
                    )
                    .await
                    .unwrap();

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

        champions_json.into_file("cache/riot/champions.json")?;

        let items_json = self
            .fetch_riot_api::<RiotCdnStandard>("item", &ENV_CONFIG.lol_language)
            .await?;

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

        items_json.into_file("cache/riot/items.json")?;

        let runes_json = self
            .fetch_riot_api::<Value>("runesReforged", &ENV_CONFIG.lol_language)
            .await?;

        runes_json.into_file("cache/riot/runes.json")?;

        self.update_language_cache().await?;

        let languages = Vec::<String>::from_file("internal/languages.json")?;
        let mut languages_data = HashMap::<String, Vec<String>>::from_iter(
            champions_json
                .data
                .iter()
                .map(|(champion_id, _)| (champion_id.clone(), Vec::new())),
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
                let champions_json = client
                    .fetch_riot_api::<RiotCdnStandard<NameField>>("champion", &language)
                    .await
                    .unwrap();

                let mut result = HashMap::new();
                for (champion_id, name_field) in champions_json.data {
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
        self.fetch_languages()
            .await?
            .into_file("internal/languages.json")
    }

    pub async fn update_meraki_cache(&self, endpoint: &str) -> MayFail {
        macro_rules! order_and_save {
            ($ty:ty) => {
                save_cache(
                    self.fetch_meraki_api::<HashMap<String, $ty>>(endpoint)
                        .await?,
                    endpoint,
                )
            };
        }

        match endpoint {
            "champions" => order_and_save!(MerakiChampion),
            "items" => order_and_save!(MerakiItem),
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

            let bytes = read_file(path)?;
            let html = Html::parse_document(&String::from_utf8(bytes)?);

            let mut result = HashMap::<String, Vec<Vec<String>>>::new();

            let combo_section = Selector::parse("div.m-1o7d3sk")?;
            let combo_span = Selector::parse("span.m-1pm4585.e1o1aytf0")?;

            for combo_div in html.select(&combo_section) {
                for combo_span in combo_div.select(&combo_span) {
                    let mut combo_strings = Vec::new();
                    if let Some(text) = combo_span.text().next() {
                        combo_strings.push(text.to_string());
                    };
                    let _ = result
                        .entry(champion_id.clone())
                        .or_insert(vec![])
                        .push(combo_strings);
                }
            }

            result.into_file(format!("internal/scraper/combos/{champion_id}.json"))?;
        }
        Ok(())
    }

    pub async fn call_scraper(&self) -> MayFail {
        let champion_ids = get_file_names("cache/riot/champions")?;
        let mut collected_results = HashMap::<String, HashMap<String, _>>::default();

        for champion_id in champion_ids {
            let mut futures_vec = Vec::<JoinHandle<Result<_, String>>>::new();
            for position in ["top", "jungle", "mid", "adc", "support"] {
                let name = champion_id.to_lowercase().clone();
                let client = self.clone();
                futures_vec.push(tokio::spawn(async move {
                    let path = format!("cache/scraper/builds/{position}/{name}.html");

                    let _ = client
                        .download(
                            format!("{}/{name}/build/{position}", ENV_CONFIG.meta_endpoint),
                            &path,
                        )
                        .await;

                    let mut result = HashMap::<String, _>::default();

                    let html = String::from_utf8(read_file(path).unwrap()).unwrap();

                    let document = Html::parse_document(&html);
                    let full_build =
                        Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img").unwrap();
                    let situational_build = Selector::parse(".m-s76v8c > div > div img").unwrap();
                    let rune_selector = Selector::parse("img.m-1nx2cdb").unwrap();
                    let legend_selector = Selector::parse("img.m-1u3ui07").unwrap();
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
                                        .replace("BladeofTheRuinedKing", "BladeoftheRuinedKing"),
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
                    Ok(result)
                }));
            }

            let mut collected_result = HashMap::<String, (Vec<String>, Vec<String>)>::default();
            for result in futures_vec {
                println!("Fetching meta items for {champion_id}");
                match result.await {
                    Ok(Ok(data)) => collected_result.extend(data),
                    Ok(Err(e)) => eprintln!("Error requesting: {e:#?}"),
                    Err(e) => eprintln!("Error awaiting future: {e:?}"),
                }
            }
            collected_results.insert(champion_id, collected_result);
        }

        collected_results.into_file("internal/scraper/data.json")
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
