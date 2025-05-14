use serde::Serialize;

#[derive(Serialize)]
pub(super) struct APIResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}
