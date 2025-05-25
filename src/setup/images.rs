use std::{collections::HashMap, fs};

use crate::model::riot::{RiotCdnChampion, RiotCdnRune};

use super::{extract_file_name, read_from_file, write_to_file};

fn get_base_uri() -> String {
    let url = std::env::var("DD_DRAGON_ENDPOINT").expect("DD_DRAGON_ENDPOINT is not set");
    let version = std::env::var("LOL_VERSION").expect("LOL_VERSION is not set");

    format!("{}/{}/img", url, version)
}

pub async fn img_download_instances() {
    let files = fs::read_dir("src/cache/riot/champions").unwrap();
    let base_uri = get_base_uri();
    let client = reqwest::Client::new();
    let mut outer_futures = Vec::new();
    for file in files {
        let outer_base_uri = base_uri.clone();
        let outer_client = client.clone();
        outer_futures.push(tokio::spawn(async move {
            let path_buf = file.unwrap().path();
            let path_name = path_buf.to_str().unwrap();
            let outer_result = read_from_file::<RiotCdnChampion>(path_name);
            let spells = outer_result.spells;
            let mut inner_futures = Vec::new();
            let champion_icon_url =
                format!("{}/champion/{}", outer_base_uri, outer_result.image.full);
            let champion_response = outer_client.get(&champion_icon_url).send().await.unwrap();
            let champion_bytes = champion_response.bytes().await.unwrap();
            let champion_dir = "src/img/champions";
            let champion_file_name = format!("{}.png", outer_result.id);
            write_to_file(
                &format!("{}/{}", champion_dir, &champion_file_name),
                &champion_bytes,
            );
            let passive_icon_url = format!(
                "{}/passive/{}",
                outer_base_uri, outer_result.passive.image.full
            );
            let passive_response = outer_client.get(&passive_icon_url).send().await.unwrap();
            let passive_bytes = passive_response.bytes().await.unwrap();
            let passive_file_name = format!("{}P.png", outer_result.id);
            write_to_file(
                &format!("src/img/abilities/{}", &passive_file_name),
                &passive_bytes,
            );
            for (index, spell) in spells.into_iter().enumerate() {
                let inner_id = outer_result.id.clone();
                let inner_base_uri = outer_base_uri.clone();
                let inner_client = outer_client.clone();
                inner_futures.push(tokio::spawn(async move {
                    let label_vec = vec!["Q", "W", "E", "R"];
                    let url = format!("{}/spell/{}", inner_base_uri, spell.image.full);
                    let spell_dir = "src/img/abilities";
                    let file_name = format!(
                        "{}{}.png",
                        &inner_id,
                        &label_vec
                            .get(index)
                            .unwrap_or(&format!("_Error_{}", index).as_str())
                    );
                    let spell_response = inner_client.get(&url).send().await.unwrap();
                    let spell_bytes = spell_response.bytes().await.unwrap();
                    write_to_file(
                        &format!("{}/{}", spell_dir, file_name.as_str()),
                        &spell_bytes,
                    );
                }));
            }
            for inner_future in inner_futures {
                let _ = inner_future.await.unwrap();
            }
        }));
    }
    for outer_future in outer_futures {
        let _ = outer_future.await.unwrap();
    }
}

pub async fn img_download_arts() {
    let files = fs::read_dir("src/cache/riot/champions").unwrap();
    let base_uri = std::env::var("DD_DRAGON_ENDPOINT").expect("DD_DRAGON_ENDPOINT is not set");
    let client = reqwest::Client::new();
    for file in files {
        let path_buf = file.unwrap().path();
        let path_name = path_buf.to_str().unwrap();
        let outer_result = read_from_file::<RiotCdnChampion>(path_name);
        let skins = outer_result.skins;
        let mut inner_futures = Vec::new();
        for skin in skins.into_iter() {
            let inner_id = outer_result.id.clone();
            let inner_base_uri = base_uri.clone();
            let inner_client = client.clone();
            inner_futures.push(tokio::spawn(async move {
                let label_vec = ["centered", "splash"];
                for label in label_vec {
                    let skin_number = skin.num;
                    let url = format!(
                        "{}/img/champion/{}/{}_{}.jpg",
                        inner_base_uri, label, inner_id, &skin_number
                    );
                    let label_dir = format!("src/img/{}", label);
                    let file_name = format!("{}_{}.jpg", &inner_id, &skin_number);
                    let label_response = inner_client.get(&url).send().await.unwrap();
                    let label_bytes = label_response.bytes().await.unwrap();
                    write_to_file(&format!("{}/{}", label_dir, &file_name), &label_bytes);
                }
            }));
        }
        for inner_future in inner_futures {
            let _ = inner_future.await.unwrap();
        }
    }
}

pub async fn img_download_runes() {
    let runes_data = read_from_file::<Vec<RiotCdnRune>>("src/cache/riot/runes.json");
    let client = reqwest::Client::new();
    let mut rune_futures = Vec::new();
    let mut runes_map = HashMap::<usize, String>::new();
    let endpoint = std::env::var("RIOT_IMAGE_ENDPOINT").expect("RIOT_IMAGE_ENDPOINT is not set");
    for value in runes_data {
        runes_map.insert(value.id, value.icon);
        for slot in value.slots {
            for rune in slot.runes {
                runes_map.insert(rune.id, rune.icon);
            }
        }
    }
    for (rune_id, rune_icon) in runes_map {
        let url = format!("{}/{}", endpoint, rune_icon);
        let file_name = format!("{}.png", rune_id);
        let cloned_client = client.clone();
        rune_futures.push(tokio::spawn(async move {
            let rune_response = cloned_client.get(&url).send().await.unwrap();
            let rune_bytes = rune_response.bytes().await.unwrap();
            write_to_file(&format!("src/img/runes/{}", file_name), &rune_bytes);
        }));
    }
    for rune_future in rune_futures {
        let _ = rune_future.await;
    }
}

pub async fn img_download_items() {
    let files = fs::read_dir("src/cache/riot/items").unwrap();
    let base_uri = get_base_uri();
    let client = reqwest::Client::new();
    let mut item_futures = Vec::new();
    for file in files {
        let cloned_base_uri = base_uri.clone();
        let cloned_client = client.clone();
        item_futures.push(tokio::spawn(async move {
            let path_buf = file.unwrap().path();
            let item_id = extract_file_name(&path_buf);
            let item_icon_url = format!("{}/item/{}.png", cloned_base_uri, item_id);
            let item_response = cloned_client.get(&item_icon_url).send().await.unwrap();
            let item_bytes = item_response.bytes().await.unwrap();
            let item_dir = "src/img/items";
            let item_file_name = format!("{}.png", item_id);
            write_to_file(&format!("{}/{}", item_dir, item_file_name), &item_bytes);
        }));
    }
    for item_future in item_futures {
        let _ = item_future.await.unwrap();
    }
}
