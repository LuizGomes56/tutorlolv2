use crate::{essentials::ext::FilePathExt, init::ENV_CONFIG};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tokio::task::JoinHandle;

/// Recovers all the common builds for the current patch so the app can recommend builds to the user
/// Average time to update is 2m30s. Making the outer loop a new task overloads the target website
/// causing requests to timeout.
#[tutorlolv2_macros::trace_time]
pub async fn data_scraper(client: Client) {
    let champion_names = "internal/champion_names.json"
        .read_json::<HashMap<String, String>>()
        .unwrap();
    let positions = ["top", "jungle", "mid", "adc", "support"];
    let mut collected_results =
        HashMap::<String, HashMap<String, (Vec<String>, Vec<String>)>>::default();

    for (_, name) in champion_names {
        let mut futures_vec =
            Vec::<JoinHandle<Result<HashMap<String, (Vec<String>, Vec<String>)>, String>>>::new();
        for position in positions {
            let champion_name = name.to_lowercase().clone();
            let client = client.clone();
            futures_vec.push(tokio::spawn(async move {
                let url = format!(
                    "{}/{}/build/{}",
                    ENV_CONFIG.meta_endpoint, champion_name, position
                );

                let res = client.get(&url).send().await.unwrap();
                let mut result = HashMap::<String, (Vec<String>, Vec<String>)>::default();

                let html = res.text().await.unwrap();
                let _ = tokio::fs::write(
                    format!("cache/scraper/{champion_name}_{position}.html"),
                    &html,
                )
                .await;
                let document = Html::parse_document(&html);
                let full_build =
                    Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img").unwrap();
                let situational_build = Selector::parse(".m-s76v8c > div > div img").unwrap();
                let rune_selector = Selector::parse("img.m-1nx2cdb").unwrap();
                let legend_selector = Selector::parse("img.m-1u3ui07").unwrap();
                let mut runes = Vec::<String>::new();

                let push_alt_attr = |array: &mut Vec<String>, selector: &Selector| {
                    for img in document.select(selector) {
                        if let Some(alt) = img.value().attr("alt") {
                            array.push(
                                alt.replace(" ", "")
                                    .replace("-", "")
                                    .replace(")", "")
                                    .replace("(", "")
                                    .replace("'", "")
                                    .replace(".", "")
                                    .replace(",", "")
                                    .replace(":", "")
                                    .replace("BladeofTheRuinedKing", "BladeoftheRuinedKing"),
                            );
                        }
                    }
                };

                let mut items = Vec::<String>::new();

                push_alt_attr(&mut runes, &rune_selector);
                push_alt_attr(&mut runes, &legend_selector);
                push_alt_attr(&mut items, &full_build);
                push_alt_attr(&mut items, &situational_build);

                result.insert(String::from(position), (items, runes));
                Ok(result)
            }));
        }

        let mut collected_result = HashMap::<String, (Vec<String>, Vec<String>)>::default();
        for result in futures_vec {
            println!("Fetching meta items for {}", name);
            match result.await {
                Ok(Ok(data)) => {
                    collected_result.extend(data);
                }
                Ok(Err(e)) => eprintln!("Error requesting: {:#?}", e),
                Err(e) => eprintln!("Error awaiting future: {:?}", e),
            }
        }
        collected_results.insert(name, collected_result);
    }

    let json = serde_json::to_string(&collected_results).unwrap();
    "internal/scraped_data.json"
        .write_to_file(json.as_bytes())
        .unwrap();
}
