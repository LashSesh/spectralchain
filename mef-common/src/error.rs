//! Error handling utilities and common error types
//!
//! Provides standardized error handling patterns across the MEF system.

use thiserror::Error;

/// Common result type for MEF operations
pub type MefResult<T> = Result<T, MefError>;

/// Common error types for MEF system
#[derive(Error, Debug)]
pub enum MefError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Cryptographic operation error
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Network operation error
    #[error("Network error: {0}")]
    Network(String),

    /// Storage operation error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Concurrency error (lock poisoning, etc.)
    #[error("Concurrency error: {0}")]
    Concurrency(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Operation timeout
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Generic error with context
    #[error("{0}")]
    Other(String),
}

impl MefError {
    /// Create a config error
    pub fn config(msg: impl Into<String>) -> Self {
        MefError::Config(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        MefError::Validation(msg.into())
    }

    /// Create a serialization error
    pub fn serialization(msg: impl Into<String>) -> Self {
        MefError::Serialization(msg.into())
    }

    /// Create a crypto error
    pub fn crypto(msg: impl Into<String>) -> Self {
        MefError::Crypto(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        MefError::Network(msg.into())
    }

    /// Create a storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        MefError::Storage(msg.into())
    }

    /// Create a concurrency error
    pub fn concurrency(msg: impl Into<String>) -> Self {
        MefError::Concurrency(msg.into())
    }

    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        MefError::NotFound(msg.into())
    }

    /// Create a timeout error
    pub fn timeout(msg: impl Into<String>) -> Self {
        MefError::Timeout(msg.into())
    }

    /// Create an invalid state error
    pub fn invalid_state(msg: impl Into<String>) -> Self {
        MefError::InvalidState(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        MefError::Internal(msg.into())
    }

    /// Create a generic error
    pub fn other(msg: impl Into<String>) -> Self {
        MefError::Other(msg.into())
    }
}

/// Convert from anyhow::Error
impl From<anyhow::Error> for MefError {
    fn from(err: anyhow::Error) -> Self {
        MefError::Other(err.to_string())
    }
}

/// Convert from serde_json::Error
impl From<serde_json::Error> for MefError {
    fn from(err: serde_json::Error) -> Self {
        MefError::Serialization(err.to_string())
    }
}

/// Convert from std::io::Error
impl From<std::io::Error> for MefError {
    fn from(err: std::io::Error) -> Self {
        MefError::Storage(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = MefError::config("test config error");
        assert_eq!(err.to_string(), "Configuration error: test config error");

        let err = MefError::validation("invalid input");
        assert_eq!(err.to_string(), "Validation error: invalid input");

        let err = MefError::not_found("resource");
        assert_eq!(err.to_string(), "Not found: resource");
    }

    #[test]
    fn test_error_conversion() {
        let json_err = serde_json::from_str::<i32>("invalid").unwrap_err();
        let mef_err: MefError = json_err.into();
        assert!(mef_err.to_string().contains("expected"));
    }
}
