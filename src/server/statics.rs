use crate::const_bytes;
use actix_web::{HttpResponse, Responder, get};

static STATIC_CHAMPIONS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_champions.br"));
static STATIC_ITEMS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_items.br"));
static STATIC_RUNES_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_runes.br"));
static STATIC_COMPARED_ITEMS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/static_compared_items.br"));

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

#[get("/compared_items")]
pub async fn static_compared_items() -> impl Responder {
    const_bytes!(STATIC_COMPARED_ITEMS_BYTES)
}
