use crate::server::{
    codec::request_config,
    endpoint::EndpointExt,
    response::{AppError, AppResult, Trace},
};
use actix_web::{HttpResponse, http::StatusCode, web::Bytes};
use math::riot::RiotRealtime;

pub async fn realtime_handler(body: Bytes) -> AppResult<HttpResponse> {
    let (game, consumed) =
        bincode::borrow_decode_from_slice::<RiotRealtime<'_>, _>(&body, request_config())
            .trace_ctx(("Invalid realtime request", StatusCode::BAD_REQUEST))?;

    if consumed != body.len() {
        return Err(AppError::bad_request("Unexpected trailing request bytes"));
    }

    let data = math::realtime::realtime(&game).trace()?;

    Ok(::api::Realtime::reply(data).into_response())
}

api_endpoint! {
    pub Realtime {
        handler: realtime_handler, method: POST, path: "/api/games/realtime",
        path_params: {  }, query: none, body: borrowed_bincode(math::riot::RiotRealtime<'wire>),
        response: borrowed_envelope(math::model::Realtime<'wire>), success: [200], auth: anonymous,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
