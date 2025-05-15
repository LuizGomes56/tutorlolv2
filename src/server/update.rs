use super::schemas::APIResponse;
use crate::setup::update::{
    get_meta_items, initialize_items, rewrite_champion_names, setup_champion_cache, setup_folders,
    update_instances,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/setup")]
pub async fn setup_project() -> impl Responder {
    setup_folders();
    setup_champion_cache();
    rewrite_champion_names();
    initialize_items();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project is ready".to_string(),
        data: "client updated".to_string(),
    })
}

#[get("/api/update/champions")]
pub async fn update_champions() -> impl Responder {
    update_instances("champions").await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions updated on client".to_string(),
        data: "latest version added to cache".to_string(),
    })
}

#[get("/api/update/items")]
pub async fn update_items() -> impl Responder {
    update_instances("items").await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Items updated on client".to_string(),
        data: "latest version added to cache".to_string(),
    })
}

#[get("/api/update/meta_items")]
pub async fn update_meta_items() -> impl Responder {
    get_meta_items().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Meta Items updated on client".to_string(),
        data: "latest version added to cache".to_string(),
    })
}
