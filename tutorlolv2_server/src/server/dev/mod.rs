pub mod images;
pub mod internal;
pub mod setup;
pub mod update;

#[macro_export]
macro_rules! dev_response {
    ($expr:expr) => {{
        let function = stringify!($expr);
        match $expr {
            Ok(_) => {
                let message = format!("Executed fn {function}");
                println!("Ok: {message}");
                HttpResponse::Ok().body(message)
            }
            Err(err) => {
                println!("Error calling fn {function}: {err:?}");
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }};
}
