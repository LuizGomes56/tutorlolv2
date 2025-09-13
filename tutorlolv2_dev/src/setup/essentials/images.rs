use crate::{
    init::ENV_CONFIG,
    model::riot::{RiotCdnChampion, RiotCdnRune},
    setup::essentials::{
        api::riot_base_url,
        helpers::{extract_file_name, read_json_file, write_to_file},
    },
};
use reqwest::Client;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    path::Path,
};

#[tutorlolv2_macros::trace_time]
pub async fn img_download_instances(client: Client) {
    let files = fs::read_dir("cache/riot/champions").unwrap();
    let base_uri = riot_base_url();
    for file in files {
        let outer_base_uri = base_uri.clone();
        let outer_client = client.clone();

        let path_buf = file.unwrap().path();
        let path_name = path_buf.to_str().unwrap();
        let outer_result: RiotCdnChampion = read_json_file::<RiotCdnChampion>(path_name).unwrap();
        let spells = outer_result.spells;
        let mut inner_futures = Vec::new();
        let champion_dir: &'static str = "raw_img/champions";
        let champion_file_name = format!("{}.png", outer_result.id);
        let champion_file_path = format!("{}/{}", champion_dir, &champion_file_name);
        if Path::new(&champion_file_path).exists() {
            println!(
                "[ALREADY_EXISTS] fn[img_download_instances]: [Champion] {}",
                &champion_file_path
            );
        } else {
            let champion_icon_url = format!(
                "{}/raw_img/champion/{}",
                outer_base_uri, outer_result.image.full
            );
            println!(
                "[REQUESTING] fn[img_download_instances]: [Champion] {}",
                &champion_icon_url
            );
            if let Ok(champion_response) = client.get(&champion_icon_url).send().await {
                let champion_bytes = champion_response.bytes().await.unwrap();
                let _ = write_to_file(&champion_file_path, &champion_bytes)
                    .map_err(|e| println!("[ERROR] fn[img_download_instances]: {:#?}", e));
            }
        }
        let passive_file_name = format!("{}P.png", outer_result.id);
        let passive_file_path = format!("raw_img/abilities/{}", &passive_file_name);
        if Path::new(&passive_file_path).exists() {
            println!(
                "[ALREADY_EXISTS] fn[img_download_instances]: [Passive] {}",
                &passive_file_path
            );
        } else {
            let passive_icon_url = format!(
                "{}/raw_img/passive/{}",
                outer_base_uri, outer_result.passive.image.full
            );
            println!(
                "[REQUESTING] fn[img_download_instances]: [Passive] {}",
                &passive_icon_url
            );
            if let Ok(passive_response) = outer_client.get(&passive_icon_url).send().await {
                let passive_bytes = passive_response.bytes().await.unwrap();
                let _ = write_to_file(&passive_file_path, &passive_bytes)
                    .map_err(|e| println!("[ERROR] fn[img_download_instances]: {:#?}", e));
            };
        }
        for (index, spell) in spells.into_iter().enumerate() {
            let inner_id = outer_result.id.clone();
            let inner_base_uri = outer_base_uri.clone();
            let inner_client = outer_client.clone();
            inner_futures.push(tokio::spawn(async move {
                let label_vec = ["Q", "W", "E", "R"];
                let file_name = format!(
                    "{}{}.png",
                    &inner_id,
                    &label_vec
                        .get(index)
                        .unwrap_or(&format!("_Error_{}", index).as_str())
                );
                let spell_dir = "raw_img/abilities";
                let spell_file_path = format!("{}/{}", spell_dir, file_name.as_str());
                if Path::new(&spell_file_path).exists() {
                    println!(
                        "[ALREADY_EXISTS] fn[img_download_instances]: [Spell] {}",
                        &spell_file_path
                    );
                } else {
                    let url = format!("{}/raw_img/spell/{}", inner_base_uri, spell.image.full);
                    println!("[REQUESTING] fn[img_download_instances]: [Spell] {}", &url);
                    if let Ok(spell_response) = inner_client.get(&url).send().await {
                        let spell_bytes = spell_response.bytes().await.unwrap();
                        let _ = write_to_file(&spell_file_path, &spell_bytes)
                            .map_err(|e| println!("[ERROR] fn[img_download_instances]: {:#?}", e));
                    }
                }
            }));
        }
        for inner_future in inner_futures {
            let _ = inner_future.await;
        }
    }
}

