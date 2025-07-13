use crate::server::schemas::APIResponse;
use actix_web::{
    HttpRequest, HttpResponse,
    error::{InternalError, JsonPayloadError},
};

/// Avoid retuning plain text when a JSON deserialization error occurs.
/// This API should ALWAYS return a JSON response, including for unimplemented endpoints.
pub fn json_error_middleware(err: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    let error_response: APIResponse<&'_ str, ()> = match &err {
        JsonPayloadError::ContentType => APIResponse {
            success: false,
            message: "Content-Type must be application/json",
            data: (),
        },
        // Most common error. Implemented to prevent server from returning plain text.
        JsonPayloadError::Deserialize(e) => APIResponse {
            success: false,
            message: &format!(
                "[BAD REQUEST]: A JSON serialization error ocurred due to malformed, invalid or missing fields on your JSON: {}",
                e
            ),
            data: (),
        },
        JsonPayloadError::Overflow { limit } => APIResponse {
            success: false,
            message: &format!("Payload limit exceeded. Limit: {}", limit),
            data: (),
        },
        _ => APIResponse {
            success: false,
            message: "Unknown error while parsing JSON",
            data: (),
        },
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(error_response)).into()
}
