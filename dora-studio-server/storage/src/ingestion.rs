use eyre::Result;
use shared::models::{log::LogMessage, metric::NodeMetrics, trace::Span};
use tokio::sync::Mutex;

/// Appends incoming metrics to the parquet store
pub struct DailyWriter {
    pub store_path: String,
    pub _mutex: Mutex<()>,
}

impl DailyWriter {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: store_path.to_string(),
            _mutex: Mutex::new(()),
        }
    }

    pub async fn flush_metrics(&self, _metrics: Vec<NodeMetrics>) -> Result<()> {
        let _lock = self._mutex.lock().await;
        // In reality: Convert Vec<NodeMetrics> to arrow::record_batch::RecordBatch
        // Then write out to parquet formatted `self.store_path/metrics/yyyy-mm-dd-uuid.parquet`
        Ok(())
    }

    pub async fn flush_logs(&self, _logs: Vec<LogMessage>) -> Result<()> {
        let _lock = self._mutex.lock().await;
        Ok(())
    }

    pub async fn flush_spans(&self, _spans: Vec<Span>) -> Result<()> {
        let _lock = self._mutex.lock().await;
        Ok(())
    }
}
