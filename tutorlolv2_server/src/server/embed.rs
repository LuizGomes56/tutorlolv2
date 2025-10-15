use actix_files::Files;
use actix_web::{
    HttpResponse, Responder, dev::HttpServiceFactory, get, http::header::CONTENT_ENCODING,
    web::Path,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../html/zstd/champions"]
struct ChampionsDocs;

#[derive(Embed)]
#[folder = "../html/zstd/items"]
struct ItemsDocs;

#[derive(Embed)]
#[folder = "../html/zstd/runes"]
struct RunesDocs;

#[derive(Embed)]
#[folder = "../img/abilities"]
struct Abilities;

#[derive(Embed)]
#[folder = "../img/champions"]
struct Champions;

#[derive(Embed)]
#[folder = "../img/items"]
struct Items;

#[derive(Embed)]
#[folder = "../img/runes"]
struct Runes;

#[derive(Embed)]
#[folder = "../img/stats"]
struct Stats;

macro_rules! serve_embed {
    ($name:ident, $path:literal, $content_type:literal) => {
        paste::paste! {
            #[doc = "Serve files from `" $path "`"]
            #[get($path)]
            pub async fn [<serve_ $name>] (path: Path<String>) -> impl Responder {
                match [<$name:camel>]::get(&path) {
                    Some(file) => HttpResponse::Ok()
                        .content_type($content_type)
                        .body(file.data),
                    None => HttpResponse::NotFound().finish(),
                }
            }
        }
    };
    (@$compressor:ident $name:ident, $path:literal, $content_type:literal) => {
        paste::paste! {
            #[doc = "Serve files from `" $path "`"]
            #[get($path)]
            pub async fn [<serve_ $name>] (path: Path<String>) -> impl Responder {
                match [<$name:camel>]::get(&path) {
                    Some(file) => HttpResponse::Ok()
                        .content_type($content_type)
                        .insert_header((CONTENT_ENCODING, stringify!($compressor)))
                        .body(file.data),
                    None => HttpResponse::NotFound().finish(),
                }
            }
        }
    };
}

serve_embed!(abilities, "/abilities/{path:.*}", "image/avif");
serve_embed!(champions, "/champions/{path:.*}", "image/avif");
serve_embed!(items, "/items/{path:.*}", "image/avif");
serve_embed!(runes, "/runes/{path:.*}", "image/avif");
serve_embed!(stats, "/stats/{path:.*}", "image/svg+xml");
serve_embed!(@zstd champions_docs, "/champions/{path:.*}", "text/html");
serve_embed!(@zstd items_docs, "/items/{path:.*}", "text/html");
serve_embed!(@zstd runes_docs, "/runes/{path:.*}", "text/html");

macro_rules! serve_dyn {
    ($f:ident) => {
        paste::paste! {
            pub fn [<serve_dyn_ $f>]() -> impl HttpServiceFactory + 'static {
                Files::new(concat!("/", stringify!($f)), concat!("img/", stringify!($f)))
                    .use_etag(false)
                    .use_last_modified(false)
            }
        }
    };
}

serve_dyn!(other);
serve_dyn!(centered);
serve_dyn!(splash);
