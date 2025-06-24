use crate::{
    model::riot::RiotCdnStandard,
    setup::{
        api::{fetch_cdn_api, fetch_riot_api},
        helpers::{SetupError, write_to_file},
    },
};
use reqwest::Client;
use rustc_hash::FxHashMap;
use serde_json::Value;
use std::sync::Arc;
use tokio::{
    sync::Semaphore,
    task::{self, JoinHandle},
};

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
