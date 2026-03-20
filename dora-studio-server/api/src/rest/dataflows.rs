use axum::{Json, extract::Path, response::IntoResponse};
use serde_json::json;

pub async fn list_dataflows() -> impl IntoResponse {
    Json(json!([
        {
            "uuid": "df-1234-5678",
            "name": "vision-pipeline",
            "status": "Running",
            "nodes": 5
        },
        {
            "uuid": "df-8765-4321",
            "name": "audio-processing",
            "status": "Stopped",
            "nodes": 2
        }
    ]))
}

pub async fn start_dataflow() -> impl IntoResponse {
    Json(json!({ "status": "Started", "uuid": "df-new-9999" }))
}

pub async fn get_dataflow_details(Path(id): Path<String>) -> impl IntoResponse {
    Json(json!({ "uuid": id, "status": "Running" }))
}

pub async fn delete_dataflow(Path(id): Path<String>) -> impl IntoResponse {
    Json(json!({ "status": "Stopped", "uuid": id }))
}

pub async fn reload_dataflow(Path(id): Path<String>) -> impl IntoResponse {
    Json(json!({ "status": "Reloaded", "uuid": id }))
}

pub async fn list_dataflow_nodes(Path(_id): Path<String>) -> impl IntoResponse {
    Json(json!([ { "node_id": "vision", "status": "Active" } ]))
}

pub async fn validate_dataflow() -> impl IntoResponse {
    Json(json!({ "valid": true }))
}

pub async fn parse_dataflow() -> impl IntoResponse {
    Json(json!({ "nodes": [], "edges": [] }))
}
