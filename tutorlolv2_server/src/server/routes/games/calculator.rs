use crate::server::codec::{Bincode, Response, respond};
use actix_web::post;
use tutorlolv2::{calculator, model::OwnedInputGame};

#[post("/calculator")]
pub async fn calculator_handler(Bincode(request): Bincode<OwnedInputGame>) -> Response {
    let prepared = request.prepare();

    respond(calculator::calculator(prepared.as_input()))
}
