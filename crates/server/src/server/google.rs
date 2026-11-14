use crate::server::response::{AppError, AppResult, Trace};
use actix_web::http::StatusCode;
// Confidential Google web-client flow. Credentials never leave the server.
use super::{auth::DeviceAuth, db_enums::SessionKind, state::AppState};
use actix_web::cookie::{Cookie, SameSite, time::Duration as CookieDuration};
use actix_web::{
    FromRequest, HttpMessage, HttpRequest, HttpResponse,
    dev::{Payload, ServiceResponse},
    web::Data,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Mutex,
    time::{Duration, Instant},
};
use uuid::Uuid;

pub use super::routes::{
    account::account,
    auth::{
        desktop_confirm::confirm_desktop, desktop_redeem::redeem, google_callback::callback,
        google_start::start, logout::logout,
    },
};
pub use ::api::GoogleCallbackQuery as Callback;
pub use ::api::GoogleStartQuery as Start;

pub const MAX_PENDING: usize = 4096;

pub struct Login {
    pub target: Target,
    pub verifier: String,
    pub nonce: String,
    pub at: Instant,
}

pub enum Target {
    Web(String),
    Desktop(u16, String),
}

#[derive(Default)]
pub struct PendingStore {
    pub logins: Mutex<HashMap<String, Login>>,
    pub tickets: Mutex<HashMap<String, (Uuid, Instant)>>,
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

pub fn token() -> AppResult<String> {
    let mut b = [0u8; 32];

    getrandom::fill(&mut b).trace_ctx((
        "Random source unavailable",
        StatusCode::INTERNAL_SERVER_ERROR,
    ))?;

    Ok(B64.encode(b))
}

pub fn hash(s: &str) -> Option<Vec<u8>> {
    let b = B64.decode(s).ok()?;

    if b.len() != 32 {
        return None;
    }

    Some(Sha256::digest(b).to_vec())
}

impl AppState {
    /// The configured Google OAuth client id and secret, or 503 when sign-in is disabled.
    pub(crate) fn google_credentials(&self) -> AppResult<(&str, &str)> {
        match (
            &self.config.google_client_id,
            &self.config.google_client_secret,
        ) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(AppError::unavailable("Google sign-in unavailable")),
        }
    }

    /// An HTTP-only `/api` cookie honouring the configured `Secure` policy.
    pub(crate) fn auth_cookie(&self, name: &str, value: String, seconds: i64) -> Cookie<'static> {
        Cookie::build(name.to_owned(), value)
            .path("/api")
            .http_only(true)
            .secure(self.config.cookie_secure)
            .same_site(SameSite::Lax)
            .max_age(CookieDuration::seconds(seconds))
            .finish()
    }
}

pub(crate) fn redirect(url: String) -> HttpResponse {
    HttpResponse::Found()
        .insert_header(("Location", url))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

pub fn return_path(p: Option<String>) -> String {
    p.filter(|p| {
        p.starts_with('/')
            && !p.starts_with("//")
            && !p.contains(['\\', '%', '\r', '\n', '\0'])
            && !p.chars().any(char::is_control)
    })
    .unwrap_or_else(|| "/history".into())
}

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) id_token: String,
}

#[derive(Deserialize)]
pub(crate) struct Claims {
    pub(crate) iss: String,
    pub(crate) aud: String,
    pub(crate) exp: i64,
    pub(crate) nonce: String,
    pub(crate) sub: String,
    pub(crate) email: String,
}

/// Acknowledge durable native credential storage. Device locking serializes
/// this transition with redeem, logout and recording creation.

#[derive(Clone, Copy)]
pub struct AccountAuth {
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub device_id: Option<Uuid>,
    pub cookie: bool,
}

impl FromRequest for AccountAuth {
    type Error = AppError;
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
            let s = state.trace_ctx(("Signed out", StatusCode::UNAUTHORIZED))?;
            let hash = t
                .as_deref()
                .and_then(hash)
                .trace_ctx(("Signed out", StatusCode::UNAUTHORIZED))?;

