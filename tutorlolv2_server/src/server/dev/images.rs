use actix_web::{HttpResponse, Responder, get};
use tutorlolv2_avif::{clean_sprite_folder, concat_sprite_jsons, generate_spritesheet};
use tutorlolv2_dev::HTTP_CLIENT;

macro_rules! download_image {
    (@inner $msg:expr) => {{
        HttpResponse::Ok().body($msg)
    }};
    ($call:ident) => {{
        let _ = HTTP_CLIENT.$call().await;
        download_image!(@inner format!("Executed fn[{}]", stringify!($call)))
    }};
}

#[get("/instances")]
pub async fn download_instances() -> impl Responder {
    download_image!(download_general_img)
}

#[get("/items")]
pub async fn download_items() -> impl Responder {
    download_image!(download_items_img)
}

#[get("/runes")]
pub async fn download_runes() -> impl Responder {
    download_image!(download_runes_img)
}

#[get("/arts")]
pub async fn download_arts() -> impl Responder {
    download_image!(download_arts_img)
}

#[get("/all")]
pub async fn download_all() -> impl Responder {
    macro_rules! spawn_thread {
        ($call:ident) => {{
            actix_web::rt::spawn(async move {
                let _ = HTTP_CLIENT.$call().await;
            });
        }};
    }
    spawn_thread!(download_general_img);
    spawn_thread!(download_items_img);
    spawn_thread!(download_runes_img);
    spawn_thread!(download_arts_img);
    download_image!(@inner "Started process")
}

pub async fn convert_folder(source: &str, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    match tutorlolv2_avif::convert_folder_avif(&format!("{source}/{folder}")).await {
        Ok(_) => println!("Convertion of '{folder}' finished"),
        Err(e) => eprintln!("Error converting '{folder}': {e:#?}"),
    }
    Ok(())
}

pub const SPRITE_FOLDERS: [&str; 3] = ["abilities", "champions", "items"];
pub const IMG_FOLDERS: [&str; 8] = [
    "abilities",
    "centered",
    "champions",
    "items",
    "other",
    "runes",
    "splash",
    "stats",
];

#[get("/sprite")]
pub async fn generate_sprites() -> impl Responder {
    println!("{:#?}", generate_spritesheet());
    concat_sprite_jsons();
    let _ = clean_sprite_folder();
    img_convert_avif(SPRITE_FOLDERS).await;
    download_image!(@inner "Started process")
}

pub async fn img_convert_avif<const N: usize>(folders: [&'static str; N]) {
    for folder in folders {
        println!("Converting folder: {}", folder);
        let _ = convert_folder("raw_img", folder).await;
    }
}

#[get("/compress")]
pub async fn compress_images() -> impl Responder {
    img_convert_avif(IMG_FOLDERS).await;
    HttpResponse::Ok().body("Executed fn[compress_images]")
}
