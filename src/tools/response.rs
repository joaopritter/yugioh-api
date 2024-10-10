use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T> {
    pub status_code: u16,
    pub status: String,
    pub message: String,
    pub result: T,
}

pub fn build_response<T: Serialize>(
    status_code: u16,
    status: &str,
    message: &str,
    result: T,
) -> Json<serde_json::Value> {
    let res = json!({
        "status": status.to_owned(),
        "status_code": status_code,
        "message": message.to_owned(),
        "result": result,
    });
    Json(res)
}
