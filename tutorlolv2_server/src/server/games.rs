use super::codec::{Response, decode_request, request_config, respond};
use actix_web::{error::ErrorBadRequest, post, web::Bytes};
use tutorlolv2::{calculator, model::OwnedInputGame, riot::RiotRealtime};

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

#[post("/calculator")]
pub async fn calculator_handler(body: Bytes) -> Response {
    let request = decode_request::<OwnedInputGame>(&body)?;
    let prepared = request.prepare();

    respond(calculator::calculator(prepared.as_input()))
}
