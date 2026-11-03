use crate::server::codec::{Response, request_config, respond};
use actix_web::{error::ErrorBadRequest, web::Bytes};
use tutorlolv2::riot::RiotRealtime;

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

api_endpoint! {
    pub Realtime {
        handler: realtime_handler, method: POST, path: "/api/games/realtime",
        path_params: {  }, query: none, body: borrowed_bincode(tutorlolv2::riot::RiotRealtime<'wire>),
        response: borrowed_envelope(tutorlolv2::model::Realtime<'wire>), success: [200], auth: anonymous,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
