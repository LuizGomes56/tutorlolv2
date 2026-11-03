use crate::server::{codec::Response, state::AppState};
use actix_web::{
    error::*,
    web::{Data, Query},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use sha2::{Digest, Sha256};
use std::time::Instant;

use crate::server::google::*;

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

api_contract! { pub struct GoogleStartQuery { pub target: String, pub return_to: Option<String>, pub port: Option<u16>, pub app_state: Option<String> } }

api_endpoint! {
    pub GoogleStart {
        handler: start, method: GET, path: "/api/auth/google/start",
        path_params: {  }, query: form(GoogleStartQuery), body: none,
        response: browser, success: [302], auth: browser,
        request_headers: [], response_headers: ["Location", "Set-Cookie", "Cache-Control"],
    }
}
