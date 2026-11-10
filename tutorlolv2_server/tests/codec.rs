use actix_web::{App, HttpResponse, http::StatusCode, test, web};
use tutorlolv2_server::server::codec::Bincode;
use tutorlolv2_types::BINCODE_CONFIG;

async fn echo(Bincode(value): Bincode<String>) -> HttpResponse {
    HttpResponse::Ok().body(value)
}

#[actix_web::test]
async fn extracts_owned_value_and_rejects_invalid_or_trailing_bytes() {
    let app = test::init_service(App::new().route("/", web::post().to(echo))).await;
    let encoded = bincode::encode_to_vec("hello", BINCODE_CONFIG).unwrap();
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/")
            .set_payload(encoded.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(test::read_body(response).await, "hello");

    let mut trailing = encoded;
    trailing.push(0);
    for bytes in [vec![255], trailing] {
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/")
                .set_payload(bytes)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[actix_web::test]
async fn respects_actix_payload_limit() {
    let app = test::init_service(
        App::new()
            .app_data(web::PayloadConfig::new(4))
            .route("/", web::post().to(echo)),
    )
    .await;
    let bytes = bincode::encode_to_vec("hello", BINCODE_CONFIG).unwrap();
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/")
            .set_payload(bytes)
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
