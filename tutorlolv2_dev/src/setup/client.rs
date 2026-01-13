use crate::{
    EnvConfig, FileWrite, JsonRead, JsonWrite, MayFail,
    champions::{Abilities, MerakiChampion},
    get_file_names,
    init::ENV_CONFIG,
    items::MerakiItem,
    model::riot::{RiotCdnChampion, RiotCdnRune},
    read_file, resolve_path,
    riot::RiotCdnStandard,
    update::setup_project_folders,
};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Write},
    path::Path,
    sync::Arc,
};
use tokio::{sync::Semaphore, task::JoinHandle};
use tutorlolv2_fmt::pascal_case;
use tutorlolv2_gen::ChampionId;

/// Returns the directory where images will be downloaded to
macro_rules! target_dir {
    () => {
        "raw_img"
    };
    ($first:literal) => {
        concat!(target_dir!(), "/", $first)
    };
}

enum SaveTo<'a> {
    /// ChampionId
    Champion(&'a str),
    /// ChampionId, P | Q | W | E | R
    Ability(&'a str, char),
    /// ItemId
    Item(&'a str),
    /// ChampionId, Skin Number
    Centered(&'a str, usize),
    /// ChampionId, Skin Number
    Splash(&'a str, usize),
    /// RuneId
    Runes(usize),
    RiotDir(Tag),
    MerakiDir(Tag),
    RiotCache(Tag, &'a str),
    MerakiCache(Tag, &'a str),
}

enum Tag {
    Items,
    Champions,
    Runes,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Items => write!(f, "items"),
            Tag::Champions => write!(f, "champions"),
            Tag::Runes => write!(f, "runes"),
        }
    }
}

impl<'a> SaveTo<'a> {
    pub fn path(&self) -> String {
        let img = "raw_img";
        match self {
            SaveTo::Champion(s) => format!("{img}/champions/{s}.png"),
            SaveTo::Ability(s, c) => format!("{img}/abilities/{s}{c}.png"),
            SaveTo::Item(s) => format!("{img}/items/{s}.png"),
            SaveTo::Centered(s, n) => format!("{img}/centered/{s}_{n}.jpg"),
            SaveTo::Splash(s, n) => format!("{img}/splash/{s}_{n}.jpg"),
            SaveTo::Runes(n) => format!("{img}/runes/{n}.png"),
            SaveTo::RiotCache(s, f) => format!("cache/riot/{s}/{f}.json"),
            SaveTo::MerakiCache(s, f) => format!("cache/meraki/{s}/{f}.json"),
            SaveTo::RiotDir(s) => format!("cache/riot/{s}.json"),
            SaveTo::MerakiDir(s) => format!("cache/items/{s}.json"),
        }
    }
}

enum DDragon<'a> {
    Champion(&'a str),
    Passive(&'a str),
    Spell(&'a str),
    Item(&'a str),
    Rune(&'a str),
    Centered(&'a str, usize),
    Splash(&'a str, usize),
    Endpoint(&'a str),
    Version,
}

impl<'a> DDragon<'a> {
    pub fn url(&self) -> String {
        let EnvConfig {
            dd_dragon_endpoint,
            lol_version,
            riot_image_endpoint,
            lol_language,
            ..
        } = &*ENV_CONFIG;

        let path_a = format!("{dd_dragon_endpoint}/cdn");
        let path_b = format!("{path_a}/{lol_version}/img");

        match self {
            DDragon::Champion(s) => format!("{path_b}/champion/{s}"),
            DDragon::Passive(s) => format!("{path_b}/passive/{s}"),
            DDragon::Spell(s) => format!("{path_b}/spell/{s}"),
            DDragon::Item(s) => format!("{path_b}/item/{s}.png"),
            DDragon::Rune(s) => format!("{riot_image_endpoint}/{s}"),
            DDragon::Centered(s, n) => format!("{path_a}/img/champion/centered/{s}_{n}.jpg"),
            DDragon::Splash(s, n) => format!("{path_a}/img/champion/splash/{s}_{n}.jpg"),
            DDragon::Endpoint(s) => format!("{path_a}/{lol_version}/data/{lol_language}/{s}.json"),
            DDragon::Version => format!("{dd_dragon_endpoint}/api/versions.json"),
        }
    }
}

/// Wrapper around [`reqwest::Client`] that adds methods to
/// download files and cache then to avoid repeated requests
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
    /// Creates a new instance of [`HttpClient`]
    pub fn new() -> Self {
        Self(Client::new())
    }

    /// Downloads some url and saves to a file if it doesn't already exist. If it does,
    /// a message is printed to the console and an empty result is returned
    pub async fn download(&self, url: impl AsRef<str>, save_to: impl AsRef<Path>) -> MayFail {
        let url = url.as_ref();
        let save_to = save_to.as_ref();
        match save_to.exists() {
            true => {
                println!("[exists] {save_to:?}");
                Ok(())
            }
            false => {
                println!("[download] {url}");
                match self.get(url).send().await {
                    Ok(response) => {
                        let bytes = response.bytes().await?;
                        bytes.write_file(save_to)
                    }
                    Err(e) => {
                        println!("[error] {e}");
                        Err(e.to_string().into())
                    }
                }
            }
        }
    }

    /// Creates the base url of the Data Dragon API, using the environment variables
    pub fn ddragon_url() -> String {
        format!(
            "{}/cdn/{}",
            ENV_CONFIG.dd_dragon_endpoint, ENV_CONFIG.lol_version
        )
    }

    /// Downloads the images of champions, their abilities and passives.
    /// Skips images that have already been downloaded
    pub async fn download_general_img(&self) -> MayFail {
        let riot_champions = RiotCdnChampion::from_dir("cache/riot/champions")?;

        for (champion_id, champion) in riot_champions {
            let mut futures = Vec::new();

            let _ = self
                .download(
                    DDragon::Champion(&champion.image.full).url(),
                    SaveTo::Champion(&champion_id).path(),
                )
                .await;

            let _ = self
                .download(
                    DDragon::Passive(&champion.passive.image.full).url(),
                    SaveTo::Ability(&champion_id, 'P').path(),
                )
                .await;

            for (index, spell) in champion.spells.into_iter().enumerate() {
                let champion_id = champion_id.clone();
                let client = self.clone();
                futures.push(tokio::spawn(async move {
                    let _ = client
                        .download(
                            DDragon::Spell(&spell.image.full).url(),
                            SaveTo::Ability(&champion_id, ['Q', 'W', 'E', 'R'][index]).path(),
                        )
                        .await;
                }));
            }

            for future in futures {
                if let Err(e) = future.await {
                    println!("[error] requesting {champion_id} images: {e}");
                };
            }
        }
        Ok(())
    }

    /// Downloads the images of all items in the cached data. Skips the ones
    /// that have already been downloaded, and does not skip the ones that
    /// throw an error
    pub async fn download_items_img(&self) -> MayFail {
        let riot_items = get_file_names("cache/riot/items")?;
        let mut futures = Vec::new();
        for item_id in riot_items {
            let client = self.clone();
            futures.push(tokio::spawn(async move {
                let _ = client
                    .download(DDragon::Item(&item_id).url(), SaveTo::Item(&item_id).path())
                    .await;
            }));
        }
        for future in futures {
            if let Err(e) = future.await {
                println!("[error] requesting item images: {e}");
            }
        }
        Ok(())
    }

    /// Downloads the images of splash and centered arts for all champions and
    /// every skin available in the current patch. Skips the ones that emit an error
    pub async fn download_arts_img(&self) -> MayFail {
        let riot_champions = RiotCdnChampion::from_dir("cache/riot/champions")?;
        for (champion_id, champion) in riot_champions {
            let mut futures = Vec::new();
            for skin in champion.skins.into_iter() {
                let champion_id = champion_id.clone();
                let client = self.clone();
                futures.push(tokio::spawn(async move {
                    for i in [false, true] {
                        let (url, save_to) = match i {
                            false => (
                                DDragon::Splash(&champion_id, skin.num).url(),
                                SaveTo::Splash(&champion_id, skin.num).path(),
                            ),
                            true => (
                                DDragon::Centered(&champion_id, skin.num).url(),
                                SaveTo::Centered(&champion_id, skin.num).path(),
                            ),
                        };
                        let _ = client.download(url, save_to).await;
                    }
                }));
            }
            for future in futures {
                if let Err(e) = future.await {
                    println!("[error] requesting {champion_id} images: {e}");
                }
            }
        }
        Ok(())
    }

    /// Downloads the images of every rune, rune-tree and icon
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
            let url = DDragon::Rune(&rune_icon).url();
            let save_to = SaveTo::Runes(rune_id).path();
            let client = self.clone();
            futures.push(tokio::spawn(async move {
                let _ = client.download(url, save_to).await;
            }));
        }
        for future in futures {
            let _ = future.await;
        }
        Ok(())
    }

    /// Fetches the latest version of League of Legends, returning
    /// the current patch version as a string
    pub async fn fetch_version(&self) -> MayFail<String> {
        Ok(self
            .get(DDragon::Version.url())
            .send()
            .await?
            .json::<Vec<String>>()
            .await?
            .get(0)
            .ok_or("Version not found")?
            .clone())
    }

    /// Creates the riot endpoint using the environment variables.
    /// This is mainly used to download champion json files
    pub fn riot_endpoint(endpoint: &str, language: &str) -> String {
        format!("{}/data/{language}/{endpoint}.json", Self::ddragon_url())
    }

    /// Fetches League of Legends current version and updates it directly
    /// in the `.env` file if it has changed, renaming the cache folder and
    /// setting up a new empty one, which forces the application to re-download
    /// every champion, item, and rune file again. Does nothing if the version
    /// is equal
    pub async unsafe fn update_env_version(&self) -> MayFail {
        let version = self.fetch_version().await?;

        if version == ENV_CONFIG.lol_version {
            return Ok(());
        }

        std::fs::rename(
            "cache",
            format!(
                "cache_{old_version}",
                old_version = ENV_CONFIG.lol_version.replace(".", "_")
            ),
        )?;

        setup_project_folders()?;

        Ok(unsafe { set_env_var("LOL_VERSION", &version)? })
    }

    /// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
    /// Runs a maximum of 32 tokio threads at the same time
    pub async fn update_riot_cache(&self) -> MayFail {
        self.download(
            Self::riot_endpoint("champion", &ENV_CONFIG.lol_language),
            SaveTo::RiotDir(Tag::Champions).path(),
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
                println!("[error] [champions] Task join error: {e:?}");
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
                println!("[error] [items] Task join error: {e:?}");
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

    /// Fetches the available languages in league of legends and saves them to
    /// the appropriate cache location
    pub async fn update_language_cache(&self) -> MayFail {
        self.download(
            format!("{}/cdn/languages.json", ENV_CONFIG.dd_dragon_endpoint),
            "internal/languages.json",
        )
        .await
    }

    /// Fetches some endpoint of the merakianalytics api, orders the data
    /// before saving to the appropriate cache folder
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

    /// Fetches the `meta_endpoint` and scrapes the information from some champion's
    /// common ability combos and saves to a cache file
    pub async fn combo_scraper(&self) -> MayFail {
        for champion_id in ChampionId::ARRAY {
            let path = format!("cache/scraper/combos/{champion_id:?}.html");
            self.download(
                format!("{}/{champion_id:?}/combos", ENV_CONFIG.meta_endpoint),
                &path,
            )
            .await?;

            tokio::task::spawn_blocking(move || {
                let run_task = || -> MayFail {
                    let bytes = read_file(path)?;
                    let html = Html::parse_document(&String::from_utf8(bytes)?);

                    let mut result = Vec::<Vec<String>>::new();

                    let combo_section = Selector::parse("div.m-1o7d3sk").unwrap();
                    let combo_span = Selector::parse("span.m-1pm4585.e1o1aytf0").unwrap();

                    for combo_div in html.select(&combo_section) {
                        let mut combo_strings = Vec::new();

                        for combo_span in combo_div.select(&combo_span) {
                            if let Some(text) = combo_span.text().next() {
                                combo_strings.push(text.to_string());
                            };
                        }
                        result.push(combo_strings);
                    }

                    result.into_file(format!("internal/scraper/combos/{champion_id:?}.json"))
                };
                if let Err(e) = run_task() {
                    println!("[error] scraping combo for {champion_id:?}: {e:?}.")
                }
            });
        }
        Ok(())
    }

    /// Fetches the most common item builds, and rune choices for every position,
    /// for every champion, scraping from the `meta_endpoint`. At the end, a new
    /// json file is generated, aggregating all the collected information in a single
    /// location
    pub async fn call_scraper(&self) -> MayFail {
        for champion_id in ChampionId::ARRAY {
            let mut futures_vec = Vec::new();

            for position in ["top", "jungle", "mid", "adc", "support"] {
                let client = self.clone();

                futures_vec.push(tokio::spawn(async move {
                    let name = format!("{champion_id:?}").to_lowercase();

                    let path = format!("scraper/builds/{position}/{champion_id:?}");
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
                            let html = String::from_utf8(read_file(&cache_path)?)?;

                            let document = Html::parse_document(&html);
                            let full_build =
                                Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")
                                    .unwrap();
                            let situational_build =
                                Selector::parse(".m-s76v8c > div > div img").unwrap();
                            let rune_selector = Selector::parse("img.m-1nx2cdb").unwrap();
                            let legend_selector = Selector::parse("img.m-1u3ui07").unwrap();
                            let mut runes = Vec::<String>::new();

                            let push_alt_attr = |array: &mut Vec<String>, selector: &Selector| {
                                for img in document.select(selector) {
                                    if let Some(alt) = img.value().attr("alt") {
                                        array.push(pascal_case(alt));
                                    }
                                }
                            };

                            let mut items = Vec::<String>::new();

                            push_alt_attr(&mut runes, &rune_selector);
                            push_alt_attr(&mut runes, &legend_selector);
                            push_alt_attr(&mut items, &full_build);
                            push_alt_attr(&mut items, &situational_build);

                            [items, runes].into_file(internal_path)
                        };
                        if let Err(e) = run_task() {
                            println!("[error] processing HTML from {champion_id:?}: {e:#?}")
                        }
                    });
                }));
            }

            for future in futures_vec {
                if let Err(e) = future.await {
                    println!("[error] failed future for {champion_id:?}: {e:#?}")
                }
            }
        }

        type Inner = [Vec<String>; 2];
        type Data = HashMap<&'static str, Inner>;
        type FinalData = HashMap<ChampionId, Data>;

        let mut results = FinalData::new();

        for champion_id in ChampionId::ARRAY {
            let mut positions = HashMap::new();
            for position in ["top", "jungle", "mid", "adc", "support"] {
                let resolved_path = resolve_path(format!(
                    "internal/scraper/builds/{position}/{champion_id:?}.json"
                ))?;
                let data = Inner::from_file(resolved_path)?;
                positions.insert(position, data);
            }
            results.insert(champion_id, positions);
        }

        results.into_file("internal/scraper/data.json")
    }
}

/// Updates the `.env` file, setting a new key and value pair. If it already
/// exists, the value gets replaced
unsafe fn set_env_var(key: &str, value: &str) -> std::io::Result<()> {
    let path = ".env";
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut found = false;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with(&format!("{key}=")) {
            lines.push(format!("{key}={value}"));
            found = true;
        } else {
            lines.push(line);
        }
    }
    if !found {
        lines.push(format!("{key}={value}"));
    }
    let mut out = std::fs::File::create(path)?;
    for line in lines {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

/// Orders the data that comes from the JSON files to achieve
/// consistency between versions, avoiding offset changes every patch even when
/// nothing about some champion have changed
pub trait OrderJson<T: Serialize> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, T)>;
}

impl OrderJson<MerakiChampion> for HashMap<String, MerakiChampion> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, MerakiChampion)> {
        let mut vec_self = self.into_iter().collect::<Vec<_>>();
        for (_, champion) in vec_self.iter_mut() {
            let Abilities { p, q, w, e, r } = &mut champion.abilities;
            for ability_list in [q, w, e, r, p] {
                for ability in ability_list {
                    ability
                        .effects
                        .sort_by(|a, b| a.description.cmp(&b.description));
                    for effect in &mut ability.effects {
                        effect.leveling.sort_by(|a, b| {
                            a.attribute
                                .as_deref()
                                .unwrap_or_default()
                                .cmp(b.attribute.as_deref().unwrap_or_default())
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

/// Reads every key in something that implements trait [`OrderJson`], orders the data
/// and saves to the cache folder
pub fn save_cache<T: Serialize>(result: impl OrderJson<T>, endpoint: &str) -> MayFail {
    for (key, value) in result.into_iter_ord() {
        value.into_file(format!("cache/meraki/{endpoint}/{key}.json"))?;
    }
    Ok(())
}
