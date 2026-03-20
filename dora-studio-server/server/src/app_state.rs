use std::sync::Arc;
use storage::engine::DatabaseEngine;
// use coordinator_client::client::SharedClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseEngine>,
    // pub _coordinator: SharedClient,
}
