use std::fmt;

#[derive(Debug)]
pub enum StudioError {
    Internal(String),
    NotFound(String),
    Database(String),
    Coordinator(String),
}

impl fmt::Display for StudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudioError::Internal(msg) => write!(f, "Internal error: {}", msg),
            StudioError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StudioError::Database(msg) => write!(f, "Database error: {}", msg),
            StudioError::Coordinator(msg) => write!(f, "Coordinator error: {}", msg),
        }
    }
}

impl std::error::Error for StudioError {}
