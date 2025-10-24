use crate::{
    AppState, dev_response,
    server::dev::images::{IMG_FOLDERS, img_convert_avif},
};
use actix_web::{
    HttpResponse, Responder, get,
    rt::{spawn, task::spawn_blocking},
    web::Data,
};
use tutorlolv2_dev::{
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
    setup::{
        cache::*,
        essentials::{api::CdnEndpoint, images::*},
        scraper::data_scraper,
        update::*,
    },
};
use tutorlolv2_exports::*;

#[get("/project")]
pub async fn setup_project(state: Data<AppState>) -> impl Responder {
    setup_project_folders();
    let client = state.client.clone();

    spawn(async move {
        for update_future in [
            spawn(update_riot_cache(client.clone())),
            spawn(update_cdn_cache(client.clone(), CdnEndpoint::Champions)),
            spawn(update_cdn_cache(client.clone(), CdnEndpoint::Items)),
        ] {
            if let Err(e) = update_future.await {
                println!("Error joining update future at fn[setup_project]: {:#?}", e);
            }
        }

        let _ = spawn_blocking(move || setup_champion_names().ok()).await;
        let _ = spawn_blocking(move || setup_internal_items().ok()).await;
        let _ = spawn_blocking(setup_runes_json).await;
        let _ = prettify_internal_items().await;

        {
            let client = client.clone();
            spawn(async move {
                let _ = data_scraper(client).await;
                let _ = setup_damaging_items();
                ItemFactory::run_all();
            });
        }
        spawn(async move {
            let _ = ChampionFactory::create_all();
            setup_internal_champions();
        });

        // There's no need to await for image download conclusion
        // They are independent and may run in parallel
        spawn(async move {
            for future in [
                spawn(img_download_arts(client.clone())),
                spawn(img_download_instances(client.clone())),
                spawn(img_download_items(client.clone())),
                spawn(img_download_runes(client)),
            ] {
                let _ = future.await;
            }
            let _ = spawn(img_convert_avif(IMG_FOLDERS)).await;
        });
    })
    .await
    .expect("Could not finish setup tasks");

    HttpResponse::Ok().body("Setup done")
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
    dev_response!(setup_runes_json())
}
