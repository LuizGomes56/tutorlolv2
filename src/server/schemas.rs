use serde::Serialize;

/// Every API response will have this structure
/// ### Example
/// ```json
/// {
///     "success": true,
///     "message": "Success",
///     "data": {}
/// }
/// ```
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
