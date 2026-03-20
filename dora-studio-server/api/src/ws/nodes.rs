use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use serde_json::json;
use tokio::time::{Duration, sleep};

pub async fn ws_dataflow_nodes_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_nodes_socket)
}

async fn handle_nodes_socket(mut socket: WebSocket) {
    loop {
        let stats = json!({ "node_id": "vision-1", "cpu": 45.0 });
        if socket
            .send(Message::Text(stats.to_string().into()))
            .await
            .is_err()
        {
            break;
        }
        sleep(Duration::from_secs(2)).await;
    }
}
