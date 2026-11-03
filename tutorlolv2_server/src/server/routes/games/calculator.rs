use crate::server::codec::{Bincode, Response, respond};
use tutorlolv2::{calculator, model::OwnedInputGame};

pub async fn calculator_handler(Bincode(request): Bincode<OwnedInputGame>) -> Response {
    let prepared = request.prepare();

    respond(calculator::calculator(prepared.as_input()))
}

api_endpoint! {
    pub Calculator {
        handler: calculator_handler, method: POST, path: "/api/games/calculator",
        path_params: {  }, query: none, body: bincode(tutorlolv2::model::OwnedInputGame),
        response: envelope(tutorlolv2::model::OutputGame), success: [200], auth: anonymous,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
