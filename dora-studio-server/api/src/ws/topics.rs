use axum::extract::ws::WebSocketUpgrade;

pub async fn ws_topic_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(|_socket| async {
        // Handle topics
    })
}
