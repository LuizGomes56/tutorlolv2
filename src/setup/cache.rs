use crate::{
    generators::CdnChampion,
    init::dev::ENV_CONFIG,
    model::dev::{items::CdnItem, riot::RiotCdnStandard},
    setup::{
        essentials::{
            api::{CdnEndpoint, fetch_cdn_api, fetch_languages, fetch_riot_api},
            helpers::write_to_file,
        },
        generators::champions::{order_cdn_champion_effects, order_cdn_item_effects},
    },
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Semaphore,
    task::{self, JoinHandle},
};

/// Takes an instance parameter and uses CDN API to get its data and save to file system.
#[generator_macros::trace_time]
pub async fn update_cdn_cache(client: Client, instance: CdnEndpoint) {
    macro_rules! ord_and_save {
        ($type_v:ty, $func:ident) => {{
            let mut result = fetch_cdn_api::<HashMap<String, $type_v>>(client, instance)
                .await
                .unwrap();
            $func(&mut result);
            for (key, value) in result {
                let folder_name = format!("cache/cdn/{}", instance);
                let path_name = format!("{}/{}.json", folder_name, key);
                let strval = serde_json::to_string_pretty(&value).unwrap();
                write_to_file(&path_name, strval.as_bytes()).unwrap();
            }
        }};
    }
    match instance {
        CdnEndpoint::Champions => {
            ord_and_save!(CdnChampion, order_cdn_champion_effects);
        }
        CdnEndpoint::Items => {
            ord_and_save!(CdnItem, order_cdn_item_effects);
        }
    };
}

/// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
/// Runs a maximum of 32 tokio threads at the same time
#[generator_macros::trace_time]
pub async fn update_riot_cache(client: Client) {
    let champions_json =
        fetch_riot_api::<RiotCdnStandard>(client.clone(), "champion", &ENV_CONFIG.lol_language)
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

            let champion_data = fetch_riot_api::<RiotCdnStandard>(
                client,
                &format!("champion/{}", champion_id),
                &ENV_CONFIG.lol_language,
            )
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

    let items_json =
        fetch_riot_api::<RiotCdnStandard>(client.clone(), "item", &ENV_CONFIG.lol_language)
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

    let runes_json =
        fetch_riot_api::<Value>(client.clone(), "runesReforged", &ENV_CONFIG.lol_language)
            .await
            .unwrap();
    let runes_pretty = serde_json::to_string_pretty(&runes_json).unwrap();
    write_to_file("cache/riot/runes.json", runes_pretty.as_bytes()).unwrap();
    update_language_cache(client.clone()).await;

    // let champions_json_raw = std::fs::read_to_string("cache/riot/champions.json").unwrap();
    // let champions_json = serde_json::from_str::<RiotCdnStandard>(&champions_json_raw).unwrap();

    if let Ok(raw_languages) = std::fs::read_to_string("internal/languages.json") {
        let mut languages_data = HashMap::<String, Vec<String>>::from_iter(
            champions_json
                .data
                .iter()
                .map(|(champion_id, _)| (champion_id.clone(), Vec::new())),
        );
        let languages = serde_json::from_str::<Vec<String>>(&raw_languages).unwrap();
        let mut languages_future = Vec::new();

        #[derive(Deserialize)]
        struct NameField {
            name: String,
        }

        for language in languages {
            let language = language.clone();
            let client = client.clone();
            languages_future.push(tokio::spawn(async move {
                let champions_json = fetch_riot_api::<RiotCdnStandard<NameField>>(
                    client.clone(),
                    "champion",
                    &language,
                )
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

        let languages_pretty = serde_json::to_string_pretty(&languages_data).unwrap();
        write_to_file(
            "internal/champion_languages.json",
            languages_pretty.as_bytes(),
        )
        .unwrap();
    }
}

#[generator_macros::trace_time]
pub async fn update_language_cache(client: Client) {
    let lang_json = fetch_languages(client, &ENV_CONFIG.dd_dragon_endpoint)
        .await
        .expect("Failed to fetch languages");
    let lang_pretty = serde_json::to_string(&lang_json).expect("Failed to serialize languages");
    write_to_file("internal/languages.json", lang_pretty.as_bytes())
        .expect("Failed to write languages");
}
