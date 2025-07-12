use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T, U> {
    pub success: bool,
    pub message: T,
    pub data: U,
}

#[macro_export]
macro_rules! dev_response {
    (@inner $expr:expr, $msg:expr) => {{
        let _ = $expr;
        HttpResponse::Ok().json(APIResponse {
            success: true,
            message: $msg,
            data: (),
        })
    }};
    ($expr:expr) => {
        $crate::dev_response!(@inner $expr, format!("Executed fn[{}]", stringify!($expr)))
    };
}

#[macro_export]
macro_rules! const_bytes {
    ($bytes:expr) => {
        HttpResponse::Ok()
            .insert_header((crate::header::CONTENT_ENCODING, "br"))
            // .insert_header((header::CACHE_CONTROL, "public, max-age=31536000, immutable"))
            .content_type("application/octet-stream")
            .body($bytes)
    };
}

#[macro_export]
macro_rules! send_response {
    ($data:expr) => {
        if cfg!(debug_assertions) {
            HttpResponse::Ok().json(APIResponse {
                success: true,
                message: "Success",
                data: $data,
            })
        } else {
            match bincode::serde::encode_to_vec($data, bincode::config::standard()) {
                Ok(bin_data) => HttpResponse::Ok()
                    .insert_header((crate::header::CONTENT_TYPE, "application/octet-stream"))
                    .body(bin_data),
                Err(e) => {
                    eprintln!("Error serializing bincode: {:?}", e);
                    HttpResponse::InternalServerError().json(APIResponse {
                        success: false,
                        message: format!("Error serializing data: {:#?}", e),
                        data: (),
                    })
                }
            }
        }
    };
}
