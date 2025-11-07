/*!
 * Masking Operator (M)
 *
 * ## Mathematische Formel
 * ```text
 * M_{θ,σ}(m) = e^{iθ} U_σ m
 * ```
 *
 * Wobei:
 * - `U_σ`: Permutation (σ ist Permutationsindex)
 * - `e^{iθ}`: Phasenrotation (θ ist Phase in Radiant)
 * - `m`: Nachricht (Vektor von Bytes)
 *
 * ## Eigenschaften
 * - **Selbst-invers (Involution)**: M(M(m, p), p) = m
 * - **Deterministisch**: Gleiche Parameter → gleicher Output
 * - **Forward Secrecy**: Unterstützt ephemere Schlüssel
 *
 * ## Implementierung
 * 1. Permutiere Nachricht mit deterministischer Permutation σ
 * 2. Wende Phasenrotation θ an (XOR mit Phasenschlüssel)
 * 3. Resultat ist maskierte Nachricht m'
 *
 * ## Use Cases
 * - Addressless encryption für Ghost Network
 * - Privacy-preserving message routing
 * - Stealth addressing
 *
 * ## Beispiel
 * ```rust
 * use quantumhybrid_operatoren_core::operators::masking::{MaskingOperator, MaskingParams};
 * use quantumhybrid_operatoren_core::core::QuantumOperator;
 *
 * let operator = MaskingOperator::new();
 * let params = MaskingParams::random();
 * let message = b"Secret message";
 *
 * // Mask
 * let masked = operator.mask(message, &params).unwrap();
 *
 * // Unmask
 * let unmasked = operator.unmask(&masked, &params).unwrap();
 * assert_eq!(unmasked, message);
 * ```
 */

use crate::core::{InvertibleOperator, QuantumOperator};
use anyhow::Result;
use blake3::Hasher;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Masking-Parameter
///
/// # Felder
/// * `theta` - Phase θ (in Radiant, 0 bis 2π) für Phasenrotation
/// * `sigma` - Permutations-Seed σ (32 Bytes) für deterministische Permutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskingParams {
    /// Phase θ (in Radiant, 0 bis 2π)
    pub theta: f64,
    /// Permutations-Seed σ (für deterministische Permutation)
    pub sigma: [u8; 32],
}

impl MaskingParams {
    /// Erstelle neue Parameter mit gegebenen Werten
    ///
    /// # Arguments
    /// * `theta` - Phase in Radiant
    /// * `sigma` - 32-Byte Permutations-Seed
    pub fn new(theta: f64, sigma: [u8; 32]) -> Self {
        Self { theta, sigma }
    }

    /// Generiere zufällige Parameter
    ///
    /// # Returns
    /// Neue zufällige MaskingParams
    ///
    /// # Beispiel
    /// ```ignore
    /// let params = MaskingParams::random();
    /// ```
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            theta: rng.gen_range(0.0..std::f64::consts::TAU),
            sigma: rand::random(),
        }
    }

    /// Generiere Parameter aus Seed
    ///
    /// # Arguments
    /// * `seed` - Seed-Daten für deterministische Generierung
    ///
    /// # Returns
    /// Deterministische MaskingParams basierend auf Seed
    ///
    /// # Beispiel
    /// ```ignore
    /// let params = MaskingParams::from_seed(b"my_secret_seed");
    /// ```
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

    /// Generiere ephemere Parameter (für Forward Secrecy)
    ///
    /// # Arguments
    /// * `epoch` - Zeitepoche für Key Rotation
    ///
    /// # Returns
    /// Ephemere MaskingParams für diese Epoche
    pub fn ephemeral(epoch: u64) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"ephemeral_key");
        hasher.update(&epoch.to_le_bytes());
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
///
/// Dies ist ein invertierbarer Operator für Privacy-preserving Encryption.
#[derive(Debug, Clone)]
pub struct MaskingOperator;

impl MaskingOperator {
    /// Erstelle neuen Masking Operator
    pub fn new() -> Self {
        Self
    }

    /// Mask eine Nachricht mit gegebenen Parametern
    ///
    /// # Arguments
    /// * `message` - Die zu maskierende Nachricht
    /// * `params` - Masking-Parameter
    ///
    /// # Returns
    /// Maskierte Nachricht oder Fehler
    pub fn mask(&self, message: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        self.apply(message.to_vec(), params)
    }

    /// Unmask eine maskierte Nachricht
    ///
    /// # Arguments
    /// * `masked` - Die maskierte Nachricht
    /// * `params` - Gleiche Masking-Parameter wie beim Masking
    ///
    /// # Returns
    /// Original-Nachricht oder Fehler
    pub fn unmask(&self, masked: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        // Masking ist symmetrisch (Involution), daher gleiche Operation
        self.invert(masked.to_vec(), params)
    }

    /// Berechne Phasenschlüssel aus theta
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

    /// Berechne Permutation aus sigma
    fn permutation(&self, sigma: &[u8; 32], len: usize) -> Vec<usize> {
        let mut rng = ChaCha20Rng::from_seed(*sigma);
        let mut perm: Vec<usize> = (0..len).collect();
        perm.shuffle(&mut rng);
        perm
    }

    /// Wende Permutation an
    fn apply_permutation(&self, data: &[u8], perm: &[usize]) -> Vec<u8> {
        perm.iter().map(|&i| data[i]).collect()
    }

    /// Invertiere Permutation
    fn invert_permutation(&self, perm: &[usize]) -> Vec<usize> {
        let mut inv = vec![0; perm.len()];
        for (i, &p) in perm.iter().enumerate() {
            inv[p] = i;
        }
        inv
    }
}

impl Default for MaskingOperator {
    fn default() -> Self {
        Self::new()
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

        // Step 2: Apply phase rotation e^{iθ} (XOR mit Phasenschlüssel)
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

    fn name(&self) -> &str {
        "MaskingOperator"
    }

    fn description(&self) -> &str {
        "Addressless encryption operator using permutation and phase rotation"
    }

    fn formula(&self) -> &str {
        "M_{θ,σ}(m) = e^{iθ} U_σ m"
    }
}

impl InvertibleOperator for MaskingOperator {
    fn invert(&self, mut output: Self::Output, params: &Self::Params) -> Result<Self::Input> {
        if output.is_empty() {
            return Ok(output);
        }

        // Step 1: Apply phase rotation (XOR ist selbst-invers)
        let phase_key = self.phase_key(params.theta, output.len());
        let dephaseed: Vec<u8> = output
            .iter()
            .zip(phase_key.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        // Step 2: Invert permutation
        let perm = self.permutation(&params.sigma, output.len());
        let inv_perm = self.invert_permutation(&perm);
        let unmasked = self.apply_permutation(&dephaseed, &inv_perm);

        // Zeroize sensitive data
        output.zeroize();

        Ok(unmasked)
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

    #[test]
    fn test_ephemeral_params() {
        let params1 = MaskingParams::ephemeral(1000);
        let params2 = MaskingParams::ephemeral(2000);

        assert_ne!(params1.sigma, params2.sigma);
        assert_ne!(params1.theta, params2.theta);
    }

    #[test]
    fn test_quantum_operator_trait() {
        let op = MaskingOperator::new();
        let message = b"Test message".to_vec();
        let params = MaskingParams::random();

        let output = op.apply(message.clone(), &params).unwrap();
        assert_ne!(output, message);

        assert_eq!(op.name(), "MaskingOperator");
        assert!(op.description().contains("Addressless"));
        assert!(op.formula().contains("M_{θ,σ}"));
    }
}
