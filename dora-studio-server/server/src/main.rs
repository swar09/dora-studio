pub mod app_state;
pub mod config;
pub mod routes;

use std::net::SocketAddr;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting dora-studio-server...");

    // 1. Load config
    let config = config::Config::load();

    // 2. Initialize Backend (DataFusion)
    let db_engine = storage::engine::DatabaseEngine::new(&config.db_path).await?;

    // 3. Connect to coordinator
    // let coordinator_client = coordinator_client::client::connect_rpc("127.0.0.1".parse()?, 2012).await?;

    // 4. Start retention loop
    tokio::spawn(storage::retention::run_retention_loop(
        config.db_path.clone(),
    ));

    // 5. App State
    let _app_state = app_state::AppState {
        db: db_engine,
        // _coordinator: coordinator_client,
    };

    // 6. Spin up OTLP listener (Dummy port for now)
    // tokio::spawn(otlp_listener::grpc::start_server(4317));

    // 7. Mount Axum App
    let app = routes::create_app_router() // will pass app_state later
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
