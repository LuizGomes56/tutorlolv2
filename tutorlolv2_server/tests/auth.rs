mod common;

use actix_web::{App, get, http::StatusCode, test as http};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use common::*;
use sqlx::PgPool;
use tutorlolv2_protocol::RegisterDevice;
use tutorlolv2_server::server::{auth::*, routes::devices};
use uuid::Uuid;

#[get("/whoami")]
async fn whoami(auth: DeviceAuth) -> String {
    auth.device_id.to_string()
}

#[test]
fn parse_device_header_roundtrip_and_rejects_garbage() {
    let id = Uuid::from_u128(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f);
    let secret = [7u8; 32];
    let header = format!("Device {}.{}", id.hyphenated(), URL_SAFE_NO_PAD.encode(secret));
    assert_eq!(parse_device_header(&header), Some((id, secret)));

    let short = format!("Device {}.{}", id.hyphenated(), URL_SAFE_NO_PAD.encode([7u8; 31]));
    for bad in ["Bearer x", "Device nodot", "Device not-a-uuid.AAAA", short.as_str(), ""] {
        assert_eq!(parse_device_header(bad), None, "{bad}");
    }
}

async fn whoami_status(app: &impl TestApp, header: &str) -> StatusCode {
    let request = http::TestRequest::get().uri("/whoami").insert_header(("Authorization", header)).to_request();
    http::call_service(app, request).await.status()
}

#[sqlx::test(migrations = "./migrations")]
async fn register_then_authenticate(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool.clone())).service(devices(None)).service(whoami)).await;
    let credentials = register(&app).await;

    let request = http::TestRequest::get().uri("/whoami").insert_header(("Authorization", auth_header(&credentials))).to_request();
    let response = http::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = http::read_body(response).await;
    assert_eq!(body, Uuid::from_bytes(credentials.device_id).to_string().as_bytes());

    let stored = sqlx::query_scalar!("SELECT secret_hash FROM devices WHERE id = $1", Uuid::from_bytes(credentials.device_id))
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_ne!(stored, credentials.device_secret, "only the hash is stored");
    assert_eq!(stored.len(), 32);
}

#[sqlx::test(migrations = "./migrations")]
async fn wrong_secret_and_soft_deleted_device_are_401(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool.clone())).service(devices(None)).service(whoami)).await;

    let mut wrong = register(&app).await;
    wrong.device_secret[0] ^= 1;
    assert_eq!(whoami_status(&app, &auth_header(&wrong)).await, StatusCode::UNAUTHORIZED);

    let deleted = register(&app).await;
    sqlx::query!("UPDATE devices SET deleted_at = now() WHERE id = $1", Uuid::from_bytes(deleted.device_id))
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(whoami_status(&app, &auth_header(&deleted)).await, StatusCode::UNAUTHORIZED);

    assert_eq!(whoami_status(&app, "Bearer nope").await, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "./migrations")]
async fn register_rejects_trailing_bytes(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool)).service(devices(None))).await;
    let mut body = encode(RegisterDevice { app_version: "0.3.0".into() });
    body.push(0);
    let response = http::call_service(&app, http::TestRequest::post().uri("/devices").set_payload(body).to_request()).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
