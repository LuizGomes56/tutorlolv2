use once_cell::sync::Lazy;

macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(&concat!("[env] ", $name, " is not set"))
    };
}

/// Example of `.env` file
/// ```toml
/// DATABASE_URL=postgresql://postgres:{PASSWORD}@localhost:5432/{USER}
/// HOST=127.0.0.1:*
/// LOL_VERSION=*
/// LOL_LANGUAGE=en_US
/// SYSTEM_PASSWORD={SYSTEM_PASSWORD}
/// CDN_ENDPOINT=https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US
/// DD_DRAGON_ENDPOINT=https://ddragon.leagueoflegends.com
/// RIOT_IMAGE_ENDPOINT=https://ddragon.canisback.com/img
/// META_ENDPOINT=*
/// ```
pub struct EnvConfig {
    pub lol_version: String,
    pub lol_language: String,
    pub cdn_endpoint: String,
    pub dd_dragon_endpoint: String,
    pub riot_image_endpoint: String,
    pub meta_endpoint: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        EnvConfig {
            lol_version: env_var!("LOL_VERSION"),
            lol_language: env_var!("LOL_LANGUAGE"),
            cdn_endpoint: env_var!("CDN_ENDPOINT"),
            dd_dragon_endpoint: env_var!("DD_DRAGON_ENDPOINT"),
            riot_image_endpoint: env_var!("RIOT_IMAGE_ENDPOINT"),
            meta_endpoint: env_var!("META_ENDPOINT"),
        }
    }
}

/// Environment variables are only loaded when needed
/// Likely to be used when "dev-routes" feature is enabled
pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(EnvConfig::new);
