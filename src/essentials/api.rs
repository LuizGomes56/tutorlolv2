use crate::ENV_CONFIG;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Display;

/// DDragon base URL
pub fn riot_base_url() -> String {
    format!(
        "{}/cdn/{}",
        ENV_CONFIG.dd_dragon_endpoint, ENV_CONFIG.lol_version
    )
}

#[derive(Copy, Clone)]
pub enum CdnEndpoint {
    Champions,
    Items,
}

impl Display for CdnEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CdnEndpoint::Champions => write!(f, "champions"),
            CdnEndpoint::Items => write!(f, "items"),
        }
    }
}

/// Helper function to fetch data from the CDN. Returns a FxHashMap with `any` value.
#[generator_macros::trace_time]
pub async fn fetch_cdn_api<T: DeserializeOwned>(
    client: Client,
    path_name: CdnEndpoint,
) -> Result<T, Box<dyn std::error::Error>> {
    let url = format!("{}/{}.json", ENV_CONFIG.cdn_endpoint, path_name);

    println!("fetch_cdn_api: {}", url);

    let res = client.get(&url).send().await?;
    let data = res.json::<T>().await?;
    Ok(data)
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
