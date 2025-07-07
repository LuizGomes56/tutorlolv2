use crate::{
    INTERNAL_ITEMS, INTERNAL_NAMES, INTERNAL_RUNES, send_response, server::schemas::APIResponse,
};
use actix_web::{HttpResponse, Responder, get};
use rustc_hash::FxHashMap;

#[get("/champions")]
pub async fn static_champions() -> impl Responder {
    let data = INTERNAL_NAMES
        .entries()
        .map(|(k, v)| (v, k))
        .collect::<FxHashMap<_, _>>();
    send_response!(data)
}

#[get("/items")]
pub async fn static_items() -> impl Responder {
    let data = INTERNAL_ITEMS
        .entries()
        .map(|(item_id, value)| (item_id, &value.name))
        .collect::<FxHashMap<_, _>>();
    send_response!(data)
}

#[get("/runes")]
pub async fn static_runes() -> impl Responder {
    let data = INTERNAL_RUNES
        .entries()
        .map(|(rune_id, value)| (rune_id, &value.name))
        .collect::<FxHashMap<_, _>>();
    send_response!(data)
}
