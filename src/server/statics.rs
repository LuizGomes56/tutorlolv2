use crate::{AppState, server::schemas::APIResponse};
use actix_web::{HttpResponse, Responder, get, web::Data};
use rustc_hash::FxHashMap;

#[get("/champions")]
pub async fn static_champions(state: Data<AppState>) -> impl Responder {
    let data = state
        .cache
        .champion_names
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect::<FxHashMap<String, String>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}

#[get("/items")]
pub async fn static_items(state: Data<AppState>) -> impl Responder {
    let data = state
        .cache
        .items
        .iter()
        .map(|(item_id, value)| (*item_id, value.name.clone()))
        .collect::<FxHashMap<usize, String>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}

#[get("/runes")]
pub async fn static_runes(state: Data<AppState>) -> impl Responder {
    let data = state
        .cache
        .runes
        .iter()
        .map(|(rune_id, value)| (*rune_id, value.name.clone()))
        .collect::<FxHashMap<usize, String>>();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data,
    })
}
