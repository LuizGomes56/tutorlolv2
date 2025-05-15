use crate::{model::items::Item, setup::extract_file_name};

use super::{read_from_file, write_to_file};
use std::{collections::HashMap, fs, sync::Arc};

use scraper::{Html, Selector};
use serde_json::Value;

type _MetaItemValue<T> = HashMap<String, HashMap<String, Vec<T>>>;

fn _replace_item_names_with_ids() {
    let mut meta_items = read_from_file::<_MetaItemValue<Value>>("src/internal/meta_items.json");
    let items_folder = fs::read_dir("src/internal/items").expect("Failed to read items folder");

    for entry in items_folder {
        let entry = entry.expect("Invalid DirEntry");
        let path = entry.path();
        let file_name = extract_file_name(&path);
        let item_id = file_name.parse::<usize>().unwrap_or(0);

        let internal_item =
            read_from_file::<Item>(path.to_str().expect("Failed to convert path to string"));

        for (_, positions) in meta_items.iter_mut() {
            for (_, items) in positions.iter_mut() {
                for item in items.iter_mut() {
                    if let Value::String(s) = item {
                        if s == &internal_item.name {
                            *item = Value::Number(item_id.into());
                        }
                    }
                }
            }
        }
    }
    write_to_file(
        "src/internal/meta_items.json",
        serde_json::to_string_pretty(&meta_items)
            .unwrap()
            .as_bytes(),
    );
}

// Website can't handle too many requests at once, making this function worse than a complete synchronous one
// Even with a semafor of 2, it still gets rate limited and requests start to timeout
async fn _get_meta_items() {
    let client_arc = reqwest::Client::new();

    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    let positions = ["top", "jungle", "mid", "adc", "support"];
    let mut final_map = _MetaItemValue::<usize>::new();
    let mut outer_future = Vec::new();

    let endpoint = std::env::var("META_ENDPOINT").expect("META_ENDPOINT is not set");
    let replacement_str = std::env::var("META_REPLACEMENT").expect("META_REPLACEMENT is not set");
    let semaphore = Arc::new(tokio::sync::Semaphore::new(2));

    for (_, name) in champion_names {
        let outer_client_clone = client_arc.clone();
        let endpoint = endpoint.clone();
        let replacement_str = replacement_str.clone();
        let semaphore = semaphore.clone();

        outer_future.push(tokio::spawn(async move {
            let mut inner_future = Vec::new();
            let mut temp_map = HashMap::<String, Vec<usize>>::new();

            for position in positions {
                let champion_name = name.to_lowercase().clone();
                let inner_client_clone = outer_client_clone.clone();
                let endpoint = endpoint.clone();
                let replacement_str = replacement_str.clone();
                let semaphore = semaphore.clone();
                let _permit = semaphore.acquire_owned().await.unwrap();

                inner_future.push(tokio::spawn(async move {
                    println!(
                        "Fetching meta items for {}, position: {}",
                        champion_name, position
                    );

                    let url = format!("{}/{}/build/{}", endpoint.clone(), champion_name, position);

                    let res = inner_client_clone
                        .get(url)
                        .send()
                        .await
                        .expect("Failed to send request");

                    let mut result = HashMap::<String, Vec<usize>>::new();

                    let html = res.text().await.expect("Could not read response text");
                    let document = Html::parse_document(&html);
                    let full_build = Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")
                        .expect("Failed to parse nested selector");
                    let situational_build = Selector::parse(".m-s76v8c > div > div img")
                        .expect("Failed to parse selector .m-s76v8c");

                    let mut items = Vec::<usize>::new();
                    let mut push_items = |selector: &Selector| {
                        for img in document.select(selector) {
                            if let Some(src) = img.value().attr("src") {
                                let item_str = src.replace(&replacement_str, "");
                                if let Some(id_str) = item_str.split(".png").next() {
                                    if let Ok(id) = id_str.parse::<usize>() {
                                        items.push(id);
                                    }
                                }
                            }
                        }
                    };
                    push_items(&full_build);
                    push_items(&situational_build);
                    result.insert(String::from(position), items);
                    result
                }));
            }
            for inner_result in inner_future {
                let inner_task_result = inner_result.await;
                match inner_task_result {
                    Ok(inner_task) => temp_map.extend(inner_task),
                    Err(e) => println!("Failed inner task result: {}", e),
                }
            }
            (name, temp_map)
        }));
    }
    for outer_result in outer_future {
        let outer_task_result = outer_result.await;
        match outer_task_result {
            Ok((name, map)) => {
                final_map.insert(name, map);
            }
            Err(e) => println!("Failed outer task result: {}", e),
        }
    }

    write_to_file(
        "src/internal/meta_items.json",
        serde_json::to_string(&final_map).unwrap().as_bytes(),
    );
}
