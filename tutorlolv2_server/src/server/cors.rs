//! Route-specific browser policy. Native clients do not send Origin.
use super::state::AppState;
use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::{
        Method,
        header::{self, HeaderValue},
    },
    middleware::Next,
    web::Data,
};

pub async fn policy(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, Error> {
    let path = req.path();
    let private = path == "/api/account"
        || path == "/api/auth/logout"
        || path == "/api/me"
        || path.starts_with("/api/me/");
    let sensitive = private
        || path.starts_with("/api/auth/")
        || path.starts_with("/api/recordings/") && path.ends_with("/share");
    let origin = req.headers().get(header::ORIGIN).cloned();
    let allowed = !private
        || origin.as_ref().is_none_or(|origin| {
            req.app_data::<Data<AppState>>()
                .is_some_and(|s| origin.as_bytes() == s.config.public_base_url.as_bytes())
        });
    let preflight = req.method() == Method::OPTIONS
        && origin.is_some()
        && req
            .headers()
            .contains_key(header::ACCESS_CONTROL_REQUEST_METHOD);
    let requested_method = req
        .headers()
        .get(header::ACCESS_CONTROL_REQUEST_METHOD)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let requested_headers = req
        .headers()
        .get(header::ACCESS_CONTROL_REQUEST_HEADERS)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let valid_preflight = ["GET", "POST", "PATCH", "PUT"].contains(&requested_method)
        && requested_headers.split(',').all(|v| {
            v.trim().is_empty()
                || ["authorization", "content-type", "x-content-sha256"]
                    .contains(&v.trim().to_ascii_lowercase().as_str())
        });
    let mut response = if !allowed || preflight && !valid_preflight {
        req.into_response(HttpResponse::Forbidden().finish())
            .map_into_right_body()
    } else if preflight {
        req.into_response(HttpResponse::NoContent().finish())
            .map_into_right_body()
    } else {
        next.call(req).await?.map_into_left_body()
    };
    super::google::renew_web_cookie(&mut response).await?;
    let headers = response.headers_mut();
    if sensitive {
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    }
    headers.append(header::VARY, HeaderValue::from_static("Origin"));
    if preflight {
        headers.append(
            header::VARY,
            HeaderValue::from_static(
                "Access-Control-Request-Method, Access-Control-Request-Headers",
            ),
        );
    }
    if allowed
        && let Some(origin) = origin
        && (!preflight || valid_preflight)
    {
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            if private {
                origin
            } else {
                HeaderValue::from_static("*")
            },
        );
        if private {
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }
        headers.insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_static("X-Frame-Seq, X-Captured-At, X-Game-Time"),
        );
        if preflight {
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("GET, POST, PATCH, PUT"),
            );
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static("Authorization, Content-Type, X-Content-Sha256"),
            );
            headers.insert(
                header::ACCESS_CONTROL_MAX_AGE,
                HeaderValue::from_static("3600"),
            );
        }
    }
    Ok(response)
}
