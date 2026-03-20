use api::rest;
use api::sse;
use api::ws;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_app_router() -> Router {
    let api_routes = Router::new()
        .route(
            "/dataflows",
            get(rest::dataflows::list_dataflows).post(rest::dataflows::start_dataflow),
        )
        .route(
            "/dataflows/validate",
            post(rest::dataflows::validate_dataflow),
        )
        .route("/dataflows/parse", post(rest::dataflows::parse_dataflow))
        .route(
            "/dataflows/{id}",
            get(rest::dataflows::get_dataflow_details).delete(rest::dataflows::delete_dataflow),
        )
        .route(
            "/dataflows/{id}/reload",
            post(rest::dataflows::reload_dataflow),
        )
        .route(
            "/dataflows/{id}/nodes",
            get(rest::dataflows::list_dataflow_nodes),
        )
        .route("/daemons", get(rest::daemons::list_daemons))
        .route(
            "/daemons/{machine_id}",
            get(rest::daemons::get_daemon_details),
        )
        .route("/nodes/library", get(rest::nodes::list_node_library))
        .route("/metrics", get(rest::historical::get_historical_metrics))
        .route("/logs", get(rest::historical::get_historical_metrics))
        .route("/traces", get(rest::historical::get_historical_metrics))
        .route("/metrics/stream", get(sse::stream::get_metrics_stream))
        .route("/events", get(sse::events::get_events_stream));

    Router::new()
        .nest("/api/v1", api_routes)
        .route("/ws/dataflows", get(ws::status::ws_status_handler))
        .route(
            "/ws/dataflows/{id}/nodes",
            get(ws::nodes::ws_dataflow_nodes_handler),
        )
        .route("/ws/logs", get(ws::logs::ws_logs_handler))
        .route("/ws/topics/{topic}", get(ws::topics::ws_topic_handler))
}


