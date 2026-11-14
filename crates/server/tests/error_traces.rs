use actix_web::http::StatusCode;
use server::server::response::*;
use std::error::Error;
#[test]
fn traces_preserve_caller_source_status_and_order() {
    let expected = (line!() + 2, 10);
    let error = Err::<(), _>(std::io::Error::other("db"))
        .trace()
        .unwrap_err();
    assert_eq!(
        (
            error.as_api_error().traces[0].line,
            error.as_api_error().traces[0].column
        ),
        expected
    );
    let error = error.status(StatusCode::CONFLICT);
    let line = line!() + 1;
    let error = Err::<(), _>(error).trace_ctx("outer").unwrap_err();
    assert_eq!(error.status_code(), StatusCode::CONFLICT);
    assert_eq!(error.as_api_error().traces.len(), 2);
    assert_eq!(error.as_api_error().traces[1].line, line);
    assert_eq!(
        error.as_api_error().traces[1].message.as_deref(),
        Some("outer")
    );
    assert_eq!(error.source().unwrap().to_string(), "db");
}
#[test]
fn deliberate_error_traces_once_at_each_explicit_call() {
    let error = AppError::new(StatusCode::GONE, "ended");
    assert!(error.as_api_error().traces.is_empty());
    let line = line!() + 1;
    let error = error.trace();
    assert_eq!(error.as_api_error().traces.len(), 1);
    assert_eq!(error.as_api_error().traces[0].line, line);
    assert_eq!(error.as_api_error().traces[0].file, file!());
    let line = line!() + 1;
    let error = error.trace_ctx("read capability");
    assert_eq!(error.as_api_error().traces.len(), 2);
    assert_eq!(error.as_api_error().traces[1].line, line);
    assert_eq!(error.status_code(), StatusCode::GONE);
}
#[test]
fn option_absence_requires_explicit_domain_status() {
    assert_eq!(
        None::<()>.trace().unwrap_err().status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    let line = line!() + 2;
    let error = None::<()>
        .trace_ctx(("missing", StatusCode::NOT_FOUND))
        .unwrap_err();
    assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
    assert_eq!(error.as_api_error().traces.len(), 1);
    assert_eq!(error.as_api_error().traces[0].line, line);
}
#[actix_web::test]
async fn completed_async_result_captures_await_site() {
    let line = line!() + 3;
    let error = async { Err::<(), _>(std::io::Error::other("async")) }
        .await
        .trace()
        .unwrap_err();
    assert_eq!(error.as_api_error().traces[0].line, line);
}
