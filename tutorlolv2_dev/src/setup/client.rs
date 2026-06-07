use crate::{
    EnvConfig, FileWrite, JsonRead, JsonWrite, MayFail,
    init::ENV_CONFIG,
    riot::RiotCdn,
    setup::riot::{RiotCdnChampion, RiotCdnRune},
};
use rand::RngExt;
use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Display,
    io::{BufRead, BufReader, Write},
    path::Path,
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Semaphore, task::JoinHandle};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{Key, Position};

#[derive(Copy, Clone)]
pub enum SaveTo<'a> {
    GeneratorDir(Tag),
    GeneratorRaw(Tag, &'a str),
    RiotChampions,
    RiotItems,
    RiotItemsDir,
    RiotChampionsDir,
    RiotRunes,
    RiotLangDir(&'a str),
    RiotRawChampions(&'a str),
    RiotCache(Tag, &'a (dyn Display + Send + Sync)),
    InternalRaw(Tag, &'a str),
    InternalDir(Tag),
    InternalScraperData,
    InternalChampionLanguages,
    InternalDamagingItems,
    InternalLanguages,
    InternalMaps,
    InternalRuneNames,
    InternalRunes,
    ImgChampion(&'a str),
    ImgAbility(&'a str, Key),
    ImgItem(&'a str),
    ImgCentered(&'a str, usize),
    ImgSplash(&'a str, usize),
    ImgRunes(usize),
    ScraperBuilds(Position, &'a str),
    ScraperCombos(&'a str),
    InternalScraperBuilds(Position, &'a str),
    InternalScraperCombos(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Items,
    Champions,
    Runes,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tag::Items => "items",
                Tag::Champions => "champions",
                Tag::Runes => "runes",
            }
        )
    }
}

impl<'a> SaveTo<'a> {
    pub fn path(&self) -> String {
        let img = "raw_img";

        match self {
            SaveTo::GeneratorDir(tag) => format!("tutorlolv2_dev/src/generators/gen_{tag}"),
            SaveTo::GeneratorRaw(tag, s) => {
                let path = Self::GeneratorDir(*tag).path();
                let file = match tag {
                    Tag::Items | Tag::Runes => to_ssnake(s),
                    Tag::Champions => s.to_string(),
                }
                .to_lowercase();
                format!("{path}/{file}.rs")
            }
            SaveTo::ImgChampion(s) => format!("{img}/champions/{s}.png"),
            SaveTo::ImgAbility(s, c) => format!("{img}/abilities/{s}{c:?}.png"),
            SaveTo::ImgItem(s) => format!("{img}/items/{s}.png"),
            SaveTo::ImgCentered(s, n) => format!("{img}/centered/{s}_{n}.jpg"),
            SaveTo::ImgSplash(s, n) => format!("{img}/splash/{s}_{n}.jpg"),
            SaveTo::ImgRunes(n) => format!("{img}/runes/{n}.png"),
            SaveTo::RiotCache(s, f) => format!("cache/riot/{s}/{f}.json"),
            SaveTo::RiotItems => "cache/riot/items.json".into(),
            SaveTo::RiotChampions => "cache/riot/champions.json".into(),
            SaveTo::RiotItemsDir => "cache/riot/items".into(),
            SaveTo::RiotChampionsDir => "cache/riot/champions".into(),
            SaveTo::RiotRunes => "cache/riot/runes.json".into(),
            SaveTo::RiotLangDir(s) => format!("cache/riot/champions_lang/{s}.json"),
            SaveTo::RiotRawChampions(s) => format!("cache/riot/raw_champions/{s}.json"),
            SaveTo::ScraperBuilds(position, s) => {
                format!("cache/scraper/builds/{position:?}/{s}.html")
            }
            SaveTo::ScraperCombos(s) => format!("cache/scraper/combos/{s}.html"),
            SaveTo::InternalRaw(tag, s) => format!("internal/{tag}/{s}.json"),
            SaveTo::InternalDir(tag) => format!("internal/{tag}"),
            SaveTo::InternalScraperBuilds(position, s) => {
                format!("internal/scraper/builds/{position:?}/{s}.json")
            }
            SaveTo::InternalScraperCombos(champion_id) => {
                format!("internal/scraper/combos/{champion_id}.json")
            }
            SaveTo::InternalScraperData => "internal/scraper/data.json".into(),
            SaveTo::InternalChampionLanguages => "internal/champion_languages.json".into(),
            SaveTo::InternalDamagingItems => "internal/damaging_items.json".into(),
            SaveTo::InternalLanguages => "internal/languages.json".into(),
            SaveTo::InternalMaps => "internal/maps.json".into(),
            SaveTo::InternalRuneNames => "internal/rune_names.json".into(),
            SaveTo::InternalRunes => "internal/runes.json".into(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DDragon<'a> {
    Champion(&'a str),
    Passive(&'a str),
    Spell(&'a str),
    Item(&'a str),
    Rune(&'a str),
    Centered(&'a str, usize),
    Splash(&'a str, usize),
    Endpoint(&'a str),
    Version,
    Riot(&'a str, Option<&'a str>),
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

        let path_a = format_args!("{dd_dragon_endpoint}/cdn");
        let path_b = format_args!("{path_a}/{lol_version}/img");

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
            DDragon::Riot(endpoint, language) => {
                let language = language.unwrap_or(&lol_language);
                let path = format_args!("{dd_dragon_endpoint}/cdn/{lol_version}",);
                format!("{path}/data/{language}/{endpoint}.json")
            }
        }
    }
}

/// Wrapper around [`reqwest::Client`] that adds methods to
/// download files and cache then to avoid repeated requests
#[derive(Clone)]
#[repr(transparent)]
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

pub fn randomize_sleep() {
    let time = rand::rng().random_range(2000..10000);
    std::thread::sleep(Duration::from_millis(time))
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

        match save_to.try_exists() {
            Ok(true) => {
                println!("[exists] {save_to:?}");
                Ok(())
            }
            Ok(false) => {
                println!("[download] {url}");
                match self.get(url).send().await {
                    Ok(response) => {
                        let bytes = response.bytes().await?;

                        const ERROR_TAG: &[u8] = b"<Code>AccessDenied</Code>";

                        if bytes.windows(ERROR_TAG.len()).any(|w| w == ERROR_TAG) {
                            return [].write_file(save_to);
                        }

                        bytes.write_file(save_to)
                    }
                    Err(e) => {
                        println!("[error] {e}");
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                println!("[error] Unknown error on method Path::try_exists() for {save_to:?}: {e}");
                Err(e.into())
            }
        }
    }

    async fn parallel_task<T, F, Fut>(&self, limit: usize, dir: SaveTo<'_>, f: F) -> MayFail
    where
        T: DeserializeOwned,
        F: FnOnce(Self, String, T) -> Fut + 'static + Copy + Send + Sync,
        Fut: Future<Output = MayFail> + Send,
    {
        let entries = crate::read_dir(dir.path())?;
        let (lower, upper) = entries.size_hint();
        let mut futures = Vec::with_capacity(upper.unwrap_or(lower));
        let semaphore = Arc::new(Semaphore::new(limit));

        for entry in entries {
            let semaphore = semaphore.clone();
            let client = self.clone();
            let task = tokio::task::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                if let Err(e) = async || -> MayFail {
                    let path = entry.path();
                    let name = path.file_stem().ok_or("Can't recover system file name")?;
                    let enumv = name.to_string_lossy();
                    let bytes = tokio::fs::read(&path).await?;
                    let value = serde_json::from_slice::<T>(&bytes)?;
                    f(client, enumv.into(), value).await
                }()
                .await
                {
                    println!("[error] Joining parallel future: {e:?}");
                }
            });
            futures.push(task);
        }

        for future in futures {
            if let Err(e) = future.await {
                println!("Failed to join future: {e:?}")
            }
        }

        Ok(())
    }

    /// Downloads the images of champions, their abilities and passives.
    /// Skips images that have already been downloaded
    pub async fn download_general_img(&self) -> MayFail {
        println!("Called fn [download_general_img]");

        self.parallel_task(
            20,
            SaveTo::RiotChampionsDir,
            async move |client, _, champion: RiotCdnChampion| {
                let champion_id = &champion.id;

                client
                    .download(
                        DDragon::Champion(&champion.image.full).url(),
                        SaveTo::ImgChampion(champion_id).path(),
                    )
                    .await?;

                client
                    .download(
                        DDragon::Passive(&champion.passive.image.full).url(),
                        SaveTo::ImgAbility(champion_id, Key::P).path(),
                    )
                    .await?;

                for (index, spell) in champion.spells.into_iter().enumerate() {
                    client
                        .download(
                            DDragon::Spell(&spell.image.full).url(),
                            SaveTo::ImgAbility(
                                champion_id,
                                [Key::Q, Key::W, Key::E, Key::R][index],
                            )
                            .path(),
                        )
                        .await?;
                }

                Ok(())
            },
        )
        .await
    }

    /// Downloads the images of all items in the cached data. Skips the ones
    /// that have already been downloaded, and does not skip the ones that
    /// throw an error
    pub async fn download_items_img(&self) -> MayFail {
        println!("Called fn [download_items_img]");

        self.parallel_task(
            32,
            SaveTo::RiotItemsDir,
            async move |client, item_id, _: Value| {
                client
                    .download(
                        DDragon::Item(&item_id).url(),
                        SaveTo::ImgItem(&item_id).path(),
                    )
                    .await
            },
        )
        .await
    }

    /// Downloads the images of splash and centered arts for all champions and
    /// every skin available in the current patch. Skips the ones that emit an error
    pub async fn download_arts_img(&self) -> MayFail {
        println!("Called fn [download_arts_img]");

        self.parallel_task(
            16,
            SaveTo::RiotChampionsDir,
            async move |client, champion_id, champion: RiotCdnChampion| {
                for skin in champion.skins {
                    let num = skin.num;

                    for i in [false, true] {
                        let (url, save_to) = match i {
                            false => (
                                DDragon::Splash(&champion_id, num).url(),
                                SaveTo::ImgSplash(&champion_id, num).path(),
                            ),
                            true => (
                                DDragon::Centered(&champion_id, num).url(),
                                SaveTo::ImgCentered(&champion_id, num).path(),
                            ),
                        };

                        let _ = client.download(url, save_to).await;
                    }
                }

                Ok(())
            },
        )
        .await
    }

    /// Downloads the images of every rune, rune-tree and icon
    pub async fn download_runes_img(&self) -> MayFail {
        println!("Called fn [download_runes_img]");

        let path = SaveTo::RiotRunes.path();

        for rune in Vec::<RiotCdnRune>::from_file(&path)? {
            let mut icon_map = vec![(rune.id, rune.icon)];

            for slot in rune.slots {
                for rune in slot.runes {
                    icon_map.push((rune.id, rune.icon));
                }
            }

            for (rune_id, rune_icon) in icon_map {
                let _ = self
                    .download(
                        DDragon::Rune(&rune_icon).url(),
                        SaveTo::ImgRunes(rune_id).path(),
                    )
                    .await;
            }
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
            .first()
            .ok_or("Version not found")?
            .to_owned())
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

        Ok(unsafe { set_env_var("LOL_VERSION", &version)? })
    }

    /// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
    pub async fn update_riot_cache(&self) -> MayFail {
        self.download(
            DDragon::Riot("champion", None).url(),
            SaveTo::RiotChampions.path(),
        )
        .await?;

        let champions_json = RiotCdn::<String, Value>::from_file(SaveTo::RiotChampions.path())?;

        let champion_ids = champions_json
            .data
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        let mut champions_futures = Vec::<JoinHandle<_>>::new();
        let semaphore = Arc::new(Semaphore::new(16));

        for champion_id in champion_ids.clone() {
            let client = self.clone();
            let semaphore = semaphore.clone();

            champions_futures.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let save_to = SaveTo::RiotRawChampions(&champion_id).path();

                client
                    .download(
                        DDragon::Riot(&format!("champion/{champion_id}"), None).url(),
                        &save_to,
                    )
                    .await
                    .unwrap();

                let champion_data = RiotCdn::<String, Value>::from_file(save_to).unwrap();

                champion_data
                    .data
                    .get(&champion_id)
                    .unwrap()
                    .into_file(SaveTo::RiotCache(Tag::Champions, &champion_id).path())
                    .unwrap();
            }));
        }

        for future in champions_futures {
            if let Err(e) = future.await {
                println!("[error] [champions] Task join error: {e:?}");
            }
        }

        let items_path = SaveTo::RiotItems.path();

        self.download(DDragon::Riot("item", None).url(), &items_path)
            .await?;

        let items_json = RiotCdn::<u32, Value>::from_file(items_path)?;

        for (item_id, item_data) in items_json.data {
            item_data.into_file(SaveTo::RiotCache(Tag::Items, &item_id).path())?;
        }

        self.download(
            DDragon::Riot("runesReforged", None).url(),
            SaveTo::RiotRunes.path(),
        )
        .await?;

        self.update_language_cache().await?;
        let languages = Vec::<String>::from_file(SaveTo::InternalLanguages.path())?;

        let mut languages_data = BTreeMap::<String, BTreeSet<String>>::from_iter(
            champion_ids
                .into_iter()
                .map(|champion_id| (champion_id, BTreeSet::new())),
        );

        let mut languages_future = Vec::new();

        for language in languages {
            let champion_file = SaveTo::RiotLangDir(&language).path();
            let client = self.clone();

            languages_future.push(tokio::spawn(async move {
                client
                    .download(
                        DDragon::Riot("champion", Some(&language)).url(),
                        &champion_file,
                    )
                    .await
                    .unwrap();

                #[derive(Deserialize)]
                struct NameField {
                    name: String,
                }

                let champion_lang = RiotCdn::<String, NameField>::from_file(champion_file).unwrap();

                let mut result = HashMap::new();

                for (champion_id, name_field) in champion_lang.data {
                    result.insert(champion_id, name_field.name);
                }

                result
            }))
        }

        for future in languages_future {
            if let Ok(data) = future.await {
                for (champion_id, champion_name) in data {
                    match languages_data.get_mut(&champion_id) {
                        Some(v) => {
                            v.insert(champion_name);
                        }
                        None => {
                            languages_data.insert(champion_id, BTreeSet::from([champion_name]));
                        }
                    };
                }
            }
        }

        languages_data.into_file(SaveTo::InternalChampionLanguages.path())
    }

    /// Fetches the available languages in league of legends and saves them to
    /// the appropriate cache location
    pub async fn update_language_cache(&self) -> MayFail {
        self.download(
            format!("{}/cdn/languages.json", ENV_CONFIG.dd_dragon_endpoint),
            SaveTo::InternalLanguages.path(),
        )
        .await
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
