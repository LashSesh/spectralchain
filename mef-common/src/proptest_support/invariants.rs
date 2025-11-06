//! Common invariants for property-based testing
//!
//! Provides reusable invariant checks that should hold across MEF components.

use crate::types::ResonanceTriplet;

/// Invariant: Normalized vectors have unit length
///
/// # Example
///
/// ```rust
/// use mef_common::types::ResonanceTriplet;
/// use mef_common::proptest_support::invariants::assert_unit_length;
///
/// let triplet = ResonanceTriplet::new(3.0, 4.0, 0.0);
/// let normalized = triplet.normalize();
/// assert_unit_length(&normalized, 1e-10);
/// ```
pub fn assert_unit_length(triplet: &ResonanceTriplet, epsilon: f64) {
    let magnitude = triplet.magnitude();
    assert!(
        (magnitude - 1.0).abs() < epsilon,
        "Expected unit length, got magnitude {}",
        magnitude
    );
}

/// Invariant: Normalization is idempotent
///
/// normalize(normalize(x)) == normalize(x)
pub fn assert_normalization_idempotent(triplet: &ResonanceTriplet, epsilon: f64) {
    let once = triplet.normalize();
    let twice = once.normalize();

    assert!(
        (once.psi - twice.psi).abs() < epsilon,
        "psi differs: {} vs {}",
        once.psi,
        twice.psi
    );
    assert!(
        (once.rho - twice.rho).abs() < epsilon,
        "rho differs: {} vs {}",
        once.rho,
        twice.rho
    );
    assert!(
        (once.omega - twice.omega).abs() < epsilon,
        "omega differs: {} vs {}",
        once.omega,
        twice.omega
    );
}

/// Invariant: Hash roundtrip preserves data
///
/// from_hex(to_hex(x)) == x
pub fn assert_hash_roundtrip<T, F, G>(value: &T, to_hex: F, from_hex: G)
where
    T: PartialEq + std::fmt::Debug,
    F: Fn(&T) -> String,
    G: Fn(&str) -> Result<T, Box<dyn std::error::Error>>,
{
    let hex = to_hex(value);
    let parsed = from_hex(&hex).expect("Failed to parse hex");
    assert_eq!(
        value, &parsed,
        "Roundtrip failed: original != parsed"
    );
}

/// Invariant: Serialization roundtrip preserves data
///
/// deserialize(serialize(x)) == x
pub fn assert_serde_roundtrip<T>(value: &T)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("Failed to serialize");
    let parsed: T = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(value, &parsed, "Serde roundtrip failed");
}

/// Invariant: Encryption/masking is reversible
///
/// decrypt(encrypt(plaintext, key), key) == plaintext
pub fn assert_reversible<T, E, D>(
    plaintext: &T,
    key: &[u8],
    encrypt: E,
    decrypt: D,
)
where
    T: PartialEq + std::fmt::Debug,
    E: Fn(&T, &[u8]) -> Vec<u8>,
    D: Fn(&[u8], &[u8]) -> Result<T, Box<dyn std::error::Error>>,
{
    let ciphertext = encrypt(plaintext, key);
    let decrypted = decrypt(&ciphertext, key).expect("Decryption failed");
    assert_eq!(
        plaintext, &decrypted,
        "Encryption not reversible"
    );
}

/// Invariant: Operation is commutative
///
/// op(a, b) == op(b, a)
pub fn assert_commutative<T, F>(a: &T, b: &T, op: F)
where
    T: PartialEq + std::fmt::Debug,
    F: Fn(&T, &T) -> T,
{
    let ab = op(a, b);
    let ba = op(b, a);
    assert_eq!(ab, ba, "Operation not commutative");
}

/// Invariant: Operation is associative
///
/// op(op(a, b), c) == op(a, op(b, c))
pub fn assert_associative<T, F>(a: &T, b: &T, c: &T, op: F)
where
    T: PartialEq + std::fmt::Debug,
    F: Fn(&T, &T) -> T + Copy,
{
    let abc = op(&op(a, b), c);
    let ab_c = op(a, &op(b, c));
    assert_eq!(abc, ab_c, "Operation not associative");
}

