use serde::Serialize;

#[derive(Serialize)]
pub(super) struct APIResponse<T, U> {
    pub success: bool,
    pub message: T,
    pub data: U,
}
