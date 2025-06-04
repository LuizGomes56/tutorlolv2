use actix_web::{
    HttpRequest, HttpResponse,
    error::{InternalError, JsonPayloadError},
};

use crate::server::schemas::APIResponse;

pub fn json_error_middleware(err: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    let error_response = match &err {
        JsonPayloadError::ContentType => APIResponse {
            success: false,
            message: "Content-Type must be application/json".to_string(),
            data: (),
        },
        JsonPayloadError::Deserialize(e) => APIResponse {
            success: false,
            message: format!(
                "[BAD REQUEST]: A JSON serialization error ocurred due to malformed, invalid or missing fields on your JSON: {}",
                e
            ),
            data: (),
        },
        JsonPayloadError::Overflow { limit } => APIResponse {
            success: false,
            message: format!("Payload limit exceeded. Limit: {}", limit),
            data: (),
        },
        _ => APIResponse {
            success: false,
            message: "Unknown error while parsing JSON".to_string(),
            data: (),
        },
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(error_response)).into()
}
