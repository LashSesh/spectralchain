//! Property-based tests for mef-quantum-ops
//!
//! These tests use proptest to verify invariants across thousands of
//! randomly-generated test cases.

use mef_common::proptest_support::*;
use mef_quantum_ops::*;
use proptest::prelude::*;

// ============================================================================
// Masking Operator Property Tests
// ============================================================================

/// Helper: Generate arbitrary masking parameters
fn arb_masking_params() -> impl Strategy<Value = MaskingParams> {
    (
        0.0..std::f64::consts::TAU,  // theta in [0, 2π]
        prop::array::uniform32(any::<u8>()),  // sigma: 32 random bytes
    )
        .prop_map(|(theta, sigma)| MaskingParams::new(theta, sigma))
}

proptest! {
    /// INVARIANT: Masking is reversible
    ///
    /// For all messages m and parameters (θ, σ):
    ///   unmask(mask(m, (θ, σ)), (θ, σ)) = m
    #[test]
    fn masking_is_reversible(
        message in arb_nonempty_bytes(1000),
        params in arb_masking_params()
    ) {
        let masker = MaskingOperator::new();

        let masked = masker.mask(&message, &params)?;
        let unmasked = masker.unmask(&masked, &params)?;

        prop_assert_eq!(message, unmasked,
            "Masking not reversible: original != unmasked");
    }

    /// INVARIANT: Masking preserves length
    ///
    /// |mask(m)| = |m|
    #[test]
    fn masking_preserves_length(
        message in arb_bytes(1000),
        params in arb_masking_params()
    ) {
        let masker = MaskingOperator::new();
        let masked = masker.mask(&message, &params)?;

        prop_assert_eq!(message.len(), masked.len(),
            "Length not preserved: {} != {}", message.len(), masked.len());
    }

    /// INVARIANT: Masking is deterministic
    ///
    /// Same message + same params → same masked output
    #[test]
    fn masking_is_deterministic(
        message in arb_nonempty_bytes(500),
        params in arb_masking_params()
    ) {
        let masker = MaskingOperator::new();

        let masked1 = masker.mask(&message, &params)?;
        let masked2 = masker.mask(&message, &params)?;

        prop_assert_eq!(masked1, masked2,
            "Masking not deterministic");
    }

    /// INVARIANT: Different parameters produce different outputs
    ///
    /// (with high probability for non-trivial messages)
    #[test]
    fn different_params_produce_different_outputs(
        message in arb_nonempty_bytes(100),
        params1 in arb_masking_params(),
        params2 in arb_masking_params()
    ) {
        // Skip if parameters are identical
        if params1.theta == params2.theta && params1.sigma == params2.sigma {
            return Ok(());
        }

        let masker = MaskingOperator::new();

        let masked1 = masker.mask(&message, &params1)?;
        let masked2 = masker.mask(&message, &params2)?;

        // Different params should produce different outputs (with high probability)
        prop_assert_ne!(masked1, masked2,
            "Different params produced same output");
    }

    /// INVARIANT: Masking params from seed are deterministic
    #[test]
    fn params_from_seed_deterministic(
        seed in arb_nonempty_bytes(64)
    ) {
        let params1 = MaskingParams::from_seed(&seed);
        let params2 = MaskingParams::from_seed(&seed);

        prop_assert_eq!(params1.theta, params2.theta);
        prop_assert_eq!(params1.sigma, params2.sigma);
    }

    /// INVARIANT: Theta is always in valid range [0, 2π]
    #[test]
    fn params_theta_in_valid_range(
        seed in arb_nonempty_bytes(64)
    ) {
        let params = MaskingParams::from_seed(&seed);

        prop_assert!(params.theta >= 0.0);
        prop_assert!(params.theta < std::f64::consts::TAU);
    }
}

// ============================================================================
// Resonance Operator Property Tests
// ============================================================================

/// Helper: Generate arbitrary resonance params
fn arb_resonance_params() -> impl Strategy<Value = ResonanceParams> {
    (
        0.0..=1.0,  // epsilon (threshold)
        arb_bytes(32).prop_map(|v| {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&v[..32.min(v.len())]);
            arr
        }),  // seed
    )
        .prop_map(|(epsilon, seed)| {
            ResonanceParams::new(epsilon, seed)
        })
}

