use crate::client::HttpClient;
use once_cell::sync::Lazy;

/// Loads environment variables or panics if they're not set
macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(&concat!("[env] ", $name, " is not set"))
    };
}

/// Holds all environment variables. Example of `.env` file
/// ```toml
/// DATABASE_URL=postgresql://postgres:{PASSWORD}@localhost:5432/{USER}
/// HOST=127.0.0.1:*
/// LOL_VERSION=*
/// LOL_LANGUAGE=en_US
/// MERAKI_ENDPOINT=https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US
/// DD_DRAGON_ENDPOINT=https://ddragon.leagueoflegends.com
/// RIOT_IMAGE_ENDPOINT=https://ddragon.canisback.com/img
/// META_ENDPOINT=*
/// ```
pub struct EnvConfig {
    pub lol_version: String,
    pub lol_language: String,
    pub meraki_endpoint: String,
    pub dd_dragon_endpoint: String,
    pub riot_image_endpoint: String,
    pub meta_endpoint: String,
}

impl EnvConfig {
    /// Creates a new struct containing all relevant environment variables
    pub fn new() -> Self {
        EnvConfig {
            lol_version: env_var!("LOL_VERSION"),
            lol_language: env_var!("LOL_LANGUAGE"),
            meraki_endpoint: env_var!("MERAKI_ENDPOINT"),
            dd_dragon_endpoint: env_var!("DD_DRAGON_ENDPOINT"),
            riot_image_endpoint: env_var!("RIOT_IMAGE_ENDPOINT"),
            meta_endpoint: env_var!("META_ENDPOINT"),
        }
    }
}

/// Holds all useful environment variables this application will use
pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(EnvConfig::new);

/// Wrapper around [`reqwest::Client`] which implements methods
/// to download and save files to a local cache and avoids requests
/// to the same URLs
pub static HTTP_CLIENT: Lazy<HttpClient> = Lazy::new(HttpClient::new);
