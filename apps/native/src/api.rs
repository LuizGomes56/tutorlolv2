//! Requests to the HTTP API, for builds without `local-calculation` (the web app's
//! `WebClient::public`). Blocking: callers run them on a background thread.
use ::api::{
    Endpoint, NoHeaders, NoPath, NoQuery, ResponseBuffer, ResponseHeaders,
    protocol::response::MAX_RESPONSE_BYTES,
};
use std::{io::Read, sync::LazyLock};

pub type Error = Box<dyn core::error::Error + Send + Sync>;

/// The API origin, set at compile time.
fn base() -> &'static str {
    option_env!("TUTORLOLV2_API_URL")
        .or(option_env!("TUTORLOL_API_URL"))
        .unwrap_or("http://127.0.0.1:8082")
        .trim_end_matches('/')
        .trim_end_matches("/games")
}

/// Shared so requests reuse connections. No decompression features: these
/// endpoints answer with raw bincode.
static CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(reqwest::blocking::Client::new);

/// Posts `body` to `E` and returns the response, ready to decode.
pub fn send<E>(body: &E::Body<'_>) -> Result<ResponseBuffer<E>, Error>
where
    E: Endpoint<Path = NoPath, Query = NoQuery, Headers = NoHeaders>,
{
    let parts = E::request(base(), &NoPath, &NoQuery, &NoHeaders, body)?;

    // Error statuses are not errors here: they carry an encoded `ApiError`, decoded
    // by the contract.
    let response = CLIENT
        .post(&parts.url)
        .header("Content-Type", "application/octet-stream")
        .header("Accept", "application/octet-stream")
        .body(parts.body)
        .send()?;

    let status = response.status().as_u16();
    let mut headers = ResponseHeaders::default();

    for name in E::RESPONSE_HEADERS {
        if let Some(value) = response.headers().get(*name).and_then(|v| v.to_str().ok()) {
            headers.0.push(((*name).into(), value.into()));
        }
    }

    let mut bytes = Vec::new();

    response
        .take(MAX_RESPONSE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;

    Ok(ResponseBuffer::<E>::new(status, headers, bytes)?)
}
