#[macro_export]
macro_rules! dev_response {
    ($expr:expr) => {{
        let _ = $expr;
        HttpResponse::Ok().body(format!("Executed fn[{}]", stringify!($expr)))
    }};
}

#[macro_export]
macro_rules! send_response {
    ($data:expr) => {
        match bincode::encode_to_vec($data, bincode::config::standard()) {
            Ok(bin_data) => HttpResponse::Ok()
                .insert_header((crate::header::CONTENT_TYPE, "application/octet-stream"))
                .body(actix_web::web::Bytes::from(bin_data)),
            Err(e) => {
                eprintln!("Error serializing bincode: {:?}", e);
                HttpResponse::InternalServerError()
                    .body(format!("Error serializing data: {:#?}", e))
            }
        }
    };
}
