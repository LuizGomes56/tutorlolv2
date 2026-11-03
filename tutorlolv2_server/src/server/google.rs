//! Confidential Google web-client flow. Credentials never leave the server.
use super::{
    auth::DeviceAuth,
    codec::{Bincode, Response, respond},
    state::AppState,
};
use actix_web::cookie::{Cookie, SameSite, time::Duration as CookieDuration};
use actix_web::{
    FromRequest, HttpMessage, HttpRequest, HttpResponse,
    dev::{Payload, ServiceResponse},
    error::*,
    web::{Data, Query},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::Row;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Mutex,
    time::{Duration, Instant},
};
use tutorlolv2_protocol::{AccountInfo, DesktopSession, RedeemTicket};
use uuid::Uuid;
const MAX_PENDING: usize = 4096;
struct Login {
    target: Target,
    verifier: String,
    nonce: String,
    at: Instant,
}
enum Target {
    Web(String),
    Desktop(u16, String),
}
#[derive(Default)]
pub struct PendingStore {
    logins: Mutex<HashMap<String, Login>>,
    tickets: Mutex<HashMap<String, (Uuid, Instant)>>,
}
impl PendingStore {
    pub fn sweep(&self) {
        self.logins
            .lock()
            .unwrap()
            .retain(|_, v| v.at.elapsed() < Duration::from_secs(600));
        self.tickets
            .lock()
            .unwrap()
            .retain(|_, v| v.1.elapsed() < Duration::from_secs(120));
    }
}
fn token() -> Result<String, actix_web::Error> {
    let mut b = [0u8; 32];
    getrandom::fill(&mut b).map_err(|_| ErrorInternalServerError("Random source unavailable"))?;
    Ok(B64.encode(b))
}
fn hash(s: &str) -> Option<Vec<u8>> {
    let b = B64.decode(s).ok()?;
    if b.len() != 32 {
        return None;
    }
    Some(Sha256::digest(b).to_vec())
}
fn storage(_: sqlx::Error) -> actix_web::Error {
    ErrorInternalServerError("Account storage unavailable")
}
fn enabled(s: &AppState) -> actix_web::Result<(&str, &str)> {
    match (&s.config.google_client_id, &s.config.google_client_secret) {
        (Some(a), Some(b)) => Ok((a, b)),
        _ => Err(ErrorServiceUnavailable("Google sign-in unavailable")),
    }
}
fn cookie(s: &AppState, name: &str, value: String, seconds: i64) -> Cookie<'static> {
    Cookie::build(name.to_owned(), value)
        .path("/api")
        .http_only(true)
        .secure(s.config.cookie_secure)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::seconds(seconds))
        .finish()
}
fn redirect(url: String) -> HttpResponse {
    HttpResponse::Found()
        .insert_header(("Location", url))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}
