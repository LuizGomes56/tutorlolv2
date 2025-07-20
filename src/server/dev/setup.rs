use crate::{
    AppState, dev_response,
    essentials::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
    server::schemas::APIResponse,
    setup::{
        cache::{update_cdn_cache, update_riot_cache},
        generators::{
            champions::{GeneratorMode, create_generator_files, order_cdn_champion_effects},
            items::assign_item_damages,
        },
        scraper::meta_items_scraper,
        update::{
            prettify_internal_items, setup_champion_names, setup_damaging_items,
            setup_internal_champions, setup_internal_items, setup_internal_runes, setup_meta_items,
            setup_project_folders,
        },
    },
};
use actix_web::{HttpResponse, Responder, post, web::Data};
use tokio::task;

#[post("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    let _ = setup_project_folders();
    let client = state.client.clone();

    tokio::spawn(async move {
        let mut update_futures = Vec::new();

        update_futures.push(tokio::spawn(update_riot_cache(client.clone())));
        update_futures.push(tokio::spawn(update_cdn_cache(client.clone(), "champions")));
        update_futures.push(tokio::spawn(update_cdn_cache(client.clone(), "items")));

        for update_future in update_futures {
            if let Err(e) = update_future.await {
                println!("Error joining update future at fn[setup_project]: {:#?}", e);
            }
        }

        let _ = task::spawn_blocking(setup_champion_names).await.ok();
        let _ = task::spawn_blocking(setup_internal_items).await.ok();
        let _ = task::spawn_blocking(setup_internal_runes).await.ok();
        let _ = prettify_internal_items().await;

        let client_1 = client.clone();

        tokio::spawn(async move {
            let _ = meta_items_scraper(client_1).await;
            let _ = setup_meta_items();
            let _ = setup_damaging_items();
            let _ = assign_item_damages();
        });

        tokio::spawn(async move {
            let _ = order_cdn_champion_effects();
            let _ = create_generator_files(GeneratorMode::Partial).await;
            let _ = setup_internal_champions();
        });

        // There's no need to await for image download conclusion
        // They are independent and may run in parallel
        tokio::spawn(img_download_arts(client.clone()));
        tokio::spawn(img_download_instances(client.clone()));
        tokio::spawn(img_download_items(client.clone()));
        tokio::spawn(img_download_runes(client));
    });

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project setup started. Expected time to complete: 3-5 minutes",
        data: (),
    })
}

#[post("/folders")]
pub async fn setup_folders() -> impl Responder {
    dev_response!(setup_project_folders())
}

#[post("/champions")]
pub async fn setup_champions() -> impl Responder {
    dev_response!(setup_internal_champions())
}

#[post("/items")]
pub async fn setup_items() -> impl Responder {
    dev_response!(setup_internal_items())
}

#[post("/runes")]
pub async fn setup_runes() -> impl Responder {
    dev_response!(setup_internal_runes())
}
