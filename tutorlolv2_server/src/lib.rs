mod server;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    dev::HttpServiceFactory,
    http::header,
    middleware::DefaultHeaders,
    web::{
        self,
        // Data,
        scope,
    },
};
use server::{embed::*, games::*};
// use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

// pub struct AppState {
//     pub db: Pool<Postgres>,
// }

fn api_scope() -> impl HttpServiceFactory + 'static {
    scope("/api").service(
        scope("/games")
            .service(realtime_handler)
            .service(calculator_handler),
        // .service(create_game_handler),
        // .service(get_by_code_handler),
    )
}

pub async fn run() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8082".to_string());

    println!("Starting server on port {port}");

    let host = format!("127.0.0.1:{port}");
    // let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&dsn)
    //     .await
    //     .expect("Error while attempting to connect to the database");

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
            .wrap(actix_web::middleware::Logger::default())
            // .app_data(Data::new(AppState { db: pool.clone() }))
            .service(api_scope())
            .service(
                scope("")
                    // .service(
                    //     scope("/docs")
                    //         .service(serve_champions_docs)
                    //         .service(serve_items_docs)
                    //         .service(serve_runes_docs),
                    // )
                    .service(
                        scope("/img")
                            .wrap(DefaultHeaders::new().add((
                                header::CACHE_CONTROL,
                                "public, max-age=31536000, immutable",
                            )))
                            .service(serve_dyn_centered())
                            .service(serve_dyn_splash())
                            .service(serve_dyn_other())
                            .service(serve_abilities)
                            .service(serve_champions)
                            .service(serve_items)
                            .service(serve_runes)
                            .service(serve_stats),
                    ),
            )
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().body("Unimplemented route. Check methods and paths")
            }))
    })
    .bind(host)
    .expect("Some error has ocurred when starting the server")
    .run()
    .await
}
