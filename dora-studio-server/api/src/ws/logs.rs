use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use serde_json::json;
use tokio::time::{Duration, sleep};

pub async fn ws_logs_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_logs_socket)
}

async fn handle_logs_socket(mut socket: WebSocket) {
    let mut i = 0;
    loop {
        let log = json!({
            "timestamp": "2026-03-20T03:25:00Z",
            "level": "INFO",
            "message": format!("Mock log message #{} from node-vision", i),
        });
        if socket
            .send(Message::Text(log.to_string().into()))
            .await
            .is_err()
        {
            break;
        }
        i += 1;
        sleep(Duration::from_millis(1500)).await;
    }
}
