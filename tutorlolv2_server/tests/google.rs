use actix_web::ResponseError;
use actix_web::web::Data;
use actix_web::web::Query;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};
use tutorlolv2_protocol::RedeemTicket;
use tutorlolv2_server::server::codec::Bincode;
use tutorlolv2_server::server::{auth::DeviceAuth, google::*, state::AppState};
use uuid::Uuid;
#[test]
fn pending_states_and_tickets_expire_independently() {
    let store = PendingStore::default();
    let now = Instant::now();
    let login = |at| Login {
        target: Target::Web("/history".into()),
        verifier: "test".into(),
        nonce: "test".into(),
        at,
    };
    store
        .logins
        .lock()
        .unwrap()
        .insert("expired".into(), login(now - Duration::from_secs(601)));
    store
        .logins
        .lock()
        .unwrap()
        .insert("current".into(), login(now));
    store.tickets.lock().unwrap().insert(
        "expired".into(),
        (Uuid::nil(), now - Duration::from_secs(121)),
    );
    store
        .tickets
        .lock()
        .unwrap()
        .insert("current".into(), (Uuid::nil(), now));
    store.sweep();
    assert_eq!(
        store
            .logins
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        vec!["current"]
    );
    assert_eq!(
        store
            .tickets
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        vec!["current"]
    );
}
#[test]
fn session_hashes_decode_exactly_thirty_two_random_bytes() {
    let a = token().unwrap();
    let b = token().unwrap();
    assert_ne!(a, b);
    assert_eq!(
        hash(&a),
        Some(Sha256::digest(B64.decode(&a).unwrap()).to_vec())
    );
    for invalid in ["", "malformed!", &B64.encode([1; 31]), &B64.encode([1; 33])] {
        assert!(hash(invalid).is_none());
    }
}
#[test]
fn return_destinations_stay_on_the_website() {
    for invalid in [
        "https://evil.test",
        "//evil.test",
        "/\\evil.test",
        "/%2f%2fevil.test",
        "/\nLocation:evil",
        "/\0",
        "/\u{7f}",
    ] {
        assert_eq!(return_path(Some(invalid.into())), "/history");
    }
    assert_eq!(
        return_path(Some("/history?before=123".into())),
        "/history?before=123"
    );
}
#[actix_web::test]
async fn expired_or_over_capacity_logins_do_not_reach_google_or_storage() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://unused:unused@127.0.0.1/unused")
        .unwrap();
    let mut config = tutorlolv2_server::server::config::Config::from_lookup(|key| {
        (key == "DATABASE_URL").then(|| "unused".into())
    })
    .unwrap();
    config.google_client_id = Some("test".into());
    config.google_client_secret = Some("test".into());
    let s = Data::new(AppState::new(pool, config));
    s.google.logins.lock().unwrap().insert(
        "expired".into(),
        Login {
            target: Target::Web("/history".into()),
            verifier: "test".into(),
            nonce: "test".into(),
            at: Instant::now() - Duration::from_secs(601),
        },
    );
    let result = callback(
        s.clone(),
        actix_web::test::TestRequest::get().to_http_request(),
        Query(Callback {
            state: "expired".into(),
            code: Some("code".into()),
            error: None,
        }),
    )
    .await;
    assert_eq!(result.unwrap().status(), 400);
    s.google.tickets.lock().unwrap().insert(
        "expired".into(),
        (Uuid::nil(), Instant::now() - Duration::from_secs(121)),
    );
    let result = redeem(
        s.clone(),
        DeviceAuth {
            device_id: Uuid::nil(),
        },
        Bincode(RedeemTicket {
            ticket: "expired".into(),
        }),
    )
    .await;
    assert_eq!(result.err().unwrap().status_code(), 410);
    for i in 0..MAX_PENDING {
        s.google.logins.lock().unwrap().insert(
            format!("{i}"),
            Login {
                target: Target::Web("/history".into()),
                verifier: "test".into(),
                nonce: "test".into(),
                at: Instant::now(),
            },
        );
    }
    let result = start(
        s,
        Query(Start {
            target: "web".into(),
            return_to: None,
            port: None,
            app_state: None,
        }),
    )
    .await;
    assert_eq!(result.err().unwrap().status_code(), 429);
}
