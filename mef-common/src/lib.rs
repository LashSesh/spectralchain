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

pub mod time;
pub mod error;
pub mod concurrency;
pub mod result_ext;
pub mod types;

// Re-export commonly used items
pub use error::{MefError, MefResult};
pub use time::{current_timestamp, current_timestamp_millis};
pub use concurrency::{SafeRwLock, SafeRwLockExt};
pub use result_ext::ResultExt;
