use std::time::Instant;

use actix_web::{HttpResponse, Responder, post, web::Data};
use reqwest::Client;
use tokio::task::{self, JoinHandle};

use crate::AppState;
use crate::{
    server::schemas::APIResponse,
    setup::{
        images::{
            img_download_arts, img_download_instances, img_download_items, img_download_runes,
        },
        update::{
            append_prettified_item_stats, generate_writer_files, get_meta_items,
            identify_damaging_items, initialize_items, replace_item_names_with_ids,
            rewrite_champion_names, setup_champion_cache, setup_project_folders, update_cdn_cache,
            update_riot_cache,
        },
    },
};

#[post("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    setup_project_folders();

    let client: Client = state.client.clone();

    tokio::spawn(async move {
        let start_time: Instant = Instant::now();
        let mut update_futures: Vec<JoinHandle<()>> = Vec::new();

        update_futures.push(tokio::spawn(update_riot_cache(client.clone())));
        update_futures.push(tokio::spawn(update_cdn_cache(client.clone(), "champions")));
        update_futures.push(tokio::spawn(update_cdn_cache(client.clone(), "items")));

        for update_future in update_futures {
            let _ = update_future.await;
        }

        let _ = task::spawn_blocking(rewrite_champion_names).await.ok();
        let _ = task::spawn_blocking(initialize_items).await.ok();

        append_prettified_item_stats().await;

        let client_1: Client = client.clone();

        tokio::spawn(async move {
            get_meta_items(client_1).await;
            replace_item_names_with_ids();
            // #![dev]
            identify_damaging_items();
        });

        // #![dev]
        tokio::spawn(async move {
            generate_writer_files().await;
            setup_champion_cache();
        });

        // There's no need to await for image download conclusion
        // They are independent and may run in parallel
        tokio::spawn(img_download_arts(client.clone()));
        tokio::spawn(img_download_instances(client.clone()));
        tokio::spawn(img_download_items(client.clone()));
        tokio::spawn(img_download_runes(client));

        println!(
            "Project setup completed in {}ms",
            start_time.elapsed().as_millis()
        );
    });

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project setup started. Expected time to complete: 3-5 minutes",
        data: "Using 10 tokio threads",
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