#[tutorlolv2_macros::trace_time]
pub async fn img_download_arts(client: Client) {
    let files: ReadDir = fs::read_dir("cache/riot/champions").unwrap();
    let base_uri = format!("{}/cdn", ENV_CONFIG.dd_dragon_endpoint);
    for file in files {
        let path_buf = file.unwrap().path();
        let path_name = path_buf.to_str().unwrap();
        let outer_result: RiotCdnChampion = read_json_file::<RiotCdnChampion>(path_name).unwrap();
        let skins = outer_result.skins;
        let mut inner_futures = Vec::new();
        for skin in skins.into_iter() {
            let inner_id = outer_result.id.clone();
            let inner_base_uri = base_uri.clone();
            let inner_client = client.clone();
            inner_futures.push(tokio::spawn(async move {
                let label_vec: [&'static str; 2] = ["centered", "splash"];
                for label in label_vec {
                    let skin_number = skin.num;
                    let url = format!(
                        "{}/raw_img/champion/{}/{}_{}.jpg",
                        inner_base_uri, label, inner_id, &skin_number
                    );
                    let label_dir = format!("raw_img/{}", label);
                    let file_name = format!("{}_{}.jpg", &inner_id, &skin_number);
                    let final_name = format!("{}/{}", label_dir, &file_name);
                    if Path::new(&final_name).exists() {
                        println!(
                            "[ALREADY_EXISTS] fn[img_download_arts]: [Arts] {}",
                            &final_name
                        );
                    } else {
                        println!("[REQUESTING] fn[img_download_arts]: [Arts] {}", &url);
                        if let Ok(label_response) = inner_client.get(&url).send().await {
                            let label_bytes = label_response.bytes().await.unwrap();
                            let _ = write_to_file(&final_name, &label_bytes)
                                .map_err(|e| println!("[ERROR] fn[img_download_arts]: {:#?}", e));
                        }
                    }
                }
            }));
        }
        for inner_future in inner_futures {
            let _ = inner_future.await;
        }
    }
}

#[tutorlolv2_macros::trace_time]
pub async fn img_download_runes(client: Client) {
    let runes_data = read_json_file::<Vec<RiotCdnRune>>("cache/riot/runes.json").unwrap();
    let mut rune_futures = Vec::new();
    let mut runes_map = HashMap::<usize, String>::default();
    for value in runes_data {
        runes_map.insert(value.id, value.icon);
        for slot in value.slots {
            for rune in slot.runes {
                runes_map.insert(rune.id, rune.icon);
            }
        }
    }
    for (rune_id, rune_icon) in runes_map {
        let url = format!("{}/{}", ENV_CONFIG.riot_image_endpoint, rune_icon);
        let file_name = format!("{}.png", rune_id);
        let cloned_client = client.clone();
        rune_futures.push(tokio::spawn(async move {
            let rune_file_path = format!("raw_img/runes/{}", file_name);
            if Path::new(&rune_file_path).exists() {
                println!(
                    "[ALREADY_EXISTS] fn[img_download_runes]: [Rune] {}",
                    &rune_file_path
                );
            } else {
                println!("[REQUESTING] fn[img_download_runes]: [Rune] {}", &url);
                if let Ok(rune_response) = cloned_client.get(&url).send().await {
                    let rune_bytes = rune_response.bytes().await.unwrap();
                    let _ = write_to_file(&rune_file_path, &rune_bytes)
                        .map_err(|e| println!("[ERROR] fn[img_download_runes]: {:#?}", e));
                };
            }
        }));
    }
    for rune_future in rune_futures {
        let _ = rune_future.await;
    }
}

#[tutorlolv2_macros::trace_time]
pub async fn img_download_items(client: Client) {
    let files = fs::read_dir("cache/riot/items").unwrap();
    let base_uri = riot_base_url();
    let mut item_futures = Vec::new();
    for file in files {
        let cloned_base_uri = base_uri.clone();
        let cloned_client = client.clone();
        item_futures.push(tokio::spawn(async move {
            let path_buf = file.unwrap().path();
            let item_id = extract_file_name(&path_buf);
            let item_dir: &'static str = "raw_img/items";
            let item_file_name = format!("{}.png", item_id);
            let item_file_path = format!("{}/{}", item_dir, item_file_name);
            if Path::new(&item_file_path).exists() {
                println!(
                    "[ALREADY_EXISTS] fn[img_download_items]: [Item] {}",
                    &item_file_path
                );
            } else {
                let item_icon_url = format!("{}/raw_img/item/{}.png", cloned_base_uri, item_id);
                println!(
                    "[REQUESTING] fn[img_download_items]: [Item] {}",
                    &item_icon_url
                );
                if let Ok(item_response) = cloned_client.get(&item_icon_url).send().await {
                    let item_bytes = item_response.bytes().await.unwrap();
                    let _ = write_to_file(&item_file_path, &item_bytes)
                        .map_err(|e| println!("[ERROR] fn[img_download_items]: {:#?}", e));
                };
            }
        }));
    }
    for item_future in item_futures {
        let _ = item_future.await;
    }
}