proptest! {
    /// INVARIANT: Resonance values are in [0, 1]
    #[test]
    fn resonance_in_valid_range(
        data1 in arb_nonempty_bytes(100),
        data2 in arb_nonempty_bytes(100),
        params in arb_resonance_params()
    ) {
        let resonator = ResonanceOperator::new();
        let resonance = resonator.compute_resonance(&data1, &data2, &params)?;

        prop_assert!(resonance >= 0.0 && resonance <= 1.0,
            "Resonance {} not in [0, 1]", resonance);
    }

    /// INVARIANT: Resonance is symmetric
    ///
    /// R(a, b) = R(b, a)
    #[test]
    fn resonance_is_symmetric(
        data1 in arb_nonempty_bytes(100),
        data2 in arb_nonempty_bytes(100),
        params in arb_resonance_params()
    ) {
        let resonator = ResonanceOperator::new();

        let r_ab = resonator.compute_resonance(&data1, &data2, &params)?;
        let r_ba = resonator.compute_resonance(&data2, &data1, &params)?;

        prop_assert!((r_ab - r_ba).abs() < 1e-10,
            "Resonance not symmetric: {} != {}", r_ab, r_ba);
    }

    /// INVARIANT: Self-resonance is maximum (1.0)
    ///
    /// R(x, x) = 1.0
    #[test]
    fn self_resonance_is_maximum(
        data in arb_nonempty_bytes(100),
        params in arb_resonance_params()
    ) {
        let resonator = ResonanceOperator::new();
        let self_resonance = resonator.compute_resonance(&data, &data, &params)?;

        prop_assert!((self_resonance - 1.0).abs() < 1e-10,
            "Self-resonance {} != 1.0", self_resonance);
    }

    /// INVARIANT: Resonance check is consistent
    ///
    /// check_resonance returns true iff computed resonance >= epsilon
    #[test]
    fn resonance_check_consistent(
        data1 in arb_nonempty_bytes(100),
        data2 in arb_nonempty_bytes(100),
        params in arb_resonance_params()
    ) {
        let resonator = ResonanceOperator::new();

        let resonance = resonator.compute_resonance(&data1, &data2, &params)?;
        let check = resonator.check_resonance(&data1, &data2, &params)?;

        let expected_check = resonance >= params.epsilon;
        prop_assert_eq!(check, expected_check,
            "Check inconsistent: resonance={}, epsilon={}, check={}, expected={}",
            resonance, params.epsilon, check, expected_check);
    }
}

// ============================================================================
// Steganography Operator Property Tests
// ============================================================================

proptest! {
    /// INVARIANT: Steganography is reversible
    ///
    /// extract(hide(message, carrier)) = message
    #[test]
    fn steganography_is_reversible(
        secret in arb_nonempty_bytes(100),
        carrier_size in 200usize..1000
    ) {
        // Create carrier (must be larger than secret)
        let carrier: Vec<u8> = (0..carrier_size).map(|i| (i % 256) as u8).collect();

        let stego_op = SteganographyOperator::new();

        let stego_data = stego_op.hide(&secret, &carrier)?;
        let extracted = stego_op.extract(&stego_data, secret.len())?;

        prop_assert_eq!(secret, extracted,
            "Steganography not reversible");
    }

    /// INVARIANT: Steganography preserves carrier size
    ///
    /// |hide(m, c)| = |c|
    #[test]
    fn steganography_preserves_carrier_size(
        secret in arb_bytes(50),
        carrier_size in 100usize..500
    ) {
        let carrier: Vec<u8> = (0..carrier_size).map(|i| (i % 256) as u8).collect();
        let stego_op = SteganographyOperator::new();

        let stego_data = stego_op.hide(&secret, &carrier)?;

        prop_assert_eq!(carrier.len(), stego_data.len(),
            "Carrier size not preserved");
    }

    /// INVARIANT: Cannot hide message larger than carrier capacity
    #[test]
    fn steganography_rejects_oversized_message(
        secret in arb_bytes(100),
        small_carrier_size in 1usize..50
    ) {
        let carrier: Vec<u8> = vec![0u8; small_carrier_size];
        let stego_op = SteganographyOperator::new();

        // Should return error for oversized message
        let result = stego_op.hide(&secret, &carrier);
        prop_assert!(result.is_err(),
            "Should reject oversized message");
    }
}

