use super::schemas::APIResponse;
use crate::setup::update::{get_meta_items, update_instances, update_riot_cache};
use actix_web::{HttpResponse, Responder, post};

#[post("/riot")]
pub async fn update_riot() -> impl Responder {
    update_riot_cache().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Riot updated on client",
        data: (),
    })
}

#[post("/champions")]
pub async fn update_champions() -> impl Responder {
    update_instances("champions").await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions updated on client",
        data: (),
    })
}

#[post("/items")]
pub async fn update_items() -> impl Responder {
    update_instances("items").await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Items updated on client",
        data: (),
    })
}

#[post("/meta_items")]
pub async fn update_meta_items() -> impl Responder {
    get_meta_items().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Meta Items updated on client",
        data: (),
    })
}
