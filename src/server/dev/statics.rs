use crate::const_bytes;
use actix_web::{HttpResponse, Responder, post};

static SPRITE_MAP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sprite_map.br"));
static EXPORTED_CODE: &'static str = include_str!("../../../comptime_exports/export_code.txt");

#[post("/comptime")]
pub async fn static_comptime() -> impl Responder {
    HttpResponse::Ok().body(EXPORTED_CODE)
}

#[post("/sprite_map")]
pub async fn static_sprite_map() -> impl Responder {
    const_bytes!(SPRITE_MAP)
}
