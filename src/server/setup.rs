use actix_web::{HttpResponse, Responder, post};

use crate::{
    server::schemas::APIResponse,
    setup::update::{
        initialize_items, rewrite_champion_names, setup_champion_cache, setup_project_folders,
    },
};

#[post("/project")]
pub async fn setup_project() -> impl Responder {
    setup_project_folders();
    setup_champion_cache();
    rewrite_champion_names();
    initialize_items();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project is ready".to_string(),
        data: (),
    })
}

#[post("/folders")]
pub async fn setup_folders() -> impl Responder {
    setup_project_folders();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Folders are ready".to_string(),
        data: (),
    })
}

#[post("/champions")]
pub async fn setup_champions() -> impl Responder {
    setup_champion_cache();

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions are ready".to_string(),
        data: (),
    })
}
