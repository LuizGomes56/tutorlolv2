#![allow(unused_imports)]
mod model;
mod server;
mod services;
mod setup;

use model::{application::GlobalCache, riot::RiotRealtime};
use server::{
    games::{calculator_handler, realtime_handler},
    images::{download_arts, download_instances, download_items, download_runes},
    update::{setup_project, update_champions, update_items, update_meta_items, update_riot},
};

use actix_web::{App, HttpServer, main, web::Data};
use dotenvy::dotenv;
use services::realtime::realtime;
use setup::update::{
    append_prettified_item_stats, generate_writers, initialize_items, load_cache, read_from_file,
    setup_champion_cache, update_riot_cache, write_to_file,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{env, io::Result, sync::Arc};

pub struct AppState {
    db: Pool<Postgres>,
    cache: Arc<GlobalCache>,
    password: String,
}

#[allow(unreachable_code)]
#[main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cache = Arc::new(load_cache().await);
    let dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the environment");
    let port = env::var("PORT").expect("PORT is not set in the environment");
    let host = format!("127.0.0.1:{}", port);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .expect("Error when trying to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                cache: cache.clone(),
                password: env::var("SYSTEM_PASSWORD").expect("SYSTEM_PASSWORD is not set"),
            }))
            .service(realtime_handler)
            .service(calculator_handler)
            .service(setup_project)
            .service(update_riot)
            .service(update_champions)
            .service(update_items)
            .service(update_meta_items)
            .service(download_instances)
            .service(download_items)
            .service(download_arts)
            .service(download_runes)
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
