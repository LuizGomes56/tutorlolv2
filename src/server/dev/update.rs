use crate::{
    APIResponse, AppState, dev_response,
    essentials::api::CdnEndpoint,
    setup::{
        cache::{update_cdn_cache, update_riot_cache},
        schedule::update_env_version,
        scraper::meta_items_scraper,
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/riot")]
pub async fn update_riot(state: Data<AppState>) -> impl Responder {
    dev_response!(update_riot_cache(state.client.clone()).await)
}

#[post("/champions")]
pub async fn update_champions(state: Data<AppState>) -> impl Responder {
    dev_response!(update_cdn_cache(state.client.clone(), CdnEndpoint::Champions).await)
}

#[post("/items")]
pub async fn update_items(state: Data<AppState>) -> impl Responder {
    dev_response!(update_cdn_cache(state.client.clone(), CdnEndpoint::Items).await)
}

#[post("/meta_items")]
pub async fn update_meta_items(state: Data<AppState>) -> impl Responder {
    dev_response!(meta_items_scraper(state.client.clone()).await)
}

#[post("/version")]
pub async fn update_version(state: Data<AppState>) -> impl Responder {
    dev_response!(unsafe { update_env_version(state.client.clone()).await })
}
