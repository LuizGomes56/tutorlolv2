use crate::{
    dev_response,
    server::dev::images::{IMG_FOLDERS, img_convert_avif},
};
use actix_web::{
    HttpResponse, Responder, get,
    rt::{spawn, task::spawn_blocking},
};
use tutorlolv2_dev::{
    HTTP_CLIENT,
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
    setup::update::*,
};
use tutorlolv2_exports::*;

#[get("/project")]
pub async fn setup_project() -> impl Responder {
    setup_project_folders();

    spawn(async move {
        let _ = HTTP_CLIENT.update_riot_cache().await;
        let _ = HTTP_CLIENT.update_meraki_cache("champions").await;
        let _ = HTTP_CLIENT.update_meraki_cache("items").await;

        let _ = spawn_blocking(move || setup_champion_names().ok()).await;
        let _ = spawn_blocking(move || setup_internal_items().ok()).await;
        let _ = setup_runes_json();
        let _ = prettify_internal_items().await;

        let _ = HTTP_CLIENT.data_scraper().await;
        let _ = setup_damaging_items();
        ItemFactory::run_all();

        spawn(async move {
            let _ = ChampionFactory::create_all();
            ChampionFactory::run_all();
        });

        let _ = HTTP_CLIENT.download_arts_img().await;
        let _ = HTTP_CLIENT.download_general_img().await;
        let _ = HTTP_CLIENT.download_items_img().await;
        let _ = HTTP_CLIENT.download_runes_img().await;
        let _ = spawn(img_convert_avif(IMG_FOLDERS)).await;
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
    dev_response!(ChampionFactory::run_all())
}

#[get("/items")]
pub async fn setup_items() -> impl Responder {
    dev_response!(setup_internal_items())
}

#[get("/runes")]
pub async fn setup_runes() -> impl Responder {
    dev_response!(setup_runes_json())
}
