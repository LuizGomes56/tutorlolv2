use crate::{AppState, dev_response};
use actix_web::{HttpResponse, Responder, get, web::Data};
use tutorlolv2_dev::setup::{
    cache::{update_cdn_cache, update_riot_cache},
    essentials::api::CdnEndpoint,
    schedule::update_env_version,
    scraper::meta_items_scraper,
};

#[get("/riot")]
pub async fn update_riot(state: Data<AppState>) -> impl Responder {
    dev_response!(update_riot_cache(state.client.clone()).await)
}

#[get("/champions")]
pub async fn update_champions(state: Data<AppState>) -> impl Responder {
    dev_response!(update_cdn_cache(state.client.clone(), CdnEndpoint::Champions).await)
}

#[get("/items")]
pub async fn update_items(state: Data<AppState>) -> impl Responder {
    dev_response!(update_cdn_cache(state.client.clone(), CdnEndpoint::Items).await)
}

#[get("/meta_items")]
pub async fn update_meta_items(state: Data<AppState>) -> impl Responder {
    dev_response!(meta_items_scraper(state.client.clone()).await)
}

#[get("/version")]
pub async fn update_version(state: Data<AppState>) -> impl Responder {
    dev_response!(unsafe { update_env_version(state.client.clone()).await })
}
