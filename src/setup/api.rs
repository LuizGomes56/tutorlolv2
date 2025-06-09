use std::{collections::HashMap, sync::Arc};

use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::{EnvConfig, setup::helpers::SetupError};

// Helper function to fetch data from the CDN. Returns a HashMap with `any` value.
pub async fn fetch_cdn_api(
    client: Client,
    envcfg: Arc<EnvConfig>,
    path_name: &str,
) -> Result<HashMap<String, Value>, SetupError> {
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

// Fetches DataDragon API from Riot Games. Only the final file path needs to be provided
pub async fn fetch_riot_api<T: DeserializeOwned>(
    client: Client,
    envcfg: Arc<EnvConfig>,
    path_name: &str,
) -> Result<T, SetupError> {
    let url: &String = &format!(
        "{}/{}/data/{}/{}.json",
        envcfg.dd_dragon_endpoint.trim_end_matches('/'),
        envcfg.lol_version,
        envcfg.lol_language,
        path_name
    );

    println!("fetch_riot_api: {}", url);

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
