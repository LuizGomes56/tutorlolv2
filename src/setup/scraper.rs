use crate::{
    EnvConfig,
    setup::helpers::{SetupError, read_json_file, write_to_file},
};
use reqwest::{Client, Response};
use scraper::{Html, Selector, error::SelectorErrorKind};
use std::{collections::HashMap, sync::Arc};
use tokio::task::JoinHandle;

// Recovers all the common builds for the current patch so the app can recommend builds to the user
// Average time to update is 2m30s. Making the outer loop a new task overloads the target website
// causing requests to timeout.
pub async fn meta_items_scraper(client: Client, envcfg: Arc<EnvConfig>) -> Result<(), SetupError> {
    let champion_names: HashMap<String, String> = read_json_file("internal/champion_names.json")?;
    let endpoint: String = envcfg.meta_endpoint.clone();
    let positions: [&'static str; 5] = ["top", "jungle", "mid", "adc", "support"];
    let mut collected_results: HashMap<String, HashMap<String, Vec<String>>> =
        HashMap::<String, HashMap<String, Vec<String>>>::new();

    for (_, name) in champion_names {
        let mut futures_vec: Vec<JoinHandle<Result<HashMap<String, Vec<String>>, SetupError>>> =
            Vec::new();
        for position in positions {
            let endpoint: String = endpoint.clone();
            let champion_name: String = name.to_lowercase().clone();
            let client: Client = client.clone();
            futures_vec.push(tokio::spawn(async move {
                let url: String = format!("{}/{}/build/{}", endpoint, champion_name, position);

                let res: Response = client
                    .get(url)
                    .send()
                    .await
                    .map_err(|e: reqwest::Error| SetupError(e.to_string()))?;

                let mut result: HashMap<String, Vec<String>> =
                    HashMap::<String, Vec<String>>::new();

                let html: String = res
                    .text()
                    .await
                    .map_err(|e: reqwest::Error| SetupError(e.to_string()))?;
                let document: Html = Html::parse_document(&html);
                let full_build: Selector =
                    Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")
                        .map_err(|e: SelectorErrorKind<'_>| SetupError(e.to_string()))?;
                let situational_build: Selector = Selector::parse(".m-s76v8c > div > div img")
                    .map_err(|e: SelectorErrorKind<'_>| SetupError(e.to_string()))?;

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

        let mut collected_result: HashMap<String, Vec<String>> = HashMap::new();
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

    let json: String = serde_json::to_string(&collected_results)
        .map_err(|e: serde_json::Error| SetupError(e.to_string()))?;

    write_to_file("internal/meta_items.json", json.as_bytes())?;

    Ok(())
}
