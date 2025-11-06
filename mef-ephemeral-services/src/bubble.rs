//! Resonance Bubble

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::ResonanceState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BubbleConfig {
    pub resonance: ResonanceState,
    pub radius: f64,
    pub duration_seconds: u64,
    pub max_participants: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubbleState {
    Active,
    Dissolved,
}

pub struct ResonanceBubble {
    id: Uuid,
    config: BubbleConfig,
    state: BubbleState,
    created_at: u64,
}

impl ResonanceBubble {
    pub fn new(id: Uuid, config: BubbleConfig) -> Result<Self> {
        Ok(Self {
            id,
            config,
            state: BubbleState::Active,
            created_at: Self::now(),
        })
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
