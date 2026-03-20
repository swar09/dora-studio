use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn get_historical_metrics() -> impl IntoResponse {
    Json(json!({
        "metrics": [
            { "timestamp": "2026-03-20T03:00:00Z", "cpu_usage": 45.2, "memory_usage": 1024.5 },
            { "timestamp": "2026-03-20T03:01:00Z", "cpu_usage": 50.1, "memory_usage": 1048.2 }
        ]
    }))
}
