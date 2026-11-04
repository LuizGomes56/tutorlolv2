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

impl AppError {
    /// Encodes this error as the standard bincode failure envelope.
    pub(crate) fn encode_response(&self) -> HttpResponse {
        self.encode_response_with(|envelope| {
            let mut buffer = [0u8; 65536];
            let length = bincode::encode_into_slice(
                envelope,
                &mut buffer,
                tutorlolv2_types::BINCODE_CONFIG,
            )?;
            Ok(buffer[..length].to_vec())
        })
    }

    /// Like [`Self::encode_response`] with a custom encoder; an encoding failure or an
    /// oversized body falls back to a fixed, bounded 500 without re-entering `ResponseError`.
    pub fn encode_response_with(
        &self,
        encode: impl FnOnce(
            &(u16, Result<(), &tutorlolv2_protocol::ApiError>),
        ) -> Result<Vec<u8>, bincode::error::EncodeError>,
    ) -> HttpResponse {
        // A tuple has the same ordered bincode fields as ApiResponse, without cloning unbounded metadata.
        let envelope = (self.http_status.as_u16(), Err(&self.api));
        let (status, bytes, metadata) = match encode(&envelope) {
            Ok(bytes) if bytes.len() <= 65536 => (self.http_status, bytes, self.api.clone()),
            _ => {
                // Independent, bounded encoder; never re-enter ResponseError on failure.
                let fallback = ApiResponse::<()> {
                    status: 500,
                    result: Err(tutorlolv2_protocol::ApiError {
                        code: tutorlolv2_protocol::ErrorCode::Internal,
                        message: "Error response encoding failed".into(),
                        details: None,
                        traces: vec![tutorlolv2_protocol::ErrorTrace {
                            file: "tutorlolv2_server/src/server/response/mod.rs".into(),
                            line: line!(),
                            column: column!(),
                            message: None,
                        }],
                    }),
                };

                let metadata = fallback.result.as_ref().unwrap_err().clone();
                let mut buffer = [0u8; 256];
                let length = bincode::encode_into_slice(
                    fallback,
                    &mut buffer,
                    tutorlolv2_types::BINCODE_CONFIG,
                )
                .expect("fixed fallback fits bounded buffer");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    buffer[..length].to_vec(),
                    metadata,
                )
            }
        };

        let mut response = HttpResponse::build(status);

        for (name, value) in &self.headers {
            response.insert_header((name.clone(), value.clone()));
        }

        if self.no_store {
            response.insert_header((CACHE_CONTROL, "no-store"));
        }

        if let Some(seconds) = self.retry_after_seconds {
            response.insert_header((RETRY_AFTER, seconds.to_string()));
        }

        let mut response = response
            .content_type(super::codec::OCTET_STREAM)
            .body(bytes);

        // Only clone bounded, successfully encoded metadata. Browser middleware can
        // render pre-handler errors without buffering or decoding response bodies.
        response.extensions_mut().insert(metadata);
        response
    }
}

/// Human-facing OAuth navigation cannot decode a binary API error response.
#[derive(Debug)]
pub struct BrowserError(pub AppError);

impl std::fmt::Display for BrowserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for BrowserError {}

impl From<AppError> for BrowserError {
    fn from(error: AppError) -> Self {
        Self(error)
    }
}

impl ResponseError for BrowserError {
    fn status_code(&self) -> StatusCode {
        self.0.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        let escape = |value: &str| {
            value
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#39;")
        };

        let error = self.0.as_api_error();
        let mut details = error.details.clone().unwrap_or_default();

        for trace in &error.traces {
            use std::fmt::Write;

            let _ = writeln!(
                details,
                "\n{}:{}:{} {}",
                trace.file,
                trace.line,
                trace.column,
                trace.message.as_deref().unwrap_or("")
            );
        }

        HttpResponse::build(self.status_code())
            .insert_header((CACHE_CONTROL, "no-store"))
            .content_type("text/html; charset=utf-8")
            .body(format!(
                "
                <!doctype html>
                <title>Sign-in error</title>
                <h1>{}</h1>
                <p>Return to the app and retry sign-in.</p>
                <pre>{}</pre>
                ",
                escape(&error.message),
                escape(&details)
            ))
    }
}

pub trait BrowserContext<T> {
    fn browser(self) -> Result<T, BrowserError>;
}

impl<T> BrowserContext<T> for AppResult<T> {
    fn browser(self) -> Result<T, BrowserError> {
        self.map_err(BrowserError::from)
    }
}
