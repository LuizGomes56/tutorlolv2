use actix_web::{HttpResponse, Responder, get};

static EXPORTED_CODE: &'static str = include_str!("../../../tutorlolv2_exports/export_code.txt");

#[get("/comptime")]
pub async fn static_comptime() -> impl Responder {
    HttpResponse::Ok().body(EXPORTED_CODE)
}
