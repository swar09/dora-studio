use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use serde_json::json;
use tokio::time::{Duration, sleep};

pub async fn ws_status_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut i = 0;
    loop {
        let status = json!({
            "uptime_secs": i,
            "state": "HEALTHY",
            "active_nodes": 3
        });
        if socket
            .send(Message::Text(status.to_string().into()))
            .await
            .is_err()
        {
            break;
        }
        i += 5;
        sleep(Duration::from_secs(5)).await;
    }
}
