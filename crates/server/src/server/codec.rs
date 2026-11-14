use super::response::Trace;
// Bincode request/response helpers shared by every `/api` handler.

use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Bytes};
use bincode::{Decode, config::Config};
use std::{
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};

/// An owned bincode request body, using the shared configuration and Actix payload limit.
#[derive(Debug)]
pub struct Bincode<T>(pub T);

impl<T> Bincode<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Bincode<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Bincode<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Decode<()> + 'static> FromRequest for Bincode<T> {
    type Error = super::response::AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let body = Bytes::from_request(req, payload);

        Box::pin(async move { decode_request(&body.await.trace()?).map(Self) })
    }
}

pub const OCTET_STREAM: &str = "application/octet-stream";

pub fn request_config() -> impl Config {
    types::BINCODE_CONFIG.with_limit::<1_048_576>()
}

fn decode_request<T: Decode<()>>(body: &[u8]) -> super::response::AppResult<T> {
    let (value, consumed) = bincode::decode_from_slice(body, request_config()).trace_ctx((
        "Invalid request body",
        actix_web::http::StatusCode::BAD_REQUEST,
    ))?;

    if consumed != body.len() {
        return Err(super::response::AppError::bad_request(
            "Unexpected trailing request bytes",
        ));
    }

    Ok(value)
}
