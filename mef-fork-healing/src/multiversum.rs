//! Multiversum Support for Fork Tracking

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkCandidate {
    pub block_id: Uuid,
    pub height: u64,
    pub branch_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkResolution {
    pub winner: Uuid,
    pub alternatives: Vec<Uuid>,
    pub timestamp: u64,
}

pub struct Multiversum {
    branches: Vec<ForkCandidate>,
}

impl Multiversum {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
        }
    }

    pub fn track_fork(&mut self, candidate: ForkCandidate) {
        self.branches.push(candidate);
    }
}
