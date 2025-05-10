use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct APIResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}
