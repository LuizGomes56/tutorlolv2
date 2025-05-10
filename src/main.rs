mod model;
mod server;
mod services;

use server::{
    games::realtime_handler,
    update::{setup_project, update_champions, update_items},
};

use actix_web::{App, HttpServer, web::Data};
use dotenvy::dotenv;
use services::setup::setup_champion_cache;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup_champion_cache();

    // return Ok(());

    dotenv().ok();

    let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL não definida no ambiente");
    let port = std::env::var("PORT").expect("PORT não definida no ambiente");

    let host = format!("127.0.0.1:{}", port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .expect("Erro ao conectar ao banco de dados");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(realtime_handler)
            .service(setup_project)
            .service(update_champions)
            .service(update_items)
    })
    .bind(host)
    .expect("A porta do servidor não foi definida")
    .run()
    .await
}
