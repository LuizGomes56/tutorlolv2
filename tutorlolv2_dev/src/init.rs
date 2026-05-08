use crate::client::HttpClient;
use std::sync::LazyLock;

/// Loads environment variables or panics if they're not set
macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(&concat!("[env] ", $name, " is not set"))
    };
}

/// Holds all environment variables. Example of `.env` file
/// ```toml
/// LOL_VERSION=*
/// LOL_LANGUAGE=en_US
/// DD_DRAGON_ENDPOINT=https://ddragon.leagueoflegends.com
/// RIOT_IMAGE_ENDPOINT=https://ddragon.canisback.com/img
/// META_ENDPOINT=*
/// META_ASSETS=*
/// ```
pub struct EnvConfig {
    pub lol_version: String,
    pub lol_language: String,
    pub dd_dragon_endpoint: String,
    pub riot_image_endpoint: String,
    pub meta_endpoint: String,
    pub meta_assets: String,
}

impl EnvConfig {
    /// Creates a new struct containing all relevant environment variables
    pub fn new() -> Self {
        EnvConfig {
            lol_version: env_var!("LOL_VERSION"),
            lol_language: env_var!("LOL_LANGUAGE"),
            dd_dragon_endpoint: env_var!("DD_DRAGON_ENDPOINT"),
            riot_image_endpoint: env_var!("RIOT_IMAGE_ENDPOINT"),
            meta_endpoint: env_var!("META_ENDPOINT"),
            meta_assets: env_var!("META_ASSETS"),
        }
    }
}

/// Holds all useful environment variables this application will use
pub static ENV_CONFIG: LazyLock<EnvConfig> = LazyLock::new(EnvConfig::new);

/// Wrapper around [`reqwest::Client`] which implements methods
/// to download and save files to a local cache and avoids requests
/// to the same URLs
pub static HTTP_CLIENT: LazyLock<HttpClient> = LazyLock::new(HttpClient::new);
