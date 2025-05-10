use super::schemas::APIResponse;
use crate::services::setup::{setup_champion_cache, setup_folders, write_champions, write_items};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/setup")]
pub async fn setup_project() -> impl Responder {
    setup_folders();
    setup_champion_cache();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project folders setup up successfully".to_string(),
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
