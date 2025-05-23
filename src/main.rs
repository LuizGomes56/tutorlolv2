#![allow(unused_imports)]

mod model;
mod server;
mod services;
mod setup;

use model::{application::GlobalCache, riot::RiotRealtime};
use server::{
    games::realtime_handler,
    images::{download_arts, download_instances, download_items, download_runes},
    update::{setup_project, update_champions, update_items, update_meta_items, update_riot},
};

use actix_web::{App, HttpServer, main, web::Data};
use dotenvy::dotenv;
use services::realtime::calculate;
use setup::update::{
    append_prettified_item_stats, generate_writers, initialize_items, load_cache, read_from_file,
    setup_champion_cache, update_riot_cache, write_to_file,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{io::Result, sync::Arc};

pub struct AppState {
    db: Pool<Postgres>,
    cache: Arc<GlobalCache>,
}

#[allow(unreachable_code)]
#[main]
async fn main() -> Result<()> {
    dotenv().ok();

    // unsafe {
    //     generate_writers().await;
    // }
    // setup_champion_cache();
    // initialize_items();

    // append_prettified_item_stats().await;
    // return Ok(());

    let cache = Arc::new(load_cache().await);

    // let game = read_from_file::<RiotRealtime>("sample.json");

    // let start_time = std::time::Instant::now();

    // let data = calculate(&cache, &game, String::from("4645"));

    // println!("Time elapsed: {:.2?}", start_time.elapsed());

    // write_to_file(
    //     "test.json",
    //     serde_json::to_string_pretty(&data).unwrap().as_bytes(),
    // );

    // return Ok(());

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
