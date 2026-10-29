mod common;

use actix_web::{App, test as http};
use common::*;
use serde_json::json;
use sqlx::PgPool;
use std::time::Duration;
use tutorlolv2_protocol::{LcuGame, Platform, Region, Regional};
use tutorlolv2_server::server::{
    db_enums::RiotGameSource,
    routes::{devices, recordings},
    riot_api::*,
};
use uuid::Uuid;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

const KEY: &str = "RGAPI-test";
const USER_AGENT: &str = concat!("tutorlolv2-server/", env!("CARGO_PKG_VERSION"));
const ACCOUNT: &str = "/riot/account/v1/accounts/by-riot-id/Me/BR1";
const SPECTATOR: &str = "/lol/spectator/v5/active-games/by-summoner/P";

/// The game Spectator-V5 reports, started `started_secs_ago` (the test recordings are at game time 120 s).
fn active_game_started(game_id: u64, started_secs_ago: u64) -> serde_json::Value {
    json!({
        "gameId": game_id, "platformId": "NA1", "gameType": "MATCHED", "gameMode": "KIWI",
        "gameQueueConfigId": 2400, "mapId": 12, "gameStartTime": now_ms() - started_secs_ago * 1000, "gameLength": 120,
        "participants": [{"puuid": "someone-else", "riotId": "Foe#1", "championId": 1}],
        "observers": {"encryptionKey": "secret"}
    })
}

fn active_game() -> serde_json::Value {
    active_game_started(LIVE_GAME, 125)
}

async fn mock_account(server: &MockServer, expected_calls: u64) {
    Mock::given(method("GET"))
        .and(path(ACCOUNT))
        .and(header("X-Riot-Token", KEY))
        .and(header("User-Agent", USER_AGENT))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"puuid": "P", "gameName": "Me", "tagLine": "BR1"})))
        .expect(expected_calls)
        .mount(server)
        .await;
}

/// A recording of `ME` started with LCU values (not yet verified).
async fn lcu_recording(pool: &PgPool) -> Uuid {
    recording_with(pool, Some(LIVE_GAME)).await
}

/// `lcu_game_id: None` = the League client wasn't available.
async fn recording_with(pool: &PgPool, lcu_game_id: Option<u64>) -> Uuid {
    let app = http::init_service(App::new().app_data(state(pool.clone())).service(devices(None)).service(recordings(None))).await;
    let device = register(&app).await;
    let mut request = session(probe(120.0, 0.0276, "ahri"));
    request.lcu = lcu_game_id.map(|game_id| LcuGame { game_id, queue_id: Some(450), is_custom: Some(false) });
    Uuid::from_bytes(start(&app, &device, request).await.recording_id)
}

const LIVE_GAME: u64 = 5669550383;

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
}

fn job(recording_id: Uuid) -> EnrichJob {
    EnrichJob { recording_id, riot_id: ME.into(), platform: Platform::Na1 }
}

#[test]
fn urls_use_enum_hosts() {
    let riot = RiotClient::new(KEY.into(), None);
    assert_eq!(
        riot.spectator_url(&Platform::Na1, "P").as_deref(),
        Some("https://na1.api.riotgames.com/lol/spectator/v5/active-games/by-summoner/P")
    );
    assert_eq!(riot.spectator_url(&Platform::from("XX1"), "P"), None);
    assert!(riot.account_url(Regional::Americas, ME).unwrap().starts_with("https://americas.api.riotgames.com/"));
}

#[test]
fn account_url_percent_encodes_name_and_tag() {
    let riot = RiotClient::new(KEY.into(), None);
    assert!(riot.account_url(Regional::Americas, "Giga Gnar#BRZL").unwrap().ends_with("/riot/account/v1/accounts/by-riot-id/Giga%20Gnar/BRZL"));
    assert!(
        riot.account_url(Regional::Americas, "深海à#002")
            .unwrap()
            .ends_with("/by-riot-id/%E6%B7%B1%E6%B5%B7%C3%A0/002")
    );
    assert!(riot.account_url(Regional::Americas, "a#b#c").unwrap().ends_with("/by-riot-id/a%23b/c"), "splits at the last '#'");
    assert_eq!(riot.account_url(Regional::Americas, "noTag"), None);
}

#[sqlx::test(migrations = "./migrations")]
async fn spectator_200_marks_recording_verified(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game())).mount(&server).await;
    let id = lcu_recording(&pool).await;

    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Spectator);

    let row = sqlx::query!(
        r#"SELECT riot_game_id, platform_id AS "platform_id: Platform", riot_game_source AS "source: RiotGameSource",
                  queue_id, is_custom, active_puuid FROM recordings WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.riot_game_id, Some(5669550383));
    assert_eq!(row.platform_id, Some(Platform::Na1));
    assert_eq!(row.source, Some(RiotGameSource::Spectator));
    assert_eq!((row.queue_id, row.is_custom, row.active_puuid.as_deref()), (Some(2400), Some(false), Some("P")));
}

