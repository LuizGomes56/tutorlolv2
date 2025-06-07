use actix_web::{HttpResponse, Responder, post, web::Data};
use reqwest::Client;

use crate::AppState;
#[allow(unused_imports)]
use crate::{
    server::schemas::APIResponse,
    setup::{
        images::{
            img_download_arts, img_download_instances, img_download_items, img_download_runes,
        },
        update::{
            append_prettified_item_stats, generate_writer_files, get_meta_items,
            identify_damaging_items, initialize_items, replace_item_names_with_ids,
            rewrite_champion_names, setup_champion_cache, setup_project_folders, update_instances,
            update_riot_cache,
        },
    },
};

#[post("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    setup_project_folders();

    let client: Client = state.client.clone();

    tokio::spawn(async move {
        let (_, _, _, _) = tokio::join!(
            tokio::spawn(update_riot_cache(client.clone())),
            tokio::spawn(update_instances(client.clone(), "champions")),
            // #![dev]
            // tokio::spawn(generate_writer_files()),
            tokio::spawn(update_instances(client.clone(), "items")),
            tokio::spawn(get_meta_items(client)),
        );

        replace_item_names_with_ids();
        setup_champion_cache();
        initialize_items();
        // #![dev]
        // identify_damaging_items();
        rewrite_champion_names();
        append_prettified_item_stats().await;
    });

    let (_, _, _, _) = tokio::join!(
        tokio::spawn(img_download_arts(state.client.clone())),
        tokio::spawn(img_download_instances(state.client.clone())),
        tokio::spawn(img_download_items(state.client.clone())),
        tokio::spawn(img_download_runes(state.client.clone())),
    );

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project setup started. Expected time to complete: 3-5 minutes",
        data: "Using 9 green threads",
    })
}

#[post("/folders")]
pub async fn setup_folders() -> impl Responder {
    setup_project_folders();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Folders are ready",
        data: (),
    })
}

#[post("/champions")]
pub async fn setup_champions() -> impl Responder {
    setup_champion_cache();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions are ready",
        data: (),
    })
}
