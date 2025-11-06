/*!
 * Masking Operator (M)
 *
 * Blueprint Formel: M_{θ,σ}(m) = e^{iθ} U_σ m
 *
 * Wobei:
 * - U_σ: Permutation (σ ist Permutationsindex)
 * - e^{iθ}: Phasenrotation (θ ist Phase in Radiant)
 * - m: Nachricht (Vektor von Bytes)
 *
 * Implementierung:
 * 1. Permutiere Nachricht mit deterministischer Permutation σ
 * 2. Wende Phasenrotation θ an (XOR mit Phasenschlüssel)
 * 3. Resultat ist maskierte Nachricht m'
 */

use crate::{QuantumOperator, Result};
use blake3::Hasher;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Masking-Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskingParams {
    /// Phase θ (in Radiant, 0 bis 2π)
    pub theta: f64,
    /// Permutations-Seed σ (für deterministische Permutation)
    pub sigma: [u8; 32],
}

impl MaskingParams {
    pub fn new(theta: f64, sigma: [u8; 32]) -> Self {
        Self { theta, sigma }
    }

    /// Generiere zufällige Parameter
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            theta: rng.gen_range(0.0..std::f64::consts::TAU),
            sigma: rand::random(),
        }
    }

    /// Generiere aus Seed
    pub fn from_seed(seed: &[u8]) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(seed);
        hasher.update(b"masking_params");
        let hash = hasher.finalize();

        let sigma: [u8; 32] = hash.as_bytes()[0..32].try_into().unwrap();
        let theta_bytes: [u8; 8] = hash.as_bytes()[32..40].try_into().unwrap();
        let theta = f64::from_le_bytes(theta_bytes) % std::f64::consts::TAU;

        Self { theta, sigma }
    }
}

/// Masking Operator
///
/// Implementiert M_{θ,σ}(m) = e^{iθ} U_σ m
pub struct MaskingOperator;

impl MaskingOperator {
    pub fn new() -> Self {
        Self
    }

    /// Mask a message with given parameters
    pub fn mask(&self, message: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        self.apply(message.to_vec(), params)
    }

    /// Unmask a message with given parameters
    pub fn unmask(&self, masked: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        // Masking ist symmetrisch (Involution), daher gleiche Operation
        self.apply(masked.to_vec(), params)
    }

    /// Compute phase key from theta
    fn phase_key(&self, theta: f64, len: usize) -> Vec<u8> {
        let mut hasher = Hasher::new();
        hasher.update(&theta.to_le_bytes());
        hasher.update(b"phase_key");

        let mut key = Vec::with_capacity(len);
        let mut counter = 0u64;

        while key.len() < len {
            let mut h = hasher.clone();
            h.update(&counter.to_le_bytes());
            let hash = h.finalize();
            key.extend_from_slice(hash.as_bytes());
            counter += 1;
        }

        key.truncate(len);
        key
    }

    /// Compute permutation from sigma
    fn permutation(&self, sigma: &[u8; 32], len: usize) -> Vec<usize> {
        let mut rng = ChaCha20Rng::from_seed(*sigma);
        let mut perm: Vec<usize> = (0..len).collect();
        perm.shuffle(&mut rng);
        perm
    }

    /// Apply permutation
    fn apply_permutation(&self, data: &[u8], perm: &[usize]) -> Vec<u8> {
        perm.iter().map(|&i| data[i]).collect()
    }

    /// Invert permutation
    fn invert_permutation(&self, perm: &[usize]) -> Vec<usize> {
        let mut inv = vec![0; perm.len()];
        for (i, &p) in perm.iter().enumerate() {
            inv[p] = i;
        }
        inv
    }
}

impl QuantumOperator for MaskingOperator {
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type Params = MaskingParams;

    fn apply(&self, mut input: Self::Input, params: &Self::Params) -> Result<Self::Output> {
        if input.is_empty() {
            return Ok(input);
        }

        // Step 1: Apply permutation U_σ
        let perm = self.permutation(&params.sigma, input.len());
        let permuted = self.apply_permutation(&input, &perm);

        // Step 2: Apply phase rotation e^{iθ} (XOR with phase key)
        let phase_key = self.phase_key(params.theta, permuted.len());
        let masked: Vec<u8> = permuted
            .iter()
            .zip(phase_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        // Zeroize sensitive data
        input.zeroize();

        Ok(masked)
    }
}

impl Default for MaskingOperator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masking_roundtrip() {
        let op = MaskingOperator::new();
        let message = b"Hello, Quantum Resonant Blockchain!";
        let params = MaskingParams::random();

        let masked = op.mask(message, &params).unwrap();
        assert_ne!(
            &masked[..],
            &message[..],
            "Masked should differ from original"
        );

        let unmasked = op.unmask(&masked, &params).unwrap();
        assert_eq!(
            &unmasked[..],
            &message[..],
            "Unmasking should restore original"
        );
    }

    #[test]
    fn test_masking_deterministic() {
        let op = MaskingOperator::new();
        let message = b"Deterministic test";
        let params = MaskingParams::from_seed(b"test_seed");

        let masked1 = op.mask(message, &params).unwrap();
        let masked2 = op.mask(message, &params).unwrap();

        assert_eq!(
            masked1, masked2,
            "Masking should be deterministic with same params"
        );
    }

    #[test]
    fn test_masking_different_params() {
        let op = MaskingOperator::new();
        let message = b"Different params test";
        let params1 = MaskingParams::from_seed(b"seed1");
        let params2 = MaskingParams::from_seed(b"seed2");

        let masked1 = op.mask(message, &params1).unwrap();
        let masked2 = op.mask(message, &params2).unwrap();

        assert_ne!(
            masked1, masked2,
            "Different params should produce different masks"
        );
    }

    #[test]
    fn test_empty_message() {
        let op = MaskingOperator::new();
        let message: &[u8] = b"";
        let params = MaskingParams::random();

        let masked = op.mask(message, &params).unwrap();
        assert_eq!(masked.len(), 0);
    }

    #[test]
    fn test_large_message() {
        let op = MaskingOperator::new();
        let message = vec![42u8; 10_000];
        let params = MaskingParams::random();

        let masked = op.mask(&message, &params).unwrap();
        assert_eq!(masked.len(), message.len());

        let unmasked = op.unmask(&masked, &params).unwrap();
        assert_eq!(unmasked, message);
    }
}
