use datafusion::execution::context::SessionContext;
use eyre::Result;
use std::sync::Arc;
use tokio::fs;

pub struct DatabaseEngine {
    pub ctx: SessionContext,
    pub store_path: String,
}

impl DatabaseEngine {
    pub async fn new(store_path: &str) -> Result<Arc<Self>> {
        // Ensure storage directories exist
        fs::create_dir_all(format!("{}/metrics", store_path)).await?;
        fs::create_dir_all(format!("{}/logs", store_path)).await?;
        fs::create_dir_all(format!("{}/spans", store_path)).await?;

        let ctx = SessionContext::new();

        // Normally here we would map physical parquet directories
        // into logical DataFusion tables:
        // ctx.register_parquet("metrics", &format!("{}/metrics/", store_path), Default::default()).await?;

        Ok(Arc::new(Self {
            ctx,
            store_path: store_path.to_string(),
        }))
    }
}
