//! Native-only account credentials and bounded loopback sign-in.
use crate::{BINCODE_CONFIG, delivery::Delivery};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use bincode::{Decode, Encode};
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tutorlolv2_protocol::{
    AccountError, AccountErrorKind as K, AccountState, DesktopSession, Id16, RedeemTicket,
};
#[derive(Encode, Decode)]
pub struct Credential {
    pub device_id: Id16,
    pub token: String,
    pub email: String,
}
pub struct Account {
    state: Mutex<AccountState>,
    pub transition: tokio::sync::Mutex<()>,
    checked: Mutex<Option<Instant>>,
}
impl Account {
    pub fn new(c: Option<&Credential>) -> Self {
        Self {
            state: Mutex::new(AccountState {
                email: c.map(|c| c.email.clone()),
                signing_in: false,
                last_error: None,
            }),
            transition: tokio::sync::Mutex::new(()),
            checked: Mutex::new(None),
        }
    }
    pub fn reset(&self) {
        self.set(None, false, None);
    }
    pub fn state(&self) -> AccountState {
        self.state.lock().unwrap().clone()
    }
    fn set(&self, email: Option<String>, signing_in: bool, last_error: Option<String>) {
        *self.state.lock().unwrap() = AccountState {
            email,
            signing_in,
            last_error,
        };
    }
}
pub fn error(kind: K, message: &str, retryable: bool) -> AccountError {
    AccountError {
        kind,
        message: message.into(),
        retryable,
    }
}
pub fn authorize(label: &str) -> Result<(), AccountError> {
    if label == "main" {
        Ok(())
    } else {
        Err(error(
            K::Unauthorized,
            "Account requires the main window",
            false,
        ))
    }
}
pub fn callback(request: &str, port: u16, state: &str) -> Option<String> {
    if request.len() > 8192 || !request.ends_with("\r\n\r\n") {
        return None;
    }
    let mut lines = request.split("\r\n");
    let mut words = lines.next()?.split(' ');
    if words.next()? != "GET" {
        return None;
    }
    let target = words.next()?;
    if words.next()? != "HTTP/1.1" || words.next().is_some() {
        return None;
    }
    let hosts: Vec<_> = lines
        .filter_map(|l| l.split_once(':'))
        .filter(|(k, _)| k.eq_ignore_ascii_case("host"))
        .collect();
    if hosts.len() != 1 || hosts[0].1.trim() != format!("127.0.0.1:{port}") {
        return None;
    }
    let (path, query) = target.split_once('?')?;
    if path != "/callback" {
        return None;
    }
    let mut ticket = None;
    let mut valid = false;
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=')?;
        match k {
            "app_state" if !valid && v == state => valid = true,
            "error" if v == "cancelled" && ticket.is_none() => ticket = Some("!cancelled".into()),
            "ticket"
                if ticket.is_none()
                    && !v.is_empty()
                    && v.len() <= 512
                    && v.bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') =>
            {
                ticket = Some(v.to_owned())
            }
            _ => return None,
        }
    }
    valid.then_some(ticket).flatten()
}
pub fn receive(
    listener: TcpListener,
    state: &str,
    timeout: Duration,
) -> Result<String, AccountError> {
    listener
        .set_nonblocking(true)
        .map_err(|_| error(K::Unavailable, "Callback unavailable", true))?;
    let port = listener.local_addr().unwrap().port();
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match listener.accept() {
            Ok((mut stream, peer)) => {
                if stream.set_nonblocking(false).is_err() {
                    continue;
                }
                if !peer.ip().is_loopback() {
                    continue;
                }
                let remaining = deadline
                    .saturating_duration_since(Instant::now())
                    .min(Duration::from_secs(2));
                if remaining.is_zero() {
                    break;
                }
                let _ = stream.set_read_timeout(Some(remaining));
                let _ = stream.set_write_timeout(Some(remaining));
                let request_deadline = (Instant::now() + remaining).min(deadline);
                let mut bytes = Vec::new();
                let mut part = [0; 1024];
                while bytes.len() < 8192 {
                    let left = request_deadline.saturating_duration_since(Instant::now());
                    if left.is_zero() {
                        break;
                    }
                    let _ = stream.set_read_timeout(Some(left));
                    match stream.read(&mut part) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            bytes.extend_from_slice(&part[..n]);
                            if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
                                break;
                            }
                        }
                    }
                }
                let ticket = std::str::from_utf8(&bytes)
                    .ok()
                    .and_then(|s| callback(s, port, state));
                let response = if ticket.is_some() {
                    "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                } else {
                    "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                };
                let _ = stream.write_all(response.as_bytes());
                if let Some(t) = ticket {
                    if t == "!cancelled" {
                        return Err(error(K::Cancelled, "Sign-in cancelled", true));
                    }
                    return Ok(t);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10))
            }
            Err(_) => return Err(error(K::Unavailable, "Callback failed", true)),
        }
    }
    Err(error(K::Cancelled, "Sign-in timed out", true))
}
impl Delivery {
    pub(crate) async fn cached_history(
        self: &Arc<Self>,
        offset: u32,
        limit: u32,
    ) -> Result<tutorlolv2_protocol::HistoryList, tutorlolv2_protocol::HistoryError> {
        // A known 401 must invalidate cached identity before returning rows.
        // Never wait for an in-flight network validation or identity transition.
        if self.api.read().await.account_invalid() {
            let _guard = self.account.transition.try_lock().map_err(|_| {
                crate::history::error(
                    tutorlolv2_protocol::HistoryErrorKind::Unauthorized,
                    "Account identity is changing",
                    true,
                )
            })?;
            if self.api.read().await.account_invalid() {
                self.clear_account().await.map_err(|e| {
                    crate::history::error(
                        tutorlolv2_protocol::HistoryErrorKind::Storage,
                        &e.message,
                        e.retryable,
                    )
                })?;
            }
        }
        let cached = self.history.list(offset, limit);
        let shared = self.clone();
        tauri::async_runtime::spawn(async move {
            let _ = shared.account_state().await;
        });
        Ok(cached)
    }
    pub async fn account_state(&self) -> Result<AccountState, AccountError> {
        let Ok(_guard) = self.account.transition.try_lock() else {
            return Ok(self.account.state());
        };
        if self.api.read().await.account_invalid() {
            self.clear_account().await?;
            return Ok(self.account.state());
        }
        let check = {
            let mut checked = self.account.checked.lock().unwrap();
            if checked.is_some_and(|t| t.elapsed() < Duration::from_secs(60)) {
                false
            } else {
                *checked = Some(Instant::now());
                true
            }
        };
        if (check || self.api.read().await.account_invalid())
            && self.account.state().email.is_some()
        {
            let api = self.api.read().await.clone();
            // Confirm only a credential already loaded from DPAPI or persisted by install.
            if let Err(e) = self.confirm_account(&api).await {
                if e.kind == K::Unauthorized {
                    self.clear_account().await?;
                } else {
                    self.account
                        .set(self.account.state().email, false, Some(e.message));
                }
                return Ok(self.account.state());
            }
            match api.account().await {
                Err(e) if e.kind == K::Unauthorized => self.clear_account().await?,
                Ok(info) => self.account.set(Some(info.email), false, None),
                _ => {}
            }
        }
        Ok(self.account.state())
    }
    async fn confirm_account(&self, api: &crate::uploader::ApiClient) -> Result<(), AccountError> {
        api.confirm_desktop().await.map(|_| ()).map_err(|e| {
            error(
                e.kind,
                "Account saved; confirmation pending. Retry when connected",
                true,
            )
        })
    }

