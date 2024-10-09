use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub data: serde_json::Value,
    pub message: String
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}