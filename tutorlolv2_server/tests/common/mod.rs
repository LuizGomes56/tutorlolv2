//! Helpers shared by the server's integration tests.
#![allow(dead_code)]

use actix_web::web::Data;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sqlx::PgPool;
use tutorlolv2_protocol::DeviceCredentials;
use tutorlolv2_server::server::{config::Config, state::AppState};
use uuid::Uuid;

pub fn config() -> Config {
    Config::from_lookup(|key| (key == "DATABASE_URL").then(|| "postgres://unused-in-tests".into())).unwrap()
}

pub fn state(pool: PgPool) -> Data<AppState> {
    Data::new(AppState::new(pool, config()))
}

pub fn encode<T: bincode::Encode>(value: T) -> Vec<u8> {
    bincode::encode_to_vec(value, tutorlolv2_types::BINCODE_CONFIG).unwrap()
}

pub fn decode<T: bincode::Decode<()>>(bytes: &[u8]) -> T {
    let (value, used) = bincode::decode_from_slice(bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    value
}

pub fn auth_header(credentials: &DeviceCredentials) -> String {
    format!(
        "Device {}.{}",
        Uuid::from_bytes(credentials.device_id).hyphenated(),
        URL_SAFE_NO_PAD.encode(credentials.device_secret)
    )
}

use actix_web::{
    dev::{Service, ServiceResponse},
    http::StatusCode,
    test as http,
};
use tutorlolv2_protocol::{EventKey, GameMode, Probe, RegisterDevice, RosterEntry, SessionStarted, StartSession, Team, fnv1a32};

pub trait TestApp: Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error> {}
impl<T: Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error>> TestApp for T {}

pub const ME: &str = "Me#BR1";

pub async fn register(app: &impl TestApp) -> DeviceCredentials {
    let response = http::call_service(
        app,
        http::TestRequest::post()
            .uri("/devices")
            .set_payload(encode(RegisterDevice { app_version: "0.3.0".into() }))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    decode(&http::read_body(response).await)
}

/// POSTs `body` to `uri` as the device and returns the status and body.
pub async fn post_as(app: &impl TestApp, device: &DeviceCredentials, uri: &str, body: Vec<u8>) -> (StatusCode, actix_web::web::Bytes) {
    let request = http::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", auth_header(device)))
        .set_payload(body)
        .to_request();
    let response = http::call_service(app, request).await;
    let status = response.status();
    (status, http::read_body(response).await)
}

pub async fn start(app: &impl TestApp, device: &DeviceCredentials, request: StartSession) -> SessionStarted {
    let (status, body) = post_as(app, device, "/recordings/sessions", encode(request)).await;
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
    decode(&body)
}

pub fn session(probe: Probe) -> StartSession {
    StartSession { probe, region: None, lcu: None, app_version: "0.3.0".into() }
}

/// A 2-player probe at game time `t`; `start_time` is the `GameStart` time, which differs between matches.
pub fn probe(t: f32, start_time: f32, champion: &str) -> Probe {
    let mut roster = vec![
        RosterEntry { team: Team::Order, riot_id: ME.into(), champion: champion.into(), skin_id: 0, keystone: None, is_bot: false },
        RosterEntry { team: Team::Chaos, riot_id: "Foe#1".into(), champion: "zed".into(), skin_id: 0, keystone: None, is_bot: false },
    ];
    roster.sort();
    Probe {
        active_riot_id: ME.into(),
        game_mode: GameMode::Classic,
        map_number: 11,
        roster,
        game_time: t,
        events: vec![EventKey { id: 0, name_hash: fnv1a32(b"GameStart"), time_bits: start_time.to_bits() }],
        riot_game: None,
    }
}

/// PATCHes `body` to `uri` as the device and returns the status.
pub async fn patch_as(app: &impl TestApp, device: &DeviceCredentials, uri: &str, body: Vec<u8>) -> StatusCode {
    let request = http::TestRequest::patch()
        .uri(uri)
        .insert_header(("Authorization", auth_header(device)))
        .set_payload(body)
        .to_request();
    http::call_service(app, request).await.status()
}

pub fn state_with(pool: PgPool, config: Config) -> Data<AppState> {
    Data::new(AppState::new(pool, config))
}
