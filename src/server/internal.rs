use actix_web::{HttpResponse, Responder, post};

use crate::{
    server::schemas::APIResponse,
    setup::update::{
        append_prettified_item_stats, generate_writer_files, identify_damaging_items,
        replace_item_names_with_ids,
    },
};

#[post("/generate_writer_files")]
pub async fn internal_generate_writer_files() -> impl Responder {
    generate_writer_files().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Writers generated".to_string(),
        data: (),
    })
}

#[post("/append_prettified_item_stats")]
pub async fn internal_append_prettified_item_stats() -> impl Responder {
    append_prettified_item_stats().await;

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Item stats prettified".to_string(),
        data: (),
    })
}

#[post("/identify_damaging_items")]
pub async fn internal_identify_damaging_items() -> impl Responder {
    identify_damaging_items();

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Damaging items identified".to_string(),
        data: (),
    })
}

#[post("/replace_item_names_with_ids")]
pub async fn internal_replace_item_names_with_ids() -> impl Responder {
    replace_item_names_with_ids();

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Item names replaced with IDs".to_string(),
        data: (),
    })
}
