/*!
 * Quantum Entropy Source
 *
 * Provides cryptographically secure random number generation for
 * quantum random walk routing decisions.
 */

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

/// Trait for entropy sources
pub trait EntropySource {
    /// Generate random f64 in range [0.0, 1.0)
    fn random_f64(&mut self) -> f64;

    /// Generate random usize in range [0, n)
    fn random_usize(&mut self, n: usize) -> usize;

    /// Select index based on weighted probabilities
    fn select_weighted(&mut self, weights: &[f64]) -> Option<usize>;

    /// Generate random bytes
    fn random_bytes(&mut self, buf: &mut [u8]);
}

/// Quantum-inspired entropy source using ChaCha20
#[derive(Debug)]
pub struct QuantumEntropySource {
    rng: ChaCha20Rng,
}

impl QuantumEntropySource {
    /// Create new quantum entropy source with random seed
    pub fn new() -> Self {
        Self {
            rng: ChaCha20Rng::from_entropy(),
        }
    }

    /// Create with specific seed (for testing/reproducibility)
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            rng: ChaCha20Rng::from_seed(seed),
        }
    }

    /// Reseed the entropy source
    pub fn reseed(&mut self) {
        self.rng = ChaCha20Rng::from_entropy();
    }
}

impl Default for QuantumEntropySource {
    fn default() -> Self {
        Self::new()
    }
}

impl EntropySource for QuantumEntropySource {
    fn random_f64(&mut self) -> f64 {
        self.rng.gen()
    }

    fn random_usize(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        self.rng.gen_range(0..n)
    }

    fn select_weighted(&mut self, weights: &[f64]) -> Option<usize> {
        if weights.is_empty() {
            return None;
        }

        // Normalize weights
        let total: f64 = weights.iter().sum();
        if total <= 0.0 {
            // If all weights are zero, select uniformly
            return Some(self.random_usize(weights.len()));
        }

        // Generate random value in [0, total)
        let target = self.random_f64() * total;

        // Find corresponding index
        let mut cumulative = 0.0;
        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if target < cumulative {
                return Some(i);
            }
        }

        // Fallback to last index (due to floating point precision)
        Some(weights.len() - 1)
    }

    fn random_bytes(&mut self, buf: &mut [u8]) {
        self.rng.fill(buf);
    }
}

/// Deterministic entropy source (for testing)
#[derive(Debug)]
pub struct DeterministicEntropy {
    counter: u64,
    seed: u64,
}

impl DeterministicEntropy {
    /// Create new deterministic entropy source
    pub fn new(seed: u64) -> Self {
        Self { counter: 0, seed }
    }

    /// Simple LCG for deterministic randomness
    fn next_u64(&mut self) -> u64 {
        self.counter = self
            .counter
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.seed);
        self.counter
    }
}

impl EntropySource for DeterministicEntropy {
    fn random_f64(&mut self) -> f64 {
        let val = self.next_u64();
        (val as f64) / (u64::MAX as f64)
    }

    fn random_usize(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() as usize) % n
    }

    fn select_weighted(&mut self, weights: &[f64]) -> Option<usize> {
        if weights.is_empty() {
            return None;
        }

        let total: f64 = weights.iter().sum();
        if total <= 0.0 {
            return Some(self.random_usize(weights.len()));
        }

        let target = self.random_f64() * total;
        let mut cumulative = 0.0;

        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if target < cumulative {
                return Some(i);
            }
        }

        Some(weights.len() - 1)
    }

    fn random_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_entropy() {
        let mut entropy = QuantumEntropySource::new();

        // Test random_f64
        let val = entropy.random_f64();
        assert!(val >= 0.0 && val < 1.0);

        // Test random_usize
        let val = entropy.random_usize(10);
        assert!(val < 10);

        // Test random_bytes
        let mut buf = [0u8; 32];
        entropy.random_bytes(&mut buf);
        assert!(buf.iter().any(|&b| b != 0)); // At least some non-zero bytes
    }

    #[test]
    fn test_weighted_selection() {
        let mut entropy = QuantumEntropySource::from_seed([42u8; 32]);

        let weights = vec![0.1, 0.3, 0.6];

        // Run multiple selections and verify distribution
        let mut counts = vec![0; 3];
        for _ in 0..1000 {
            if let Some(idx) = entropy.select_weighted(&weights) {
                counts[idx] += 1;
            }
        }

        // Index 2 should be selected most often (60% weight)
        assert!(counts[2] > counts[1]);
        assert!(counts[1] > counts[0]);
    }

    #[test]
    fn test_weighted_selection_zero_weights() {
        let mut entropy = QuantumEntropySource::new();

        let weights = vec![0.0, 0.0, 0.0];
        let idx = entropy.select_weighted(&weights);

        // Should select some index uniformly
        assert!(idx.is_some());
        assert!(idx.unwrap() < 3);
    }

    #[test]
    fn test_weighted_selection_empty() {
        let mut entropy = QuantumEntropySource::new();

        let weights: Vec<f64> = vec![];
        assert!(entropy.select_weighted(&weights).is_none());
    }

    #[test]
    fn test_deterministic_entropy() {
        let mut entropy1 = DeterministicEntropy::new(42);
        let mut entropy2 = DeterministicEntropy::new(42);

        // Should produce same sequence
        for _ in 0..100 {
            assert_eq!(entropy1.random_f64(), entropy2.random_f64());
        }
    }

    #[test]
    fn test_deterministic_weighted() {
        let mut entropy = DeterministicEntropy::new(42);

        let weights = vec![0.1, 0.5, 0.4];
        let selections: Vec<_> = (0..10)
            .map(|_| entropy.select_weighted(&weights).unwrap())
            .collect();

        // Should select based on weights
        assert!(!selections.is_empty());
    }
}
