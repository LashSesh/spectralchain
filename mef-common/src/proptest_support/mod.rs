//! Property-based testing support
//!
//! Provides proptest generators and strategies for MEF types and common patterns.
//! This enables exhaustive, generative testing of invariants and properties.
//!
//! # Property-Based Testing Philosophy
//!
//! Instead of writing individual test cases, property-based testing:
//! 1. Defines **invariants** that should always hold
//! 2. Generates hundreds/thousands of random test inputs
//! 3. Verifies invariants hold for all generated inputs
//! 4. Shrinks failing cases to minimal reproducible examples
//!
//! # Example
//!
//! ```rust
//! use proptest::prelude::*;
//! use mef_common::proptest_support::arb_resonance_triplet;
//!
//! proptest! {
//!     #[test]
//!     fn test_resonance_normalization_is_unit_length(triplet in arb_resonance_triplet()) {
//!         let normalized = triplet.normalize();
//!         let magnitude = normalized.magnitude();
//!
//!         // INVARIANT: Normalized vectors have unit length
//!         prop_assert!((magnitude - 1.0).abs() < 1e-10);
//!     }
//! }
//! ```

pub mod generators;
pub mod strategies;
pub mod invariants;

pub use generators::*;
pub use strategies::*;
pub use invariants::*;
