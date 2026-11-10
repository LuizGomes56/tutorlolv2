use actix_web::Responder;
use actix_web::{ResponseError, body::to_bytes, http::StatusCode};
use tutorlolv2_protocol::ApiResponse;
use tutorlolv2_server::server::response::*;
#[actix_web::test]
async fn error_response_preserves_status_and_policy() {
    let response = AppError::new(StatusCode::TOO_MANY_REQUESTS, "wait")
        .retry_after(3)
        .no_store()
        .error_response();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(response.headers().get("retry-after").unwrap(), "3");
    assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
    let bytes = to_bytes(response.into_body()).await.unwrap();
    assert!(
        ApiResponse::<String>::decode(&bytes, 429)
            .unwrap()
            .result
            .is_err()
    );
}
struct CannotEncode;
impl bincode::Encode for CannotEncode {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        _: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Err(bincode::error::EncodeError::Other("deliberate"))
    }
}
#[actix_web::test]
async fn oversized_success_becomes_bounded_error() {
    let response = ApiReply::ok(vec![0u8; tutorlolv2_protocol::MAX_RESPONSE_BYTES]).into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let bytes = to_bytes(response.into_body()).await.unwrap();
    assert!(bytes.len() < 65536);
    assert!(
        ApiResponse::<()>::decode(&bytes, 500)
            .unwrap()
            .result
            .is_err()
    );
}
#[actix_web::test]
async fn head_does_not_encode_a_body() {
    let request = actix_web::test::TestRequest::default()
        .method(actix_web::http::Method::HEAD)
        .to_http_request();
    let response = ApiReply::ok(CannotEncode).no_store().respond_to(&request);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
    assert!(to_bytes(response.into_body()).await.unwrap().is_empty());
}
#[actix_web::test]
async fn encoding_failure_produces_bounded_internal_error() {
    let response = ApiReply::ok(CannotEncode).into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let bytes = to_bytes(response.into_body()).await.unwrap();
    assert!(
        ApiResponse::<()>::decode(&bytes, 500)
            .unwrap()
            .result
            .is_err()
    );
    let response = AppError::new(StatusCode::BAD_REQUEST, "bad")
        .encode_response_with(|_| Err(bincode::error::EncodeError::Other("failed")));
    let bytes = to_bytes(response.into_body()).await.unwrap();
    assert!(bytes.len() < 1024);
    assert!(
        ApiResponse::<()>::decode(&bytes, 500)
            .unwrap()
            .result
            .is_err()
    );
}
