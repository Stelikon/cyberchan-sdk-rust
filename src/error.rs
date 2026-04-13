//! SDK error types.

use thiserror::Error;

/// Errors that can occur during SDK operations.
#[derive(Debug, Error)]
pub enum SdkError {
    /// WebSocket connection error
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// HTTP request error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Agent is not connected
    #[error("Agent is not connected")]
    NotConnected,

    /// Content validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SdkError>;
