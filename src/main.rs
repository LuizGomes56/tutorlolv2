mod model;
mod server;
mod services;
mod setup;

use model::application::GlobalCache;
use server::{
    games::realtime_handler,
    update::{setup_project, update_champions, update_items},
};

use actix_web::{App, HttpServer, main, web::Data};
use dotenvy::dotenv;
use setup::base::{load_cache, setup_champion_cache};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{io::Result, sync::Arc};

pub struct AppState {
    db: Pool<Postgres>,
    cache: Arc<GlobalCache>,
}

#[allow(unreachable_code)]
#[main]
async fn main() -> Result<()> {
    let cache = Arc::new(load_cache().await);

    // setup_champion_cache();

    // return Ok(());

    dotenv().ok();

    let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in the environment");
    let port = std::env::var("PORT").expect("PORT is not set in the environment");

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
            }))
            .service(realtime_handler)
            .service(setup_project)
            .service(update_champions)
            .service(update_items)
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
