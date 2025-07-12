use crate::{
    essentials::{
        api::{fetch_cdn_api, fetch_riot_api},
        helpers::write_to_file,
    },
    model::riot::RiotCdnStandard,
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
#[generator_macros::trace_time]
pub async fn update_cdn_cache(client: Client, instance: &str) {
    let result: FxHashMap<String, Value> = fetch_cdn_api(client, &format!("{}.json", instance))
        .await
        .unwrap();

    for (key, value) in result {
        let folder_name = format!("cache/cdn/{}", instance);

        let path_name = format!("{}/{}.json", folder_name, key);
        let strval = value.to_string();
        write_to_file(&path_name, strval.as_bytes()).unwrap();
    }
}

/// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
/// Runs a maximum of 32 tokio threads at the same time
#[generator_macros::trace_time]
pub async fn update_riot_cache(client: Client) {
    let champions_json = fetch_riot_api::<RiotCdnStandard>(client.clone(), "champion")
        .await
        .unwrap();

    let mut champions_futures = Vec::<JoinHandle<_>>::new();
    let semaphore = Arc::new(Semaphore::new(1 << 5));

    for (champion_id, _) in champions_json.data.clone() {
        let client = client.clone();
        let semaphore = semaphore.clone();

        champions_futures.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();

            let path_name = format!("cache/riot/champions/{}.json", champion_id);

            let champion_data =
                fetch_riot_api::<RiotCdnStandard>(client, &format!("champion/{}", champion_id))
                    .await
                    .unwrap();

            let data_field = champion_data.data;
            let real_data = data_field.get(&champion_id).unwrap();

            let json_str = serde_json::to_string(real_data).unwrap();
            write_to_file(&path_name, json_str.as_bytes()).unwrap();
        }));
    }

    for future in champions_futures {
        if let Err(e) = future.await {
            println!(
                "fn[update_riot_cache] [champions] Task join error (champion): {}",
                e
            );
        }
    }

    let champions_pretty = serde_json::to_string_pretty(&champions_json).unwrap();
    write_to_file("cache/riot/champions.json", champions_pretty.as_bytes()).unwrap();

    let items_json = fetch_riot_api::<RiotCdnStandard>(client.clone(), "item")
        .await
        .unwrap();

    let mut items_futures = Vec::<_>::new();

    for (item_id, item_data) in items_json.data.clone() {
        items_futures.push(task::spawn_blocking(move || {
            let path_name = format!("cache/riot/items/{}.json", item_id);
            let json_str = serde_json::to_string(&item_data).unwrap();
            write_to_file(&path_name, json_str.as_bytes()).unwrap();
        }));
    }

    for future in items_futures {
        if let Err(e) = future.await {
            println!(
                "fn[update_riot_cache] [items] Task join error (champion): {}",
                e
            );
        }
    }

    let items_pretty = serde_json::to_string_pretty(&items_json).unwrap();
    write_to_file("cache/riot/items.json", items_pretty.as_bytes()).unwrap();

    let runes_json = fetch_riot_api::<Value>(client, "runesReforged")
        .await
        .unwrap();
    let runes_pretty = serde_json::to_string_pretty(&runes_json).unwrap();
    write_to_file("cache/riot/runes.json", runes_pretty.as_bytes()).unwrap();
}
