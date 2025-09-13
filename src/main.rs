mod server;

#[cfg(feature = "dev")]
use crate::server::dev::{images::*, internal::*, setup::*, statics::*, update::*};
use crate::server::stream::realtime_ws;
use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    dev::HttpServiceFactory,
    http::header,
    middleware::DefaultHeaders,
    web::{self, Data, scope},
};
use dotenvy::dotenv;
use server::{games::*, img::*};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
    #[cfg(feature = "dev")]
    pub client: reqwest::Client,
}

fn api_scope() -> impl HttpServiceFactory + 'static {
    let api_routes = scope("/api").service(
        scope("/games")
            .service(realtime_ws)
            .service(realtime_handler)
            .service(calculator_handler)
            .service(create_game_handler)
            .service(get_by_code_handler),
    );

    #[cfg(feature = "dev")]
    let api_routes = api_routes.service(
        scope("")
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    #[cfg(feature = "dev")]
    let client = reqwest::Client::new();
    let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = std::env::var("HOST").expect("HOST is not set");
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
                let not_found_text = "Unimplemented route. Check methods and paths";
                HttpResponse::NotFound().body(not_found_text)
            }))
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
