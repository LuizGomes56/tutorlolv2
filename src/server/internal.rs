use crate::{
    AppState,
    server::schemas::APIResponse,
    setup::{
        generators::generate_writer_files,
        update::{
            prettify_internal_items, setup_champion_names, setup_damaging_items, setup_meta_items,
        },
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};

#[post("/generate_writer_files")]
pub async fn internal_generate_writer_files(state: Data<AppState>) -> impl Responder {
    match generate_writer_files(state.envcfg.clone()).await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Writers generated",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Unexpected error: {:#?}", e),
            data: (),
        }),
    }
}

#[post("/append_prettified_item_stats")]
pub async fn internal_append_prettified_item_stats() -> impl Responder {
    match prettify_internal_items().await {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Item stats prettified",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Unexpected error: {:#?}", e),
            data: (),
        }),
    }
}

#[post("/identify_damaging_items")]
pub async fn internal_identify_damaging_items() -> impl Responder {
    match setup_damaging_items() {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Damaging items identified",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Unexpected error: {:#?}", e),
            data: (),
        }),
    }
}

#[post("/rewrite_champion_names")]
pub async fn internal_rewrite_champion_names() -> impl Responder {
    match setup_champion_names() {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Champion names rewritten",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Unexpected error: {:#?}", e),
            data: (),
        }),
    }
}

#[post("/replace_item_names_with_ids")]
pub async fn internal_replace_item_names_with_ids() -> impl Responder {
    match setup_meta_items() {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Item names replaced with IDs",
            data: (),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Unexpected error: {:#?}", e),
            data: (),
        }),
    }
}
