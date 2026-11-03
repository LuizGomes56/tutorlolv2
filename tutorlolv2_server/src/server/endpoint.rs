//! Actix registration consumes the same method/path metadata as Rust clients.
use actix_web::{Resource, Route, http::Method, web};
use tutorlolv2_api::Endpoint;

pub trait EndpointExt: Endpoint {
    fn resource(prefix: &str) -> Resource {
        let path = Self::PATH
            .strip_prefix(prefix)
            .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'))
            .expect("endpoint must belong to its declared mounting scope");
        web::resource(path)
    }
    fn route() -> Route {
        web::method(
            Method::from_bytes(Self::METHOD.as_str().as_bytes()).expect("known endpoint method"),
        )
    }
}
impl<E: Endpoint> EndpointExt for E {}
