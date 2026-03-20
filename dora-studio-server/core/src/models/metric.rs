use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetrics {
    pub dataflow_uuid: Option<Uuid>,
    pub node_id: Option<String>,
    pub pid: u32,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes: Option<u64>,
    pub disk_write_bytes: Option<u64>,
    pub timestamp: i64, // Unix timestamp in ms
}
