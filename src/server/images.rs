use actix_web::{HttpResponse, Responder, get};

use crate::{
    server::schemas::APIResponse,
    setup::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
};

#[get("/api/images/instances")]
pub async fn download_instances() -> impl Responder {
    img_download_instances().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded instances".to_string(),
        data: (),
    })
}

#[get("/api/images/items")]
pub async fn download_items() -> impl Responder {
    img_download_items().await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded items".to_string(),
        data: (),
    })
}

#[get("/api/images/runes")]
pub async fn download_runes() -> impl Responder {
    img_download_runes().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded runes".to_string(),
        data: (),
    })
}

#[get("/api/images/arts")]
pub async fn download_arts() -> impl Responder {
    img_download_arts().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded centered art".to_string(),
        data: (),
    })
}
