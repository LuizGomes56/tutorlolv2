use crate::dev_response;
use actix_web::{HttpResponse, Responder, get};
use tutorlolv2_dev::HTTP_CLIENT;

#[get("/riot")]
pub async fn update_riot() -> impl Responder {
    dev_response!(HTTP_CLIENT.update_riot_cache().await)
}

#[get("/champions")]
pub async fn update_champions() -> impl Responder {
    dev_response!(HTTP_CLIENT.update_meraki_cache("champions").await)
}

#[get("/items")]
pub async fn update_items() -> impl Responder {
    dev_response!(HTTP_CLIENT.update_meraki_cache("items").await)
}

#[get("/data_scraper")]
pub async fn update_scraped_data() -> impl Responder {
    dev_response!(HTTP_CLIENT.data_scraper().await)
}

#[get("/version")]
pub async fn update_version() -> impl Responder {
    dev_response!(unsafe { HTTP_CLIENT.update_env_version().await })
}
