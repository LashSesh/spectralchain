//! Proptest generators for MEF types
//!
//! Provides strategies for generating random instances of MEF types
//! for property-based testing.

use crate::types::{ContentHash, NodeId, ResonanceTriplet, TxId};
use proptest::prelude::*;

/// Generate arbitrary ResonanceTriplet values
///
/// Generates triplets with components in range [-10.0, 10.0]
///
/// # Example
///
/// ```rust
/// use proptest::prelude::*;
/// use mef_common::proptest_support::arb_resonance_triplet;
///
/// proptest! {
///     #[test]
///     fn test_resonance_magnitude_non_negative(triplet in arb_resonance_triplet()) {
///         prop_assert!(triplet.magnitude() >= 0.0);
///     }
/// }
/// ```
pub fn arb_resonance_triplet() -> impl Strategy<Value = ResonanceTriplet> {
    (-10.0..=10.0, -10.0..=10.0, -10.0..=10.0)
        .prop_map(|(psi, rho, omega)| ResonanceTriplet::new(psi, rho, omega))
}

/// Generate unit-length ResonanceTriplet values
///
/// Generates triplets that are already normalized (magnitude = 1.0)
pub fn arb_unit_resonance_triplet() -> impl Strategy<Value = ResonanceTriplet> {
    arb_resonance_triplet().prop_map(|triplet| triplet.normalize())
}

/// Generate non-zero ResonanceTriplet values
///
/// Ensures at least one component is non-zero
pub fn arb_nonzero_resonance_triplet() -> impl Strategy<Value = ResonanceTriplet> {
    arb_resonance_triplet().prop_filter("triplet must be non-zero", |t| {
        t.psi != 0.0 || t.rho != 0.0 || t.omega != 0.0
    })
}

/// Generate arbitrary ContentHash values
///
/// # Example
///
/// ```rust
/// use proptest::prelude::*;
/// use mef_common::proptest_support::arb_content_hash;
///
/// proptest! {
///     #[test]
///     fn test_content_hash_hex_roundtrip(hash in arb_content_hash()) {
///         let hex = hash.to_hex();
///         let parsed = ContentHash::from_hex(&hex).unwrap();
///         prop_assert_eq!(hash, parsed);
///     }
/// }
/// ```
pub fn arb_content_hash() -> impl Strategy<Value = ContentHash> {
    prop::array::uniform32(any::<u8>()).prop_map(ContentHash::from_bytes)
}

/// Generate arbitrary NodeId values
///
/// Generates IDs with format: "node_{random_alphanumeric}"
pub fn arb_node_id() -> impl Strategy<Value = NodeId> {
    "[a-zA-Z0-9]{8,32}".prop_map(|s| NodeId::new(format!("node_{}", s)))
}

/// Generate arbitrary TxId values
///
/// Generates IDs with format: "tx_{random_alphanumeric}"
pub fn arb_tx_id() -> impl Strategy<Value = TxId> {
    "[a-zA-Z0-9]{8,64}".prop_map(|s| TxId::new(format!("tx_{}", s)))
}

/// Generate arbitrary timestamps (seconds since UNIX epoch)
///
/// Range: 2020-01-01 to 2030-01-01
pub fn arb_timestamp() -> impl Strategy<Value = u64> {
    1577836800u64..=1893456000u64 // 2020-2030
}

/// Generate arbitrary TTL values (time-to-live in seconds)
///
/// Range: 1 second to 1 year
pub fn arb_ttl() -> impl Strategy<Value = u64> {
    1u64..=31536000u64 // 1 sec to 1 year
}

/// Generate arbitrary byte vectors
///
/// Length range: 0 to max_len bytes
pub fn arb_bytes(max_len: usize) -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 0..=max_len)
}

/// Generate non-empty byte vectors
pub fn arb_nonempty_bytes(max_len: usize) -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 1..=max_len)
}

/// Generate arbitrary f64 values in a specific range
pub fn arb_f64_range(min: f64, max: f64) -> impl Strategy<Value = f64> {
    (min..=max).prop_map(|x| x)
}

/// Generate small positive integers (useful for indices, counts)
pub fn arb_small_uint() -> impl Strategy<Value = usize> {
    0usize..1000
}

/// Generate large positive integers (useful for IDs, hashes)
pub fn arb_large_uint() -> impl Strategy<Value = u64> {
    any::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn test_arb_resonance_triplet_generates_valid(triplet in arb_resonance_triplet()) {
            // Should be able to compute magnitude
            let _magnitude = triplet.magnitude();
            // Should be able to normalize
            let _normalized = triplet.normalize();
        }

        #[test]
        fn test_arb_unit_resonance_has_unit_length(triplet in arb_unit_resonance_triplet()) {
            let magnitude = triplet.magnitude();
            prop_assert!((magnitude - 1.0).abs() < 1e-6, "magnitude={}", magnitude);
        }

        #[test]
        fn test_arb_nonzero_resonance_is_nonzero(triplet in arb_nonzero_resonance_triplet()) {
            prop_assert!(triplet.magnitude() > 0.0);
        }

        #[test]
        fn test_arb_content_hash_roundtrip(hash in arb_content_hash()) {
            let hex = hash.to_hex();
            prop_assert_eq!(hex.len(), 64);
            let parsed = ContentHash::from_hex(&hex).unwrap();
            prop_assert_eq!(hash, parsed);
        }

        #[test]
        fn test_arb_node_id_format(node_id in arb_node_id()) {
            prop_assert!(node_id.as_str().starts_with("node_"));
        }

        #[test]
        fn test_arb_tx_id_format(tx_id in arb_tx_id()) {
            prop_assert!(tx_id.as_str().starts_with("tx_"));
        }

        #[test]
        fn test_arb_timestamp_in_range(ts in arb_timestamp()) {
            prop_assert!(ts >= 1577836800); // >= 2020
            prop_assert!(ts <= 1893456000); // <= 2030
        }

        #[test]
        fn test_arb_ttl_positive(ttl in arb_ttl()) {
            prop_assert!(ttl > 0);
            prop_assert!(ttl <= 31536000); // <= 1 year
        }

        #[test]
        fn test_arb_bytes_length(bytes in arb_bytes(1000)) {
            prop_assert!(bytes.len() <= 1000);
        }

        #[test]
        fn test_arb_nonempty_bytes_nonempty(bytes in arb_nonempty_bytes(1000)) {
            prop_assert!(!bytes.is_empty());
        }
    }
}
