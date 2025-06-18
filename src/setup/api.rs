use crate::{EnvConfig, setup::helpers::SetupError};
use reqwest::{Client, Response};
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::sync::Arc;

/// DDragon base URL
pub fn riot_base_url(envcfg: Arc<EnvConfig>) -> String {
    let url: &str = &envcfg.dd_dragon_endpoint;
    let version: &str = &envcfg.lol_version;
    format!("{}/cdn/{}", url, version)
}

/// Helper function to fetch data from the CDN. Returns a FxHashMap with `any` value.
#[writer_macros::trace_time]
pub async fn fetch_cdn_api(
    client: Client,
    envcfg: Arc<EnvConfig>,
    path_name: &str,
) -> Result<FxHashMap<String, Value>, SetupError> {
    let uri: &String = &envcfg.cdn_endpoint;
    let url: String = format!("{}/{}", uri.trim_end_matches('/'), path_name);

    println!("fetch_cdn_api: {}", url);

    let res: Response = client
        .get(&url)
        .send()
        .await
        .map_err(|e: reqwest::Error| SetupError(format!("Failed to send request: {}", e)))?;

    let data: Value = res
        .json()
        .await
        .map_err(|e: reqwest::Error| SetupError(format!("Failed to parse JSON: {}", e)))?;

    let result: &Map<String, Value> = data
        .as_object()
        .ok_or_else(|| SetupError("Expected JSON object at root".to_string()))?;

    Ok(result.clone().into_iter().collect())
}

/// Fetches DataDragon API from Riot Games. Only the final file path needs to be provided
#[writer_macros::trace_time]
pub async fn fetch_riot_api<T: DeserializeOwned>(
    client: Client,
    envcfg: Arc<EnvConfig>,
    path_name: &str,
) -> Result<T, SetupError> {
    let lang: String = envcfg.lol_language.clone();
    let url: &String = &format!("{}/data/{}/{}.json", riot_base_url(envcfg), lang, path_name);

    let res: Response = client
        .get(url)
        .send()
        .await
        .map_err(|e: reqwest::Error| SetupError(format!("Failed to send request: {}", e)))?;

    let data: T = res
        .json::<T>()
        .await
        .map_err(|e: reqwest::Error| SetupError(e.to_string()))?;
    Ok(data)
}

/// Get current game version
#[writer_macros::trace_time]
pub async fn fetch_version(client: Client, envcfg: Arc<EnvConfig>) -> Result<String, SetupError> {
    let url: &String = &format!("{}/api/versions.json", envcfg.dd_dragon_endpoint);

    println!("fetch_version: {}", url);

    let res: Response = client
        .get(url)
        .send()
        .await
        .map_err(|e: reqwest::Error| SetupError(format!("Failed to send request: {}", e)))?;

    let data: Vec<String> = res
        .json::<Vec<String>>()
        .await
        .map_err(|e: reqwest::Error| SetupError(e.to_string()))?;
    Ok(data
        .get(0)
        .ok_or_else(|| SetupError("Version not found".to_string()))?
        .clone())
}
