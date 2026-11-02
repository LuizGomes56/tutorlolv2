mod common;
use actix_web::{App, test};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use common::*;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tutorlolv2_protocol::*;
use tutorlolv2_server::server::routes;
use uuid::Uuid;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

fn oauth_config(endpoint: &str) -> tutorlolv2_server::server::config::Config {
    let mut c = config();
    c.google_client_id = Some("test-client".into());
    c.google_client_secret = Some("test-secret".into());
    c.google_token_endpoint = Some(endpoint.into());
    c.cookie_secure = false;
    c
}

async fn begin(
    app: &impl TestApp,
    target: &str,
) -> (String, String, Option<actix_web::cookie::Cookie<'static>>) {
    let r = test::call_service(
        app,
        test::TestRequest::get()
            .uri(&format!("/api/auth/google/start?{target}"))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 302);
    assert_eq!(r.headers().get("Cache-Control").unwrap(), "no-store");
    let url = reqwest::Url::parse(r.headers().get("Location").unwrap().to_str().unwrap()).unwrap();
    assert_eq!(url.host_str(), Some("accounts.google.com"));
    let q: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(q["code_challenge_method"], "S256");
    assert_eq!(B64.decode(&q["code_challenge"]).unwrap().len(), 32);
    let cookie = r
        .response()
        .cookies()
        .find(|c| c.name() == "tl_login")
        .map(|c| c.into_owned());
    (q["state"].clone(), q["nonce"].clone(), cookie)
}
async fn identity(server: &MockServer, nonce: &str, sub: &str) {
    let claims = serde_json::json!({"iss":"https://accounts.google.com","aud":"test-client","exp":chrono::Utc::now().timestamp()+3600,"nonce":nonce,"sub":sub,"email":format!("{sub}@example.test")});
    token_response(server, claims).await;
}
async fn token_response(server: &MockServer, claims: serde_json::Value) {
    let jwt = format!(
        "e30.{}.ignored",
        B64.encode(serde_json::to_vec(&claims).unwrap())
    );
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("client_secret=test-secret"))
        .and(body_string_contains("code_verifier="))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id_token":jwt})))
        .mount(server)
        .await;
}
async fn web_login(
    app: &impl TestApp,
    server: &MockServer,
    sub: &str,
) -> actix_web::cookie::Cookie<'static> {
    server.reset().await;
    let (state, nonce, cookie) = begin(app, "target=web&return_to=/history").await;
    identity(server, &nonce, sub).await;
    let r = test::call_service(
        app,
        test::TestRequest::get()
            .uri(&format!(
                "/api/auth/google/callback?state={state}&code=valid"
            ))
            .cookie(cookie.unwrap())
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 302);
    let session = r
        .response()
        .cookies()
        .find(|c| c.name() == "tl_session")
        .unwrap()
        .into_owned();
    assert_eq!(session.http_only(), Some(true));
    assert_eq!(session.same_site(), Some(actix_web::cookie::SameSite::Lax));
    assert_eq!(session.path(), Some("/api"));
    session
}
async fn desktop_login(
    app: &impl TestApp,
    server: &MockServer,
    device: &DeviceCredentials,
    sub: &str,
) -> DesktopSession {
    server.reset().await;
    let (state, nonce, _) = begin(
        app,
        &format!(
            "target=desktop&port=18181&app_state={}",
            B64.encode([9; 32])
        ),
    )
    .await;
    identity(server, &nonce, sub).await;
    let r = test::call_service(
        app,
        test::TestRequest::get()
            .uri(&format!(
                "/api/auth/google/callback?state={state}&code=valid"
            ))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 302);
    let url = reqwest::Url::parse(r.headers().get("Location").unwrap().to_str().unwrap()).unwrap();
    assert_eq!(url.host_str(), Some("127.0.0.1"));
    let ticket = url
        .query_pairs()
        .find(|(k, _)| k == "ticket")
        .unwrap()
        .1
        .into_owned();
    let request = || {
        test::TestRequest::post()
            .uri("/api/auth/desktop/redeem")
            .insert_header(("Authorization", auth_header(device)))
            .set_payload(encode(RedeemTicket {
                ticket: ticket.clone(),
            }))
            .to_request()
    };
    let r = test::call_service(app, request()).await;
    assert_eq!(r.status(), 200);
    let session = decode(&test::read_body(r).await);
    assert_eq!(test::call_service(app, request()).await.status(), 410);
    session
}
async fn device(app: &impl TestApp) -> DeviceCredentials {
    let r = test::call_service(
        app,
        test::TestRequest::post()
            .uri("/api/devices")
            .set_payload(encode(RegisterDevice {
                app_version: "test".into(),
            }))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    decode(&test::read_body(r).await)
}
async fn recording(app: &impl TestApp, d: &DeviceCredentials, start_time: f32) -> Uuid {
    let (s, b) = post_as(
        app,
        d,
        "/api/recordings/sessions",
        encode(session(probe(10.0, start_time, "ashe"))),
    )
    .await;
    assert_eq!(s, 200, "{}", String::from_utf8_lossy(&b));
    Uuid::from_bytes(decode::<SessionStarted>(&b).recording_id)
}

#[sqlx::test(migrations = "./migrations")]
async fn unavailable_google_keeps_anonymous_routes(pool: PgPool) {
    let app = test::init_service(App::new().app_data(state(pool)).service(routes::api(None))).await;
    for path in [
        "/api/auth/google/start?target=web",
        "/api/auth/google/callback?state=x&code=y",
    ] {
        assert_eq!(
            test::call_service(&app, test::TestRequest::get().uri(path).to_request())
                .await
                .status(),
            503
        );
    }
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get().uri("/api/account").to_request()
        )
        .await
        .status(),
        401
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn cors_and_private_errors_use_real_route_policy(pool: PgPool) {
    let app = test::init_service(App::new().app_data(state(pool)).service(routes::api(None))).await;
    for path in ["/api/account", "/api/me/recordings"] {
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(path)
                .insert_header(("Origin", "http://127.0.0.1:8000"))
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 401);
        assert_eq!(
            r.headers().get("Access-Control-Allow-Origin").unwrap(),
            "http://127.0.0.1:8000"
        );
        assert_eq!(
            r.headers().get("Access-Control-Allow-Credentials").unwrap(),
            "true"
        );
        assert_eq!(r.headers().get("Cache-Control").unwrap(), "no-store");
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(path)
                .insert_header(("Origin", "https://evil.test"))
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 403);
        assert!(!r.headers().contains_key("Access-Control-Allow-Origin"));
    }
    let r = test::call_service(
        &app,
        test::TestRequest::default()
            .method(actix_web::http::Method::OPTIONS)
            .uri("/api/auth/logout")
            .insert_header(("Origin", "http://127.0.0.1:8000"))
            .insert_header(("Access-Control-Request-Method", "POST"))
            .insert_header(("Access-Control-Request-Headers", "content-type"))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 204);
    assert_eq!(
        r.headers().get("Access-Control-Allow-Credentials").unwrap(),
        "true"
    );
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/share/UNKNOWN")
            .insert_header(("Origin", "https://any.test"))
            .to_request(),
    )
    .await;
    assert_eq!(r.headers().get("Access-Control-Allow-Origin").unwrap(), "*");
    assert!(!r.headers().contains_key("Access-Control-Allow-Credentials"));
}

