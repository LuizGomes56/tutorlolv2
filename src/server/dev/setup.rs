use crate::{AppState, dev_response};
use actix_web::{
    HttpResponse, Responder, get,
    rt::{spawn, task::spawn_blocking},
    web::Data,
};
use tutorlolv2_dev::setup::{
    cache::*,
    essentials::{api::CdnEndpoint, images::*},
    generators::{champions::*, items::assign_item_damages},
    scraper::meta_items_scraper,
    update::*,
};
use tutorlolv2_exports::*;

#[get("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    let _ = setup_project_folders();
    let client = state.client.clone();

    spawn(async move {
        let mut update_futures = Vec::new();

        update_futures.push(spawn(update_riot_cache(client.clone())));
        update_futures.push(spawn(update_cdn_cache(
            client.clone(),
            CdnEndpoint::Champions,
        )));
        update_futures.push(spawn(update_cdn_cache(client.clone(), CdnEndpoint::Items)));

        for update_future in update_futures {
            if let Err(e) = update_future.await {
                println!("Error joining update future at fn[setup_project]: {:#?}", e);
            }
        }

        let _ = spawn_blocking(setup_champion_names).await.ok();
        let _ = spawn_blocking(setup_internal_items).await.ok();
        let _ = spawn_blocking(setup_internal_runes).await.ok();
        let _ = prettify_internal_items().await;

        let client_1 = client.clone();

        spawn(async move {
            let _ = meta_items_scraper(client_1).await;
            let _ = setup_meta_items();
            let _ = setup_damaging_items();
            let _ = assign_item_damages();
        });

        spawn(async move {
            let _ = create_generator_files(GeneratorMode::Partial).await;
            let _ = setup_internal_champions();
        });

        // There's no need to await for image download conclusion
        // They are independent and may run in parallel
        spawn(img_download_arts(client.clone()));
        spawn(img_download_instances(client.clone()));
        spawn(img_download_items(client.clone()));
        spawn(img_download_runes(client));
    });

    HttpResponse::Ok().body("Project setup started. Expected time to complete: 3-5 minutes")
}

#[get("/docs")]
pub async fn setup_docs() -> impl Responder {
    spawn_blocking(generate_champion_html);
    spawn_blocking(generate_item_html);
    spawn_blocking(generate_rune_html);
    HttpResponse::Ok().body("Docs setup started. Expected time to complete: less than 1 minute")
}

#[get("/folders")]
pub async fn setup_folders() -> impl Responder {
    dev_response!(setup_project_folders())
}

#[get("/champions")]
pub async fn setup_champions() -> impl Responder {
    dev_response!(setup_internal_champions())
}

#[get("/items")]
pub async fn setup_items() -> impl Responder {
    dev_response!(setup_internal_items())
}

#[get("/runes")]
pub async fn setup_runes() -> impl Responder {
    dev_response!(setup_internal_runes())
}
