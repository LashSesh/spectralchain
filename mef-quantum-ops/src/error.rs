/*!
 * Error types for quantum operations
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantumOpsError {
    #[error("Invalid masking parameters: {0}")]
    InvalidMaskingParams(String),

    #[error("Resonance window mismatch: node_phase={node_phase}, packet_phase={packet_phase}, epsilon={epsilon}")]
    ResonanceMismatch {
        node_phase: f64,
        packet_phase: f64,
        epsilon: f64,
    },

    #[error("Steganography operation failed: {0}")]
    SteganographyError(String),

    #[error("Zero-knowledge proof verification failed: {0}")]
    ZKProofError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, QuantumOpsError>;
