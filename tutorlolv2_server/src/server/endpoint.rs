//! Actix registration consumes the same method/path metadata as Rust clients.
use actix_web::{Resource, ResponseError, Route, guard, http::Method, web};
use tutorlolv2_api::Endpoint;

pub trait EndpointExt: Endpoint {
    fn reply<'a>(value: Self::Output<'a>) -> super::response::ApiReply<Self::Output<'a>>
    where
        Self::Output<'a>: bincode::Encode,
    {
        super::response::ApiReply::ok(value)
    }

    fn resource(prefix: &str) -> Resource {
        let path = Self::PATH
            .strip_prefix(prefix)
            .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'))
            .expect("endpoint must belong to its declared mounting scope");
        let mut methods = Vec::new();
        for (method, pattern) in tutorlolv2_api::ROUTES {
            if *pattern == Self::PATH {
                methods.push(method.as_str());
                if *method == tutorlolv2_api::Method::GET {
                    methods.push("HEAD");
                }
            }
        }
        let allow = methods.join(", ");
        web::resource(path).default_service(web::route().to(move || {
            let allow = allow.clone();
            async move {
                let mut response =
                    super::response::AppError::method_not_allowed("Method not allowed")
                        .error_response();
                response.headers_mut().insert(
                    actix_web::http::header::ALLOW,
                    actix_web::http::header::HeaderValue::from_str(&allow)
                        .expect("declared methods"),
                );
                response
            }
        }))
    }
    fn route() -> Route {
        if Self::METHOD == tutorlolv2_api::Method::GET {
            web::route().guard(guard::Any(guard::Get()).or(guard::Head()))
        } else {
            web::method(
                Method::from_bytes(Self::METHOD.as_str().as_bytes())
                    .expect("known endpoint method"),
            )
        }
    }
}
impl<E: Endpoint> EndpointExt for E {}
