use crate::{
    AppState,
    server::schemas::APIResponse,
    setup::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

macro_rules! download_image {
    ($state:expr, $fn:expr) => {{
        $fn($state.client.clone(), $state.envcfg.clone()).await;
        HttpResponse::Ok().json(APIResponse {
            success: true,
            message: &format!("Executed fn[{}]", stringify!($fn)),
            data: (),
        })
    }};
}

#[post("/instances")]
pub async fn download_instances(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_instances)
}

#[post("/items")]
pub async fn download_items(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_items)
}

#[post("/runes")]
pub async fn download_runes(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_runes)
}

#[post("/arts")]
pub async fn download_arts(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_arts)
}
