//! Bincode request/response helpers shared by every `/api` handler.

use actix_web::{
    FromRequest, HttpRequest, HttpResponse,
    dev::Payload,
    error::{ErrorBadRequest, ErrorInternalServerError},
    web::Bytes,
};
use bincode::{Decode, Encode, config::Config};
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
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let body = Bytes::from_request(req, payload);

        Box::pin(async move { decode_request(&body.await?).map(Self) })
    }
}

pub type Response = actix_web::Result<HttpResponse>;
pub const OCTET_STREAM: &str = "application/octet-stream";

pub fn request_config() -> impl Config {
    tutorlolv2_types::BINCODE_CONFIG.with_limit::<1_048_576>()
}

pub fn respond(data: impl Encode) -> Response {
    let bytes = bincode::encode_to_vec(data, tutorlolv2_types::BINCODE_CONFIG)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type(OCTET_STREAM).body(bytes))
}

fn decode_request<T: Decode<()>>(body: &[u8]) -> actix_web::Result<T> {
    let (value, consumed) =
        bincode::decode_from_slice(body, request_config()).map_err(ErrorBadRequest)?;

    if consumed != body.len() {
        return Err(ErrorBadRequest("Unexpected trailing request bytes"));
    }

    Ok(value)
}