    pub(crate) async fn clear_account(&self) -> Result<(), AccountError> {
        let removed = self
            .store
            .save_account(None)
            .map_err(|_| error(K::Storage, "Could not remove account credential", true));
        let mut api = self.api.write().await;
        let generation = api.owner_generation.wrapping_add(1);
        api.set_account(None, generation);
        let invalidated = self.history.bind_owner(api.owner_id());
        self.account.set(None, false, None);
        removed?;
        invalidated.map_err(|_| error(K::Storage, "Could not save account history", true))
    }
    pub async fn sign_out(&self) -> Result<AccountState, AccountError> {
        let _guard = self.account.transition.lock().await;
        if self.account.state().email.is_some() {
            let api = self.api.read().await.clone();
            api.logout().await?;
            self.clear_account().await?;
        }
        Ok(self.account.state())
    }
    // Called under the transition lock. Redemption is provisional until a saved
    // credential is confirmed; an orphan redemption cannot own future recordings.
    async fn install_account(&self, credential: Credential) -> Result<(), AccountError> {
        let mut current = self.api.write().await;
        if current.device_id != Some(credential.device_id) {
            return Err(error(
                K::Unauthorized,
                "Device changed during sign-in",
                true,
            ));
        }
        let generation = current.owner_generation.wrapping_add(1);
        let mut next = current.clone();
        next.set_account(Some(&credential.token), generation);
        if self.store.save_account(Some(&credential)).is_err() {
            drop(current);
            // Best-effort revoke the provisional session. Even if offline, it
            // remains unconfirmed and cannot assign subsequent recordings.
            let _ = next.logout().await;
            // Do not expose the unsaved token to account polling: it must never
            // confirm a credential that would disappear on restart.
            self.clear_account().await?;
            return Err(error(
                K::Storage,
                "Could not save account credential; retry sign-in",
                true,
            ));
        }
        *current = next.clone();
        self.account.set(Some(credential.email), false, None);
        let history_saved = self.history.bind_owner(current.owner_id());
        drop(current);
        history_saved.map_err(|_| error(K::Storage, "Could not save account history", true))?;
        if let Err(e) = self.confirm_account(&next).await {
            if e.kind == K::Unauthorized {
                self.clear_account().await?;
            }
            return Err(e);
        }
        Ok(())
    }

