use crate::AppState;
use actix_web::{HttpResponse, Responder, get, web::Data};
use tutorlolv2_avif::{clean_sprite_folder, concat_sprite_jsons, generate_spritesheet};
use tutorlolv2_dev::setup::essentials::images::{
    img_download_arts, img_download_instances, img_download_items, img_download_runes,
};

macro_rules! download_image {
    (@inner $msg:expr) => {{
        HttpResponse::Ok().body($msg)
    }};
    ($state:expr, $call:expr) => {{
        $call($state.client.clone()).await;
        download_image!(@inner format!("Executed fn[{}]", stringify!($call)))
    }};
}

#[get("/instances")]
pub async fn download_instances(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_instances)
}

#[get("/items")]
pub async fn download_items(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_items)
}

#[get("/runes")]
pub async fn download_runes(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_runes)
}

#[get("/arts")]
pub async fn download_arts(state: Data<AppState>) -> impl Responder {
    download_image!(state, img_download_arts)
}

#[get("/all")]
pub async fn download_all(state: Data<AppState>) -> impl Responder {
    macro_rules! spawn_thread {
        ($func:ident) => {{
            let _ = actix_web::rt::spawn($func(state.client.clone()));
        }};
    }
    spawn_thread!(img_download_instances);
    spawn_thread!(img_download_items);
    spawn_thread!(img_download_runes);
    spawn_thread!(img_download_arts);
    download_image!(@inner "Started process")
}

macro_rules! convert_folder {
    ($src:literal, $folder:expr) => {{
        match tutorlolv2_avif::convert_folder_avif(&format!("{}/{}", $src, $folder)).await {
            Ok(_) => println!("Conversão de '{}' concluída", $folder),
            Err(e) => eprintln!("Erro na conversão de '{}': {:#?}", $folder, e),
        }
    }};
}

#[get("/sprite")]
pub async fn generate_sprites() -> impl Responder {
    println!("{:#?}", generate_spritesheet());
    concat_sprite_jsons();
    let _ = clean_sprite_folder();
    let folders = ["abilities", "champions", "items"];
    for folder in folders {
        println!("Converting folder: {}", folder);
        convert_folder!("sprite", folder);
    }
    download_image!(@inner "Started process")
}

#[get("/compress")]
pub async fn compress_images() -> impl Responder {
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
        println!("Converting folder: {}", folder);
        convert_folder!("raw_img", folder);
    }

    HttpResponse::Ok().body("Executed fn[compress_images]")
}
