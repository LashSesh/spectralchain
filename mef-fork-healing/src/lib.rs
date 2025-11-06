/*!
 * MEF Fork Healing - Self-Healing via MEF-Attractor
 *
 * Implements deterministic fork resolution using Mandorla Eigenstate Fractals.
 * When multiple competing blocks exist, the MEF-Attractor selects the one
 * with highest resonance coherence.
 *
 * # Principle
 *
 * Fork Resolution via Mandorla Attractor:
 * 1. Fork detected: Multiple blocks at same height
 * 2. Compute Mandorla coherence for each candidate
 * 3. Select block with highest coherence (strongest attractor)
 * 4. Ledger remains deterministic and invariant
 */

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod attractor;
pub mod multiversum;

pub use attractor::{AttractorConfig, CoherenceScore, MandorlaAttractor};
pub use multiversum::{ForkCandidate, ForkResolution, Multiversum};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Resonance state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResonanceState {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
}

impl ResonanceState {
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    pub fn coherence_with(&self, other: &ResonanceState) -> f64 {
        let dpsi = self.psi - other.psi;
        let drho = self.rho - other.rho;
        let domega = self.omega - other.omega;
        let distance = (dpsi * dpsi + drho * drho + domega * domega).sqrt();
        1.0 / (1.0 + distance)
    }
}

/// Block representation (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub height: u64,
    pub resonance: ResonanceState,
    pub prev_hash: Vec<u8>,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

impl Block {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.prev_hash);
        hasher.update(&self.data);
        hasher.finalize().to_vec()
    }

    pub fn compute_coherence(&self, field_resonance: &ResonanceState) -> f64 {
        self.resonance.coherence_with(field_resonance)
    }
}

/// Fork Healer - Main interface
pub struct ForkHealer {
    attractor: MandorlaAttractor,
    multiversum: Multiversum,
}

impl ForkHealer {
    pub fn new(config: AttractorConfig) -> Self {
        Self {
            attractor: MandorlaAttractor::new(config),
            multiversum: Multiversum::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(AttractorConfig::default())
    }

    pub fn resolve_fork(
        &self,
        candidates: Vec<Block>,
        field_resonance: ResonanceState,
    ) -> Result<Block> {
        self.attractor.resolve_fork(candidates, field_resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_coherence() {
        let r1 = ResonanceState::new(1.0, 1.0, 1.0);
        let r2 = ResonanceState::new(1.0, 1.0, 1.0);
        assert!((r1.coherence_with(&r2) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_fork_healing() {
        let healer = ForkHealer::default();
        let field = ResonanceState::new(1.0, 1.0, 1.0);

        let block1 = Block {
            id: Uuid::new_v4(),
            height: 10,
            resonance: ResonanceState::new(1.0, 1.0, 1.0),
            prev_hash: vec![0; 32],
            data: vec![1, 2, 3],
            timestamp: 0,
        };

        let block2 = Block {
            id: Uuid::new_v4(),
            height: 10,
            resonance: ResonanceState::new(5.0, 5.0, 5.0),
            prev_hash: vec![0; 32],
            data: vec![4, 5, 6],
            timestamp: 0,
        };

        let winner = healer
            .resolve_fork(vec![block1.clone(), block2], field)
            .unwrap();
        assert_eq!(winner.id, block1.id);
    }
}
