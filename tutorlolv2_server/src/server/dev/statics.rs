use actix_web::{HttpResponse, Responder, get};

static EXPORTED_CODE: &str = include_str!("../../../../tutorlolv2_gen/src/data.rs");

#[get("/comptime")]
pub async fn static_comptime() -> impl Responder {
    HttpResponse::Ok().body(EXPORTED_CODE)
}
