use actix_web::{HttpResponse, Responder, post};

static EXPORTED_CODE: &'static str = include_str!("../../../comptime_exports/export_code.txt");

#[post("/comptime")]
pub async fn static_comptime() -> impl Responder {
    HttpResponse::Ok().body(EXPORTED_CODE)
}
