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
    ($expr:expr) => {
        match $expr {
            Ok(_) => HttpResponse::Ok().json(APIResponse {
                success: true,
                message: &format!("Executed fn[{}]", stringify!($expr)),
                data: (),
            }),
            Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!("Unexpected error at fn[{}]: {:#?}", stringify!($expr), e),
                data: (),
            }),
        }
    };
}