#[sqlx::test(migrations = "./migrations")]
async fn web_session_cookie_kind_csrf_and_logout(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    let cookie = web_login(&app, &server, "alice").await;
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/account")
            .cookie(cookie.clone())
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    assert_eq!(
        decode::<AccountInfo>(&test::read_body(r).await).email,
        "alice@example.test"
    );
    let hash: Vec<u8> = sqlx::query_scalar("SELECT token_hash FROM account_sessions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        hash,
        Sha256::digest(B64.decode(cookie.value()).unwrap()).as_slice()
    );
    // A web credential may not cross into the native header session kind.
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/account")
                .insert_header(("Authorization", format!("Session {}", cookie.value())))
                .to_request()
        )
        .await
        .status(),
        401
    );
    for (origin, ct) in [
        (None, None),
        (Some("https://evil.test"), Some("application/octet-stream")),
        (Some("http://127.0.0.1:8000"), Some("text/plain")),
    ] {
        let mut req = test::TestRequest::post()
            .uri("/api/auth/logout")
            .cookie(cookie.clone());
        if let Some(o) = origin {
            req = req.insert_header(("Origin", o));
        }
        if let Some(c) = ct {
            req = req.insert_header(("Content-Type", c));
        }
        assert_eq!(
            test::call_service(&app, req.to_request()).await.status(),
            403
        );
    }
    let r = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/auth/logout")
            .cookie(cookie.clone())
            .insert_header(("Origin", "http://127.0.0.1:8000"))
            .insert_header(("Content-Type", "application/octet-stream"))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 204);
    let deleted = r
        .response()
        .cookies()
        .find(|c| c.name() == "tl_session")
        .unwrap();
    assert_eq!(deleted.path(), Some("/api"));
    assert_eq!(deleted.max_age().unwrap().whole_seconds(), 0);
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/account")
                .cookie(cookie)
                .to_request()
        )
        .await
        .status(),
        401
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn desktop_claims_only_anonymous_history_and_keeps_ingest_device_only(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    let d = device(&app).await;
    let first = recording(&app, &d, 1.0).await;
    let alice = desktop_login(&app, &server, &d, "alice").await;
    let alice_id: Uuid = sqlx::query_scalar("SELECT id FROM accounts WHERE google_sub='alice'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let owner: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(first)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(owner, Some(alice_id));
    let second = recording(&app, &d, 2.0).await;
    let owner: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(second)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(owner, Some(alice_id));
    let header = format!("Session {}", alice.token);
    // Session auth never authorizes an upload/live control endpoint.
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/recordings/sessions")
                .insert_header(("Authorization", header.clone()))
                .set_payload(encode(session(probe(10.0, 3.0, "ashe"))))
                .to_request()
        )
        .await
        .status(),
        401
    );
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/recordings")
            .insert_header(("Authorization", header.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    assert_eq!(
        decode::<HistoryPage>(&test::read_body(r).await).items.len(),
        2
    );
    // Session header cannot be presented as a web cookie.
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/account")
                .cookie(actix_web::cookie::Cookie::new(
                    "tl_session",
                    alice.token.clone()
                ))
                .to_request()
        )
        .await
        .status(),
        401
    );
    // New login changes future claims only; old logout cannot undo the new binding.
    let bob = desktop_login(&app, &server, &d, "bob").await;
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/logout")
                .insert_header(("Authorization", header))
                .to_request()
        )
        .await
        .status(),
        204
    );
    let third = recording(&app, &d, 3.0).await;
    let bob_id: Uuid = sqlx::query_scalar("SELECT id FROM accounts WHERE google_sub='bob'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let owner: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(third)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(owner, Some(bob_id));
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/recordings")
            .insert_header(("Authorization", format!("Session {}", bob.token)))
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    assert_eq!(
        decode::<HistoryPage>(&test::read_body(r).await).items.len(),
        1
    );
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/me/recordings/{first}"))
                .insert_header(("Authorization", format!("Session {}", bob.token)))
                .to_request()
        )
        .await
        .status(),
        404
    );
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/logout")
                .insert_header(("Authorization", format!("Session {}", bob.token)))
                .to_request()
        )
        .await
        .status(),
        204
    );
    let fourth = recording(&app, &d, 4.0).await;
    let owner: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(fourth)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(owner, None);
    // The recording device retains access independently of claims.
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/me/recordings/{first}"))
                .insert_header(("Authorization", auth_header(&d)))
                .to_request()
        )
        .await
        .status(),
        200
    );
    let carol_cookie = web_login(&app, &server, "carol").await;
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me/recordings")
            .cookie(carol_cookie)
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    assert!(
        decode::<HistoryPage>(&test::read_body(r).await)
            .items
            .is_empty()
    );
    let _carol = desktop_login(&app, &server, &d, "carol").await;
    let owners: Vec<Option<Uuid>> =
        sqlx::query_scalar("SELECT account_id FROM recordings ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(owners[0], Some(alice_id));
    assert_eq!(owners[1], Some(alice_id));
    assert_eq!(owners[2], Some(bob_id));
    assert!(owners[3].is_some());
}

