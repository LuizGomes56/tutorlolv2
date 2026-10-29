use sqlx::PgPool;
use tutorlolv2_protocol::{CodeMode, EndReason, GameResult};
use tutorlolv2_server::server::{
    config::Config,
    db_enums::{RiotGameSource, ShareDeleteReason},
};

fn only_database_url(key: &str) -> Option<String> {
    (key == "DATABASE_URL").then(|| "postgres://x".into())
}

#[test]
fn config_defaults() {
    let c = Config::from_lookup(only_database_url).unwrap();
    assert_eq!(c.database_url, "postgres://x");
    assert_eq!(
        (c.db_max_connections, c.recording_timeout_secs, c.trusted_proxy),
        (10, 900, false)
    );
    assert_eq!((c.riot_api_key, c.riot_api_base_override), (None, None));
}

#[test]
fn config_reads_every_variable() {
    let c = Config::from_lookup(|key| {
        Some(match key {
            "DATABASE_URL" => "postgres://y".into(),
            "DB_MAX_CONNECTIONS" => "3".into(),
            "RIOT_API_KEY" => "RGAPI-test".into(),
            "RECORDING_TIMEOUT_SECS" => "60".into(),
            "TRUSTED_PROXY" => "1".into(),
            "RIOT_API_BASE_OVERRIDE" => "http://127.0.0.1:1".into(),
            _ => return None,
        })
    })
    .unwrap();
    assert_eq!((c.db_max_connections, c.recording_timeout_secs, c.trusted_proxy), (3, 60, true));
    assert_eq!(c.riot_api_key.as_deref(), Some("RGAPI-test"));
    assert_eq!(c.riot_api_base_override.as_deref(), Some("http://127.0.0.1:1"));
}

#[test]
fn config_requires_database_url() {
    assert!(Config::from_lookup(|_| None).is_err());
}

#[test]
fn config_rejects_bad_numbers() {
    let bad = |key: &str| match key {
        "DATABASE_URL" => Some("postgres://x".into()),
        "RECORDING_TIMEOUT_SECS" => Some("soon".into()),
        _ => None,
    };
    assert!(Config::from_lookup(bad).is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn migration_creates_tables(pool: PgPool) {
    let tables: Vec<String> = ["devices", "recordings", "recording_chunks", "share_codes", "riot_id_cache"].map(String::from).to_vec();
    let count = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM information_schema.tables WHERE table_schema = 'public' AND table_name = ANY($1)"#,
        &tables[..],
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 5);
}

/// Binds `$value` as `$ty` and reads it back through the Postgres type `$cast`.
macro_rules! roundtrip {
    ($pool:expr, $sql:literal, $ty:ty, $value:expr) => {{
        let value: $ty = $value;
        let back = sqlx::query_scalar!($sql, value.clone() as $ty).fetch_one($pool).await.unwrap();
        assert_eq!(back, value);
    }};
}

#[sqlx::test(migrations = "./migrations")]
async fn rust_enums_match_pg_enums(pool: PgPool) {
    for v in [EndReason::GameEndEvent, EndReason::LcuEndOfGame, EndReason::LcuLeftGame, EndReason::Superseded, EndReason::Timeout] {
        roundtrip!(&pool, r#"SELECT $1::end_reason AS "v!: EndReason""#, EndReason, v);
    }
    for v in [GameResult::Win, GameResult::Lose, GameResult::Remake, GameResult::Unknown] {
        roundtrip!(&pool, r#"SELECT $1::game_result AS "v!: GameResult""#, GameResult, v);
    }
    for v in [CodeMode::Live, CodeMode::Public] {
        roundtrip!(&pool, r#"SELECT $1::share_mode AS "v!: CodeMode""#, CodeMode, v);
    }
    for v in [RiotGameSource::Lcu, RiotGameSource::Spectator] {
        roundtrip!(&pool, r#"SELECT $1::riot_game_source AS "v!: RiotGameSource""#, RiotGameSource, v);
    }
    for v in [ShareDeleteReason::GameEnded, ShareDeleteReason::Revoked] {
        roundtrip!(&pool, r#"SELECT $1::share_delete_reason AS "v!: ShareDeleteReason""#, ShareDeleteReason, v);
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn open_enums_are_stored_as_text(pool: PgPool) {
    use tutorlolv2_protocol::{GameMode, Platform};
    roundtrip!(&pool, r#"SELECT $1::text AS "v!: Platform""#, Platform, Platform::Na1);
    roundtrip!(&pool, r#"SELECT $1::text AS "v!: GameMode""#, GameMode, GameMode::from("NEWMODE"));
}
