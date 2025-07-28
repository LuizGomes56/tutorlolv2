use crate::{
    AppState,
    essentials::images::{
        img_download_arts, img_download_instances, img_download_items, img_download_runes,
    },
    server::schemas::APIResponse,
    setup::avif_converter::{clean_sprite_folder, concat_sprite_jsons, generate_spritesheet},
};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Path},
};
use rust_embed::Embed;

macro_rules! download_image {
    (@inner $msg:expr) => {{
        HttpResponse::Ok().json(APIResponse {
            success: true,
            message: $msg,
            data: (),
        })
    }};
    ($state:expr, $call:expr) => {{
        $call($state.client.clone()).await;
        download_image!(@inner format!("Executed fn[{}]", stringify!($call)))
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

#[post("/all")]
pub async fn download_all(state: Data<AppState>) -> impl Responder {
    macro_rules! spawn_thread {
        ($func:ident) => {{
            let _ = tokio::spawn($func(state.client.clone()));
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
        match crate::setup::avif_converter::convert_folder_avif(&format!("{}/{}", $src, $folder))
            .await
        {
            Ok(_) => println!("Conversão de '{}' concluída", $folder),
            Err(e) => eprintln!("Erro na conversão de '{}': {}", $folder, e),
        }
    }};
}

#[post("/sprite")]
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

#[post("/compress")]
pub async fn compress_images() -> impl Responder {
    #[cfg(feature = "dev-routes")]
    {
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
            convert_folder!("img", folder);
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

#[derive(Embed)]
#[folder = "img/abilities"]
struct Abilities;

// #[derive(Embed)]
// #[folder = "img/centered"]
// struct Centered;

// #[derive(Embed)]
// #[folder = "img/splash"]
// struct Splash;

#[derive(Embed)]
#[folder = "img/champions"]
struct Champions;

#[derive(Embed)]
#[folder = "img/items"]
struct Items;

// #[derive(Embed)]
// #[folder = "img/other"]
// struct Other;

#[derive(Embed)]
#[folder = "img/runes"]
struct Runes;

// #[derive(generator_macros::ServeEmbed)]
struct _Stats;

// macro_rules! embed {
// () => {
// paste::paste! {
//     #[derive(Embed)]
//     struct [<$name:camel>];
// pub async fn [<serve_ $name:snake>](path: Path<String>) -> impl Responder {
//     let embedded_file = $name::get(&path);
//     match embedded_file {
//         Some(file) => HttpResponse::Ok()
//             .content_type($content_type)
//             .body(file.data),
//         None => HttpResponse::NotFound().finish(),
//     }
// }
// }
// };
// }

// #[get("/{folder}/{filename}")]
// pub async fn serve_imgs(path_args: Path<ImgFile>) -> impl Responder {
//     let embedded_file = match path_args.folder {
//         ImgFolder::Abilities => Abilities::get(&path_args.filename),
//         ImgFolder::Champions => Champions::get(&path_args.filename),
//         ImgFolder::Items => Items::get(&path_args.filename),
//         ImgFolder::Other => Other::get(&path_args.filename),
//         ImgFolder::Runes => Runes::get(&path_args.filename),
//         ImgFolder::Stats => Stats::get(&path_args.filename),
//     };

//     match embedded_file {
//         Some(file) => HttpResponse::Ok()
//             .content_type(file.metadata.c)
//             .body(file.data),
//         None => HttpResponse::NotFound().finish(),
//     }
// }