    pub async fn sign_in(
        self: &Arc<Self>,
        app: tauri::AppHandle,
    ) -> Result<AccountState, AccountError> {
        use tauri::Emitter;
        if self.account.state().signing_in {
            return Err(error(K::Unavailable, "Sign-in already in progress", true));
        }
        self.flush().await;
        let guard = self
            .account
            .transition
            .try_lock()
            .map_err(|_| error(K::Unavailable, "Account change already in progress", true))?;
        let old = self.account.state().email;
        self.account.set(old.clone(), true, None);
        let _ = app.emit("account-changed", self.account.state());
        let result = async {
            let api = self.api.read().await.clone();
            let device_id = api
                .device_id
                .ok_or_else(|| error(K::Unavailable, "Device registration required", true))?;
            let listener = TcpListener::bind(("127.0.0.1", 0))
                .map_err(|_| error(K::Unavailable, "Callback unavailable", true))?;
            let mut random = [0; 32];
            getrandom::fill(&mut random)
                .map_err(|_| error(K::Unavailable, "Secure random unavailable", true))?;
            let state = URL_SAFE_NO_PAD.encode(random);
            let url = api.login_url(listener.local_addr().unwrap().port(), &state);
            #[cfg(windows)]
            std::process::Command::new("explorer.exe")
                .arg(&url)
                .spawn()
                .map_err(|_| error(K::Unavailable, "Could not open browser", true))?;
            #[cfg(not(windows))]
            std::process::Command::new("xdg-open")
                .arg(&url)
                .spawn()
                .map_err(|_| error(K::Unavailable, "Could not open browser", true))?;
            let ticket = tauri::async_runtime::spawn_blocking(move || {
                receive(listener, &state, Duration::from_secs(300))
            })
            .await
            .map_err(|_| error(K::Other, "Sign-in interrupted", true))??;
            let session: DesktopSession = api.redeem(RedeemTicket { ticket }).await?;
            if URL_SAFE_NO_PAD
                .decode(&session.token)
                .ok()
                .is_none_or(|b| b.len() != 32)
                || session.email.trim().is_empty()
            {
                return Err(error(K::InvalidData, "Invalid account session", false));
            }
            let credential = Credential {
                device_id,
                token: session.token,
                email: session.email,
            };
            self.install_account(credential).await?;
            Ok(self.account.state())
        }
        .await;
        drop(guard);
        if result.is_ok() {
            self.flush().await;
            let refreshed = self.api.read().await.clone();
            if self.history.begin(true) {
                self.history.refresh(refreshed, true).await;
            }
        }
        if let Err(e) = &result {
            let email = self.account.state().email;
            self.account.set(email, false, Some(e.message.clone()));
        }
        let _ = app.emit("account-changed", self.account.state());
        if result.is_ok() {
            Ok(self.account.state())
        } else {
            result
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn callback_checks_identity() {
        let r =
            "GET /callback?app_state=secret&ticket=abc HTTP/1.1\r\nHost: 127.0.0.1:1234\r\n\r\n";
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
        assert!(
            matches!(receive(l,"state",Duration::from_millis(20)),Err(e)if e.kind==K::Cancelled)
        );
        assert!(start.elapsed() < Duration::from_secs(1));
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    #[test]
    fn listener_rejects_invalid_then_accepts_and_cancels() {
        for cancel in [false, true] {
            let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
            let port = listener.local_addr().unwrap().port();
            let worker =
                std::thread::spawn(move || receive(listener, "state", Duration::from_secs(2)));
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
        let store = crate::store::Store::new(dir.clone());
        let credential = Credential {
            device_id: [4; 16],
            token: "secret-session-token".into(),
            email: "test@example.invalid".into(),
        };
        store.save_account(Some(&credential)).unwrap();
        let loaded = crate::store::Store::new(dir.clone())
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
        let mut api = crate::uploader::ApiClient::new(None);
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
}

#[cfg(test)]
mod account_state_tests {
    use super::*;
    #[tokio::test]
    async fn pending_signin_state_poll_never_waits_for_browser() {
        let dir = std::env::temp_dir().join(format!("m4-pending-{}", std::process::id()));
        let delivery = Delivery::new(
            crate::recorder::Recorder::new(None, None, "test".into()),
            crate::uploader::ApiClient::new(None),
            crate::spool::Spool::new(dir.join("spool"), crate::spool::Spool::DEFAULT_MAX_BYTES),
            crate::store::Store::new(dir),
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
}

#[cfg(test)]
mod credential_transition_tests {
    use super::*;
    fn delivery(dir: std::path::PathBuf, base: String) -> Delivery {
        Delivery::new(
            crate::recorder::Recorder::new(None, None, "test".into()),
            crate::uploader::ApiClient::with_base_url(
                base,
                Some(&tutorlolv2_protocol::DeviceCredentials {
                    device_id: [1; 16],
                    device_secret: [2; 32],
                }),
            ),
            crate::spool::Spool::new(dir.join("spool"), crate::spool::Spool::DEFAULT_MAX_BYTES),
            crate::store::Store::new(dir),
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
                        crate::store::Store::new(dir.clone())
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
            let dir =
                std::env::temp_dir().join(format!("m4-rollback-{status}-{}", std::process::id()));
            let store = crate::store::Store::new(dir.clone());
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
            crate::store::Store::new(dir.clone())
                .load_account(Some([1; 16]))
                .is_some()
        );
        assert!(worker.join().unwrap()[0].starts_with("post /api/auth/desktop/confirm "));
        drop(d);
        let info = bincode::encode_to_vec(
            tutorlolv2_protocol::ApiResponse::new(
                200,
                Ok(tutorlolv2_protocol::AccountInfo {
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
        let store = crate::store::Store::new(dir.clone());
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
        crate::store::Store::new(dir.clone())
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
        let result =
            tokio::time::timeout(Duration::from_millis(100), d.cached_history(0, 50)).await;
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
        crate::store::Store::new(dir.clone())
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
}
