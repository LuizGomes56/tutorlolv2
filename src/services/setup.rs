use crate::model::champions::CdnChampion;
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, io::Write, path::Path};

async fn fetch_api(path: &str) -> HashMap<String, Value> {
    let url = std::env::var("CDN_ENDPOINT").expect("CDN_ENDPOINT is not set");
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/{}", url.trim_end_matches('/'), path))
        .send()
        .await
        .expect("Failed to send request");

    let data: Value = res.json().await.expect("Failed to parse JSON");

    let result = data.as_object().expect("Failed to convert JSON to object");

    result.clone().into_iter().collect()
}

pub async fn write_champions() {
    let result = fetch_api("champions.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/champions/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

pub async fn write_items() {
    let result = fetch_api("items.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/items/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

pub fn setup_folders() {
    for dir in &[
        "cache",
        "cache/cdn",
        "cache/cdn/champions",
        "cache/cdn/items",
    ] {
        let path = Path::new(dir);
        if !path.exists() {
            let error_msg = format!("Unable to create directory '{}'", dir);

            fs::create_dir_all(path).expect(&error_msg);
        }
    }
}

fn write_to_file(path_name: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(path_name).expect("Unable to create file");
    file.write_all(bytes).expect("Unable to write data");
}

pub fn setup_champion_cache() {
    let files = fs::read_dir("cache/cdn/champions").expect("Unable to read directory");

    for file in files {
        let path = file.unwrap().path();
        generate_champion_file(path.to_str().expect("Failed to convert path to string"));
    }
}

pub fn generate_champion_file(path: &str) {
    let error_msg = format!("Unable to parse JSON for file '{}'", path);

    let data = fs::read_to_string(path).expect("Unable to read file");
    let result: CdnChampion = serde_json::from_str(&data).expect(&error_msg);

    println!("{:?}", result.name);
}
