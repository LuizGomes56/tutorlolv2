use crate::server::codec::{Response, request_config, respond};
use actix_web::{error::ErrorBadRequest, post, web::Bytes};
use tutorlolv2::riot::RiotRealtime;

#[post("/realtime")]
pub async fn realtime_handler(body: Bytes) -> Response {
    let (game, consumed) =
        bincode::borrow_decode_from_slice::<RiotRealtime<'_>, _>(&body, request_config())
            .map_err(ErrorBadRequest)?;

    if consumed != body.len() {
        return Err(ErrorBadRequest("Unexpected trailing request bytes"));
    }

    let data = tutorlolv2::realtime::realtime(&game).map_err(|e| ErrorBadRequest(e.to_string()))?;

    respond(data)
}
