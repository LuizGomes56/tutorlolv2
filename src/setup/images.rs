use crate::{
    EnvConfig,
    model::riot::{RiotCdnChampion, RiotCdnInstance, RiotCdnRune, RiotCdnSkin},
    setup::{
        api::riot_base_url,
        helpers::{SetupError, extract_file_name, read_json_file, write_to_file},
    },
};
use reqwest::{Client, Response};
use rustc_hash::FxHashMap;
use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tokio::task::JoinHandle;

pub async fn img_download_instances(client: Client, envcfg: Arc<EnvConfig>) {
    let files: ReadDir = fs::read_dir("cache/riot/champions").unwrap();
    let base_uri: String = riot_base_url(envcfg);
    for file in files {
        let outer_base_uri: String = base_uri.clone();
        let outer_client: Client = client.clone();

        let path_buf: PathBuf = file.unwrap().path();
        let path_name: &str = path_buf.to_str().unwrap();
        let outer_result: RiotCdnChampion = read_json_file::<RiotCdnChampion>(path_name).unwrap();
        let spells: Vec<RiotCdnInstance> = outer_result.spells;
        let mut inner_futures: Vec<JoinHandle<()>> = Vec::new();
        let champion_dir: &'static str = "img/champions";
        let champion_file_name: String = format!("{}.png", outer_result.id);
        let champion_file_path: String = format!("{}/{}", champion_dir, &champion_file_name);
        if Path::new(&champion_file_path).exists() {
            println!(
                "[ALREADY_EXISTS] fn[img_download_instances]: [Champion] {}",
                &champion_file_path
            );
        } else {
            let champion_icon_url: String = format!(
                "{}/img/champion/{}",
                outer_base_uri, outer_result.image.full
            );
            println!(
                "[REQUESTING] fn[img_download_instances]: [Champion] {}",
                &champion_icon_url
            );
            let champion_response: Response = client.get(&champion_icon_url).send().await.unwrap();
            let champion_bytes = champion_response.bytes().await.unwrap();
            let _ = write_to_file(&champion_file_path, &champion_bytes)
                .map_err(|e: SetupError| println!("[ERROR] fn[img_download_instances]: {:#?}", e));
        }
        let passive_file_name: String = format!("{}P.png", outer_result.id);
        let passive_file_path: String = format!("img/abilities/{}", &passive_file_name);
        if Path::new(&passive_file_path).exists() {
            println!(
                "[ALREADY_EXISTS] fn[img_download_instances]: [Passive] {}",
                &passive_file_path
            );
        } else {
            let passive_icon_url: String = format!(
                "{}/img/passive/{}",
                outer_base_uri, outer_result.passive.image.full
            );
            println!(
                "[REQUESTING] fn[img_download_instances]: [Passive] {}",
                &passive_icon_url
            );
            let passive_response: Response =
                outer_client.get(&passive_icon_url).send().await.unwrap();
            let passive_bytes = passive_response.bytes().await.unwrap();
            let _ = write_to_file(&passive_file_path, &passive_bytes)
                .map_err(|e: SetupError| println!("[ERROR] fn[img_download_instances]: {:#?}", e));
        }
        for (index, spell) in spells.into_iter().enumerate() {
            let inner_id: String = outer_result.id.clone();
            let inner_base_uri: String = outer_base_uri.clone();
            let inner_client: Client = outer_client.clone();
            inner_futures.push(tokio::spawn(async move {
                let label_vec: [&'static str; 4] = ["Q", "W", "E", "R"];
                let file_name: String = format!(
                    "{}{}.png",
                    &inner_id,
                    &label_vec
                        .get(index)
                        .unwrap_or(&format!("_Error_{}", index).as_str())
                );
                let spell_dir: &'static str = "img/abilities";
                let spell_file_path = format!("{}/{}", spell_dir, file_name.as_str());
                if Path::new(&spell_file_path).exists() {
                    println!(
                        "[ALREADY_EXISTS] fn[img_download_instances]: [Spell] {}",
                        &spell_file_path
                    );
                } else {
                    let url: String = format!("{}/img/spell/{}", inner_base_uri, spell.image.full);
                    println!("[REQUESTING] fn[img_download_instances]: [Spell] {}", &url);
                    let spell_response: Response = inner_client.get(&url).send().await.unwrap();
                    let spell_bytes = spell_response.bytes().await.unwrap();
                    let _ =
                        write_to_file(&spell_file_path, &spell_bytes).map_err(|e: SetupError| {
                            println!("[ERROR] fn[img_download_instances]: {:#?}", e)
                        });
                }
            }));
        }
        for inner_future in inner_futures {
            let _ = inner_future.await.unwrap();
        }
    }
    println!(
        "[COMPLETED] fn[img_download_instances]: {:#?}",
        Instant::now()
    );
}

pub async fn img_download_arts(client: Client, envcfg: Arc<EnvConfig>) {
    let files: ReadDir = fs::read_dir("cache/riot/champions").unwrap();
    let base_uri: String = format!("{}/cdn", envcfg.dd_dragon_endpoint.clone());
    for file in files {
        let path_buf: PathBuf = file.unwrap().path();
        let path_name: &str = path_buf.to_str().unwrap();
        let outer_result: RiotCdnChampion = read_json_file::<RiotCdnChampion>(path_name).unwrap();
        let skins: Vec<RiotCdnSkin> = outer_result.skins;
        let mut inner_futures: Vec<JoinHandle<()>> = Vec::new();
        for skin in skins.into_iter() {
            let inner_id: String = outer_result.id.clone();
            let inner_base_uri: String = base_uri.clone();
            let inner_client: Client = client.clone();
            inner_futures.push(tokio::spawn(async move {
                let label_vec: [&'static str; 2] = ["centered", "splash"];
                for label in label_vec {
                    let skin_number: usize = skin.num;
                    let url: String = format!(
                        "{}/img/champion/{}/{}_{}.jpg",
                        inner_base_uri, label, inner_id, &skin_number
                    );
                    let label_dir: String = format!("img/{}", label);
                    let file_name: String = format!("{}_{}.jpg", &inner_id, &skin_number);
                    let final_name: String = format!("{}/{}", label_dir, &file_name);
                    if Path::new(&final_name).exists() {
                        println!(
                            "[ALREADY_EXISTS] fn[img_download_arts]: [Arts] {}",
                            &final_name
                        );
                    } else {
                        println!("[REQUESTING] fn[img_download_arts]: [Arts] {}", &url);
                        let label_response: Response = inner_client.get(&url).send().await.unwrap();
                        let label_bytes = label_response.bytes().await.unwrap();
                        let _ =
                            write_to_file(&final_name, &label_bytes).map_err(|e: SetupError| {
                                println!("[ERROR] fn[img_download_arts]: {:#?}", e)
                            });
                    }
                }
            }));
        }
        for inner_future in inner_futures {
            let _ = inner_future.await.unwrap();
        }
    }
    println!("[COMPLETED] fn[img_download_arts]: {:#?}", Instant::now());
}

pub async fn img_download_runes(client: Client, envcfg: Arc<EnvConfig>) {
    let runes_data: Vec<RiotCdnRune> =
        read_json_file::<Vec<RiotCdnRune>>("cache/riot/runes.json").unwrap();
    let mut rune_futures: Vec<JoinHandle<()>> = Vec::new();
    let mut runes_map: FxHashMap<usize, String> = FxHashMap::<usize, String>::default();
    let endpoint: &str = &envcfg.riot_image_endpoint;
    for value in runes_data {
        runes_map.insert(value.id, value.icon);
        for slot in value.slots {
            for rune in slot.runes {
                runes_map.insert(rune.id, rune.icon);
            }
        }
    }
    for (rune_id, rune_icon) in runes_map {
        let url: String = format!("{}/{}", endpoint, rune_icon);
        let file_name: String = format!("{}.png", rune_id);
        let cloned_client: Client = client.clone();
        rune_futures.push(tokio::spawn(async move {
            let rune_file_path = format!("img/runes/{}", file_name);
            if Path::new(&rune_file_path).exists() {
                println!(
                    "[ALREADY_EXISTS] fn[img_download_runes]: [Rune] {}",
                    &rune_file_path
                );
            } else {
                println!("[REQUESTING] fn[img_download_runes]: [Rune] {}", &url);
                let rune_response: Response = cloned_client.get(&url).send().await.unwrap();
                let rune_bytes = rune_response.bytes().await.unwrap();
                let _ = write_to_file(&rune_file_path, &rune_bytes)
                    .map_err(|e: SetupError| println!("[ERROR] fn[img_download_runes]: {:#?}", e));
            }
        }));
    }
    for rune_future in rune_futures {
        let _ = rune_future.await;
    }
    println!("[COMPLETED] fn[img_download_runes]: {:#?}", Instant::now());
}

pub async fn img_download_items(client: Client, envcfg: Arc<EnvConfig>) {
    let files: ReadDir = fs::read_dir("cache/riot/items").unwrap();
    let base_uri: String = riot_base_url(envcfg);
    let mut item_futures: Vec<JoinHandle<()>> = Vec::new();
    for file in files {
        let cloned_base_uri: String = base_uri.clone();
        let cloned_client: Client = client.clone();
        item_futures.push(tokio::spawn(async move {
            let path_buf: PathBuf = file.unwrap().path();
            let item_id: &str = extract_file_name(&path_buf);
            let item_dir: &'static str = "img/items";
            let item_file_name: String = format!("{}.png", item_id);
            let item_file_path: String = format!("{}/{}", item_dir, item_file_name);
            if Path::new(&item_file_path).exists() {
                println!(
                    "[ALREADY_EXISTS] fn[img_download_items]: [Item] {}",
                    &item_file_path
                );
            } else {
                let item_icon_url: String = format!("{}/item/{}.png", cloned_base_uri, item_id);
                println!(
                    "[REQUESTING] fn[img_download_items]: [Item] {}",
                    &item_icon_url
                );
                let item_response: Response =
                    cloned_client.get(&item_icon_url).send().await.unwrap();
                let item_bytes = item_response.bytes().await.unwrap();
                let _ = write_to_file(&item_file_path, &item_bytes)
                    .map_err(|e: SetupError| println!("[ERROR] fn[img_download_items]: {:#?}", e));
            }
        }));
    }
    for item_future in item_futures {
        let _ = item_future.await.unwrap();
    }
    println!("[COMPLETED] fn[img_download_items]: {:#?}", Instant::now());
}
