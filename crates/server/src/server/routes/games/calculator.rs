use crate::server::{codec::Bincode, endpoint::EndpointExt, response::ApiResult};
use math::{
    calculator,
    model::{OutputGame, OwnedInputGame},
};

pub async fn calculator_handler(
    Bincode(request): Bincode<OwnedInputGame>,
) -> ApiResult<OutputGame> {
    // The shared InputGame view and this owned request have the same tested bincode layout.
    let prepared = request.prepare();

    Ok(::api::Calculator::reply(calculator::calculator(
        prepared.as_input(),
    )))
}

api_endpoint! {
    pub Calculator {
        handler: calculator_handler, method: POST, path: "/api/games/calculator",
        path_params: {  }, query: none, body: borrowed_bincode(math::model::InputGame<'wire>),
        response: envelope(math::model::OutputGame), success: [200], auth: anonymous,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
