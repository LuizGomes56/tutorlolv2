use actix_web::{
    App,
    http::{StatusCode, header},
    test,
    web::{Data, scope},
};
use std::{io::Read, path::Path};
use tutorlolv2_server::server::statics::*;

fn statics() -> Data<StaticDir> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    Data::new(StaticDir::new(root).unwrap())
}

#[actix_web::test]
async fn docs_are_served_with_brotli_or_decompressed_for_other_clients() {
    let app = test::init_service(
        App::new()
            .app_data(statics())
            .service(scope("/docs").service(serve_docs)),
    )
    .await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/docs/champions/Aatrox")
            .insert_header((header::ACCEPT_ENCODING, "gzip, br"))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_ENCODING).unwrap(),
        "br"
    );
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
    let compressed = test::read_body(response).await;
    let mut page = Vec::new();
    brotli_decompressor::Decompressor::new(&compressed[..], 4096)
        .read_to_end(&mut page)
        .unwrap();
    assert!(page.starts_with(b"<!DOCTYPE html>"));

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/docs/champions/Aatrox")
            .insert_header((header::ACCEPT_ENCODING, "gzip"))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get(header::CONTENT_ENCODING).is_none());
    assert_eq!(test::read_body(response).await, page);
}

#[actix_web::test]
async fn unknown_docs_pages_return_not_found() {
    let app = test::init_service(
        App::new()
            .app_data(statics())
            .service(scope("/docs").service(serve_docs)),
    )
    .await;
    for uri in [
        "/docs/champions/NotAChampion",
        "/docs/spells/Aatrox",
        "/docs/champions/Aatrox.html",
        "/docs/champions/..%2F..%2FCargo",
    ] {
        let response =
            test::call_service(&app, test::TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{uri}");
    }
}

#[actix_web::test]
async fn images_are_served_from_the_static_folder() {
    let app =
        test::init_service(App::new().service(scope("/img").service(statics().images()))).await;
    for (uri, content_type) in [
        ("/img/champions/Aatrox.avif", "image/avif"),
        ("/img/centered/Aatrox_0.avif", "image/avif"),
        ("/img/stats/ability_power.svg", "image/svg+xml"),
    ] {
        let response =
            test::call_service(&app, test::TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(response.status(), StatusCode::OK, "{uri}");
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            content_type
        );
    }
    for uri in ["/img/champions/Missing.avif", "/img/../Cargo.toml"] {
        let response =
            test::call_service(&app, test::TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{uri}");
    }
}
