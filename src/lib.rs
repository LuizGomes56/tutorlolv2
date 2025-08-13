mod generators;
mod init;
mod middlewares;
mod model;
mod server;
mod services;
mod setup;

use crate::init::release::AppState;
#[cfg(feature = "dev")]
use crate::{
    middlewares::{
        jsonmw::json_error_middleware, logger::logger_middleware, password::password_middleware,
    },
    server::dev::{images::*, internal::*, setup::*, statics::*, update::*},
};
use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    dev::HttpServiceFactory,
    http::header,
    middleware::DefaultHeaders,
    web::{self, Data, scope},
};
#[cfg(feature = "dev")]
use actix_web::{middleware::from_fn, mime, web::JsonConfig};
use dotenvy::dotenv;
use server::{games::*, img::*, schemas::APIResponse};
use sqlx::postgres::PgPoolOptions;
use std::io;

pub use internal_comptime::*;

fn api_scope() -> impl HttpServiceFactory + 'static {
    let api_routes = scope("/api").service(
        scope("/games")
            .service(realtime_handler)
            .service(calculator_handler)
            .service(create_game_handler)
            .service(get_by_code_handler),
    );

    #[cfg(feature = "dev")]
    let api_routes = api_routes.wrap(from_fn(logger_middleware)).service(
        scope("")
            .wrap(from_fn(password_middleware))
            .app_data(
                JsonConfig::default()
                    .content_type(|mime| mime == mime::APPLICATION_JSON)
                    .error_handler(json_error_middleware),
            )
            .service(scope("/static").service(static_comptime))
            .service(
                scope("/setup")
                    .service(setup_champions)
                    .service(setup_folders)
                    .service(setup_project)
                    .service(setup_items)
                    .service(setup_runes),
            )
            .service(
                scope("/update")
                    .service(update_riot)
                    .service(update_champions)
                    .service(update_items)
                    .service(update_meta_items)
                    .service(update_version),
            )
            .service(
                scope("/internal")
                    .service(internal_create_generator_files)
                    .service(internal_prettify_item_stats)
                    .service(internal_create_damaging_items)
                    .service(internal_create_meta_items)
                    .service(internal_rewrite_champion_names)
                    .service(internal_assign_item_damages),
            )
            .service(
                scope("/images")
                    .service(download_instances)
                    .service(download_items)
                    .service(download_arts)
                    .service(download_runes)
                    .service(download_all)
                    .service(generate_sprites)
                    .service(compress_images),
            ),
    );

    api_routes
}

pub async fn run() -> io::Result<()> {
    dotenv().ok();

    #[cfg(feature = "dev")]
    let client = reqwest::Client::new();
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
                    #[cfg(feature = "dev")]
                    client: client.clone(),
                })
            })
            .service(api_scope())
            .service(
                scope("/img")
                    .wrap(
                        DefaultHeaders::new()
                            .add((header::CACHE_CONTROL, "public, max-age=31536000, immutable")),
                    )
                    .service(serve_dyn_centered())
                    .service(serve_dyn_splash())
                    .service(serve_dyn_other())
                    .service(serve_abilities)
                    .service(serve_champions)
                    .service(serve_items)
                    .service(serve_runes)
                    .service(serve_stats),
            )
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
