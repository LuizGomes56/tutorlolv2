use crate::{ENV_CONFIG, setup::helpers::SetupError};
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
pub(crate) async fn fetch_cdn_api(
    client: Client,
    path_name: &str,
) -> Result<FxHashMap<String, Value>, SetupError> {
    let url = format!("{}/{}", ENV_CONFIG.cdn_endpoint, path_name);

    println!("fetch_cdn_api: {}", url);

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| SetupError(format!("Failed to send request: {}", e)))?;

    let data: Value = res
        .json()
        .await
        .map_err(|e| SetupError(format!("Failed to parse JSON: {}", e)))?;

    let result = data
        .as_object()
        .ok_or_else(|| SetupError("Expected JSON object at root".to_string()))?;

    Ok(result.clone().into_iter().collect())
}

/// Fetches DataDragon API from Riot Games. Only the final file path needs to be provided
#[generator_macros::trace_time]
pub(crate) async fn fetch_riot_api<T: DeserializeOwned>(
    client: Client,
    path_name: &str,
) -> Result<T, SetupError> {
    let url = format!(
        "{}/data/{}/{}.json",
        riot_base_url(),
        ENV_CONFIG.lol_language,
        path_name
    );

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| SetupError(format!("Failed to send request: {}", e)))?;

    let data: T = res
        .json::<T>()
        .await
        .map_err(|e| SetupError(e.to_string()))?;
    Ok(data)
}

/// Get current game version
#[generator_macros::trace_time]
pub async fn fetch_version(client: Client, endpoint: &str) -> Result<String, SetupError> {
    let res = client
        .get(format!("{}/api/versions.json", endpoint))
        .send()
        .await
        .map_err(|e| SetupError(format!("Failed to send request: {}", e)))?;

    let data = res
        .json::<Vec<String>>()
        .await
        .map_err(|e| SetupError(e.to_string()))?;
    Ok(data
        .get(0)
        .ok_or_else(|| SetupError("Version not found".to_string()))?
        .clone())
}
