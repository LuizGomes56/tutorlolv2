mod common;
use actix_web::{App, http::Method, test};
use common::*;
use sqlx::PgPool;
use tutorlolv2_protocol::*;
use tutorlolv2_server::server::routes::{RateLimits, api};

#[sqlx::test(migrations = "./migrations")]
async fn framework_errors_are_bincode_with_real_status_and_policy(pool: PgPool) {
    let app = test::init_service(App::new().app_data(state(pool)).service(api(None))).await;
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/devices")
            .set_payload(encode(RegisterDevice {
                app_version: "test".into(),
            }))
            .to_request(),
    )
    .await;
    let device = ApiResponse::<DeviceCredentials>::decode(&test::read_body(response).await, 200)
        .unwrap()
        .result
        .unwrap();
    for (method, path, payload, want) in [
        (Method::GET, "/api/missing", vec![], 404),
        (Method::DELETE, "/api/games/calculator", vec![], 405),
        (Method::POST, "/api/games/calculator", vec![255], 400),
        (Method::POST, "/api/games/calculator", vec![0; 65537], 413),
        (Method::GET, "/api/me/recordings?limit=no", vec![], 400),
        (
            Method::GET,
            "/api/me/recordings/not-a-uuid/frames/1",
            vec![],
            400,
        ),
    ] {
        let response = test::call_service(
            &app,
            test::TestRequest::default()
                .method(method)
                .uri(path)
                .insert_header(("Authorization", auth_header(&device)))
                .set_payload(payload)
                .to_request(),
        )
        .await;
        assert_eq!(response.status().as_u16(), want, "{path}");
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/octet-stream"
        );
        if want == 405 {
            assert!(
                response
                    .headers()
                    .get("Allow")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("POST")
            );
        }
        if path.starts_with("/api/me/") {
            assert_eq!(response.headers().get("Cache-Control").unwrap(), "no-store");
        }
        let error = ApiResponse::<()>::decode(&test::read_body(response).await, want)
            .unwrap()
            .result
            .unwrap_err();
        assert!(!error.traces.is_empty(), "{path}");
    }
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/account")
            .insert_header(("Origin", "https://untrusted.invalid"))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), 403);
    let error = ApiResponse::<()>::decode(&test::read_body(response).await, 403)
        .unwrap()
        .result
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::Forbidden);
    let response = test::call_service(
        &app,
        test::TestRequest::default()
            .method(Method::HEAD)
            .uri("/api/account")
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), 401);
    assert!(test::read_body(response).await.is_empty());
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/auth/google/callback?code=missing-state")
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), 400);
    assert!(
        response
            .headers()
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn governor_errors_keep_binary_body_and_retry_after(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(state(pool))
            .service(api(Some(RateLimits::new(false)))),
    )
    .await;
    for i in 0..6 {
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/devices")
                .peer_addr("127.0.0.1:12345".parse().unwrap())
                .set_payload(encode(RegisterDevice {
                    app_version: "test".into(),
                }))
                .to_request(),
        )
        .await;
        if i == 5 {
            assert_eq!(response.status(), 429);
            assert!(response.headers().get("Retry-After").is_some());
            let error = ApiResponse::<()>::decode(&test::read_body(response).await, 429)
                .unwrap()
                .result
                .unwrap_err();
            assert_eq!(error.code, ErrorCode::RateLimited);
            assert!(!error.traces.is_empty());
        } else {
            assert_eq!(response.status(), 200);
        }
    }
}
