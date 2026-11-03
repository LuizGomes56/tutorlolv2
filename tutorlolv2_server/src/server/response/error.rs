use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::{error::Error, fmt, panic::Location};
use tutorlolv2_protocol::{ApiError, ErrorCode, ErrorTrace};

#[derive(Debug)]
pub struct AppError {
    pub(crate) http_status: StatusCode,
    pub(crate) api: ApiError,
    pub(crate) retry_after_seconds: Option<u64>,
    pub(crate) no_store: bool,
    source: Option<Box<dyn Error + Send + Sync>>,
}
impl AppError {
    #[track_caller]
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            http_status: if status.is_client_error() || status.is_server_error() {
                status
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            api: ApiError {
                code: code_for(status),
                message: message.into(),
                details: None,
                traces: Vec::new(),
            },
            retry_after_seconds: None,
            no_store: false,
            source: None,
        }
    }
    #[track_caller]
    pub fn status(mut self, status: StatusCode) -> Self {
        self.http_status = if status.is_client_error() || status.is_server_error() {
            status
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        self.api.code = code_for(self.http_status);
        self
    }
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.api.message = message.into();
        self
    }
    pub fn code(mut self, code: ErrorCode) -> Self {
        self.api.code = code;
        self
    }
    pub fn no_store(mut self) -> Self {
        self.no_store = true;
        self
    }
    pub fn retry_after(mut self, seconds: u64) -> Self {
        self.retry_after_seconds = Some(seconds);
        self
    }
    pub fn status_code(&self) -> StatusCode {
        self.http_status
    }
    pub fn as_api_error(&self) -> &ApiError {
        &self.api
    }
    #[track_caller]
    pub fn trace(self) -> Self {
        self.push_trace(Location::caller(), None)
    }
    #[track_caller]
    pub fn trace_ctx(self, context: impl Into<ErrorContext>) -> Self {
        self.with_context(Location::caller(), context.into())
    }
    fn push_trace(mut self, location: &'static Location<'static>, message: Option<String>) -> Self {
        let file = location
            .file()
            .strip_prefix(concat!(env!("CARGO_MANIFEST_DIR"), "/../"))
            .unwrap_or(location.file());
        self.api.traces.push(ErrorTrace {
            file: file.into(),
            line: location.line(),
            column: location.column(),
            message,
        });
        self
    }
    fn with_context(self, location: &'static Location<'static>, context: ErrorContext) -> Self {
        let error = if let Some(status) = context.status {
            self.status(status).message(context.message.clone())
        } else {
            self
        };
        error.push_trace(location, Some(context.message))
    }
    /// Request boundaries deliberately classify malformed input as 400. Stored data uses From's 500.
    #[track_caller]
    pub fn request_decode(error: bincode::error::DecodeError) -> Self {
        Self::from(error)
            .status(StatusCode::BAD_REQUEST)
            .message("Invalid request body")
    }
    fn from_source(error: impl Error + Send + Sync + 'static) -> Self {
        let mut value = Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
        value.source = Some(Box::new(error));
        value
    }
}
fn code_for(status: StatusCode) -> ErrorCode {
    match status.as_u16() {
        400 | 422 => ErrorCode::InvalidRequest,
        401 => ErrorCode::Unauthorized,
        403 => ErrorCode::Forbidden,
        404 => ErrorCode::NotFound,
        409 => ErrorCode::Conflict,
        410 => ErrorCode::Gone,
        413 => ErrorCode::PayloadTooLarge,
        415 => ErrorCode::UnsupportedFormat,
        429 => ErrorCode::RateLimited,
        502..=504 => ErrorCode::Upstream,
        _ => ErrorCode::Internal,
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.http_status, self.api.message)
    }
}
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as _)
    }
}
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.http_status
    }
    fn error_response(&self) -> HttpResponse {
        super::encode_error(self)
    }
}
macro_rules! source_from {
    ($($ty:ty),* $(,)?) => {$ (impl From<$ty> for AppError {#[track_caller] fn from(error: $ty) -> Self {Self::from_source(error)}})*};
}
source_from!(
    sqlx::Error,
    bincode::error::EncodeError,
    bincode::error::DecodeError,
    serde_json::Error,
    reqwest::Error,
    std::io::Error,
    uuid::Error,
    std::num::ParseIntError,
    std::num::TryFromIntError,
    getrandom::Error,
    tokio::task::JoinError
);

pub struct ErrorContext {
    message: String,
    status: Option<StatusCode>,
}
impl From<&str> for ErrorContext {
    fn from(message: &str) -> Self {
        Self {
            message: message.into(),
            status: None,
        }
    }
}
impl From<String> for ErrorContext {
    fn from(message: String) -> Self {
        Self {
            message,
            status: None,
        }
    }
}
impl<M: Into<String>> From<(M, StatusCode)> for ErrorContext {
    fn from((message, status): (M, StatusCode)) -> Self {
        Self {
            message: message.into(),
            status: Some(status),
        }
    }
}

pub trait Trace<T>: Sized {
    #[track_caller]
    fn trace(self) -> Result<T, AppError>;
    #[track_caller]
    fn trace_ctx(self, context: impl Into<ErrorContext>) -> Result<T, AppError>;
}
impl<T, E: Into<AppError>> Trace<T> for Result<T, E> {
    #[track_caller]
    fn trace(self) -> Result<T, AppError> {
        let location = Location::caller();
        self.map_err(|error| error.into().push_trace(location, None))
    }
    #[track_caller]
    fn trace_ctx(self, context: impl Into<ErrorContext>) -> Result<T, AppError> {
        let location = Location::caller();
        self.map_err(|error| error.into().with_context(location, context.into()))
    }
}
impl<T> Trace<T> for Option<T> {
    #[track_caller]
    fn trace(self) -> Result<T, AppError> {
        let location = Location::caller();
        self.ok_or_else(|| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Unexpected absence")
                .push_trace(location, None)
        })
    }
    #[track_caller]
    fn trace_ctx(self, context: impl Into<ErrorContext>) -> Result<T, AppError> {
        let location = Location::caller();
        self.ok_or_else(|| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Unexpected absence")
                .with_context(location, context.into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn traces_preserve_caller_source_status_and_order() {
        let expected = (line!() + 2, 14);
        let error = Err::<(), _>(std::io::Error::other("db"))
            .trace()
            .unwrap_err();
        assert_eq!(
            (error.api.traces[0].line, error.api.traces[0].column),
            expected
        );
        let error = error.status(StatusCode::CONFLICT);
        let line = line!() + 1;
        let error = Err::<(), _>(error).trace_ctx("outer").unwrap_err();
        assert_eq!(error.http_status, StatusCode::CONFLICT);
        assert_eq!(error.api.traces.len(), 2);
        assert_eq!(error.api.traces[1].line, line);
        assert_eq!(error.api.traces[1].message.as_deref(), Some("outer"));
        assert_eq!(error.source().unwrap().to_string(), "db");
    }
    #[test]
    fn deliberate_error_traces_once_at_each_explicit_call() {
        let error = AppError::new(StatusCode::GONE, "ended");
        assert!(error.api.traces.is_empty());
        let line = line!() + 1;
        let error = error.trace();
        assert_eq!(error.api.traces.len(), 1);
        assert_eq!(error.api.traces[0].line, line);
        assert_eq!(error.api.traces[0].file, file!());
        let line = line!() + 1;
        let error = error.trace_ctx("read capability");
        assert_eq!(error.api.traces.len(), 2);
        assert_eq!(error.api.traces[1].line, line);
        assert_eq!(error.http_status, StatusCode::GONE);
    }
    #[test]
    fn option_absence_requires_explicit_domain_status() {
        assert_eq!(
            None::<()>.trace().unwrap_err().http_status,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        let line = line!() + 2;
        let error = None::<()>
            .trace_ctx(("missing", StatusCode::NOT_FOUND))
            .unwrap_err();
        assert_eq!(error.http_status, StatusCode::NOT_FOUND);
        assert_eq!(error.api.traces.len(), 1);
        assert_eq!(error.api.traces[0].line, line);
    }
    #[actix_web::test]
    async fn completed_async_result_captures_await_site() {
        let line = line!() + 3;
        let error = async { Err::<(), _>(std::io::Error::other("async")) }
            .await
            .trace()
            .unwrap_err();
        assert_eq!(error.api.traces[0].line, line);
    }
}