#[sqlx::test(migrations = "./migrations")]
async fn oauth_rejects_bad_claims_state_and_browser_binding(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    for (field, value) in [
        ("iss", serde_json::json!("https://evil.test")),
        ("aud", serde_json::json!("other-client")),
        ("exp", serde_json::json!(0)),
        ("nonce", serde_json::json!("wrong")),
        ("sub", serde_json::json!("")),
        ("email", serde_json::json!("")),
    ] {
        server.reset().await;
        let (state, nonce, cookie) = begin(&app, "target=web").await;
        let mut claims = serde_json::json!({"iss":"accounts.google.com","aud":"test-client","exp":chrono::Utc::now().timestamp()+3600,"nonce":nonce,"sub":"alice","email":"alice@example.test"});
        claims[field] = value;
        token_response(&server, claims).await;
        let request = || {
            test::TestRequest::get()
                .uri(&format!(
                    "/api/auth/google/callback?state={state}&code=valid"
                ))
                .cookie(cookie.clone().unwrap())
                .to_request()
        };
        assert_eq!(
            test::call_service(&app, request()).await.status(),
            401,
            "{field}"
        );
        assert_eq!(test::call_service(&app, request()).await.status(), 400);
    }
    let (state, _, _) = begin(&app, "target=web").await;
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!(
                    "/api/auth/google/callback?state={state}&code=valid"
                ))
                .to_request()
        )
        .await
        .status(),
        400
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM accounts")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    for target in [
        "target=desktop&port=80&app_state=x",
        "target=desktop&port=18181&app_state=wrong",
        "target=evil",
    ] {
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::get()
                    .uri(&format!("/api/auth/google/start?{target}"))
                    .to_request()
            )
            .await
            .status(),
            400
        );
    }
    for destination in [
        "//evil.test",
        "%2F%2Fevil.test",
        "/%5Cevil.test",
        "/%252f%252fevil.test",
        "/%0aLocation:evil",
    ] {
        let (state, _, cookie) = begin(&app, &format!("target=web&return_to={destination}")).await;
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!(
                    "/api/auth/google/callback?state={state}&error=access_denied"
                ))
                .cookie(cookie.unwrap())
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 302);
        assert_eq!(
            r.headers().get("Location").unwrap(),
            "http://127.0.0.1:8000/history?signin=cancelled"
        );
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn sessions_slide_and_reject_deleted_or_expired_owners(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    let d = device(&app).await;
    let native = desktop_login(&app, &server, &d, "alice").await;
    let request = || {
        test::TestRequest::get()
            .uri("/api/account")
            .insert_header(("Authorization", format!("Session {}", native.token)))
            .to_request()
    };
    sqlx::query("UPDATE account_sessions SET last_used_at=now()-interval '2 days',expires_at=now()+interval '1 hour'").execute(&pool).await.unwrap();
    assert_eq!(test::call_service(&app, request()).await.status(), 200);
    let expires: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT expires_at FROM account_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(expires > chrono::Utc::now() + chrono::Duration::days(89));
    assert_eq!(test::call_service(&app, request()).await.status(), 200);
    let same: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT expires_at FROM account_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(expires, same);
    sqlx::query("UPDATE account_sessions SET expires_at=now()-interval '1 second'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(test::call_service(&app, request()).await.status(), 401);
    sqlx::query("UPDATE account_sessions SET expires_at=now()+interval '1 day'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE accounts SET deleted_at=now()")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(test::call_service(&app, request()).await.status(), 401);
    sqlx::query("UPDATE accounts SET deleted_at=NULL")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE devices SET deleted_at=now()")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(test::call_service(&app, request()).await.status(), 401);
}

#[sqlx::test(migrations = "./migrations")]
async fn recording_creation_serializes_with_claim_and_logout(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(state(pool.clone()))
            .service(routes::api(None)),
    )
    .await;
    let d = device(&app).await;
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/me/recordings")
                .insert_header(("Authorization", auth_header(&d)))
                .to_request()
        )
        .await
        .status(),
        200
    );
    let account: Uuid = sqlx::query_scalar(
        "INSERT INTO accounts(google_sub,email) VALUES('racer','racer@example.test') RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let binding: Uuid=sqlx::query_scalar("INSERT INTO account_sessions(account_id,token_hash,kind,device_id,expires_at) VALUES($1,$2,'desktop',$3,now()+interval '1 day') RETURNING id")
        .bind(account).bind(vec![3u8;32]).bind(Uuid::from_bytes(d.device_id)).fetch_one(&pool).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE devices SET account_id=$1,account_session_id=$3 WHERE id=$2")
        .bind(account)
        .bind(Uuid::from_bytes(d.device_id))
        .bind(binding)
        .execute(&mut *tx)
        .await
        .unwrap();
    let request = test::TestRequest::post()
        .uri("/api/recordings/sessions")
        .insert_header(("Authorization", auth_header(&d)))
        .set_payload(encode(session(probe(10.0, 1.0, "ashe"))))
        .to_request();
    let future = test::call_service(&app, request);
    tokio::pin!(future);
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(100), &mut future)
            .await
            .is_err(),
        "recording creation must wait for account transition's device lock"
    );
    tx.commit().await.unwrap();
    let r = future.await;
    assert_eq!(r.status(), 200);
    let started = decode::<SessionStarted>(&test::read_body(r).await);
    let actual: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(Uuid::from_bytes(started.recording_id))
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(actual, Some(account));
}

