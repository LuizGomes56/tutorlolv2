use super::schemas::APIResponse;
use crate::setup::update::{
    initialize_items, rewrite_champion_names, setup_champion_cache, setup_folders, write_champions,
    write_items,
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
    write_champions().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions updated on client".to_string(),
        data: "latest version added to cache".to_string(),
    })
}

#[get("/api/update/items")]
pub async fn update_items() -> impl Responder {
    write_items().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Items updated on client".to_string(),
        data: "latest version added to cache".to_string(),
    })
}
