use serde::{Deserialize, Serialize};
use uuid::Uuid;
// Need to map from dora internal structs
// For now, mirroring what was in the proposal

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogMessage {
    pub build_id: Option<String>,
    pub dataflow_id: Option<Uuid>,
    pub node_id: Option<String>,
    pub daemon_id: Option<String>,
    pub level: String, // Stringified log level
    pub target: Option<String>,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub message: String,
    pub timestamp: i64,              // Unix timestamp in ms
    pub fields_json: Option<String>, // JSON string of BTreeMap
}
