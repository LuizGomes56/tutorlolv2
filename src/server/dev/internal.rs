use crate::dev_response;
use actix_web::{HttpResponse, Responder, get};
use tutorlolv2_dev::setup::{
    generators::{
        champions::{GeneratorMode, create_generator_files},
        items::assign_item_damages,
    },
    update::{prettify_internal_items, setup_champion_names, setup_damaging_items},
};

#[get("/create_generator_files")]
pub async fn internal_create_generator_files() -> impl Responder {
    dev_response!(create_generator_files(GeneratorMode::Total).await)
}

#[get("/prettify_item_stats")]
pub async fn internal_prettify_item_stats() -> impl Responder {
    dev_response!(prettify_internal_items().await)
}

#[get("/create_damaging_items")]
pub async fn internal_create_damaging_items() -> impl Responder {
    dev_response!(setup_damaging_items())
}

#[get("/assign_item_damages")]
pub async fn internal_assign_item_damages() -> impl Responder {
    dev_response!(assign_item_damages())
}

#[get("/rewrite_champion_names")]
pub async fn internal_rewrite_champion_names() -> impl Responder {
    dev_response!(setup_champion_names())
}
