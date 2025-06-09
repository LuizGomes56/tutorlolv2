use crate::{
    EnvConfig,
    model::{
        application::GlobalCache, champions::Champion, internal::MetaItems, items::Item,
        riot::RiotCdnStandard, runes::Rune,
    },
    setup::{
        api::{fetch_cdn_api, fetch_riot_api},
        helpers::{SetupError, extract_file_name, read_json_file, write_to_file},
    },
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    hash::Hash,
    io,
    num::ParseIntError,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::{AcquireError, Semaphore, SemaphorePermit},
    task::{self, JoinHandle},
};

fn load_json_cache<K, V, F, G>(
    dir_path: &str,
    key_fn: F,
    convert_key: G,
) -> Result<HashMap<K, V>, SetupError>
where
    F: Fn(&PathBuf) -> Result<&str, SetupError>,
    G: Fn(&str) -> Result<K, SetupError>,
    V: DeserializeOwned,
    K: Eq + Hash,
{
    let mut map: HashMap<K, V> = HashMap::new();
    let read_dir: ReadDir =
        fs::read_dir(dir_path).map_err(|e: io::Error| SetupError(e.to_string()))?;

    for entry in read_dir {
        let path_buf: PathBuf = entry
            .map_err(|e: io::Error| SetupError(e.to_string()))?
            .path();
        let path_name: &str = path_buf
            .to_str()
            .ok_or_else(|| SetupError("fn[load_json_map]: Invalid path".to_string()))?;
        let file_key_str: &str = key_fn(&path_buf)?;
        let file_key: K = convert_key(file_key_str)?;
        let data: V = read_json_file(path_name)?;
        map.insert(file_key, data);
    }

    Ok(map)
}

fn load_optional_json_cache<T: Default + DeserializeOwned>(path: &str, label: &str) -> T {
    if Path::new(path).exists() {
        println!("fn[load_cache]: started loading {}", label);
        read_json_file::<T>(path).unwrap()
    } else {
        T::default()
    }
}

// Syncronously loads all the cache that will live in memory through the entire execution
pub fn load_cache() -> Result<GlobalCache, SetupError> {
    println!("fn[load_cache]: started loading champion_files");
    let champions: HashMap<String, Champion> = load_json_cache(
        "internal/champions",
        |path: &PathBuf| Ok(extract_file_name(path)),
        |s: &str| Ok(s.to_string()),
    )?;

    println!("fn[load_cache]: started loading item_files");
    let items: HashMap<usize, Item> = load_json_cache(
        "internal/items",
        |path: &PathBuf| Ok(extract_file_name(path)),
        |s: &str| {
            s.parse::<usize>()
                .map_err(|e: ParseIntError| SetupError(e.to_string()))
        },
    )?;

    let champion_names: HashMap<String, String> =
        load_optional_json_cache("internal/champion_names.json", "champion_names");
    let meta_items: MetaItems = load_optional_json_cache("internal/meta_items.json", "meta_items");
    let runes: HashMap<usize, Rune> = load_optional_json_cache("internal/runes.json", "runes");

    Ok(GlobalCache {
        champions,
        items,
        runes,
        champion_names,
        meta_items,
    })
}

// Takes an instance parameter and uses CDN API to get its data and save to file system.
pub async fn update_cdn_cache(
    client: Client,
    envcfg: Arc<EnvConfig>,
    instance: &str,
) -> Result<(), SetupError> {
    let result: HashMap<String, Value> =
        fetch_cdn_api(client, envcfg, &format!("{}.json", instance)).await?;

    for (key, value) in result {
        let folder_name: String = format!("cache/cdn/{}", instance);
        task::spawn_blocking(move || {
            let path_name: String = format!("{}/{}.json", folder_name, key);
            let strval: String = value.to_string();
            let _ = write_to_file(&path_name, strval.as_bytes())
                .map_err(|e: SetupError| eprintln!("fn[update_cdn_cache]: {:#?}", e));
        });
    }
    Ok(())
}

// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
// Runs a maximum of 32 tokio threads at the same time
pub async fn update_riot_cache(client: Client, envcfg: Arc<EnvConfig>) -> Result<(), SetupError> {
    let champions_json: RiotCdnStandard =
        fetch_riot_api::<RiotCdnStandard>(client.clone(), envcfg.clone(), "champion")
            .await
            .map_err(|e: SetupError| SetupError(format!("Failed to fetch champions: {}", e.0)))?;

    let mut champions_futures = Vec::<JoinHandle<Result<(), SetupError>>>::new();
    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(1 << 5));

    for (champion_id, _) in champions_json.data.clone() {
        let client: Client = client.clone();
        let semaphore: Arc<Semaphore> = semaphore.clone();
        let envcfg: Arc<EnvConfig> = envcfg.clone();

        champions_futures.push(tokio::spawn(async move {
            let _permit: SemaphorePermit<'_> = semaphore
                .acquire()
                .await
                .map_err(|e: AcquireError| SetupError(format!("Semaphore error: {}", e)))?;

            let path_name: String = format!("cache/riot/champions/{}.json", champion_id);

            let champion_data: RiotCdnStandard = fetch_riot_api::<RiotCdnStandard>(
                client,
                envcfg,
                &format!("champion/{}", champion_id),
            )
            .await
            .map_err(|e: SetupError| {
                SetupError(format!("Failed to fetch data for {}: {}", champion_id, e.0))
            })?;

            let data_field: HashMap<String, Value> = champion_data.data;
            let real_data: &Value = data_field.get(&champion_id).ok_or_else(|| {
                SetupError(format!("Champion {} not found in result", champion_id))
            })?;

            let json_str: String = serde_json::to_string(real_data).map_err(|e| {
                SetupError(format!("Serialization error for {}: {}", champion_id, e))
            })?;

            write_to_file(&path_name, json_str.as_bytes())?;

            Ok(())
        }));
    }

    for future in champions_futures {
        if let Err(e) = future.await {
            return Err(SetupError(format!(
                "fn[update_riot_cache] [champions] Task join error (champion): {}",
                e
            )));
        }
    }

    let champions_pretty: String =
        serde_json::to_string_pretty(&champions_json).map_err(|e: serde_json::Error| {
            SetupError(format!("Failed to serialize champions: {}", e))
        })?;
    write_to_file("cache/riot/champions.json", champions_pretty.as_bytes())?;

    let items_json: RiotCdnStandard =
        fetch_riot_api::<RiotCdnStandard>(client.clone(), envcfg.clone(), "item")
            .await
            .map_err(|e: SetupError| SetupError(format!("Failed to fetch items: {}", e.0)))?;

    let mut items_futures: Vec<JoinHandle<Result<(), SetupError>>> =
        Vec::<JoinHandle<Result<(), SetupError>>>::new();

    for (item_id, item_data) in items_json.data.clone() {
        items_futures.push(task::spawn_blocking(move || {
            let path_name: String = format!("cache/riot/items/{}.json", item_id);
            let json_str: String = serde_json::to_string(&item_data).map_err(|e| {
                SetupError(format!("Serialization error for item {}: {}", item_id, e))
            })?;

            write_to_file(&path_name, json_str.as_bytes())?;
            Ok(())
        }));
    }

    for future in items_futures {
        if let Err(e) = future.await {
            return Err(SetupError(format!(
                "fn[update_riot_cache] [items] Task join error (champion): {}",
                e
            )));
        }
    }

    let items_pretty: String = serde_json::to_string_pretty(&items_json)
        .map_err(|e: serde_json::Error| SetupError(format!("Failed to serialize items: {}", e)))?;
    write_to_file("cache/riot/items.json", items_pretty.as_bytes())?;

    // Runes
    let runes_json: Value = fetch_riot_api::<Value>(client, envcfg, "runesReforged")
        .await
        .map_err(|e: SetupError| SetupError(format!("Failed to fetch runes: {}", e.0)))?;

    let runes_pretty: String = serde_json::to_string_pretty(&runes_json)
        .map_err(|e: serde_json::Error| SetupError(format!("Failed to serialize runes: {}", e)))?;
    write_to_file("cache/riot/runes.json", runes_pretty.as_bytes())?;

    Ok(())
}
