pub mod server;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    http::header,
    middleware::DefaultHeaders,
    web::{self, Data, scope},
};
use server::{
    config::Config,
    janitor,
    routes::{RateLimits, api},
    state::AppState,
    statics::*,
};
use sqlx::postgres::PgPoolOptions;

pub async fn run() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8082u16);

    let statics = Data::new(StaticDir::from_env()?);
    let config = Config::from_env().map_err(std::io::Error::other)?;

    let db = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .map_err(std::io::Error::other)?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .map_err(std::io::Error::other)?;

    // Built once: every worker's App shares the same counters
    let limits = RateLimits::new(config.trusted_proxy);
    let state = Data::new(AppState::new(db, config));

    janitor::spawn(state.clone());

    println!("Starting server on port {port}");
    println!("Serving static files from {}", statics.path().display());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(state.clone())
            .app_data(statics.clone())
            .service(api(Some(limits.clone())))
            .service(
                scope("")
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allowed_methods(["GET"])
                            .max_age(3600),
                    )
                    .service(scope("/docs").service(serve_docs))
                    .service(
                        scope("/img")
                            .wrap(DefaultHeaders::new().add((
                                header::CACHE_CONTROL,
                                "public, max-age=31536000, immutable",
                            )))
                            .service(statics.images()),
                    ),
            )
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().body("Unimplemented route. Check methods and paths")
            }))
    })
    .bind(("127.0.0.1", port))
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
