use crate::{
    AppState,
    server::schemas::APIResponse,
    setup::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/instances")]
pub async fn download_instances(state: Data<AppState>) -> impl Responder {
    img_download_instances(state.client.clone(), state.envcfg.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded instances",
        data: (),
    })
}

#[post("/items")]
pub async fn download_items(state: Data<AppState>) -> impl Responder {
    img_download_items(state.client.clone(), state.envcfg.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded items",
        data: (),
    })
}

#[post("/runes")]
pub async fn download_runes(state: Data<AppState>) -> impl Responder {
    img_download_runes(state.client.clone(), state.envcfg.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded runes",
        data: (),
    })
}

#[post("/arts")]
pub async fn download_arts(state: Data<AppState>) -> impl Responder {
    img_download_arts(state.client.clone(), state.envcfg.clone()).await;
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded centered art",
        data: (),
    })
}
