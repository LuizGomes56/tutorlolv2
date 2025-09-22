use actix_web::{HttpResponse, Responder, get};

static EXPORTED_CODE: &'static str = include_str!("../../../tutorlolv2_exports/src/export_code.rs");

#[get("/comptime")]
pub async fn static_comptime() -> impl Responder {
    HttpResponse::Ok().body(EXPORTED_CODE)
}
