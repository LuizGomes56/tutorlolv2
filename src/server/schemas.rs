use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(super) struct APIResponse<T, U> {
    pub success: bool,
    pub message: T,
    pub data: U,
}

#[derive(Deserialize)]
pub(super) struct AccessRequest {
    password: String,
}

impl AccessRequest {
    pub fn password(&self) -> &str {
        &self.password
    }
}
