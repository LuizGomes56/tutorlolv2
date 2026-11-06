//! Requests to the HTTP API, for builds without `local-calculation` (the web app's
//! `WebClient::public`). Blocking: callers run them on a background thread.
use std::io::Read;
use tutorlolv2_api::{
    Endpoint, NoHeaders, NoPath, NoQuery, ResponseBuffer, ResponseHeaders,
    protocol::response::MAX_RESPONSE_BYTES,
};

pub type Error = Box<dyn core::error::Error + Send + Sync>;

/// The API origin, set at compile time.
fn base() -> &'static str {
    option_env!("TUTORLOLV2_API_URL")
        .or(option_env!("TUTORLOL_API_URL"))
        .unwrap_or("http://127.0.0.1:8082")
        .trim_end_matches('/')
        .trim_end_matches("/games")
}

/// Posts `body` to `E` and returns the response, ready to decode.
pub fn send<E>(body: &E::Body<'_>) -> Result<ResponseBuffer<E>, Error>
where
    E: Endpoint<Path = NoPath, Query = NoQuery, Headers = NoHeaders>,
{
    let parts = E::request(base(), &NoPath, &NoQuery, &NoHeaders, body)?;

    let mut response = ureq::post(&parts.url)
        .header("Content-Type", "application/octet-stream")
        .header("Accept", "application/octet-stream")
        .config()
        // Error statuses carry an encoded `ApiError`, decoded by the contract.
        .http_status_as_error(false)
        .build()
        .send(&parts.body)?;

    let status = response.status().as_u16();
    let mut headers = ResponseHeaders::default();

    for name in E::RESPONSE_HEADERS {
        if let Some(value) = response.headers().get(*name).and_then(|v| v.to_str().ok()) {
            headers.0.push(((*name).into(), value.into()));
        }
    }

    // ureq decodes `Content-Encoding` (gzip and Brotli).
    let mut bytes = Vec::new();

    response
        .body_mut()
        .as_reader()
        .take(MAX_RESPONSE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;

    Ok(ResponseBuffer::<E>::new(status, headers, bytes)?)
}
