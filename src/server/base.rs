use crate::server::schemas::APIResponse;
use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Server is up and running",
        data: (),
    })
}