#[sqlx::test(migrations = "./migrations")]
async fn spectator_404_keeps_lcu_values(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET"))
        .and(path(SPECTATOR))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"httpStatus": 404, "errorCode": "NOT_FOUND", "implementationDetails": "filtered"})))
        .mount(&server)
        .await;
    let id = lcu_recording(&pool).await;

    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::NotSpectatable);
    let row = sqlx::query!(
        r#"SELECT riot_game_id, riot_game_source AS "source: RiotGameSource", active_puuid FROM recordings WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((row.riot_game_id, row.source, row.active_puuid.as_deref()), (Some(LIVE_GAME as i64), Some(RiotGameSource::Lcu), Some("P")));
}

#[sqlx::test(migrations = "./migrations")]
async fn service_429_is_retried_after_retry_after(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET"))
        .and(path(SPECTATOR))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "1").insert_header("X-Rate-Limit-Type", "server"))
        .up_to_n_times(1)
        .with_priority(1)
        .mount(&server)
        .await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game())).mount(&server).await;
    let id = lcu_recording(&pool).await;

    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    let started = std::time::Instant::now();
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Spectator);
    assert!(started.elapsed() >= Duration::from_millis(900), "waited for Retry-After");
}

#[sqlx::test(migrations = "./migrations")]
async fn persistent_429_gives_up(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET"))
        .and(path(SPECTATOR))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
        .expect(5)
        .mount(&server)
        .await;
    let id = lcu_recording(&pool).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::GaveUp);
}

#[sqlx::test(migrations = "./migrations")]
async fn every_request_sends_user_agent_and_key(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET"))
        .and(path(SPECTATOR))
        .and(header("X-Riot-Token", KEY))
        .and(header("User-Agent", USER_AGENT))
        .respond_with(ResponseTemplate::new(200).set_body_json(active_game()))
        .expect(1)
        .mount(&server)
        .await;
    let id = lcu_recording(&pool).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Spectator);
}

#[sqlx::test(migrations = "./migrations")]
async fn puuid_is_cached_for_24h(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(404)).expect(2).mount(&server).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    let first = lcu_recording(&pool).await;
    let second = lcu_recording(&pool).await;
    assert_eq!(enrich_one(&pool, &riot, &job(first)).await, EnrichOutcome::NotSpectatable);
    assert_eq!(enrich_one(&pool, &riot, &job(second)).await, EnrichOutcome::NotSpectatable);
}

#[sqlx::test(migrations = "./migrations")]
async fn new_session_enqueues_enrichment(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game())).mount(&server).await;

    let mut config = config();
    config.riot_api_key = Some(KEY.into());
    config.riot_api_base_override = Some(server.uri());
    let app = http::init_service(App::new().app_data(state_with(pool.clone(), config)).service(devices(None)).service(recordings(None))).await;
    let device = register(&app).await;
    let mut request = session(probe(120.0, 0.0276, "ahri"));
    request.region = Some(Region::Na);
    let id = Uuid::from_bytes(start(&app, &device, request).await.recording_id);

    for _ in 0..50 {
        let source = sqlx::query_scalar!(r#"SELECT riot_game_source AS "s: RiotGameSource" FROM recordings WHERE id = $1"#, id)
            .fetch_one(&pool)
            .await
            .unwrap();
        if source == Some(RiotGameSource::Spectator) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("the recording was never enriched");
}

async fn source_and_game(pool: &PgPool, id: Uuid) -> (Option<RiotGameSource>, Option<i64>) {
    let row = sqlx::query!(r#"SELECT riot_game_source AS "source: RiotGameSource", riot_game_id FROM recordings WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
        .unwrap();
    (row.source, row.riot_game_id)
}

#[sqlx::test(migrations = "./migrations")]
async fn spectator_game_differing_from_the_lcu_game_is_not_trusted(pool: PgPool) {
    // e.g. a session sent late after an outage while the player is already in the next game
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game_started(999, 125))).mount(&server).await;
    let id = recording_with(&pool, Some(LIVE_GAME)).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Mismatch);
    assert_eq!(source_and_game(&pool, id).await, (Some(RiotGameSource::Lcu), Some(LIVE_GAME as i64)));
}

#[sqlx::test(migrations = "./migrations")]
async fn spectator_game_that_started_long_before_is_not_trusted(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game_started(LIVE_GAME, 3600))).mount(&server).await;
    let id = recording_with(&pool, None).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Mismatch);
    assert_eq!(source_and_game(&pool, id).await, (None, None));
}

#[sqlx::test(migrations = "./migrations")]
async fn spectator_game_is_trusted_by_start_time_without_lcu(pool: PgPool) {
    let server = MockServer::start().await;
    mock_account(&server, 1).await;
    Mock::given(method("GET")).and(path(SPECTATOR)).respond_with(ResponseTemplate::new(200).set_body_json(active_game_started(LIVE_GAME, 125))).mount(&server).await;
    let id = recording_with(&pool, None).await;
    let riot = RiotClient::new(KEY.into(), Some(server.uri()));
    assert_eq!(enrich_one(&pool, &riot, &job(id)).await, EnrichOutcome::Spectator);
    assert_eq!(source_and_game(&pool, id).await, (Some(RiotGameSource::Spectator), Some(LIVE_GAME as i64)));
}
