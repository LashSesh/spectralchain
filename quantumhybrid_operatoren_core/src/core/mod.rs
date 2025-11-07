/*!
 * Core Module - Grundlegende Abstraktionen für Quantum-Resonant Operators
 */

pub mod traits;

// Re-export wichtiger Traits
pub use traits::{
    ComposableOperator, ContractiveOperator, IdempotentOperator, InvertibleOperator,
    OperatorStats, QuantumOperator, ResonanceOperator, StatefulOperator, UnitaryOperator,
};

// Re-export Hilfsfunktionen
pub use traits::{euclidean_distance, normalize_vector};

/// Fehlertypen für Operatoren
#[derive(Debug, thiserror::Error)]
pub enum OperatorError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Not invertible: {0}")]
    NotInvertible(String),

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Numerical error: {0}")]
    NumericalError(String),
}

/// Result-Typ für Operator-Operationen
pub type Result<T> = std::result::Result<T, OperatorError>;
