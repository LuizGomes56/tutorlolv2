use super::schemas::APIResponse;
use crate::{
    AppState,
    setup::update::{get_meta_items, update_instances, update_riot_cache},
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/riot")]
pub async fn update_riot(state: Data<AppState>) -> impl Responder {
    update_riot_cache(state.client.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Riot updated on client",
        data: (),
    })
}

#[post("/champions")]
pub async fn update_champions(state: Data<AppState>) -> impl Responder {
    update_instances(state.client.clone(), "champions").await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions updated on client",
        data: (),
    })
}

#[post("/items")]
pub async fn update_items(state: Data<AppState>) -> impl Responder {
    update_instances(state.client.clone(), "items").await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Items updated on client",
        data: (),
    })
}

#[post("/meta_items")]
pub async fn update_meta_items(state: Data<AppState>) -> impl Responder {
    get_meta_items(state.client.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Meta Items updated on client",
        data: (),
    })
}
