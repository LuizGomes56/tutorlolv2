use crate::{
    dev_response,
    server::schemas::APIResponse,
    setup::{
        generators::{champions::create_generator_files, items::assign_item_damages},
        update::{
            prettify_internal_items, setup_champion_names, setup_damaging_items, setup_meta_items,
        },
    },
};
use actix_web::{HttpResponse, Responder, post};

#[post("/create_generator_files")]
pub async fn internal_create_generator_files() -> impl Responder {
    dev_response!(create_generator_files().await)
}

#[post("/prettify_item_stats")]
pub async fn internal_prettify_item_stats() -> impl Responder {
    dev_response!(prettify_internal_items().await)
}

#[post("/create_damaging_items")]
pub async fn internal_create_damaging_items() -> impl Responder {
    dev_response!(setup_damaging_items())
}

#[post("/assign_item_damages")]
pub async fn internal_assign_item_damages() -> impl Responder {
    dev_response!(assign_item_damages())
}

#[post("/rewrite_champion_names")]
pub async fn internal_rewrite_champion_names() -> impl Responder {
    dev_response!(setup_champion_names())
}

#[post("/create_meta_items")]
pub async fn internal_create_meta_items() -> impl Responder {
    dev_response!(setup_meta_items())
}
