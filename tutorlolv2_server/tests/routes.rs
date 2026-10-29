mod common;

use actix_web::{App, http::StatusCode, test as http};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use common::*;
use sqlx::PgPool;
use std::net::SocketAddr;
use tutorlolv2_protocol::{DeviceCredentials, RegisterDevice};
use tutorlolv2_server::server::routes::{RateLimits, api};
use uuid::Uuid;

fn limits() -> Option<RateLimits> {
    Some(RateLimits::new(false))
}

fn peer() -> SocketAddr {
    "203.0.113.7:50000".parse().unwrap()
}

async fn register_from(app: &impl TestApp, peer: SocketAddr) -> (StatusCode, actix_web::web::Bytes) {
    let request = http::TestRequest::post()
        .uri("/api/devices")
        .peer_addr(peer)
        .set_payload(encode(RegisterDevice { app_version: "0.3.0".into() }))
        .to_request();
    let response = http::call_service(app, request).await;
    let status = response.status();
    (status, http::read_body(response).await)
}

#[sqlx::test(migrations = "./migrations")]
async fn sixth_registration_within_an_hour_is_429(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    for _ in 0..5 {
        assert_eq!(register_from(&app, peer()).await.0, StatusCode::OK);
    }
    assert_eq!(register_from(&app, peer()).await.0, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(register_from(&app, "198.51.100.1:1".parse().unwrap()).await.0, StatusCode::OK, "limits are per IP");
}

#[sqlx::test(migrations = "./migrations")]
async fn thirty_first_chunk_burst_is_429_with_retry_after(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    let device = format!("Device {}.{}", Uuid::from_u128(1).hyphenated(), URL_SAFE_NO_PAD.encode([1u8; 32]));
    let chunk = |seq: u32| {
        http::TestRequest::post()
            .uri(&format!("/api/recordings/{}/chunks/{seq}", Uuid::from_u128(100 + seq as u128)))
            .peer_addr(peer())
            .insert_header(("Authorization", device.as_str()))
            .set_payload(vec![0u8; 8])
            .to_request()
    };
    for seq in 0..30 {
        let status = http::call_service(&app, chunk(seq)).await.status();
        assert_ne!(status, StatusCode::TOO_MANY_REQUESTS, "request {seq}");
    }
    let response = http::call_service(&app, chunk(30)).await;
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert!(response.headers().contains_key("retry-after"));

}

#[sqlx::test(migrations = "./migrations")]
async fn default_payload_limit_rejects_100_kib_session_body(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    let (status, body) = register_from(&app, peer()).await;
    assert_eq!(status, StatusCode::OK);
    let device: DeviceCredentials = decode(&body);
    let request = http::TestRequest::post()
        .uri("/api/recordings/sessions")
        .peer_addr(peer())
        .insert_header(("Authorization", auth_header(&device)))
        .set_payload(vec![0u8; 100 * 1024])
        .to_request();
    assert_eq!(http::call_service(&app, request).await.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[sqlx::test(migrations = "./migrations")]
async fn games_routes_stay_mounted(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    let request = http::TestRequest::post().uri("/api/games/calculator").peer_addr(peer()).set_payload(vec![255]).to_request();
    assert_eq!(http::call_service(&app, request).await.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "./migrations")]
async fn limits_are_shared_between_workers(pool: PgPool) {
    // actix builds one App per worker: they must share the same budgets
    let shared = RateLimits::new(false);
    let one = http::init_service(App::new().app_data(state(pool.clone())).service(api(Some(shared.clone())))).await;
    let two = http::init_service(App::new().app_data(state(pool)).service(api(Some(shared)))).await;
    for _ in 0..3 {
        assert_eq!(register_from(&one, peer()).await.0, StatusCode::OK);
    }
    for _ in 0..2 {
        assert_eq!(register_from(&two, peer()).await.0, StatusCode::OK);
    }
    assert_eq!(register_from(&two, peer()).await.0, StatusCode::TOO_MANY_REQUESTS);
}

#[sqlx::test(migrations = "./migrations")]
async fn unverified_device_headers_share_the_ip_budget(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    let chunk = |n: u128| {
        let header = format!("Device {}.{}", Uuid::from_u128(n).hyphenated(), URL_SAFE_NO_PAD.encode([n as u8; 32]));
        http::TestRequest::post()
            .uri(&format!("/api/recordings/{}/chunks/0", Uuid::from_u128(1000 + n)))
            .peer_addr(peer())
            .insert_header(("Authorization", header))
            .set_payload(vec![0u8; 8])
            .to_request()
    };
    for n in 0..30 {
        assert_ne!(http::call_service(&app, chunk(n)).await.status(), StatusCode::TOO_MANY_REQUESTS, "request {n}");
    }
    assert_eq!(http::call_service(&app, chunk(30)).await.status(), StatusCode::TOO_MANY_REQUESTS, "a fresh made-up device id gets no fresh budget");
}

#[sqlx::test(migrations = "./migrations")]
async fn verified_devices_keep_their_own_budget(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(api(limits()))).await;
    let (status, body) = register_from(&app, "198.51.100.9:1".parse().unwrap()).await;
    assert_eq!(status, StatusCode::OK);
    let device: DeviceCredentials = decode(&body);
    // one authenticated request makes the credentials known
    let request = http::TestRequest::post()
        .uri("/api/recordings/sessions")
        .peer_addr(peer())
        .insert_header(("Authorization", auth_header(&device)))
        .set_payload(encode(session(probe(1.0, 0.0276, "ahri"))))
        .to_request();
    assert_eq!(http::call_service(&app, request).await.status(), StatusCode::OK);

    let fake = format!("Device {}.{}", Uuid::from_u128(7).hyphenated(), URL_SAFE_NO_PAD.encode([7u8; 32]));
    for n in 0..31 {
        let request = http::TestRequest::post()
            .uri(&format!("/api/recordings/{}/chunks/0", Uuid::from_u128(2000 + n)))
            .peer_addr(peer())
            .insert_header(("Authorization", fake.as_str()))
            .set_payload(vec![0u8; 8])
            .to_request();
        http::call_service(&app, request).await;
    }
    let request = http::TestRequest::post()
        .uri(&format!("/api/recordings/{}/chunks/0", Uuid::from_u128(3000)))
        .peer_addr(peer())
        .insert_header(("Authorization", auth_header(&device)))
        .set_payload(vec![0u8; 8])
        .to_request();
    assert_ne!(http::call_service(&app, request).await.status(), StatusCode::TOO_MANY_REQUESTS, "same IP, own device budget");
}
