use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
    #[cfg(feature = "dev")]
    pub client: reqwest::Client,
}
