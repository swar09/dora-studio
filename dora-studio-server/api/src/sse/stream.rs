use axum::response::{
    IntoResponse,
    sse::{Event, Sse},
};
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub async fn get_metrics_stream() -> impl IntoResponse {
    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(10);
    tokio::spawn(async move {
        let mut i = 0;
        loop {
            let metric = format!(r#"{{"cpu": {}, "memory": {}}}"#, 30 + i % 10, 1024 + i * 10);
            if tx.send(Ok(Event::default().data(metric))).await.is_err() {
                break;
            }
            i += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    Sse::new(ReceiverStream::new(rx)).keep_alive(axum::response::sse::KeepAlive::new())
}
