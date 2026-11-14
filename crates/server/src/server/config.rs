//! Server settings from the environment (spec §10.4).

const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_RECORDING_TIMEOUT_SECS: i64 = 900;

#[derive(Clone, PartialEq, Eq)]
pub struct Config {
    pub public_base_url: String,
    pub api_public_base_url: String,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_token_endpoint: Option<String>,
    pub session_ttl_days: i64,
    pub cookie_secure: bool,
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

        let session_ttl_days = number::<i64>(&get, "SESSION_TTL_DAYS", 90)?;
        if !(1..=365).contains(&session_ttl_days) {
            return Err("SESSION_TTL_DAYS must be 1..365".into());
        }
        Ok(Config {
            api_public_base_url: get("API_PUBLIC_BASE_URL")
                .unwrap_or_else(|| "http://127.0.0.1:8082".into())
                .trim_end_matches('/')
                .into(),
            google_client_id: get("GOOGLE_CLIENT_ID").filter(|s| !s.is_empty()),
            google_client_secret: get("GOOGLE_CLIENT_SECRET").filter(|s| !s.is_empty()),
            google_token_endpoint: get("GOOGLE_TOKEN_ENDPOINT_TRUSTED_OVERRIDE"),
            session_ttl_days,
            cookie_secure: get("COOKIE_SECURE").as_deref() != Some("false"),
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

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("public_base_url", &self.public_base_url)
            .field("credentials", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}
