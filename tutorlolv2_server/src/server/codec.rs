//! Bincode request/response helpers shared by every `/api` handler.

use actix_web::{
    HttpResponse,
    error::{ErrorBadRequest, ErrorInternalServerError},
};
use bincode::{Decode, Encode, config::Config};

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

pub fn decode_request<T: Decode<()>>(body: &[u8]) -> actix_web::Result<T> {
    let (value, consumed) =
        bincode::decode_from_slice(body, request_config()).map_err(ErrorBadRequest)?;

    if consumed != body.len() {
        return Err(ErrorBadRequest("Unexpected trailing request bytes"));
    }

    Ok(value)
}