            if !csrf {
                return Err(AppError::forbidden(
                    "Invalid request origin or content type",
                ));
            }

            let row = sqlx::query!(
                r#"
                SELECT s.id,
                    s.account_id,
                    s.device_id
                FROM account_sessions s
                JOIN accounts a ON a.id = s.account_id
                LEFT JOIN devices d ON d.id = s.device_id
                WHERE s.token_hash = $1
                    AND s.kind = $2
                    AND s.revoked_at IS NULL
                    AND s.expires_at > now()
                    AND a.deleted_at IS NULL
                    AND (s.device_id IS NULL
                         OR d.deleted_at IS NULL
                         AND d.id IS NOT NULL)
                "#,
                hash,
                if cookie {
                    SessionKind::Web
                } else {
                    SessionKind::Desktop
                },
            )
            .fetch_optional(&s.db)
            .await
            .trace_ctx("Account storage unavailable")?
            .trace_ctx(("Session expired", StatusCode::UNAUTHORIZED))?;

            let auth = AccountAuth {
                account_id: row.account_id,
                session_id: row.id,
                device_id: row.device_id,
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
                sqlx::query!(
                    r#"
                    UPDATE account_sessions
                    SET last_used_at = now(),
                        expires_at = now() + $2 * interval '1 day'
                    WHERE id = $1
                        AND last_used_at < now() - interval '1 day'
                    "#,
                    auth.session_id,
                    s.config.session_ttl_days as f64,
                )
                .execute(&s.db)
                .await
                .trace_ctx("Account storage unavailable")?;
            }

            Ok(auth)
        })
    }
}

// This native server-only marker has no Debug/Serde implementation. It never
// enters a response body or a webview; only the HttpOnly cookie carries it.
#[derive(Clone)]
pub(crate) struct WebSessionRenewal {
    pub(crate) session_id: Uuid,
    pub(crate) token: String,
}

pub(crate) async fn renew_web_cookie<B>(response: &mut ServiceResponse<B>) -> AppResult<()> {
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
        .trace_ctx((
            "Account storage unavailable",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

    let updated = sqlx::query!(
        r#"
        UPDATE account_sessions s
        SET last_used_at = now(),
            expires_at = now() + $2 * interval '1 day'
        WHERE s.id = $1
            AND s.kind = 'Web'
            AND s.revoked_at IS NULL
            AND s.expires_at > now()
            AND s.last_used_at < now() - interval '1 day'
            AND EXISTS
                (SELECT 1
                 FROM accounts a
                 WHERE a.id = s.account_id
                     AND a.deleted_at IS NULL)
        "#,
        renewal.session_id,
        state.config.session_ttl_days as f64,
    )
    .execute(&state.db)
    .await
    .trace_ctx("Account storage unavailable")?;

    if updated.rows_affected() == 1 {
        response
            .response_mut()
            .add_cookie(&state.auth_cookie(
                "tl_session",
                renewal.token,
                state.config.session_ttl_days * 86400,
            ))
            .trace()?;
    }

    Ok(())
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
    type Error = AppError;
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
                let a = f.await.trace()?;
                Ok(Self {
                    device_id: None,
                    account_id: Some(a.account_id),
                })
            })
        }
    }
}

pub(crate) struct GoogleAuthService<'a> {
    pub(crate) state: &'a AppState,
}

impl GoogleAuthService<'_> {
    pub(crate) async fn create_session(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        account: Uuid,
        device: Option<Uuid>,
    ) -> AppResult<(String, Uuid)> {
        let s = self.state;
        let t = token().trace()?;

        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO account_sessions (
                account_id,
                token_hash,
                kind,
                device_id,
                expires_at
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                now() + $5 * interval '1 day'
            )
            RETURNING id
            "#,
            account,
            hash(&t).unwrap(),
            if device.is_some() {
                SessionKind::Desktop
            } else {
                SessionKind::Web
            },
            device,
            s.config.session_ttl_days as f64,
        )
        .fetch_one(&mut **tx)
        .await
        .trace_ctx("Account storage unavailable")?;

        Ok((t, id))
    }
}
