use crate::engine::DatabaseEngine;
use datafusion::dataframe::DataFrame;
use eyre::Result;
use std::sync::Arc;

pub async fn query_historical_metrics(
    engine: Arc<DatabaseEngine>,
    limit: usize,
) -> Result<DataFrame> {
    // Query registered parquet folder
    let df = engine
        .ctx
        .sql(&format!(
            "SELECT * FROM metrics ORDER BY ts DESC LIMIT {}",
            limit
        ))
        .await?;
    Ok(df)
}

pub async fn query_historical_logs(engine: Arc<DatabaseEngine>, limit: usize) -> Result<DataFrame> {
    let df = engine
        .ctx
        .sql(&format!(
            "SELECT * FROM logs ORDER BY ts DESC LIMIT {}",
            limit
        ))
        .await?;
    Ok(df)
}

pub async fn query_historical_spans(
    engine: Arc<DatabaseEngine>,
    limit: usize,
) -> Result<DataFrame> {
    let df = engine
        .ctx
        .sql(&format!(
            "SELECT * FROM spans ORDER BY ts DESC LIMIT {}",
            limit
        ))
        .await?;
    Ok(df)
}
