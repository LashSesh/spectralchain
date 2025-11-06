//! Mandorla Attractor for Fork Resolution

use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::{Block, ResonanceState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractorConfig {
    pub coherence_weight: f64,
    pub timestamp_weight: f64,
}

impl Default for AttractorConfig {
    fn default() -> Self {
        Self {
            coherence_weight: 0.8,
            timestamp_weight: 0.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoherenceScore {
    pub block_id: uuid::Uuid,
    pub coherence: f64,
    pub timestamp_score: f64,
    pub total_score: f64,
}

pub struct MandorlaAttractor {
    config: AttractorConfig,
}

impl MandorlaAttractor {
    pub fn new(config: AttractorConfig) -> Self {
        Self { config }
    }

    pub fn resolve_fork(
        &self,
        candidates: Vec<Block>,
        field_resonance: ResonanceState,
    ) -> Result<Block> {
        if candidates.is_empty() {
            anyhow::bail!("No candidates for fork resolution");
        }

        let scores: Vec<CoherenceScore> = candidates
            .iter()
            .map(|block| {
                let coherence = block.compute_coherence(&field_resonance);
                let timestamp_score = 1.0 / (1.0 + block.timestamp as f64 / 1000.0);
                let total_score = self.config.coherence_weight * coherence
                    + self.config.timestamp_weight * timestamp_score;

                CoherenceScore {
                    block_id: block.id,
                    coherence,
                    timestamp_score,
                    total_score,
                }
            })
            .collect();

        let winner_idx = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_score.partial_cmp(&b.total_score).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        Ok(candidates[winner_idx].clone())
    }
}