#[sqlx::test(migrations = "./migrations")]
async fn expired_oauth_state_has_browser_readable_retry_page(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(state_with(pool, oauth_config("http://127.0.0.1:1/token")))
            .service(routes::api(None)),
    )
    .await;
    let r = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/auth/google/callback?state=unknown&code=valid")
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 400);
    assert!(
        r.headers()
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );
    let body = String::from_utf8(test::read_body(r).await.to_vec()).unwrap();
    assert!(body.contains("Sign-in expired"));
    assert!(body.contains("try again"));
}

#[sqlx::test(migrations = "./migrations")]
async fn account_owner_replays_other_device_and_shares_finished_only(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    let d1 = device(&app).await;
    let id = recording(&app, &d1, 1.0).await;
    let _first = desktop_login(&app, &server, &d1, "alice").await;
    let d2 = device(&app).await;
    let second = desktop_login(&app, &server, &d2, "alice").await;
    let cookie = web_login(&app, &server, "alice").await;
    sqlx::query("INSERT INTO recording_chunks (recording_id,seq,first_game_time,last_game_time,snapshot_count,raw_format,raw_sha256,raw_size,raw,frames_format,frames) VALUES ($1,0,30,40,1,1,decode(repeat('00',32),'hex'),2,decode('7b7d','hex'),$2,$3)").bind(id).bind(FRAMES_FORMAT as i16).bind(vec![1u8,2,3]).execute(&pool).await.unwrap();
    for suffix in [format!("/{id}"), format!("/{id}/frames/0")] {
        let uri = format!("/api/me/recordings{suffix}");
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&uri)
                .insert_header(("Authorization", format!("Session {}", second.token)))
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 200);
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&uri)
                .cookie(cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 200);
        if suffix.ends_with("/0") {
            assert_eq!(r.headers().get("Content-Encoding").unwrap(), "br");
            assert_eq!(test::read_body(r).await.as_ref(), &[1u8, 2, 3]);
        }
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::get()
                    .uri(&uri)
                    .insert_header(("Authorization", auth_header(&d2)))
                    .to_request()
            )
            .await
            .status(),
            404
        );
    }
    let share = || {
        test::TestRequest::put()
            .uri(&format!("/api/me/recordings/{id}/share"))
            .cookie(cookie.clone())
            .insert_header(("Origin", "http://127.0.0.1:8000"))
            .insert_header(("Content-Type", "application/octet-stream"))
            .set_payload(encode(SetShareMode {
                mode: ShareMode::Public,
            }))
            .to_request()
    };
    assert_eq!(test::call_service(&app, share()).await.status(), 409);
    sqlx::query(
        "UPDATE recordings SET ended_at=now(),end_reason='timeout',is_custom=false WHERE id=$1",
    )
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();
    let r = test::call_service(&app, share()).await;
    assert_eq!(r.status(), 200);
    let public: ShareState = decode(&test::read_body(r).await);
    assert_eq!(public.mode, ShareMode::Public);
    assert!(public.code.is_some());
    let outsider = web_login(&app, &server, "bob").await;
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::put()
                .uri(&format!("/api/me/recordings/{id}/share"))
                .cookie(outsider)
                .insert_header(("Origin", "http://127.0.0.1:8000"))
                .insert_header(("Content-Type", "application/octet-stream"))
                .set_payload(encode(SetShareMode {
                    mode: ShareMode::Private
                }))
                .to_request()
        )
        .await
        .status(),
        404
    );
    // Capability viewing stays separate from account owner access.
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/share/{}", public.code.unwrap()))
                .to_request()
        )
        .await
        .status(),
        200
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn google_exchange_rejects_redirects_large_or_malformed_responses(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool,
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    for response in [
        ResponseTemplate::new(302).insert_header("Location", format!("{}/leak", server.uri())),
        ResponseTemplate::new(200).set_body_string("x".repeat(65537)),
        ResponseTemplate::new(200).set_body_json(serde_json::json!({"id_token":"malformed"})),
    ] {
        server.reset().await;
        let (state, _, cookie) = begin(&app, "target=web").await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(response)
            .mount(&server)
            .await;
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!(
                    "/api/auth/google/callback?state={state}&code=valid"
                ))
                .cookie(cookie.unwrap())
                .to_request(),
        )
        .await;
        assert!([401, 502].contains(&r.status().as_u16()));
        assert_eq!(r.headers().get("Cache-Control").unwrap(), "no-store");
        let text = String::from_utf8(test::read_body(r).await.to_vec()).unwrap();
        assert!(!text.contains("test-secret"));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn invalid_desktop_binding_cannot_own_future_recordings(pool: PgPool) {
    let server = MockServer::start().await;
    let app = test::init_service(
        App::new()
            .app_data(state_with(
                pool.clone(),
                oauth_config(&format!("{}/token", server.uri())),
            ))
            .service(routes::api(None)),
    )
    .await;
    let d = device(&app).await;
    for (i, invalid) in ["expired", "revoked", "deleted_account"]
        .into_iter()
        .enumerate()
    {
        let login = desktop_login(&app, &server, &d, &format!("owner{i}")).await;
        let old = recording(&app, &d, (i * 2 + 1) as f32).await;
        let old_owner: Option<Uuid> =
            sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
                .bind(old)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(old_owner.is_some());
        let token_hash = Sha256::digest(B64.decode(&login.token).unwrap()).to_vec();
        match invalid {
            "expired" => {
                sqlx::query("UPDATE account_sessions SET expires_at=now()-interval '1 second' WHERE token_hash=$1").bind(&token_hash).execute(&pool).await.unwrap();
            }
            "revoked" => {
                sqlx::query("UPDATE account_sessions SET revoked_at=now() WHERE token_hash=$1")
                    .bind(&token_hash)
                    .execute(&pool)
                    .await
                    .unwrap();
            }
            _ => {
                sqlx::query("UPDATE accounts SET deleted_at=now() WHERE id=$1")
                    .bind(old_owner)
                    .execute(&pool)
                    .await
                    .unwrap();
            }
        }
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::get()
                    .uri("/api/account")
                    .insert_header(("Authorization", format!("Session {}", login.token)))
                    .to_request()
            )
            .await
            .status(),
            401
        );
        let new = recording(&app, &d, (i * 2 + 2) as f32).await;
        let owner: Option<Uuid> =
            sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
                .bind(new)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(
            owner, None,
            "{invalid} desktop binding must not attach a new game"
        );
        let link: (Option<Uuid>, Option<Uuid>) =
            sqlx::query_as("SELECT account_id,account_session_id FROM devices WHERE id=$1")
                .bind(Uuid::from_bytes(d.device_id))
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(link, (None, None));
        let retained: Option<Uuid> =
            sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
                .bind(old)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(retained, old_owner);
    }
    // Invalid old sessions must not cause a later, valid binding to be cleared.
    let fresh = desktop_login(&app, &server, &d, "fresh").await;
    let new = recording(&app, &d, 8.0).await;
    let owner: Option<Uuid> = sqlx::query_scalar("SELECT account_id FROM recordings WHERE id=$1")
        .bind(new)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(owner.is_some());
    assert_eq!(
        test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/account")
                .insert_header(("Authorization", format!("Session {}", fresh.token)))
                .to_request()
        )
        .await
        .status(),
        200
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn web_day_slide_renews_cookie_only_on_success_and_never_logout(pool: PgPool) {
    let server = MockServer::start().await;
    for secure in [false, true] {
        let mut config = oauth_config(&format!("{}/token", server.uri()));
        config.cookie_secure = secure;
        let app = test::init_service(
            App::new()
                .app_data(state_with(pool.clone(), config))
                .service(routes::api(None)),
        )
        .await;
        let cookie = web_login(&app, &server, "alice").await;
        let hash = Sha256::digest(B64.decode(cookie.value()).unwrap()).to_vec();
        let age = || {
            sqlx::query("UPDATE account_sessions SET last_used_at=now()-interval '2 days',expires_at=now()+interval '1 hour' WHERE token_hash=$1").bind(&hash).execute(&pool)
        };
        age().await.unwrap();
        let before: chrono::DateTime<chrono::Utc> =
            sqlx::query_scalar("SELECT expires_at FROM account_sessions WHERE token_hash=$1")
                .bind(&hash)
                .fetch_one(&pool)
                .await
                .unwrap();
        let r = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/api/me/recordings/{}", Uuid::nil()))
                .cookie(cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 404);
        assert!(!r.headers().contains_key("Set-Cookie"));
        let unchanged: chrono::DateTime<chrono::Utc> =
            sqlx::query_scalar("SELECT expires_at FROM account_sessions WHERE token_hash=$1")
                .bind(&hash)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(
            before, unchanged,
            "failed owner reads must not consume the day-slide"
        );
        let request = || {
            test::TestRequest::get()
                .uri("/api/account")
                .cookie(cookie.clone())
                .to_request()
        };
        let r = test::call_service(&app, request()).await;
        assert_eq!(r.status(), 200);
        let renewed = r
            .response()
            .cookies()
            .find(|c| c.name() == "tl_session")
            .expect("day-slide must renew the browser cookie");
        assert_eq!(renewed.value(), cookie.value());
        assert_eq!(renewed.http_only(), Some(true));
        assert_eq!(renewed.path(), Some("/api"));
        assert_eq!(renewed.same_site(), Some(actix_web::cookie::SameSite::Lax));
        assert_eq!(renewed.secure().unwrap_or(false), secure);
        assert_eq!(renewed.max_age().unwrap().whole_seconds(), 90 * 86400);
        let after: chrono::DateTime<chrono::Utc> =
            sqlx::query_scalar("SELECT expires_at FROM account_sessions WHERE token_hash=$1")
                .bind(&hash)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(after > chrono::Utc::now() + chrono::Duration::days(89));
        let r = test::call_service(&app, request()).await;
        assert_eq!(r.status(), 200);
        assert!(!r.headers().contains_key("Set-Cookie"));
        age().await.unwrap();
        let r = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/logout")
                .cookie(cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 403);
        assert!(!r.headers().contains_key("Set-Cookie"));
        let r = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/logout")
                .cookie(cookie.clone())
                .insert_header(("Origin", "http://127.0.0.1:8000"))
                .insert_header(("Content-Type", "application/octet-stream"))
                .to_request(),
        )
        .await;
        assert_eq!(r.status(), 204);
        let cookies: Vec<_> = r
            .response()
            .cookies()
            .filter(|c| c.name() == "tl_session")
            .collect();
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].max_age().unwrap().whole_seconds(), 0);
        let r = test::call_service(&app, request()).await;
        assert_eq!(r.status(), 401);
        assert!(!r.headers().contains_key("Set-Cookie"));
    }
}