fn return_path(p: Option<String>) -> String {
    p.filter(|p| {
        p.starts_with('/')
            && !p.starts_with("//")
            && !p.contains(['\\', '%', '\r', '\n', '\0'])
            && !p.chars().any(char::is_control)
    })
    .unwrap_or_else(|| "/history".into())
}
#[derive(Deserialize)]
pub struct Start {
    target: String,
    return_to: Option<String>,
    port: Option<u16>,
    app_state: Option<String>,
}
pub async fn start(s: Data<AppState>, q: Query<Start>) -> Response {
    let (client, _) = enabled(&s)?;
    let target = match q.target.as_str() {
        "web" => Target::Web(return_path(q.return_to.clone())),
        "desktop" => {
            let port = q
                .port
                .filter(|p| *p >= 1024)
                .ok_or_else(|| ErrorBadRequest("Invalid callback port"))?;
            let state = q
                .app_state
                .clone()
                .filter(|v| hash(v).is_some())
                .ok_or_else(|| ErrorBadRequest("Invalid app state"))?;
            Target::Desktop(port, state)
        }
        _ => return Err(ErrorBadRequest("Invalid target")),
    };
    let state = token()?;
    let nonce = token()?;
    let verifier = token()?;
    let mut url = reqwest::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
    url.query_pairs_mut().extend_pairs([
        ("response_type", "code"),
        ("client_id", client),
        (
            "redirect_uri",
            &format!("{}/api/auth/google/callback", s.config.api_public_base_url),
        ),
        ("scope", "openid email"),
        ("state", &state),
        ("nonce", &nonce),
        (
            "code_challenge",
            &B64.encode(Sha256::digest(verifier.as_bytes())),
        ),
        ("code_challenge_method", "S256"),
        ("prompt", "select_account"),
    ]);
    s.google.sweep();
    let mut pending = s.google.logins.lock().unwrap();
    if pending.len() >= MAX_PENDING {
        return Err(ErrorTooManyRequests("Sign-in busy"));
    }
    let web = matches!(target, Target::Web(_));
    pending.insert(
        state.clone(),
        Login {
            target,
            nonce,
            verifier,
            at: Instant::now(),
        },
    );
    let mut r = redirect(url.into());
    if web {
        r.add_cookie(&cookie(&s, "tl_login", state, 600))
            .map_err(ErrorInternalServerError)?;
    }
    Ok(r)
}
#[derive(Deserialize)]
pub struct Callback {
    state: String,
    code: Option<String>,
    error: Option<String>,
}
#[derive(Deserialize)]
struct TokenResponse {
    id_token: String,
}
#[derive(Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    exp: i64,
    nonce: String,
    sub: String,
    email: String,
}
pub async fn callback(s: Data<AppState>, r: HttpRequest, q: Query<Callback>) -> Response {
    let (client, secret) = enabled(&s)?;
    let login = s
        .google
        .logins
        .lock()
        .unwrap()
        .remove(&q.state)
        .filter(|v| v.at.elapsed() < Duration::from_secs(600));
    let Some(login) = login else {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body(
                r#"
                <!doctype html>
                    <html lang="en">
                        <meta charset="utf-8">
                        <title>Sign-in expired</title>
                        <p>Sign-in expired, try again from TutorLoL.</p>
                    </html>
                "#,
            ));
    };
    if matches!(login.target, Target::Web(_))
        && r.cookie("tl_login")
            .map(|c| c.value().to_owned())
            .as_deref()
            != Some(&q.state)
    {
        return Err(ErrorBadRequest("Sign-in browser mismatch"));
    }
    if q.error.is_some() {
        return Ok(match login.target {
            Target::Web(path) => redirect(format!(
                "{}{}{}signin=cancelled",
                s.config.public_base_url,
                path,
                if path.contains('?') { "&" } else { "?" }
            )),
            Target::Desktop(port, state) => redirect(format!(
                "http://127.0.0.1:{port}/callback?error=cancelled&app_state={state}"
            )),
        });
    }
    let code = q
        .code
        .as_ref()
        .filter(|v| v.len() <= 4096 && !v.is_empty())
        .ok_or_else(|| ErrorBadRequest("Missing code"))?;
    let client_http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| ErrorBadGateway("Google unavailable"))?;
    let endpoint = s
        .config
        .google_token_endpoint
        .as_deref()
        .unwrap_or("https://oauth2.googleapis.com/token");
    let mut response = client_http
        .post(endpoint)
        .form(&[
            ("code", code.as_str()),
            ("client_id", client),
            ("client_secret", secret),
            ("code_verifier", &login.verifier),
            (
                "redirect_uri",
                &format!("{}/api/auth/google/callback", s.config.api_public_base_url),
            ),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|_| ErrorBadGateway("Google unavailable"))?;
    if !response.status().is_success() {
        return Err(ErrorBadGateway("Google rejected sign-in"));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| ErrorBadGateway("Google unavailable"))?
    {
        if body.len() + chunk.len() > 65536 {
            return Err(ErrorBadGateway("Invalid Google response"));
        }
        body.extend_from_slice(&chunk)
    }
    let tr: TokenResponse =
        serde_json::from_slice(&body).map_err(|_| ErrorBadGateway("Invalid Google response"))?;
    // This token is accepted only from our confidential server-to-Google HTTPS
    // exchange, never from a client-supplied JWT. Google's OIDC guide permits
    // claim verification without JWKS/signature checks at this trust boundary.
    // The explicit trusted endpoint override exists only for local test servers.
    let parts: Vec<_> = tr.id_token.split('.').collect();
    if parts.len() != 3 {
        return Err(ErrorUnauthorized("Invalid identity"));
    }
    let c: Claims = serde_json::from_slice(
        &B64.decode(parts[1])
            .map_err(|_| ErrorUnauthorized("Invalid identity"))?,
    )
    .map_err(|_| ErrorUnauthorized("Invalid identity"))?;
    if !["accounts.google.com", "https://accounts.google.com"].contains(&c.iss.as_str())
        || c.aud != client
        || c.exp <= chrono::Utc::now().timestamp()
        || c.nonce != login.nonce
        || c.sub.trim().is_empty()
        || c.email.trim().is_empty()
        || c.sub.len() > 255
        || c.email.len() > 320
    {
        return Err(ErrorUnauthorized("Invalid identity"));
    }
    let account = sqlx::query_scalar!(
        "INSERT INTO accounts(google_sub, email)
        VALUES($1, $2)
        ON CONFLICT(google_sub)
        DO UPDATE
        SET email = EXCLUDED.email, last_login_at = now()
        WHERE accounts.deleted_at IS NULL
        RETURNING id",
        c.sub,
        c.email
    )
    .fetch_optional(&s.db)
    .await
    .map_err(storage)?
    .ok_or_else(|| ErrorUnauthorized("Account unavailable"))?;
    match login.target {
        Target::Web(path) => {
            let mut tx = s.db.begin().await.map_err(storage)?;
            let (t, _) = create_session(&s, &mut tx, account, None).await?;
            tx.commit().await.map_err(storage)?;
            let mut r = redirect(format!("{}{}", s.config.public_base_url, path));
            r.add_cookie(&cookie(
                &s,
                "tl_session",
                t,
                s.config.session_ttl_days * 86400,
            ))
            .map_err(ErrorInternalServerError)?;
            r.add_cookie(&cookie(&s, "tl_login", String::new(), 0))
                .map_err(ErrorInternalServerError)?;
            Ok(r)
        }
        Target::Desktop(port, state) => {
            s.google.sweep();
            let mut tickets = s.google.tickets.lock().unwrap();
            if tickets.len() >= MAX_PENDING {
                return Err(ErrorTooManyRequests("Sign-in busy"));
            }
            let t = token()?;
            tickets.insert(t.clone(), (account, Instant::now()));
            Ok(redirect(format!(
                "http://127.0.0.1:{port}/callback?ticket={t}&app_state={state}"
            )))
        }
    }
}
async fn create_session(
    s: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    device: Option<Uuid>,
) -> actix_web::Result<(String, Uuid)> {
    let t = token()?;

    let id = sqlx::query_scalar!(
        "INSERT INTO account_sessions(
            account_id,
            token_hash,
            kind,
            device_id,
            expires_at
        )
        VALUES(
            $1, $2, CAST($3::text AS session_kind),
            $4, now() + $5 * interval '1 day'
        )
        RETURNING id",
        account,
        hash(&t).unwrap(),
        if device.is_some() { "desktop" } else { "web" },
        device,
        s.config.session_ttl_days as f64
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(storage)?;
    Ok((t, id))
}
pub async fn redeem(
    s: Data<AppState>,
    auth: DeviceAuth,
    Bincode(q): Bincode<RedeemTicket>,
) -> Response {
    enabled(&s)?;
    let (account, at) = s
        .google
        .tickets
        .lock()
        .unwrap()
        .remove(&q.ticket)
        .ok_or_else(|| ErrorGone("Ticket expired or used"))?;
    if at.elapsed() >= Duration::from_secs(120) {
        return Err(ErrorGone("Ticket expired or used"));
    }
    let mut tx = s.db.begin().await.map_err(storage)?;
    // NO KEY UPDATE protects deletion/identity changes without blocking the
    // account FK KEY SHARE taken by a concurrent recording creation. The device
    // lock below is shared with start_session/logout ownership transitions.
    let email: String = sqlx::query_scalar(
        "SELECT email FROM accounts WHERE id=$1 AND deleted_at IS NULL FOR NO KEY UPDATE",
    )
    .bind(account)
    .fetch_optional(&mut *tx)
    .await
    .map_err(storage)?
    .ok_or_else(|| ErrorUnauthorized("Account unavailable"))?;
    let device: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM devices WHERE id=$1 AND deleted_at IS NULL FOR UPDATE")
            .bind(auth.device_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?;
    if device.is_none() {
        return Err(ErrorUnauthorized("Device unavailable"));
    }
    let (t, id) = create_session(&s, &mut tx, account, Some(auth.device_id)).await?;
    sqlx::query("UPDATE devices SET account_id=$1,account_session_id=$2 WHERE id=$3")
        .bind(account)
        .bind(id)
        .bind(auth.device_id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    sqlx::query("UPDATE recordings SET account_id=$1 WHERE device_id=$2 AND account_id IS NULL")
        .bind(account)
        .bind(auth.device_id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    respond(DesktopSession { token: t, email })
}
/// Acknowledge durable native credential storage. Device locking serializes
/// this transition with redeem, logout and recording creation.
pub async fn confirm_desktop(s: Data<AppState>, a: AccountAuth) -> Response {
    let device = a
        .device_id
        .filter(|_| !a.cookie)
        .ok_or_else(|| ErrorUnauthorized("Desktop session required"))?;
    let mut tx = s.db.begin().await.map_err(storage)?;
    let current: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM devices WHERE id=$1 AND deleted_at IS NULL FOR UPDATE")
            .bind(device)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?;
    if current.is_none() {
        return Err(ErrorUnauthorized("Device unavailable"));
    }
    // Recheck after acquiring the lock: the extractor may have authenticated
    // before a concurrent login/logout replaced the device binding.
    let updated = sqlx::query(
        "UPDATE account_sessions s
         SET desktop_confirmed_at=COALESCE(s.desktop_confirmed_at,clock_timestamp())
         WHERE s.id=$1 AND s.account_id=$2 AND s.device_id=$3 AND s.kind='desktop'
           AND s.revoked_at IS NULL AND s.expires_at>clock_timestamp()
           AND EXISTS (SELECT 1 FROM devices d WHERE d.id=$3
             AND d.account_session_id=s.id AND d.account_id=s.account_id AND d.deleted_at IS NULL)
           AND EXISTS (SELECT 1 FROM accounts a WHERE a.id=s.account_id AND a.deleted_at IS NULL)",
    )
    .bind(a.session_id)
    .bind(a.account_id)
    .bind(device)
    .execute(&mut *tx)
    .await
    .map_err(storage)?;
    if updated.rows_affected() != 1 {
        return Err(ErrorUnauthorized("Desktop binding changed"));
    }
    sqlx::query("UPDATE recordings SET account_id=$1 WHERE device_id=$2 AND account_id IS NULL")
        .bind(a.account_id)
        .bind(device)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    Ok(HttpResponse::NoContent().finish())
}
#[derive(Clone, Copy)]
pub struct AccountAuth {
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub device_id: Option<Uuid>,
    pub cookie: bool,
}
impl FromRequest for AccountAuth {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(r: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = r.app_data::<Data<AppState>>().cloned();
        let request = r.clone();
        let header = r
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());
        let (cookie, t) = match header {
            Some(h) => (false, h.strip_prefix("Session ").map(str::to_owned)),
            None => (true, r.cookie("tl_session").map(|v| v.value().to_owned())),
        };
        let csrf = !cookie
            || r.method().is_safe()
            || (r.headers().get("Origin").and_then(|v| v.to_str().ok())
                == state.as_ref().map(|s| s.config.public_base_url.as_str())
                && r.headers()
                    .get("Content-Type")
                    .and_then(|v| v.to_str().ok())
                    == Some("application/octet-stream"));
        Box::pin(async move {
            let s = state.ok_or_else(|| ErrorUnauthorized("Signed out"))?;
            let hash = t
                .as_deref()
                .and_then(hash)
                .ok_or_else(|| ErrorUnauthorized("Signed out"))?;
            if !csrf {
                return Err(ErrorForbidden("Invalid request origin or content type"));
            }
            let row=sqlx::query("SELECT s.id,s.account_id,s.device_id FROM account_sessions s JOIN accounts a ON a.id=s.account_id LEFT JOIN devices d ON d.id=s.device_id WHERE s.token_hash=$1 AND s.kind=CAST($2 AS session_kind) AND s.revoked_at IS NULL AND s.expires_at>now() AND a.deleted_at IS NULL AND (s.device_id IS NULL OR d.deleted_at IS NULL AND d.id IS NOT NULL)").bind(hash).bind(if cookie{"web"}else{"desktop"}).fetch_optional(&s.db).await.map_err(storage)?.ok_or_else(||ErrorUnauthorized("Session expired"))?;
            let auth = AccountAuth {
                account_id: row.get("account_id"),
                session_id: row.get("id"),
                device_id: row.get("device_id"),
                cookie,
            };
            if cookie {
                // The response middleware renews both DB expiry and cookie only
                // after a successful owner response. Failed requests and logout
                // must neither consume this daily renewal nor resurrect a cookie.
                request.extensions_mut().insert(WebSessionRenewal {
                    session_id: auth.session_id,
                    token: t.unwrap(),
                });
            } else {
                sqlx::query("UPDATE account_sessions SET last_used_at=now(),expires_at=now()+$2*interval '1 day' WHERE id=$1 AND last_used_at<now()-interval '1 day'").bind(auth.session_id).bind(s.config.session_ttl_days as f64).execute(&s.db).await.map_err(storage)?;
            }
            Ok(auth)
        })
    }
}
// This native server-only marker has no Debug/Serde implementation. It never
// enters a response body or a webview; only the HttpOnly cookie carries it.
#[derive(Clone)]
struct WebSessionRenewal {
    session_id: Uuid,
    token: String,
}

pub(super) async fn renew_web_cookie<B>(
    response: &mut ServiceResponse<B>,
) -> actix_web::Result<()> {
    if !response.status().is_success() || response.request().path() == "/api/auth/logout" {
        return Ok(());
    }
    let renewal = response
        .request()
        .extensions()
        .get::<WebSessionRenewal>()
        .cloned();
    let Some(renewal) = renewal else {
        return Ok(());
    };
    let state = response
        .request()
        .app_data::<Data<AppState>>()
        .cloned()
        .ok_or_else(|| ErrorInternalServerError("Account storage unavailable"))?;
    let updated = sqlx::query(
        "UPDATE account_sessions s SET last_used_at=now(),expires_at=now()+$2*interval '1 day'
         WHERE s.id=$1 AND s.kind='web' AND s.revoked_at IS NULL AND s.expires_at>now()
           AND s.last_used_at<now()-interval '1 day'
           AND EXISTS (SELECT 1 FROM accounts a WHERE a.id=s.account_id AND a.deleted_at IS NULL)",
    )
    .bind(renewal.session_id)
    .bind(state.config.session_ttl_days as f64)
    .execute(&state.db)
    .await
    .map_err(storage)?;
    if updated.rows_affected() == 1 {
        response
            .response_mut()
            .add_cookie(&cookie(
                &state,
                "tl_session",
                renewal.token,
                state.config.session_ttl_days * 86400,
            ))
            .map_err(ErrorInternalServerError)?;
    }
    Ok(())
}

pub async fn account(s: Data<AppState>, a: AccountAuth) -> Response {
    let row =
        sqlx::query("SELECT email,created_at FROM accounts WHERE id=$1 AND deleted_at IS NULL")
            .bind(a.account_id)
            .fetch_one(&s.db)
            .await
            .map_err(storage)?;
    respond(AccountInfo {
        email: row.get("email"),
        created_at_ms: row
            .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .timestamp_millis(),
    })
}
pub async fn logout(s: Data<AppState>, a: AccountAuth) -> Response {
    let mut tx = s.db.begin().await.map_err(storage)?;
    if let Some(d) = a.device_id {
        sqlx::query("UPDATE devices SET account_id=NULL,account_session_id=NULL WHERE id=$1 AND account_session_id=$2").bind(d).bind(a.session_id).execute(&mut *tx).await.map_err(storage)?;
    }
    sqlx::query("UPDATE account_sessions SET revoked_at=now() WHERE id=$1")
        .bind(a.session_id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    let mut r = HttpResponse::NoContent().finish();
    if a.cookie {
        r.add_cookie(&cookie(&s, "tl_session", String::new(), 0))
            .map_err(ErrorInternalServerError)?;
    }
    Ok(r)
}
#[derive(Clone, Copy)]
pub struct Owner {
    pub device_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
}
impl From<DeviceAuth> for Owner {
    fn from(a: DeviceAuth) -> Self {
        Self {
            device_id: Some(a.device_id),
            account_id: None,
        }
    }
}
impl FromRequest for Owner {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(r: &HttpRequest, p: &mut Payload) -> Self::Future {
        if r.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|h| h.starts_with("Device "))
        {
            let f = DeviceAuth::from_request(r, p);
            Box::pin(async move { f.await.map(Into::into) })
        } else {
            let f = AccountAuth::from_request(r, p);
            Box::pin(async move {
                let a = f.await?;
                Ok(Self {
                    device_id: None,
                    account_id: Some(a.account_id),
                })
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut config = super::super::config::Config::from_lookup(|key| {
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
        assert_eq!(result.unwrap_err().as_response_error().status_code(), 410);
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
        assert_eq!(result.unwrap_err().as_response_error().status_code(), 429);
    }
}
