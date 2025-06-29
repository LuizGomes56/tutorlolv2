use crate::{
    match_fn,
    server::schemas::APIResponse,
    setup::{
        generators::items::assign_item_damages,
        update::{prettify_internal_items, setup_champion_names, setup_meta_items},
    },
};
use actix_web::{HttpResponse, Responder, post};

#[post("/create_writer_files")]
pub async fn internal_create_writer_files() -> impl Responder {
    match_fn!(priv crate::setup::generators::champions::generate_writer_files().await)
}

#[post("/prettify_item_stats")]
pub async fn internal_prettify_item_stats() -> impl Responder {
    match_fn!(prettify_internal_items().await)
}

#[post("/create_damaging_items")]
pub async fn internal_create_damaging_items() -> impl Responder {
    match_fn!(priv crate::setup::update::setup_damaging_items())
}

#[post("/assign_item_damages")]
pub async fn internal_assign_item_damages() -> impl Responder {
    match_fn!(assign_item_damages())
}

#[post("/rewrite_champion_names")]
pub async fn internal_rewrite_champion_names() -> impl Responder {
    match_fn!(setup_champion_names())
}

#[post("/create_meta_items")]
pub async fn internal_create_meta_items() -> impl Responder {
    match_fn!(setup_meta_items())
}
