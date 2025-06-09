use super::schemas::APIResponse;
use crate::{
    AppState,
    setup::{
        cache::{update_cdn_cache, update_riot_cache},
        scraper::meta_items_scraper,
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/riot")]
pub async fn update_riot(state: Data<AppState>) -> impl Responder {
    match update_riot_cache(state.client.clone(), state.envcfg.clone()).await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Riot updated on client",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("{:#?}", e),
            data: (),
        }),
    }
}

#[post("/champions")]
pub async fn update_champions(state: Data<AppState>) -> impl Responder {
    match update_cdn_cache(state.client.clone(), state.envcfg.clone(), "champions").await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Champions updated on client",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("{:#?}", e),
            data: (),
        }),
    }
}

#[post("/items")]
pub async fn update_items(state: Data<AppState>) -> impl Responder {
    match update_cdn_cache(state.client.clone(), state.envcfg.clone(), "items").await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Items updated on client",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("{:#?}", e),
            data: (),
        }),
    }
}

#[post("/meta_items")]
pub async fn update_meta_items(state: Data<AppState>) -> impl Responder {
    match meta_items_scraper(state.client.clone(), state.envcfg.clone()).await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Meta Items updated on client",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("{:#?}", e),
            data: (),
        }),
    }
}
