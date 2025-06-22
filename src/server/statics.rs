use crate::{GLOBAL_CACHE, server::schemas::APIResponse};
use actix_web::{HttpResponse, Responder, get};
use rustc_hash::FxHashMap;

#[get("/champions")]
pub async fn static_champions() -> impl Responder {
    let data = GLOBAL_CACHE
        .champion_names
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<FxHashMap<_, _>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}

#[get("/items")]
pub async fn static_items() -> impl Responder {
    let data = GLOBAL_CACHE
        .items
        .iter()
        .map(|(item_id, value)| (item_id, &value.name))
        .collect::<FxHashMap<_, _>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}

#[get("/runes")]
pub async fn static_runes() -> impl Responder {
    let data = GLOBAL_CACHE
        .runes
        .iter()
        .map(|(rune_id, value)| (rune_id, &value.name))
        .collect::<FxHashMap<_, _>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}
