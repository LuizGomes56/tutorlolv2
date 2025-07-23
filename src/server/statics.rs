use crate::const_bytes;
use actix_web::{HttpResponse, Responder, get};

static STATIC_CHAMPIONS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_champions.br"));
static STATIC_ITEMS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_items.br"));
static STATIC_RUNES_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_runes.br"));
static STATIC_ITEMS_DEF_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_items_def.br"));
static STATIC_SPRITE_MAP: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/sprite_map.br"));

#[get("/champions")]
pub async fn static_champions() -> impl Responder {
    const_bytes!(STATIC_CHAMPIONS_BYTES)
}

#[get("/items")]
pub async fn static_items() -> impl Responder {
    const_bytes!(STATIC_ITEMS_BYTES)
}

#[get("/runes")]
pub async fn static_runes() -> impl Responder {
    const_bytes!(STATIC_RUNES_BYTES)
}

#[get("/items_def")]
pub async fn static_items_def() -> impl Responder {
    const_bytes!(STATIC_ITEMS_DEF_BYTES)
}

#[get("/sprite_map")]
pub async fn static_sprite_map() -> impl Responder {
    const_bytes!(STATIC_SPRITE_MAP)
}
