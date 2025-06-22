use crate::{
    model::{
        application::GlobalCache, internal::Positions, items::Item, riot::RiotCdnStandard,
        runes::Rune,
    },
    setup::{
        api::{fetch_cdn_api, fetch_riot_api},
        helpers::{SetupError, extract_file_name, read_json_file, write_to_file},
    },
    writers::Champion,
};
use reqwest::Client;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    fs::{self, ReadDir},
    hash::Hash,
    io,
    num::ParseIntError,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::Semaphore,
    task::{self, JoinHandle},
};

fn load_json_cache<K, V, F, G>(
    dir_path: &str,
    key_fn: F,
    convert_key: G,
) -> Result<FxHashMap<K, V>, SetupError>
where
    F: Fn(&PathBuf) -> Result<&str, SetupError>,
    G: Fn(&str) -> Result<K, SetupError>,
    V: DeserializeOwned,
    K: Eq + Hash + 'static,
{
    let mut map: FxHashMap<K, V> = FxHashMap::default();
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

fn load_optional_json_cache<T: Default + DeserializeOwned>(path: &str) -> T {
    if Path::new(path).exists() {
        read_json_file::<T>(path).unwrap()
    } else {
        T::default()
    }
}

/// Syncronously loads all the cache that will live in memory through the entire execution
#[writer_macros::trace_time]
pub fn load_cache() -> Result<GlobalCache, SetupError> {
    let champions: FxHashMap<String, Champion> = load_json_cache(
        "internal/champions",
        |path| Ok(extract_file_name(path)),
        |s| Ok(s.to_string()),
    )?;

    let items: FxHashMap<usize, Item> = load_json_cache(
        "internal/items",
        |path| Ok(extract_file_name(path)),
        |s| {
            s.parse::<usize>()
                .map_err(|e: ParseIntError| SetupError(e.to_string()))
        },
    )?;

    let champion_names: FxHashMap<String, String> =
        load_optional_json_cache("internal/champion_names.json");
    let meta_items: FxHashMap<String, Positions> =
        load_optional_json_cache("internal/meta_items.json");
    let runes: FxHashMap<usize, Rune> = load_optional_json_cache("internal/runes.json");
    let simulated_items = items
        .iter()
        .filter_map(|(item_id, item)| match *item_id {
            3000..8000 => (item.tier >= 3).then(|| *item_id),
            _ => None,
        })
        .collect::<Vec<usize>>();

    Ok(GlobalCache {
        champions,
        items,
        runes,
        champion_names,
        meta_items,
        simulated_items,
    })
}

/// Takes an instance parameter and uses CDN API to get its data and save to file system.
#[writer_macros::trace_time]
pub async fn update_cdn_cache(client: Client, instance: &str) -> Result<(), SetupError> {
    let result: FxHashMap<String, Value> =
        fetch_cdn_api(client, &format!("{}.json", instance)).await?;

    for (key, value) in result {
        let folder_name: String = format!("cache/cdn/{}", instance);

        let path_name: String = format!("{}/{}.json", folder_name, key);
        let strval: String = value.to_string();
        let _ = write_to_file(&path_name, strval.as_bytes())?;
    }
    Ok(())
}

/// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
/// Runs a maximum of 32 tokio threads at the same time
#[writer_macros::trace_time]
pub async fn update_riot_cache(client: Client) -> Result<(), SetupError> {
    let champions_json = fetch_riot_api::<RiotCdnStandard>(client.clone(), "champion")
        .await
        .map_err(|e| SetupError(format!("Failed to fetch champions: {}", e.0)))?;

    let mut champions_futures = Vec::<JoinHandle<Result<(), SetupError>>>::new();
    let semaphore = Arc::new(Semaphore::new(1 << 5));

    for (champion_id, _) in champions_json.data.clone() {
        let client = client.clone();
        let semaphore = semaphore.clone();

        champions_futures.push(tokio::spawn(async move {
            let _permit = semaphore
                .acquire()
                .await
                .map_err(|e| SetupError(format!("Semaphore error: {}", e)))?;

            let path_name = format!("cache/riot/champions/{}.json", champion_id);

            let champion_data =
                fetch_riot_api::<RiotCdnStandard>(client, &format!("champion/{}", champion_id))
                    .await
                    .map_err(|e: SetupError| {
                        SetupError(format!("Failed to fetch data for {}: {}", champion_id, e.0))
                    })?;

            let data_field: FxHashMap<String, Value> = champion_data.data;
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

    let items_json: RiotCdnStandard = fetch_riot_api::<RiotCdnStandard>(client.clone(), "item")
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
    let runes_json: Value = fetch_riot_api::<Value>(client, "runesReforged")
        .await
        .map_err(|e: SetupError| SetupError(format!("Failed to fetch runes: {}", e.0)))?;

    let runes_pretty: String = serde_json::to_string_pretty(&runes_json)
        .map_err(|e: serde_json::Error| SetupError(format!("Failed to serialize runes: {}", e)))?;
    write_to_file("cache/riot/runes.json", runes_pretty.as_bytes())?;

    Ok(())
}
