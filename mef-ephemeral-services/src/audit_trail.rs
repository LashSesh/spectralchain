//! Audit Trail with Zero-Knowledge Proofs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub event: String,
    pub proof: Option<Vec<u8>>,
}

pub struct AuditTrail {
    id: Uuid,
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            entries: Vec::new(),
        }
    }

    pub fn record_event(&mut self, event: &str, proof: Option<Vec<u8>>) -> Result<()> {
        self.entries.push(AuditEntry {
            timestamp: Self::now(),
            event: event.to_string(),
            proof,
        });
        Ok(())
    }

    pub fn get_proof(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        for entry in &self.entries {
            hasher.update(&entry.timestamp.to_le_bytes());
            hasher.update(entry.event.as_bytes());
        }
        hasher.finalize().to_vec()
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

pub trait ProofCarryingAudit {
    fn verify_proof(&self, proof: &[u8]) -> bool;
}
