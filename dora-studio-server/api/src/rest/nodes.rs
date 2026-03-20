use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn list_node_library() -> impl IntoResponse {
    Json(json!([ { "type": "Python", "name": "Image Processor" } ]))
}
