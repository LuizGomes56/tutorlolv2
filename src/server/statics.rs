use crate::const_bytes;
use actix_web::{HttpResponse, Responder, get};

static SPRITE_MAP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sprite_map.br"));
static EXPORTED_CODE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/export_code.br"));

#[get("/comptime")]
pub async fn static_comptime() -> impl Responder {
    const_bytes!(EXPORTED_CODE)
}

#[get("/sprite_map")]
pub async fn static_sprite_map() -> impl Responder {
    const_bytes!(SPRITE_MAP)
}
