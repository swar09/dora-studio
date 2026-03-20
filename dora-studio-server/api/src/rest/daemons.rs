use axum::{Json, extract::Path, response::IntoResponse};
use serde_json::json;

pub async fn list_daemons() -> impl IntoResponse {
    Json(json!([
        {
            "machine_id": "machine-alpha",
            "ip": "192.168.1.10",
            "status": "Active"
        },
        {
            "machine_id": "machine-beta",
            "ip": "192.168.1.11",
            "status": "Offline"
        }
    ]))
}

pub async fn get_daemon_details(Path(machine_id): Path<String>) -> impl IntoResponse {
    Json(json!({ "machine_id": machine_id, "status": "Active", "ip": "192.168.1.10" }))
}
