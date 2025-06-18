use crate::{
    AppState, EnvConfig, match_fn,
    server::schemas::APIResponse,
    setup::{
        cache::{update_cdn_cache, update_riot_cache},
        generators::generate_writer_files,
        images::{
            img_download_arts, img_download_instances, img_download_items, img_download_runes,
        },
        scraper::meta_items_scraper,
        update::{
            prettify_internal_items, setup_champion_names, setup_damaging_items,
            setup_internal_champions, setup_internal_items, setup_meta_items,
            setup_project_folders,
        },
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};
use reqwest::Client;
use std::{sync::Arc, time::Instant};
use tokio::task;

#[post("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    println!(
        "fn[setup_project]: Started execution at: {:#?}",
        Instant::now()
    );

    let _ = setup_project_folders();
    let client: Client = state.client.clone();
    let envcfg: Arc<EnvConfig> = state.envcfg.clone();

    tokio::spawn(async move {
        let mut update_futures = Vec::new();

        update_futures.push(tokio::spawn(update_riot_cache(
            client.clone(),
            envcfg.clone(),
        )));
        update_futures.push(tokio::spawn(update_cdn_cache(
            client.clone(),
            envcfg.clone(),
            "champions",
        )));
        update_futures.push(tokio::spawn(update_cdn_cache(
            client.clone(),
            envcfg.clone(),
            "items",
        )));

        for update_future in update_futures {
            if let Err(e) = update_future.await {
                println!("Error joining update future at fn[setup_project]: {:#?}", e);
            }
        }

        let _ = task::spawn_blocking(setup_champion_names).await.ok();
        let _ = task::spawn_blocking(setup_internal_items).await.ok();
        let _ = prettify_internal_items().await;

        let client_1: Client = client.clone();
        let envcfg_1: Arc<EnvConfig> = envcfg.clone();

        tokio::spawn(async move {
            let _ = meta_items_scraper(client_1, envcfg_1).await;
            let _ = setup_meta_items();
            // #![dev]
            let _ = setup_damaging_items();
        });

        // #![dev]
        let envcfg_2: Arc<EnvConfig> = envcfg.clone();

        tokio::spawn(async move {
            let _ = generate_writer_files(envcfg_2).await;
            let _ = setup_internal_champions();
        });

        // There's no need to await for image download conclusion
        // They are independent and may run in parallel
        tokio::spawn(img_download_arts(client.clone(), envcfg.clone()));
        tokio::spawn(img_download_instances(client.clone(), envcfg.clone()));
        tokio::spawn(img_download_items(client.clone(), envcfg.clone()));
        tokio::spawn(img_download_runes(client, envcfg));
    });

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project setup started. Expected time to complete: 3-5 minutes",
        data: (),
    })
}

#[post("/folders")]
pub async fn setup_folders() -> impl Responder {
    match_fn!(setup_project_folders())
}

#[post("/champions")]
pub async fn setup_champions() -> impl Responder {
    match_fn!(setup_internal_champions())
}

#[post("/items")]
pub async fn setup_items() -> impl Responder {
    match_fn!(setup_internal_items())
}
