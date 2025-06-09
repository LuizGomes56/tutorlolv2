#![allow(dead_code)]

mod middlewares;
mod model;
mod server;
mod services;
mod setup;
mod writers;

use actix_cors::Cors;
use actix_files::Files;
use middlewares::{
    error::json_error_middleware, logger::logger_middleware, password::password_middleware,
};
use model::application::GlobalCache;
use reqwest::Client;
use server::{formulas::*, games::*, images::*, internal::*, setup::*, statics::*, update::*};

use actix_web::{
    App, HttpServer, http, main,
    middleware::from_fn,
    mime::{self, Mime},
    web::{Data, JsonConfig, scope},
};
use dotenvy::dotenv;
use setup::update::load_cache;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{env, io::Result, sync::Arc};

pub struct EnvConfig {
    pub environment: String,
    pub lol_version: String,
    pub lol_language: String,
    pub system_password: String,
    pub cdn_endpoint: String,
    pub dd_dragon_endpoint: String,
    pub riot_image_endpoint: String,
    pub meta_endpoint: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        EnvConfig {
            environment: env::var("ENVIRONMENT").expect("[env] ENVIRONMENT is not set"),
            lol_version: env::var("LOL_VERSION").expect("[env] LOL_VERSION is not set"),
            lol_language: env::var("LOL_LANGUAGE").expect("[env] LOL_LANGUAGE is not set"),
            system_password: env::var("SYSTEM_PASSWORD").expect("[env] SYSTEM_PASSWORD is not set"),
            cdn_endpoint: env::var("CDN_ENDPOINT").expect("[env] CDN_ENDPOINT is not set"),
            dd_dragon_endpoint: env::var("DD_DRAGON_ENDPOINT")
                .expect("[env] DD_DRAGON_ENDPOINT is not set"),
            riot_image_endpoint: env::var("RIOT_IMAGE_ENDPOINT")
                .expect("[env] RIOT_IMAGE_ENDPOINT is not set"),
            meta_endpoint: env::var("META_ENDPOINT").expect("[env] META_ENDPOINT is not set"),
        }
    }
}

pub struct AppState {
    cache: Arc<GlobalCache>,
    client: Client,
    db: Pool<Postgres>,
    envcfg: Arc<EnvConfig>,
}

#[main]
async fn main() -> Result<()> {
    dotenv().ok();

    let envcfg: Arc<EnvConfig> = Arc::new(EnvConfig::new());
    let client: Client = Client::new();
    let cache: Arc<GlobalCache> = match load_cache() {
        Ok(cache) => Arc::new(cache),
        Err(e) => {
            println!("Failed to load cache: {:#?}", e);
            Arc::new(GlobalCache::default())
        }
    };

    let dsn: String = env::var("DATABASE_URL").expect("[env] DATABASE_URL is not set");
    let host: String = env::var("HOST").expect("[env] HOST is not set");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .expect("Error while attempting to connect to the database");

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin()
            // #![todo] Allow only frontend to send requests to this server.
            // .allowed_origin("http://localhost:8080")
            .allowed_methods(["GET", "POST"])
            .allowed_headers([http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data({
                Data::new(AppState {
                    db: pool.clone(),
                    cache: cache.clone(),
                    client: client.clone(),
                    envcfg: envcfg.clone(),
                })
            })
            .app_data(
                JsonConfig::default()
                    .content_type(|mime: Mime| mime == mime::APPLICATION_JSON)
                    .error_handler(json_error_middleware),
            )
            .service(Files::new("/cdn", "img"))
            .service(
                scope("/api")
                    .wrap(from_fn(logger_middleware))
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
                            .service(static_runes),
                    )
                    .service(scope("/formulas").service(formulas_champions))
                    .service(
                        scope("/setup")
                            .wrap(from_fn(password_middleware))
                            .service(setup_champions)
                            .service(setup_folders)
                            .service(setup_project),
                    )
                    .service(
                        scope("/update")
                            .wrap(from_fn(password_middleware))
                            .service(update_riot)
                            .service(update_champions)
                            .service(update_items)
                            .service(update_meta_items),
                    )
                    .service(
                        scope("/images")
                            .wrap(from_fn(password_middleware))
                            .service(download_instances)
                            .service(download_items)
                            .service(download_arts)
                            .service(download_runes),
                    )
                    .service(
                        scope("/internal")
                            .wrap(from_fn(password_middleware))
                            .service(internal_generate_writer_files)
                            .service(internal_append_prettified_item_stats)
                            .service(internal_identify_damaging_items)
                            .service(internal_replace_item_names_with_ids)
                            .service(internal_rewrite_champion_names),
                    ),
            )
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
