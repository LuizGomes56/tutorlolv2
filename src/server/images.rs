use std::env;

use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};

use crate::{
    AppState,
    server::schemas::{APIResponse, AccessRequest},
    setup::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
};

#[post("/api/images/instances")]
pub async fn download_instances(
    state: Data<AppState>,
    body: Json<AccessRequest>,
) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    img_download_instances().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded instances".to_string(),
        data: (),
    })
}

#[post("/api/images/items")]
pub async fn download_items(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    img_download_items().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded items".to_string(),
        data: (),
    })
}

#[post("/api/images/runes")]
pub async fn download_runes(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    img_download_runes().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded runes".to_string(),
        data: (),
    })
}

#[post("/api/images/arts")]
pub async fn download_arts(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    img_download_arts().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Downloaded centered art".to_string(),
        data: (),
    })
}
