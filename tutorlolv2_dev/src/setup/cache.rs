use crate::{
    champions::CdnChampion,
    init::ENV_CONFIG,
    model::{items::CdnItem, riot::RiotCdnStandard},
    setup::essentials::{
        api::{CdnEndpoint, fetch_cdn_api, fetch_languages, fetch_riot_api},
        ext::FilePathExt,
    },
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Semaphore,
    task::{self, JoinHandle},
};

pub trait OrderJson<T: Serialize> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, T)>;
}

impl OrderJson<CdnChampion> for HashMap<String, CdnChampion> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, CdnChampion)> {
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

impl OrderJson<CdnItem> for HashMap<String, CdnItem> {
    fn into_iter_ord(self) -> impl Iterator<Item = (String, CdnItem)> {
        let mut vec_self = self.into_iter().collect::<Vec<_>>();
        for (_, item) in vec_self.iter_mut() {
            item.active.sort_by(|a, b| a.effects.cmp(&b.effects));
            item.passives.sort_by(|a, b| a.effects.cmp(&b.effects));
        }
        vec_self.into_iter()
    }
}

pub async fn save_cache<T: Serialize>(result: impl OrderJson<T>, instance: CdnEndpoint) {
    for (key, value) in result.into_iter_ord() {
        let folder_name = format!("cache/cdn/{}", instance);
        let path_name = format!("{}/{}.json", folder_name, key);
        let json_string = serde_json::to_string_pretty(&value).unwrap();
        path_name.write_to_file(json_string.as_bytes()).unwrap();
    }
}

/// Takes an instance parameter and uses CDN API to get its data and save to file system.
#[tutorlolv2_macros::trace_time]
pub async fn update_cdn_cache(client: Client, instance: CdnEndpoint) {
    macro_rules! order_and_save {
        ($ty:ty) => {
            save_cache(
                fetch_cdn_api::<HashMap<String, $ty>>(client, instance)
                    .await
                    .unwrap(),
                instance,
            )
            .await
        };
    }

    match instance {
        CdnEndpoint::Champions => order_and_save!(CdnChampion),
        CdnEndpoint::Items => order_and_save!(CdnItem),
    }
}

/// Updates files in `cache/riot` with the corresponding ones in the patch determined by `LOL_VERSION`
/// Runs a maximum of 32 tokio threads at the same time
#[tutorlolv2_macros::trace_time]
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
            path_name.write_to_file(json_str.as_bytes()).unwrap();
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
    "cache/riot/champions.json"
        .write_to_file(champions_pretty.as_bytes())
        .unwrap();

    let items_json =
        fetch_riot_api::<RiotCdnStandard>(client.clone(), "item", &ENV_CONFIG.lol_language)
            .await
            .unwrap();

    let mut items_futures = Vec::<_>::new();

    for (item_id, item_data) in items_json.data.clone() {
        items_futures.push(task::spawn_blocking(move || {
            let path_name = format!("cache/riot/items/{}.json", item_id);
            let json_str = serde_json::to_string(&item_data).unwrap();
            path_name.write_to_file(json_str.as_bytes()).unwrap();
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
    "cache/riot/items.json"
        .write_to_file(items_pretty.as_bytes())
        .unwrap();

    let runes_json =
        fetch_riot_api::<Value>(client.clone(), "runesReforged", &ENV_CONFIG.lol_language)
            .await
            .unwrap();
    let runes_pretty = serde_json::to_string_pretty(&runes_json).unwrap();
    "cache/riot/runes.json"
        .write_to_file(runes_pretty.as_bytes())
        .unwrap();
    update_language_cache(client.clone()).await;

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

        "internal/champion_languages.json"
            .write_to_file(languages_pretty.as_bytes())
            .unwrap();
    }
}

#[tutorlolv2_macros::trace_time]
pub async fn update_language_cache(client: Client) {
    let lang_json = fetch_languages(client, &ENV_CONFIG.dd_dragon_endpoint)
        .await
        .expect("Failed to fetch languages");
    let lang_pretty = serde_json::to_string(&lang_json).expect("Failed to serialize languages");
    "internal/languages.json"
        .write_to_file(lang_pretty.as_bytes())
        .expect("Failed to write languages");
}
