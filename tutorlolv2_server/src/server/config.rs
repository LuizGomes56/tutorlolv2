//! Server settings from the environment (spec §10.4).

const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_RECORDING_TIMEOUT_SECS: i64 = 900;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub public_base_url: String,
    pub max_viewers_per_code: usize,
    pub database_url: String,
    pub db_max_connections: u32,
    /// Riot enrichment is disabled without a key.
    pub riot_api_key: Option<String>,
    /// A recording with no ingest or heartbeat for this long is ended by the janitor.
    pub recording_timeout_secs: i64,
    /// Rate limits read the client IP from `X-Forwarded-For` (only behind a reverse proxy).
    pub trusted_proxy: bool,
    /// Sends every Riot API request to this base URL instead of `*.api.riotgames.com`. Tests only.
    pub riot_api_base_override: Option<String>,
}

impl Config {
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Config, String> {
        fn number<T: std::str::FromStr>(
            get: &impl Fn(&str) -> Option<String>,
            key: &str,
            default: T,
        ) -> Result<T, String> {
            match get(key) {
                Some(value) => value
                    .parse()
                    .map_err(|_| format!("{key} is not a valid number: {value}")),
                None => Ok(default),
            }
        }

        Ok(Config {
            public_base_url: get("PUBLIC_BASE_URL")
                .unwrap_or_else(|| "http://127.0.0.1:8000".into())
                .trim_end_matches('/')
                .to_owned(),
            max_viewers_per_code: number::<usize>(&get, "MAX_VIEWERS_PER_CODE", 200)?.max(1),
            database_url: get("DATABASE_URL").ok_or("DATABASE_URL is not set")?,
            db_max_connections: number(&get, "DB_MAX_CONNECTIONS", DEFAULT_DB_MAX_CONNECTIONS)?,
            riot_api_key: get("RIOT_API_KEY"),
            recording_timeout_secs: number(
                &get,
                "RECORDING_TIMEOUT_SECS",
                DEFAULT_RECORDING_TIMEOUT_SECS,
            )?,
            trusted_proxy: get("TRUSTED_PROXY").is_some(),
            riot_api_base_override: get("RIOT_API_BASE_OVERRIDE"),
        })
    }

    pub fn from_env() -> Result<Config, String> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }
}
