use super::schemas::APIResponse;
use crate::{
    AppState,
    server::schemas::AccessRequest,
    setup::update::{
        get_meta_items, initialize_items, rewrite_champion_names, setup_champion_cache,
        setup_folders, update_instances, update_riot_cache,
    },
};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};

#[post("/api/setup")]
pub async fn setup_project(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    setup_folders();
    setup_champion_cache();
    rewrite_champion_names();
    initialize_items();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project is ready".to_string(),
        data: (),
    })
}

#[post("/api/update/riot")]
pub async fn update_riot(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    update_riot_cache().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Riot updated on client".to_string(),
        data: (),
    })
}

#[post("/api/update/champions")]
pub async fn update_champions(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    update_instances("champions").await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions updated on client".to_string(),
        data: (),
    })
}

#[post("/api/update/items")]
pub async fn update_items(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    update_instances("items").await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Items updated on client".to_string(),
        data: (),
    })
}

#[post("/api/update/meta_items")]
pub async fn update_meta_items(state: Data<AppState>, body: Json<AccessRequest>) -> impl Responder {
    if body.password() != state.password {
        return HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: (),
        });
    }

    get_meta_items().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Meta Items updated on client".to_string(),
        data: (),
    })
}
