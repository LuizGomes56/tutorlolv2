use super::schemas::APIResponse;
use crate::{
    AppState, match_fn,
    setup::{
        cache::{update_cdn_cache, update_riot_cache},
        schedule::update_env_version,
        scraper::meta_items_scraper,
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/riot")]
pub async fn update_riot(state: Data<AppState>) -> impl Responder {
    match_fn!(update_riot_cache(state.client.clone()).await)
}

#[post("/champions")]
pub async fn update_champions(state: Data<AppState>) -> impl Responder {
    match_fn!(update_cdn_cache(state.client.clone(), "champions").await)
}

#[post("/items")]
pub async fn update_items(state: Data<AppState>) -> impl Responder {
    match_fn!(update_cdn_cache(state.client.clone(), "items").await)
}

#[post("/meta_items")]
pub async fn update_meta_items(state: Data<AppState>) -> impl Responder {
    match_fn!(meta_items_scraper(state.client.clone()).await)
}

#[post("/version")]
pub async fn update_version(state: Data<AppState>) -> impl Responder {
    match_fn!(unsafe { update_env_version(state.client.clone()).await })
}
