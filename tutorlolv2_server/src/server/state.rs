use super::live::LiveHub;
use super::{
    auth::AuthCache,
    config::Config,
    riot_api::{self, EnrichQueue, RiotClient},
};
use sqlx::PgPool;

/// Shared by every handler through `web::Data`.
pub struct AppState {
    pub missing_codes: super::sharing::MissingCodes,
    pub live: LiveHub,
    pub db: PgPool,
    pub config: Config,
    pub auth_cache: AuthCache,
    pub enrich: EnrichQueue,
}

impl AppState {
    /// Starts the Riot enrichment worker when `RIOT_API_KEY` is set, so it must
    /// run inside the Tokio runtime.
    pub fn new(db: PgPool, config: Config) -> Self {
        let enrich = match &config.riot_api_key {
            Some(key) => riot_api::spawn(
                db.clone(),
                RiotClient::new(key.clone(), config.riot_api_base_override.clone()),
            ),
            None => EnrichQueue::disabled(),
        };

        Self {
            missing_codes: super::sharing::MissingCodes::default(),
            live: LiveHub::default(),
            db,
            config,
            auth_cache: AuthCache::new(),
            enrich,
        }
    }
}
