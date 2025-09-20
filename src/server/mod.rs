#[cfg(feature = "dev")]
pub mod dev;
pub mod games;
pub mod img;

#[macro_export]
macro_rules! dev_response {
    ($expr:expr) => {{
        let _ = $expr;
        HttpResponse::Ok().body(concat!("Executed fn[{}]", stringify!($expr)))
    }};
}
