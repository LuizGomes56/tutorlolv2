use desktop::{BINCODE_CONFIG, account::*, delivery::Delivery};
use protocol::AccountErrorKind as K;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::Arc,
    time::{Duration, Instant},
};
#[test]
fn callback_checks_identity() {
    let r = "GET /callback?app_state=secret&ticket=abc HTTP/1.1\r\nHost: 127.0.0.1:1234\r\n\r\n";
    assert_eq!(callback(r, 1234, "secret"), Some("abc".into()));
    for bad in [
        r.replace("secret", "wrong"),
        r.replace("GET", "POST"),
        r.replace("/callback?", "/bad?"),
        r.replace("127.0.0.1", "evil.test"),
        r.replace("ticket=abc", "token=abc"),
    ] {
        assert!(callback(&bad, 1234, "secret").is_none());
    }
    assert!(authorize("overlay").is_err());
}
#[test]
fn timeout_is_bounded() {
    let l = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let start = Instant::now();
    assert!(matches!(receive(l,"state",Duration::from_millis(20)),Err(e)if e.kind==K::Cancelled));
    assert!(start.elapsed() < Duration::from_secs(1));
}

#[test]
fn listener_rejects_invalid_then_accepts_and_cancels() {
    for cancel in [false, true] {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let worker = std::thread::spawn(move || receive(listener, "state", Duration::from_secs(2)));
        for query in [
            "app_state=wrong&ticket=abc",
            if cancel {
                "app_state=state&error=cancelled"
            } else {
                "app_state=state&ticket=abc"
            },
        ] {
            let mut stream = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
            write!(
                stream,
                "GET /callback?{query} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n\r\n"
            )
            .unwrap();
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response);
            assert!(response.starts_with("HTTP/1.1 "));
        }
        let result = worker.join().unwrap();
        if cancel {
            assert!(matches!(result,Err(e) if e.kind==K::Cancelled));
        } else {
            assert_eq!(result.unwrap(), "abc");
        }
    }
}
#[test]
fn dpapi_roundtrip_is_bound_to_device_and_contains_no_plaintext() {
    let dir = std::env::temp_dir().join(format!("m4-account-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let store = desktop::store::Store::new(dir.clone());
    let credential = Credential {
        device_id: [4; 16],
        token: "secret-session-token".into(),
        email: "test@example.invalid".into(),
    };
    store.save_account(Some(&credential)).unwrap();
    let loaded = desktop::store::Store::new(dir.clone())
        .load_account(Some([4; 16]))
        .unwrap();
    assert_eq!(loaded.token, credential.token);
    let bytes = std::fs::read(dir.join("account.bin")).unwrap();
    assert!(
        !bytes
            .windows(credential.token.len())
            .any(|w| w == credential.token.as_bytes())
    );
    assert!(store.load_account(Some([5; 16])).is_none());
    store.save_account(None).unwrap();
    assert!(store.load_account(Some([4; 16])).is_none());
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn owner_identity_changes_without_replacing_device() {
    let mut api = desktop::uploader::ApiClient::new(None);
    api.device_id = Some([7; 16]);
    let anonymous = api.owner_id();
    api.set_account(Some("account-one"), 1);
    let stale = api.clone();
    let first = api.owner_id();
    assert_ne!(first, anonymous);
    assert_eq!(api.device_id, Some([7; 16]));
    api.set_account(Some("account-two"), 2);
    assert_ne!(first, api.owner_id());
    assert!(stale.account_invalid());
    api.set_account(None, 3);
    assert_eq!(anonymous, api.owner_id());
}

#[tokio::test]
async fn pending_signin_state_poll_never_waits_for_browser() {
    let dir = std::env::temp_dir().join(format!("m4-pending-{}", std::process::id()));
    let delivery = Delivery::new(
        desktop::recorder::Recorder::new(None, None, "test".into()),
        desktop::uploader::ApiClient::new(None),
        desktop::spool::Spool::new(dir.join("spool"), desktop::spool::Spool::DEFAULT_MAX_BYTES),
        desktop::store::Store::new(dir),
    );
    let _transition = delivery.account.transition.lock().await;
    delivery.account.set(None, true, None);
    let state = tokio::time::timeout(Duration::from_millis(100), delivery.account_state())
        .await
        .unwrap()
        .unwrap();
    assert!(state.signing_in);
    let json = serde_json::to_string(&state).unwrap();
    assert!(!json.contains("token"));
    assert!(!json.contains("secret"));
}

fn delivery(dir: std::path::PathBuf, base: String) -> Delivery {
    Delivery::new(
        desktop::recorder::Recorder::new(None, None, "test".into()),
        desktop::uploader::ApiClient::with_base_url(
            base,
            Some(&protocol::DeviceCredentials {
                device_id: [1; 16],
                device_secret: [2; 32],
            }),
        ),
        desktop::spool::Spool::new(dir.join("spool"), desktop::spool::Spool::DEFAULT_MAX_BYTES),
        desktop::store::Store::new(dir),
    )
}
fn credential(token: &str, email: &str) -> Credential {
    Credential {
        device_id: [1; 16],
        token: token.into(),
        email: email.into(),
    }
}
fn account_server(
    dir: std::path::PathBuf,
    responses: Vec<(u16, Vec<u8>)>,
) -> (String, std::thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let worker = std::thread::spawn(move || {
        let mut requests = vec![];
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut buf = [0; 4096];
            let n = stream.read(&mut buf).unwrap();
            let request = String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase();
            if request.starts_with("post /api/auth/desktop/confirm ") {
                assert!(
                    desktop::store::Store::new(dir.clone())
                        .load_account(Some([1; 16]))
                        .is_some(),
                    "confirm must follow durable DPAPI persistence"
                );
            }
            requests.push(request);
            write!(
                stream,
                "HTTP/1.1 {status} Response\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            stream.write_all(&body).unwrap();
        }
        requests
    });
    (base, worker)
}
#[tokio::test]
async fn failed_save_and_rollback_never_restore_an_unsaved_owner_after_restart() {
    for status in [204, 503] {
        let dir = std::env::temp_dir().join(format!("m4-rollback-{status}-{}", std::process::id()));
        let store = desktop::store::Store::new(dir.clone());
        store
            .save_account(Some(&credential("old-native-token", "old@example.invalid")))
            .unwrap();
        std::fs::create_dir(dir.join("account.bin.tmp")).unwrap();
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut buf = [0; 4096];
            let n = stream.read(&mut buf).unwrap();
            write!(
                stream,
                "HTTP/1.1 {status} Response\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
            String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase()
        });
        let d = delivery(dir.clone(), base);
        let result = d
            .install_account(credential("new-native-token", "new@example.invalid"))
            .await;
        assert!(matches!(result,Err(e) if e.kind==K::Storage));
        let request = server.join().unwrap();
        assert!(request.starts_with("post /api/auth/logout "));
        assert!(request.contains("authorization: session new-native-token"));
        // Neither a failed save nor failed rollback may expose an unsaved owner.
        assert!(d.account.state().email.is_none());
        assert!(!dir.join("account.bin").exists());
        drop(d);
        let restored = delivery(dir.clone(), "http://127.0.0.1:1".into());
        assert!(restored.account_state().await.unwrap().email.is_none());
        assert_eq!(
            restored.api.read().await.owner_id(),
            restored.history.identity()
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
#[tokio::test]
async fn persisted_account_restores_native_owner_and_offline_email() {
    let dir = std::env::temp_dir().join(format!("m4-account-restart-{}", std::process::id()));
    let (base, worker) = account_server(dir.clone(), vec![(204, vec![])]);
    let d = delivery(dir.clone(), base);
    d.install_account(credential("native-session", "owner@example.invalid"))
        .await
        .unwrap();
    let requests = worker.join().unwrap();
    assert!(requests[0].starts_with("post /api/auth/desktop/confirm "));
    assert!(requests[0].contains("authorization: session native-session"));
    let owner = d.api.read().await.owner_id();
    drop(d);
    let restored = delivery(dir.clone(), "http://127.0.0.1:1".into());
    assert_eq!(restored.api.read().await.owner_id(), owner);
    assert_eq!(restored.history.identity(), owner);
    assert_eq!(
        restored.account.state().email.as_deref(),
        Some("owner@example.invalid")
    );
    let _ = std::fs::remove_dir_all(dir);
}
#[tokio::test]
async fn failed_confirmation_keeps_persisted_owner_and_recovers_after_restart() {
    let dir = std::env::temp_dir().join(format!("m4-confirm-restart-{}", std::process::id()));
    let (base, worker) = account_server(dir.clone(), vec![(503, vec![])]);
    let d = delivery(dir.clone(), base);
    assert!(
        d.install_account(credential(
            "pending-native-session",
            "owner@example.invalid"
        ))
        .await
        .is_err()
    );
    assert_eq!(
        d.account.state().email.as_deref(),
        Some("owner@example.invalid")
    );
    assert!(
        desktop::store::Store::new(dir.clone())
            .load_account(Some([1; 16]))
            .is_some()
    );
    assert!(worker.join().unwrap()[0].starts_with("post /api/auth/desktop/confirm "));
    drop(d);
    let info = bincode::encode_to_vec(
        protocol::ApiResponse::new(
            200,
            Ok(protocol::AccountInfo {
                email: "owner@example.invalid".into(),
                created_at_ms: 1,
            }),
        )
        .unwrap(),
        BINCODE_CONFIG,
    )
    .unwrap();
    let (base, worker) = account_server(dir.clone(), vec![(204, vec![]), (200, info)]);
    let restored = delivery(dir.clone(), base);
    assert_eq!(
        restored.account_state().await.unwrap().email.as_deref(),
        Some("owner@example.invalid")
    );
    let requests = worker.join().unwrap();
    assert!(requests[0].starts_with("post /api/auth/desktop/confirm "));
    assert!(requests[1].starts_with("get /api/account "));
    let _ = std::fs::remove_dir_all(dir);
}
#[tokio::test]
async fn expired_confirmation_clears_durable_credential_and_owner_immediately() {
    let dir = std::env::temp_dir().join(format!("m4-confirm-expired-{}", std::process::id()));
    let store = desktop::store::Store::new(dir.clone());
    store
        .save_account(Some(&credential("expired-session", "old@example.invalid")))
        .unwrap();
    let (base, worker) = account_server(dir.clone(), vec![(401, vec![])]);
    let d = delivery(dir.clone(), base);
    let old_owner = d.api.read().await.owner_id();
    assert!(d.account_state().await.unwrap().email.is_none());
    assert!(store.load_account(Some([1; 16])).is_none());
    assert_ne!(d.api.read().await.owner_id(), old_owner);
    assert_eq!(d.api.read().await.owner_id(), d.history.identity());
    worker.join().unwrap();
    let _ = std::fs::remove_dir_all(dir);
}
#[tokio::test]
async fn cached_history_does_not_wait_for_offline_account_validation() {
    let dir = std::env::temp_dir().join(format!("m4-cache-offline-{}", std::process::id()));
    desktop::store::Store::new(dir.clone())
        .save_account(Some(&credential("durable-token", "owner@example.invalid")))
        .unwrap();
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0; 4096];
        stream.read(&mut buf).unwrap();
        std::thread::sleep(Duration::from_millis(350));
        let _ = stream.write_all(
            b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
    });
    let d = Arc::new(delivery(dir.clone(), base));
    d.history.local([3; 16], Some([4; 16]), 1, true, 0).unwrap();
    let result = tokio::time::timeout(Duration::from_millis(100), d.cached_history(0, 50)).await;
    tokio::task::spawn_blocking(move || worker.join().unwrap())
        .await
        .unwrap();
    let cached = result
        .expect("cached history must not wait for validation network timeout")
        .unwrap();
    assert_eq!(cached.entries.len(), 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn cached_history_clears_known_invalid_identity_before_listing() {
    let dir = std::env::temp_dir().join(format!("m4-cache-invalid-{}", std::process::id()));
    desktop::store::Store::new(dir.clone())
        .save_account(Some(&credential("expired-token", "owner@example.invalid")))
        .unwrap();
    let (base, worker) = account_server(dir.clone(), vec![(401, vec![])]);
    let d = Arc::new(delivery(dir.clone(), base));
    let api = d.api.read().await.clone();
    let old_owner = api.owner_id();
    assert!(api.history_page(None).await.is_err());
    worker.join().unwrap();
    d.cached_history(0, 50).await.unwrap();
    assert!(d.account.state().email.is_none());
    assert_ne!(d.history.identity(), old_owner);
    let _ = std::fs::remove_dir_all(dir);
}
