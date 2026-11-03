pub mod error;
pub use error::*;

use actix_web::{
    HttpRequest, HttpResponse, Responder, ResponseError,
    body::BoxBody,
    http::{
        StatusCode,
        header::{CACHE_CONTROL, HeaderName, HeaderValue, RETRY_AFTER},
    },
};
use bincode::Encode;
use tutorlolv2_protocol::ApiResponse;

pub type ApiResult<T> = Result<ApiReply<T>, AppError>;
pub type AppResult<T> = Result<T, AppError>;

/// A structured response. Existing raw/empty/browser handlers remain unchanged.
pub struct ApiReply<T> {
    value: T,
    status: StatusCode,
    headers: Vec<(HeaderName, HeaderValue)>,
}
impl<T: Encode> ApiReply<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value,
            status: StatusCode::OK,
            headers: Vec::new(),
        }
    }
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.push((name, value));
        self
    }
    pub fn no_store(self) -> Self {
        self.header(CACHE_CONTROL, HeaderValue::from_static("no-store"))
    }
    pub fn into_response(self) -> HttpResponse {
        let envelope = match ApiResponse::new(self.status.as_u16(), Ok(self.value)) {
            Ok(value) => value,
            Err(_) => {
                return AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid response status")
                    .error_response();
            }
        };
        match encode_success(envelope) {
            Ok(bytes) => {
                let mut response = HttpResponse::build(self.status);
                for (name, value) in self.headers {
                    response.insert_header((name, value));
                }
                response
                    .content_type(super::codec::OCTET_STREAM)
                    .body(bytes)
            }
            Err(error) => AppError::from(error).trace().error_response(),
        }
    }
}
impl<T: Encode> Responder for ApiReply<T> {
    type Body = BoxBody;
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        if request.method() == actix_web::http::Method::HEAD {
            let valid = ApiResponse::new(self.status.as_u16(), Ok(()));
            let mut response = HttpResponse::build(if valid.is_ok() {
                self.status
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            });
            for (name, value) in self.headers {
                response.insert_header((name, value));
            }
            return response.content_type(super::codec::OCTET_STREAM).finish();
        }
        self.into_response()
    }
}
/// A writer enforces the shared budget before allocating or copying each write.
fn encode_success(value: impl Encode) -> Result<Vec<u8>, bincode::error::EncodeError> {
    struct BoundedWriter(Vec<u8>);
    impl bincode::enc::write::Writer for BoundedWriter {
        fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
            if bytes.len() > tutorlolv2_protocol::MAX_RESPONSE_BYTES.saturating_sub(self.0.len()) {
                return Err(bincode::error::EncodeError::UnexpectedEnd);
            }
            self.0.extend_from_slice(bytes);
            Ok(())
        }
    }
    let mut writer = BoundedWriter(Vec::new());
    bincode::encode_into_writer(value, &mut writer, tutorlolv2_types::BINCODE_CONFIG)?;
    Ok(writer.0)
}

fn encode_error(error: &AppError) -> HttpResponse {
    encode_error_with(error, |envelope| {
        let mut buffer = [0u8; 65536];
        let length =
            bincode::encode_into_slice(envelope, &mut buffer, tutorlolv2_types::BINCODE_CONFIG)?;
        Ok(buffer[..length].to_vec())
    })
}
fn encode_error_with(
    error: &AppError,
    encode: impl FnOnce(
        &(u16, Result<(), &tutorlolv2_protocol::ApiError>),
    ) -> Result<Vec<u8>, bincode::error::EncodeError>,
) -> HttpResponse {
    // A tuple has the same ordered bincode fields as ApiResponse, without cloning unbounded metadata.
    let envelope = (error.http_status.as_u16(), Err(&error.api));
    let (status, bytes) = match encode(&envelope) {
        Ok(bytes) if bytes.len() <= 65536 => (error.http_status, bytes),
        _ => {
            // Independent, bounded encoder; never re-enter ResponseError on failure.
            let fallback = ApiResponse::<()> {
                status: 500,
                result: Err(tutorlolv2_protocol::ApiError {
                    code: tutorlolv2_protocol::ErrorCode::Internal,
                    message: "Error response encoding failed".into(),
                    details: None,
                    traces: Vec::new(),
                }),
            };
            let mut buffer = [0u8; 256];
            let length =
                bincode::encode_into_slice(fallback, &mut buffer, tutorlolv2_types::BINCODE_CONFIG)
                    .expect("fixed fallback fits bounded buffer");
            (StatusCode::INTERNAL_SERVER_ERROR, buffer[..length].to_vec())
        }
    };
    let mut response = HttpResponse::build(status);
    if error.no_store {
        response.insert_header((CACHE_CONTROL, "no-store"));
    }
    if let Some(seconds) = error.retry_after_seconds {
        response.insert_header((RETRY_AFTER, seconds.to_string()));
    }
    response
        .content_type(super::codec::OCTET_STREAM)
        .body(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{ResponseError, body::to_bytes, http::StatusCode};
    use tutorlolv2_protocol::ApiResponse;
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
        let response =
            ApiReply::ok(vec![0u8; tutorlolv2_protocol::MAX_RESPONSE_BYTES]).into_response();
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
        let response = encode_error_with(&AppError::new(StatusCode::BAD_REQUEST, "bad"), |_| {
            Err(bincode::error::EncodeError::Other("failed"))
        });
        let bytes = to_bytes(response.into_body()).await.unwrap();
        assert!(bytes.len() < 1024);
        assert!(
            ApiResponse::<()>::decode(&bytes, 500)
                .unwrap()
                .result
                .is_err()
        );
    }
}