// ============================================================================
// Zero-Knowledge Proof Property Tests
// ============================================================================

proptest! {
    /// INVARIANT: Valid proofs verify successfully
    #[test]
    fn valid_zk_proofs_verify(
        statement in arb_nonempty_bytes(100),
        witness in arb_nonempty_bytes(100)
    ) {
        let zk = ZKProofOperator::new();

        let proof = zk.prove(&statement, &witness)?;
        let verified = zk.verify(&statement, &proof)?;

        prop_assert!(verified,
            "Valid proof failed verification");
    }

    /// INVARIANT: Proofs are deterministic for same input
    #[test]
    fn zk_proofs_deterministic(
        statement in arb_nonempty_bytes(100),
        witness in arb_nonempty_bytes(100)
    ) {
        let zk = ZKProofOperator::new();

        let proof1 = zk.prove(&statement, &witness)?;
        let proof2 = zk.prove(&statement, &witness)?;

        prop_assert_eq!(proof1, proof2,
            "ZK proofs not deterministic");
    }

    /// INVARIANT: Proofs don't reveal witness
    ///
    /// Proof should not contain the witness data
    #[test]
    fn zk_proofs_dont_reveal_witness(
        statement in arb_nonempty_bytes(100),
        witness in arb_nonempty_bytes(100)
    ) {
        let zk = ZKProofOperator::new();
        let proof = zk.prove(&statement, &witness)?;

        // Proof should not contain the witness
        // (This is a basic check - full zero-knowledge requires cryptographic proof)
        let proof_bytes = proof.to_bytes();
        prop_assert!(
            !contains_subslice(&proof_bytes, &witness),
            "Proof appears to contain witness data"
        );
    }

    /// INVARIANT: Different witnesses produce different proofs
    #[test]
    fn different_witnesses_different_proofs(
        statement in arb_nonempty_bytes(100),
        witness1 in arb_nonempty_bytes(100),
        witness2 in arb_nonempty_bytes(100)
    ) {
        if witness1 == witness2 {
            return Ok(());
        }

        let zk = ZKProofOperator::new();

        let proof1 = zk.prove(&statement, &witness1)?;
        let proof2 = zk.prove(&statement, &witness2)?;

        prop_assert_ne!(proof1, proof2,
            "Different witnesses produced same proof");
    }
}

// ============================================================================
// Cross-Operator Composition Property Tests
// ============================================================================

proptest! {
    /// INVARIANT: Masking then resonance check works correctly
    #[test]
    fn composed_masking_and_resonance(
        message in arb_nonempty_bytes(100),
        mask_params in arb_masking_params(),
        res_params in arb_resonance_params()
    ) {
        let masker = MaskingOperator::new();
        let resonator = ResonanceOperator::new();

        // Mask the message
        let masked = masker.mask(&message, &mask_params)?;

        // Masked message should have high self-resonance
        let self_res = resonator.compute_resonance(&masked, &masked, &res_params)?;
        prop_assert!((self_res - 1.0).abs() < 1e-10);

        // Unmasked version should match original in resonance
        let unmasked = masker.unmask(&masked, &mask_params)?;
        let orig_res = resonator.compute_resonance(&message, &unmasked, &res_params)?;
        prop_assert!((orig_res - 1.0).abs() < 1e-10);
    }

    /// INVARIANT: Stego + Masking composition preserves reversibility
    #[test]
    fn composed_stego_and_masking(
        secret in arb_nonempty_bytes(50),
        carrier_size in 200usize..500,
        mask_params in arb_masking_params()
    ) {
        let carrier: Vec<u8> = (0..carrier_size).map(|i| (i % 256) as u8).collect();

        let stego_op = SteganographyOperator::new();
        let masker = MaskingOperator::new();

        // First hide in carrier, then mask
        let hidden = stego_op.hide(&secret, &carrier)?;
        let masked = masker.mask(&hidden, &mask_params)?;

        // Reverse: unmask, then extract
        let unmasked = masker.unmask(&masked, &mask_params)?;
        let extracted = stego_op.extract(&unmasked, secret.len())?;

        prop_assert_eq!(secret, extracted,
            "Composed operations not reversible");
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if haystack contains needle as a contiguous subslice
fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
