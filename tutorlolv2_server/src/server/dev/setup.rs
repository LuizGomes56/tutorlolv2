#[cfg(feature = "avif")]
use crate::server::dev::images::avif::{IMG_FOLDERS, img_convert_avif};
use actix_web::{HttpResponse, Responder, get};
use tokio::spawn;
use tutorlolv2_dev::HTTP_CLIENT;

#[get("/project")]
pub async fn setup_project() -> impl Responder {
    spawn(async move {
        let _ = HTTP_CLIENT.update_riot_cache().await;

        // spawn(async move { HTTP_CLIENT.call_scraper().await });
        // spawn(async move { HTTP_CLIENT.combo_scraper().await });

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
    tutorlolv2_html::run();
    HttpResponse::Ok().body("Html docs setup finished")
}
