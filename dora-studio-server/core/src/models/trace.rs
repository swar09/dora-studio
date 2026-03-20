use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub duration_ms: i64,
    pub node_id: Option<String>,
    pub attributes_json: Option<String>,
}
