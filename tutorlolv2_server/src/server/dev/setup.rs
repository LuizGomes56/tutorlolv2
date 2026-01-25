use crate::dev_response;
#[cfg(feature = "avif")]
use crate::server::dev::images::avif::{IMG_FOLDERS, img_convert_avif};
use actix_web::{HttpResponse, Responder, get};
use tokio::spawn;
use tutorlolv2_dev::{
    HTTP_CLIENT,
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
    setup::update::*,
};

#[get("/project")]
pub async fn setup_project() -> impl Responder {
    let _ = setup_project_folders();

    spawn(async move {
        for future in [
            spawn(async move { HTTP_CLIENT.update_riot_cache().await }),
            spawn(async move { HTTP_CLIENT.update_meraki_cache("champions").await }),
            spawn(async move { HTTP_CLIENT.update_meraki_cache("items").await }),
        ] {
            let _ = future.await;
        }

        for f in [
            setup_runes_json,
            setup_internal_items,
            prettify_internal_items,
            setup_damaging_items,
            ItemFactory::run_all,
            ChampionFactory::create_all,
            ChampionFactory::run_all,
        ] {
            let _ = tokio::task::spawn_blocking(f);
        }

        spawn(async move { HTTP_CLIENT.call_scraper().await });
        spawn(async move { HTTP_CLIENT.combo_scraper().await });

        for future in [
            spawn(async move { HTTP_CLIENT.download_arts_img().await }),
            spawn(async move { HTTP_CLIENT.download_general_img().await }),
            spawn(async move { HTTP_CLIENT.download_items_img().await }),
            spawn(async move { HTTP_CLIENT.download_runes_img().await }),
        ] {
            let _ = future.await;
        }

        #[cfg(feature = "avif")]
        let _ = spawn(img_convert_avif(IMG_FOLDERS));
    })
    .await
    .expect("Could not finish setup tasks");

    HttpResponse::Ok().body("Setup done")
}

#[get("/docs")]
pub async fn setup_docs() -> impl Responder {
    tutorlolv2_html::run().await;
    HttpResponse::Ok().body("Html docs setup finished")
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
