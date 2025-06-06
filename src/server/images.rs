use actix_web::{HttpResponse, Responder, post};

use crate::{
    server::schemas::APIResponse,
    setup::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
};

#[post("/instances")]
pub async fn download_instances() -> impl Responder {
    img_download_instances().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded instances",
        data: (),
    })
}

#[post("/items")]
pub async fn download_items() -> impl Responder {
    img_download_items().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded items",
        data: (),
    })
}

#[post("/runes")]
pub async fn download_runes() -> impl Responder {
    img_download_runes().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded runes",
        data: (),
    })
}

#[post("/arts")]
pub async fn download_arts() -> impl Responder {
    img_download_arts().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded centered art",
        data: (),
    })
}
