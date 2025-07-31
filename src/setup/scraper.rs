use crate::{
    init::dev::ENV_CONFIG,
    setup::essentials::helpers::{read_json_file, write_to_file},
};
use reqwest::Client;
use rustc_hash::FxHashMap;
use scraper::{Html, Selector};
use tokio::task::JoinHandle;

/// Recovers all the common builds for the current patch so the app can recommend builds to the user
/// Average time to update is 2m30s. Making the outer loop a new task overloads the target website
/// causing requests to timeout.
#[generator_macros::trace_time]
pub async fn meta_items_scraper(client: Client) {
    let champion_names: FxHashMap<String, String> =
        read_json_file("internal/champion_names.json").unwrap();
    let positions = ["top", "jungle", "mid", "adc", "support"];
    let mut collected_results = FxHashMap::<String, FxHashMap<String, Vec<String>>>::default();

    for (_, name) in champion_names {
        let mut futures_vec: Vec<JoinHandle<Result<FxHashMap<String, Vec<String>>, String>>> =
            Vec::new();
        for position in positions {
            let champion_name = name.to_lowercase().clone();
            let client = client.clone();
            futures_vec.push(tokio::spawn(async move {
                let url = format!(
                    "{}/{}/build/{}",
                    ENV_CONFIG.meta_endpoint, champion_name, position
                );

                let res = client.get(&url).send().await.unwrap();

                let mut result: FxHashMap<String, Vec<String>> =
                    FxHashMap::<String, Vec<String>>::default();

                let html: String = res.text().await.unwrap();
                let document: Html = Html::parse_document(&html);
                let full_build: Selector =
                    Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img").unwrap();
                let situational_build: Selector =
                    Selector::parse(".m-s76v8c > div > div img").unwrap();

                let mut items: Vec<String> = Vec::<String>::new();
                let mut push_items = |selector: &Selector| {
                    for img in document.select(selector) {
                        if let Some(alt) = img.value().attr("alt") {
                            items.push(alt.to_string());
                        }
                    }
                };
                push_items(&full_build);
                push_items(&situational_build);
                result.insert(String::from(position), items);
                Ok(result)
            }));
        }

        let mut collected_result: FxHashMap<String, Vec<String>> = FxHashMap::default();
        for result in futures_vec {
            println!("Fetching meta items for {}", name);
            match result.await {
                Ok(Ok(data)) => {
                    collected_result.extend(data);
                }
                Ok(Err(e)) => eprintln!("Erro na requisição do meta item: {:#?}", e),
                Err(e) => eprintln!("Erro ao aguardar a task: {:?}", e),
            }
        }
        collected_results.insert(name, collected_result);
    }

    let json: String = serde_json::to_string(&collected_results).unwrap();

    write_to_file("internal/meta_items.json", json.as_bytes()).unwrap();
}
