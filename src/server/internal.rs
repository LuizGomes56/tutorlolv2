use crate::{
    match_fn,
    server::schemas::APIResponse,
    setup::update::{prettify_internal_items, setup_champion_names, setup_meta_items},
};
use actix_web::{HttpResponse, Responder, post};

#[post("/create_writer_files")]
pub async fn internal_create_writer_files() -> impl Responder {
    #[cfg(debug_assertions)]
    {
        use crate::setup::generators::generate_writer_files;
        match_fn!(generate_writer_files().await)
    }
    #[cfg(not(debug_assertions))]
    {
        match_fn!(where "fn[internal_create_writer_files] can't be called in release")
    }
}

#[post("/prettify_item_stats")]
pub async fn internal_prettify_item_stats() -> impl Responder {
    match_fn!(prettify_internal_items().await)
}

#[post("/create_damaging_items")]
pub async fn internal_create_damaging_items() -> impl Responder {
    #[cfg(debug_assertions)]
    {
        use crate::setup::update::setup_damaging_items;
        match_fn!(setup_damaging_items())
    }
    #[cfg(not(debug_assertions))]
    {
        match_fn!(where "fn[internal_create_damaging_items] can't be called in release")
    }
}

#[post("/rewrite_champion_names")]
pub async fn internal_rewrite_champion_names() -> impl Responder {
    match_fn!(setup_champion_names())
}

#[post("/create_meta_items")]
pub async fn internal_create_meta_items() -> impl Responder {
    match_fn!(setup_meta_items())
}
