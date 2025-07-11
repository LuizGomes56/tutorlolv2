use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T, U> {
    pub success: bool,
    pub message: T,
    pub data: U,
}

#[macro_export]
macro_rules! match_fn {
    (@inner $expr:expr, $msg:expr) => {
        match $expr {
            Ok(_) => HttpResponse::Ok().json(APIResponse {
                success: true,
                message: &$msg,
                data: (),
            }),
            Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!("Unexpected error at fn[{}]: {:#?}", stringify!($expr), e),
                data: (),
            }),
        }
    };
    ($expr:expr) => {
        $crate::match_fn!(@inner $expr, format!("Executed fn[{}]", stringify!($expr)))
    };
    (priv $expr:expr) => {{
        #[cfg(debug_assertions)]
        {
            $crate::match_fn!(@inner $expr, format!("Executed fn[{}]", stringify!($expr)))
        }
        #[cfg(not(debug_assertions))]
        {
            HttpResponse::Ok().json(APIResponse {
                success: true,
                message: format!("Cannot call fn[{}] in release mode", stringify!($expr)),
                data: (),
            })
        }
    }};
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
