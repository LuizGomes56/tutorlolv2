use crate::backend::{BINCODE_CONFIG, uploader::*};
use std::{
    io::{Read, Write},
    time::Duration,
};
#[tokio::test]
async fn session_auth_is_only_used_for_owner_calls() {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let mut requests = vec![];
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut buf = [0; 8192];
            let n = stream.read(&mut buf).unwrap();
            requests.push(String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase());
            stream
                .write_all(
                    b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
        }
        requests
    });
    let mut api = ApiClient::with_base_url(
        base,
        Some(&DeviceCredentials {
            device_id: [1; 16],
            device_secret: [2; 32],
        }),
    );
    api.set_account(Some("native-only-token"), 1);
    assert!(api.history_page(None).await.is_err());
    assert!(api.account_invalid());
    assert!(matches!(
        api.upload_chunk(&[3; 16], 0, vec![], &"0".repeat(64)).await,
        ChunkOutcome::Unauthorized
    ));
    let requests = server.join().unwrap();
    assert!(requests[0].contains("authorization: session native-only-token"));
    assert!(requests[1].contains("authorization: device "));
    assert!(!requests[1].contains("native-only-token"));
}

use tutorlolv2_protocol::{AccountInfo, ApiError, ApiResponse, DeviceCredentials, ErrorCode};

// Catches regressions that decode the envelope as a bare DTO or discard a remote error body.
#[tokio::test]
async fn account_decodes_bincode_success_and_remote_failure() {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let success = bincode::encode_to_vec(
            ApiResponse::new(
                200,
                Ok(AccountInfo {
                    email: "player@example.test".into(),
                    created_at_ms: 42,
                }),
            )
            .unwrap(),
            BINCODE_CONFIG,
        )
        .unwrap();
        let failure = bincode::encode_to_vec(
            ApiResponse::<()>::new(
                503,
                Err(ApiError {
                    code: ErrorCode::Upstream,
                    message: "Account service unavailable".into(),
                    details: None,
                    traces: vec![],
                }),
            )
            .unwrap(),
            BINCODE_CONFIG,
        )
        .unwrap();
        for (status, bytes) in [("200 OK", success), ("503 Service Unavailable", failure)] {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut request = [0; 4096];
            let n = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..n]).to_ascii_lowercase();
            assert!(request.starts_with("get /api/account "));
            assert!(request.contains("authorization: session test-token"));
            write!(stream, "HTTP/1.1 {status}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len()).unwrap();
            stream.write_all(&bytes).unwrap();
        }
    });
    let mut api = ApiClient::with_base_url(base, None);
    api.set_account(Some("test-token"), 1);
    let account = api.account().await.unwrap();
    assert_eq!(account.email, "player@example.test");
    assert_eq!(account.created_at_ms, 42);
    let error = api.account().await.unwrap_err();
    assert_eq!(error.message, "Account service unavailable");
    assert!(error.retryable);
    server.join().unwrap();
}

use tutorlolv2_protocol::HistoryPage;
#[tokio::test]
async fn registration_and_history_use_typed_envelopes_and_separate_auth() {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let credentials = bincode::encode_to_vec(
            ApiResponse::new(
                200,
                Ok(DeviceCredentials {
                    device_id: [1; 16],
                    device_secret: [2; 32],
                }),
            )
            .unwrap(),
            BINCODE_CONFIG,
        )
        .unwrap();
        let page = bincode::encode_to_vec(
            ApiResponse::new(
                200,
                Ok(HistoryPage {
                    items: vec![],
                    next_before: None,
                }),
            )
            .unwrap(),
            BINCODE_CONFIG,
        )
        .unwrap();
        for (path, body) in [
            ("post /api/devices ", credentials),
            ("get /api/me/recordings?limit=100 ", page),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut input = [0; 8192];
            let n = stream.read(&mut input).unwrap();
            let request = String::from_utf8_lossy(&input[..n]).to_ascii_lowercase();
            assert!(request.starts_with(path), "{request}");
            if path.starts_with("get") {
                assert!(request.contains("authorization: session owner-token"));
            } else {
                assert!(!request.contains("authorization:"));
            }
            write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();
            stream.write_all(&body).unwrap();
        }
    });
    let mut api = ApiClient::with_base_url(base, None);
    let credentials = api.register("test").await.unwrap();
    assert_eq!(credentials.device_id, [1; 16]);
    api.set_account(Some("owner-token"), 1);
    let page = api.history_page(None).await.unwrap();
    assert!(page.items.is_empty());
    server.join().unwrap();
}