/// Invariant: State machine transitions are deterministic
///
/// Same input state + same action -> same output state
pub fn assert_deterministic_transition<S, A, F>(
    initial: &S,
    action: &A,
    transition: F,
) where
    S: Clone + PartialEq + std::fmt::Debug,
    A: Clone,
    F: Fn(&S, &A) -> S,
{
    let state1 = transition(initial, action);
    let state2 = transition(initial, action);
    assert_eq!(
        state1, state2,
        "Transition not deterministic"
    );
}

/// Invariant: Content addressing is collision-resistant
///
/// hash(a) == hash(b) => a == b (contrapositive: a != b => hash(a) != hash(b))
pub fn assert_no_collision<T, H>(a: &T, b: &T, hash: H)
where
    T: PartialEq,
    H: Fn(&T) -> Vec<u8>,
{
    if a != b {
        let hash_a = hash(a);
        let hash_b = hash(b);
        assert_ne!(
            hash_a, hash_b,
            "Hash collision detected for different inputs"
        );
    }
}

/// Invariant: Monotonic increase
///
/// For sequences, each element should be >= previous
pub fn assert_monotonic_increase<T>(sequence: &[T])
where
    T: PartialOrd + std::fmt::Debug,
{
    for i in 1..sequence.len() {
        assert!(
            sequence[i] >= sequence[i - 1],
            "Not monotonic at index {}: {:?} < {:?}",
            i,
            sequence[i],
            sequence[i - 1]
        );
    }
}

/// Invariant: Strict monotonic increase
///
/// For sequences, each element should be > previous
pub fn assert_strict_monotonic_increase<T>(sequence: &[T])
where
    T: PartialOrd + std::fmt::Debug,
{
    for i in 1..sequence.len() {
        assert!(
            sequence[i] > sequence[i - 1],
            "Not strictly monotonic at index {}: {:?} <= {:?}",
            i,
            sequence[i],
            sequence[i - 1]
        );
    }
}

/// Invariant: Total sum is preserved
///
/// Useful for testing conservation laws, balance checks, etc.
pub fn assert_sum_preserved<T>(before: &[T], after: &[T])
where
    T: std::iter::Sum + Copy + PartialEq + std::fmt::Debug,
{
    let sum_before: T = before.iter().copied().sum();
    let sum_after: T = after.iter().copied().sum();
    assert_eq!(sum_before, sum_after, "Sum not preserved");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ResonanceTriplet;

    #[test]
    fn test_unit_length_invariant() {
        let triplet = ResonanceTriplet::new(3.0, 4.0, 0.0);
        let normalized = triplet.normalize();
        assert_unit_length(&normalized, 1e-10);
    }

    #[test]
    fn test_normalization_idempotent() {
        let triplet = ResonanceTriplet::new(1.0, 2.0, 3.0);
        assert_normalization_idempotent(&triplet, 1e-10);
    }

    #[test]
    fn test_serde_roundtrip() {
        let triplet = ResonanceTriplet::new(1.5, 2.5, 3.5);
        assert_serde_roundtrip(&triplet);
    }

    #[test]
    fn test_monotonic_increase() {
        let sequence = vec![1, 2, 2, 3, 5, 8];
        assert_monotonic_increase(&sequence);
    }

    #[test]
    fn test_strict_monotonic_increase() {
        let sequence = vec![1, 2, 3, 5, 8, 13];
        assert_strict_monotonic_increase(&sequence);
    }

    #[test]
    #[should_panic(expected = "Not strictly monotonic")]
    fn test_strict_monotonic_fails_on_equal() {
        let sequence = vec![1, 2, 2, 3];
        assert_strict_monotonic_increase(&sequence);
    }

    #[test]
    fn test_sum_preserved() {
        let before = vec![10, 20, 30];
        let after = vec![15, 15, 30];
        assert_sum_preserved(&before, &after);
    }
}
