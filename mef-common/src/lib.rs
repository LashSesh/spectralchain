//! # MEF Common Utilities
//!
//! Shared utilities and common patterns for the MEF system.
//! This crate eliminates code duplication and provides safe, tested implementations
//! of frequently-used patterns.
//!
//! ## Modules
//!
//! - `time`: Time and timestamp utilities
//! - `error`: Error handling and conversion utilities
//! - `concurrency`: Safe concurrency primitives and patterns
//! - `result_ext`: Extension traits for Result types
//! - `types`: Common type definitions and aliases
//! - `resilience`: Self-healing infrastructure (circuit breakers, health checks)

pub mod concurrency;
pub mod error;
pub mod resilience;
pub mod result_ext;
pub mod time;
pub mod types;

// Property-based testing support (feature-gated)
#[cfg(any(test, feature = "proptest-support"))]
pub mod proptest_support;

// Re-export commonly used items
pub use concurrency::{SafeRwLock, SafeRwLockExt};
pub use error::{MefError, MefResult};
pub use result_ext::ResultExt;
pub use time::{current_timestamp, current_timestamp_millis};
