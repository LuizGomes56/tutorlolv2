use crate::server::response::{AppError, AppResult, Trace};
use crate::server::state::AppState;
use actix_web::http::StatusCode;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Query},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use std::time::{Duration, Instant};

use crate::server::google::*;

pub async fn callback(
    s: Data<AppState>,
    r: HttpRequest,
    q: Query<Callback>,
) -> Result<HttpResponse, crate::server::response::BrowserError> {
    use crate::server::response::BrowserContext;
    let outcome: AppResult<HttpResponse> = async move {
        let (client, secret) = s.google_credentials().trace()?;
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
            return Err(AppError::bad_request("Sign-in browser mismatch"));
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
            .trace_ctx(("Missing code", StatusCode::BAD_REQUEST))?;

        let client_http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .trace_ctx(("Google unavailable", StatusCode::BAD_GATEWAY))?;

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
            .trace_ctx(("Google unavailable", StatusCode::BAD_GATEWAY))?;

        if !response.status().is_success() {
            return Err(AppError::bad_gateway("Google rejected sign-in"));
        }

        let mut body = Vec::new();

        while let Some(chunk) = response
            .chunk()
            .await
            .trace_ctx(("Google unavailable", StatusCode::BAD_GATEWAY))?
        {
            if body.len() + chunk.len() > 65536 {
                return Err(AppError::bad_gateway("Invalid Google response"));
            }

            body.extend_from_slice(&chunk)
        }

        let tr = serde_json::from_slice::<TokenResponse>(&body)
            .trace_ctx(("Invalid Google response", StatusCode::BAD_GATEWAY))?;

        // This token is accepted only from our confidential server-to-Google HTTPS
        // exchange, never from a client-supplied JWT. Google's OIDC guide permits
        // claim verification without JWKS/signature checks at this trust boundary.
        // The explicit trusted endpoint override exists only for local test servers.
        let parts = tr.id_token.split('.').collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(AppError::unauthorized("Invalid identity"));
        }

        let c = serde_json::from_slice::<Claims>(
            &B64.decode(parts[1])
                .trace_ctx(("Invalid identity", StatusCode::UNAUTHORIZED))?,
        )
        .trace_ctx(("Invalid identity", StatusCode::UNAUTHORIZED))?;

        if !["accounts.google.com", "https://accounts.google.com"].contains(&c.iss.as_str())
            || c.aud != client
            || c.exp <= chrono::Utc::now().timestamp()
            || c.nonce != login.nonce
            || c.sub.trim().is_empty()
            || c.email.trim().is_empty()
            || c.sub.len() > 255
            || c.email.len() > 320
        {
            return Err(AppError::unauthorized("Invalid identity"));
        }

        let account = sqlx::query_scalar!(
            r#"
        INSERT INTO accounts (
            google_sub,
            email
        ) VALUES (
            $1,
            $2
        )
        ON CONFLICT(google_sub) DO
                UPDATE
                SET email = EXCLUDED.email,
                    last_login_at = now()
                WHERE accounts.deleted_at IS NULL RETURNING id
        "#,
            c.sub,
            c.email,
        )
        .fetch_optional(&s.db)
        .await
        .trace_ctx("Account storage unavailable")?
        .trace_ctx(("Account unavailable", StatusCode::UNAUTHORIZED))?;

        match login.target {
            Target::Web(path) => {
                let mut tx =
                    s.db.begin()
                        .await
                        .trace_ctx("Account storage unavailable")?;
                let (t, _) = (GoogleAuthService { state: &s })
                    .create_session(&mut tx, account, None)
                    .await
                    .trace()?;

                tx.commit().await.trace_ctx("Account storage unavailable")?;

                let mut r = redirect(format!("{}{}", s.config.public_base_url, path));

                r.add_cookie(&s.auth_cookie("tl_session", t, s.config.session_ttl_days * 86400))
                    .trace()?;

                r.add_cookie(&s.auth_cookie("tl_login", String::new(), 0))
                    .trace()?;

                Ok(r)
            }
            Target::Desktop(port, state) => {
                s.google.sweep();

                let mut tickets = s.google.tickets.lock().unwrap();

                if tickets.len() >= MAX_PENDING {
                    return Err(AppError::rate_limited("Sign-in busy"));
                }

                let t = token().trace()?;
                tickets.insert(t.clone(), (account, Instant::now()));

                Ok(redirect(format!(
                    "http://127.0.0.1:{port}/callback?ticket={t}&app_state={state}"
                )))
            }
        }
    }
    .await;
    outcome.browser()
}

api_contract! { pub struct GoogleCallbackQuery { pub state: String, pub code: Option<String>, pub error: Option<String> } }

api_endpoint! {
    pub GoogleCallback {
        handler: callback, method: GET, path: "/api/auth/google/callback",
        path_params: {  }, query: form(GoogleCallbackQuery), body: none,
        response: browser, success: [200,302,400], auth: browser,
        request_headers: [], response_headers: ["Location", "Set-Cookie", "Cache-Control"],
    }
}
