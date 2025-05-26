#![allow(dead_code)]

mod middlewares;
mod model;
mod server;
mod services;
mod setup;

use actix_files::Files;
use middlewares::password::password_middleware;
use model::application::GlobalCache;
use server::{
    games::{calculator_handler, realtime_handler},
    images::{download_arts, download_instances, download_items, download_runes},
    internal::{
        internal_append_prettified_item_stats, internal_generate_writer_files,
        internal_identify_damaging_items, internal_replace_item_names_with_ids,
    },
    update::{update_champions, update_items, update_meta_items, update_project, update_riot},
};

use actix_web::{
    App, HttpServer, main,
    middleware::from_fn,
    web::{Data, scope},
};
use dotenvy::dotenv;
use setup::update::load_cache;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{env, io::Result, sync::Arc};

pub struct AppState {
    db: Pool<Postgres>,
    cache: Arc<GlobalCache>,
    password: String,
}

#[main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cache = Arc::new(load_cache());
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
            .service(Files::new("/cdn", "src/img"))
            .service(
                scope("/api")
                    .service(
                        scope("/games")
                            .service(realtime_handler)
                            .service(calculator_handler),
                    )
                    .service(
                        scope("/update")
                            .wrap(from_fn(password_middleware))
                            .service(update_project)
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
                            .service(internal_replace_item_names_with_ids),
                    ),
            )
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
