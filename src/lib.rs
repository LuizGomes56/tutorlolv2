#![allow(unused_parens)]

pub mod essentials;
pub mod generators;
mod middlewares;
mod model;
mod server;
mod services;
pub mod setup;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    App, HttpResponse, HttpServer, Scope,
    body::{BoxBody, EitherBody},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http::header,
    middleware::{DefaultHeaders, from_fn},
    mime,
    web::{self, Data, JsonConfig, scope},
};
use dotenvy::dotenv;
use middlewares::{
    error::json_error_middleware, logger::logger_middleware, password::password_middleware,
};
use model::cache::*;
use once_cell::sync::Lazy;
use reqwest::Client;
#[cfg(feature = "dev-routes")]
use server::dev::{internal::*, setup::*, update::*};
use server::{formulas::*, games::*, images::*, schemas::APIResponse, statics::*};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{env, io};

include!(concat!(env!("OUT_DIR"), "/internal_champions.rs"));
include!(concat!(env!("OUT_DIR"), "/internal_items.rs"));
include!(concat!(env!("OUT_DIR"), "/internal_runes.rs"));
include!(concat!(env!("OUT_DIR"), "/internal_meta.rs"));
include!(concat!(env!("OUT_DIR"), "/internal_names.rs"));

pub struct EnvConfig {
    pub lol_version: String,
    pub lol_language: String,
    pub system_password: String,
    pub cdn_endpoint: String,
    pub dd_dragon_endpoint: String,
    pub riot_image_endpoint: String,
    pub meta_endpoint: String,
}

macro_rules! env_var {
    ($name:literal) => {
        env::var($name).expect(&format!("[env] {} is not set", $name))
    };
}

impl EnvConfig {
    pub fn new() -> Self {
        EnvConfig {
            lol_version: env_var!("LOL_VERSION"),
            lol_language: env_var!("LOL_LANGUAGE"),
            system_password: env_var!("SYSTEM_PASSWORD"),
            cdn_endpoint: env_var!("CDN_ENDPOINT"),
            dd_dragon_endpoint: env_var!("DD_DRAGON_ENDPOINT"),
            riot_image_endpoint: env_var!("RIOT_IMAGE_ENDPOINT"),
            meta_endpoint: env_var!("META_ENDPOINT"),
        }
    }
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(EnvConfig::new);

pub struct AppState {
    client: Client,
    db: Pool<Postgres>,
}

fn api_scope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let api_routes = scope("/api")
        .wrap(from_fn(logger_middleware))
        .service(
            scope("/formulas")
                .service(formulas_champions)
                .service(formulas_items)
                .service(formulas_runes)
                .service(formulas_abilities)
                .service(formulas_champion_generator),
        )
        .service(
            scope("/games")
                .service(realtime_handler)
                .service(calculator_handler)
                .service(create_game_handler)
                .service(get_by_code_handler),
        )
        .service(
            scope("/static")
                .service(static_champions)
                .service(static_items)
                .service(static_runes)
                .service(static_compared_items),
        )
        .service(
            scope("/images")
                .wrap(from_fn(password_middleware))
                .service(download_instances)
                .service(download_items)
                .service(download_arts)
                .service(download_runes)
                .service(compress_images),
        );

    #[cfg(feature = "dev-routes")]
    let api_routes = api_routes
        .service(
            scope("/setup")
                .wrap(from_fn(password_middleware))
                .service(setup_champions)
                .service(setup_folders)
                .service(setup_project)
                .service(setup_items)
                .service(setup_runes),
        )
        .service(
            scope("/update")
                .wrap(from_fn(password_middleware))
                .service(update_riot)
                .service(update_champions)
                .service(update_items)
                .service(update_meta_items)
                .service(update_version),
        )
        .service(
            scope("/internal")
                .wrap(from_fn(password_middleware))
                .service(internal_create_generator_files)
                .service(internal_prettify_item_stats)
                .service(internal_create_damaging_items)
                .service(internal_create_meta_items)
                .service(internal_rewrite_champion_names)
                .service(internal_assign_item_damages),
        );

    api_routes
}

#[allow(unreachable_code)]
pub async fn run() -> io::Result<()> {
    dotenv().ok();

    let client = Client::new();
    let dsn = env_var!("DATABASE_URL");
    let host = env_var!("HOST");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .expect("Error while attempting to connect to the database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            // #![todo] Allow only frontend to send requests to this server.
            // .allowed_origin("http://localhost:8080")
            .allowed_methods(["GET", "POST"])
            .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data({
                Data::new(AppState {
                    db: pool.clone(),
                    client: client.clone(),
                })
            })
            .app_data(
                JsonConfig::default()
                    .content_type(|mime| mime == mime::APPLICATION_JSON)
                    .error_handler(json_error_middleware),
            )
            .service(
                scope("/img")
                    .wrap(
                        DefaultHeaders::new(), // .add(("Cache-Control", "public, max-age=31536000, immutable")),
                    )
                    .service(
                        Files::new("", "img")
                            .use_etag(false)
                            .use_last_modified(false),
                    ),
            )
            .service(api_scope())
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(APIResponse {
                    success: false,
                    message: "Unimplemented route. Check methods and paths",
                    data: (),
                })
            }))
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
