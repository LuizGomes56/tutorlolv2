use sqlx::PgPool;
use tutorlolv2_protocol::{CodeMode, EndReason, GameResult};
use tutorlolv2_server::server::{
    config::Config,
    db_enums::{RiotGameSource, SessionKind, ShareDeleteReason},
};

fn only_database_url(key: &str) -> Option<String> {
    (key == "DATABASE_URL").then(|| "postgres://x".into())
}

#[test]
fn config_defaults() {
    let c = Config::from_lookup(only_database_url).unwrap();
    assert_eq!(c.database_url, "postgres://x");
    assert_eq!(
        (
            c.db_max_connections,
            c.recording_timeout_secs,
            c.trusted_proxy
        ),
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
    assert_eq!(
        (
            c.db_max_connections,
            c.recording_timeout_secs,
            c.trusted_proxy
        ),
        (3, 60, true)
    );
    assert_eq!(c.riot_api_key.as_deref(), Some("RGAPI-test"));
    assert_eq!(
        c.riot_api_base_override.as_deref(),
        Some("http://127.0.0.1:1")
    );
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
    let tables: Vec<String> = [
        "devices",
        "recordings",
        "recording_chunks",
        "share_codes",
        "riot_id_cache",
    ]
    .map(String::from)
    .to_vec();
    let count = sqlx::query_scalar!(
        r#"
        SELECT count(*) AS "count!"
        FROM information_schema.tables
        WHERE table_schema = 'public'
            AND TABLE_NAME = ANY($1)
        "#,
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
        let back = sqlx::query_scalar!($sql, value.clone() as $ty)
            .fetch_one($pool)
            .await
            .unwrap();
        assert_eq!(back, value);
    }};
}

#[sqlx::test(migrations = "./migrations")]
async fn rust_enums_match_pg_enums(pool: PgPool) {
    for v in [
        EndReason::GameEndEvent,
        EndReason::LcuEndOfGame,
        EndReason::LcuLeftGame,
        EndReason::Superseded,
        EndReason::Timeout,
    ] {
        roundtrip!(
            &pool,
            r#"SELECT $1::end_reason AS "v!: EndReason""#,
            EndReason,
            v
        );
    }
    for v in [
        GameResult::Win,
        GameResult::Lose,
        GameResult::Remake,
        GameResult::Unknown,
    ] {
        roundtrip!(
            &pool,
            r#"SELECT $1::game_result AS "v!: GameResult""#,
            GameResult,
            v
        );
    }
    for v in [CodeMode::Live, CodeMode::Public] {
        roundtrip!(
            &pool,
            r#"SELECT $1::share_mode AS "v!: CodeMode""#,
            CodeMode,
            v
        );
    }
    for v in [RiotGameSource::Lcu, RiotGameSource::Spectator] {
        roundtrip!(
            &pool,
            r#"SELECT $1::riot_game_source AS "v!: RiotGameSource""#,
            RiotGameSource,
            v
        );
    }
    for v in [SessionKind::Web, SessionKind::Desktop] {
        roundtrip!(
            &pool,
            r#"SELECT $1::session_kind AS "v!: SessionKind""#,
            SessionKind,
            v
        );
    }
    for v in [ShareDeleteReason::GameEnded, ShareDeleteReason::Revoked] {
        roundtrip!(
            &pool,
            r#"SELECT $1::share_delete_reason AS "v!: ShareDeleteReason""#,
            ShareDeleteReason,
            v
        );
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn open_enums_are_stored_as_text(pool: PgPool) {
    use tutorlolv2_protocol::{GameMode, Platform};
    roundtrip!(
        &pool,
        r#"SELECT $1::text AS "v!: Platform""#,
        Platform,
        Platform::Na1
    );
    roundtrip!(
        &pool,
        r#"SELECT $1::text AS "v!: GameMode""#,
        GameMode,
        GameMode::from("NEWMODE")
    );
}

#[test]
fn account_config_defaults_bounds_and_secret_redaction() {
    let c = Config::from_lookup(only_database_url).unwrap();
    assert_eq!(c.api_public_base_url, "http://127.0.0.1:8082");
    assert_eq!(c.session_ttl_days, 90);
    assert!(c.cookie_secure);
    assert!(c.google_client_id.is_none());
    assert!(c.google_client_secret.is_none());
    for days in ["0", "-1", "366", "invalid"] {
        assert!(
            Config::from_lookup(|key| if key == "SESSION_TTL_DAYS" {
                Some(days.into())
            } else {
                only_database_url(key)
            })
            .is_err()
        );
    }
    let c = Config::from_lookup(|key| match key {
        "GOOGLE_CLIENT_ID" => Some("example-client-id".into()),
        "GOOGLE_CLIENT_SECRET" => Some("secret-should-stay-hidden".into()),
        "GOOGLE_TOKEN_ENDPOINT_TRUSTED_OVERRIDE" => Some("http://127.0.0.1:8096/token".into()),
        "COOKIE_SECURE" => Some("false".into()),
        "SESSION_TTL_DAYS" => Some("7".into()),
        "API_PUBLIC_BASE_URL" => Some("https://api.example.test/".into()),
        _ => only_database_url(key),
    })
    .unwrap();
    assert!(!c.cookie_secure);
    assert_eq!(c.session_ttl_days, 7);
    assert_eq!(c.api_public_base_url, "https://api.example.test");
    let debug = format!("{c:?}");
    assert!(!debug.contains("secret-should-stay-hidden"));
    assert!(!debug.contains("postgres://x"));
}

#[sqlx::test(migrations = "./migrations")]
async fn database_enum_labels_use_rust_variant_names(pool: PgPool) {
    for (name, expected) in [
        (
            "end_reason",
            vec![
                "GameEndEvent",
                "LcuEndOfGame",
                "LcuLeftGame",
                "Superseded",
                "Timeout",
            ],
        ),
        ("game_result", vec!["Win", "Lose", "Remake", "Unknown"]),
        ("riot_game_source", vec!["Lcu", "Spectator"]),
        ("share_mode", vec!["Live", "Public"]),
        ("share_delete_reason", vec!["GameEnded", "Revoked"]),
        ("session_kind", vec!["Web", "Desktop"]),
    ] {
        let labels = sqlx::query_scalar!(
            r#"
            SELECT e.enumlabel::text AS "label!"
            FROM pg_enum e
            JOIN pg_type t ON t.oid = e.enumtypid
            WHERE t.typname = $1
            ORDER BY e.enumsortorder
            "#,
            name,
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(labels, expected, "{name}");
    }
}
