use std::time::Duration;
use tokio::time;

/// Cleans old parquet files asynchronously
pub async fn run_retention_loop(store_path: String) {
    let mut interval = time::interval(Duration::from_secs(3600 * 24)); // Run daily
    loop {
        interval.tick().await;
        // Check files in store_path/metrics, delete > 7 days
        // Check files in store_path/spans, delete > 7 days
        // Check files in store_path/logs, delete > 30 days
        println!("Running retention policy loop on {}", store_path);
    }
}
