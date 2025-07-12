use crate::ENV_CONFIG;
use reqwest::Client;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// DDragon base URL
pub fn riot_base_url() -> String {
    format!(
        "{}/cdn/{}",
        ENV_CONFIG.dd_dragon_endpoint, ENV_CONFIG.lol_version
    )
}

/// Helper function to fetch data from the CDN. Returns a FxHashMap with `any` value.
#[generator_macros::trace_time]
pub async fn fetch_cdn_api(
    client: Client,
    path_name: &str,
) -> Result<FxHashMap<String, Value>, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", ENV_CONFIG.cdn_endpoint, path_name);

    println!("fetch_cdn_api: {}", url);

    let res = client.get(&url).send().await?;

    let data: Value = res.json().await?;

    let result = data
        .as_object()
        .ok_or_else(|| String::from("Expected JSON object at root"))?;
    Ok(result.clone().into_iter().collect())
}

/// Fetches DataDragon API from Riot Games. Only the final file path needs to be provided
#[generator_macros::trace_time]
pub async fn fetch_riot_api<T: DeserializeOwned>(
    client: Client,
    path_name: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/data/{}/{}.json",
        riot_base_url(),
        ENV_CONFIG.lol_language,
        path_name
    );

    let res = client.get(&url).send().await?;

    let data: T = res.json::<T>().await?;
    Ok(data)
}

/// Get current game version
#[generator_macros::trace_time]
pub async fn fetch_version(
    client: Client,
    endpoint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let res = client
        .get(format!("{}/api/versions.json", endpoint))
        .send()
        .await?;

    let data = res.json::<Vec<String>>().await?;
    Ok(data
        .get(0)
        .ok_or_else(|| String::from("Version not found"))?
        .clone())
}
