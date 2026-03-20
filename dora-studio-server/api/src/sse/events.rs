use axum::response::{
    IntoResponse,
    sse::{Event, Sse},
};
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub async fn get_events_stream() -> impl IntoResponse {
    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(10);
    tokio::spawn(async move {
        loop {
            let _ = tx
                .send(Ok(Event::default().data(r#"{"event": "heartbeat"}"#)))
                .await;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    Sse::new(ReceiverStream::new(rx)).keep_alive(axum::response::sse::KeepAlive::new())
}
