#![allow(dead_code)]

mod middlewares;
mod model;
mod server;
mod services;
mod setup;
mod writers;

use actix_cors::Cors;
use actix_files::Files;
use middlewares::{logger::logger_middleware, password::password_middleware};
use model::application::GlobalCache;
use server::{formulas::*, games::*, images::*, internal::*, setup::*, statics::*, update::*};

use actix_web::{
    App, HttpServer, http, main,
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
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState {
                db: pool.clone(),
                cache: cache.clone(),
                password: env::var("SYSTEM_PASSWORD").expect("SYSTEM_PASSWORD is not set"),
            }))
            .service(Files::new("/cdn", "src/img"))
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
                            .service(internal_replace_item_names_with_ids),
                    ),
            )
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
