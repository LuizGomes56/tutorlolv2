use crate::{
    AppState,
    essentials::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
    server::schemas::APIResponse,
};
use actix_web::{HttpResponse, Responder, post, web::Data};

macro_rules! download_image {
    ($state:expr, $call:expr) => {{
        $call($state.client.clone()).await;
        HttpResponse::Ok().json(APIResponse {
            success: true,
            message: &format!("Executed fn[{}]", stringify!($call)),
            data: (),
        })
    }};
}

#[post("/instances")]
pub async fn download_instances(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_instances)
}

#[post("/items")]
pub async fn download_items(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_items)
}

#[post("/runes")]
pub async fn download_runes(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_runes)
}

#[post("/arts")]
pub async fn download_arts(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_arts)
}

#[post("/compress")]
pub async fn compress_images() -> impl Responder {
    #[cfg(feature = "dev-routes")]
    {
        macro_rules! convert_folder {
            ($folder:expr) => {{
                match crate::setup::avif_converter::convert_folder_avif($folder).await {
                    Ok(_) => println!("Conversão de '{}' concluída", $folder),
                    Err(e) => eprintln!("Erro na conversão de '{}': {}", $folder, e),
                }
            }};
        }

        let folders = [
            "abilities",
            "centered",
            "champions",
            "items",
            "other",
            "runes",
            "splash",
            "stats",
        ];

        for folder in folders {
            println!("Convertendo pasta: {}", folder);
            convert_folder!(folder);
        }

        HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Executed fn[compress_images]",
            data: (),
        })
    }

    #[cfg(not(feature = "dev-routes"))]
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "feature [dev-routes] not enabled",
        data: (),
    })
}
