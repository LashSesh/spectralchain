/*!
 * MEF Ephemeral Services - Ghost Services Framework
 *
 * Services erscheinen als temporäre "Blasen" im Resonanzfeld,
 * verschwinden nach Benutzung, bleiben aber auditierbar per Proof.
 *
 * # Concept
 *
 * Ephemeral services are temporary, privacy-preserving services that:
 * - Appear as resonance bubbles in the field
 * - Dissolve automatically after use
 * - Leave auditable proof trails without identity disclosure
 * - Enable anonymous marketplaces, voting, messaging, etc.
 *
 * # Use Cases
 *
 * - Ephemeral Voting: Anonymous, proof-based elections
 * - Ghost Marketplaces: Temporary, privacy-first trading
 * - Transient Messaging: Self-destructing communication channels
 * - Privacy-Preserving Auctions: Sealed-bid auctions with ZK proofs
 */

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod service_registry;
pub mod lifecycle;
pub mod bubble;
pub mod audit_trail;

pub use service_registry::{ServiceRegistry, ServiceDescriptor, ServiceType};
pub use lifecycle::{LifecycleManager, LifecycleState, LifecycleEvent};
pub use bubble::{ResonanceBubble, BubbleConfig, BubbleState};
pub use audit_trail::{AuditTrail, AuditEntry, ProofCarryingAudit};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Resonance state (re-exported for convenience)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResonanceState {
    /// Psi dimension
    pub psi: f64,
    /// Rho dimension
    pub rho: f64,
    /// Omega dimension
    pub omega: f64,
}

impl ResonanceState {
    /// Create new resonance state
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }
}

/// Ephemeral Service - High-level interface
pub struct EphemeralService {
    /// Service ID (ephemeral)
    pub id: Uuid,

    /// Service descriptor
    pub descriptor: ServiceDescriptor,

    /// Resonance bubble
    pub bubble: Arc<std::sync::RwLock<ResonanceBubble>>,

    /// Lifecycle manager
    pub lifecycle: Arc<std::sync::RwLock<LifecycleManager>>,

    /// Audit trail
    pub audit: Arc<std::sync::RwLock<AuditTrail>>,
}

impl EphemeralService {
    /// Create new ephemeral service
    pub fn new(
        service_type: ServiceType,
        resonance: ResonanceState,
        duration_seconds: u64,
    ) -> Result<Self> {
        let id = Uuid::new_v4();

        let descriptor = ServiceDescriptor {
            id,
            service_type,
            resonance,
            created_at: Self::current_timestamp(),
        };

        let bubble_config = BubbleConfig {
            resonance,
            radius: 0.1,
            duration_seconds,
            max_participants: 100,
        };

        let bubble = ResonanceBubble::new(id, bubble_config)?;
        let lifecycle = LifecycleManager::new(id, duration_seconds);
        let audit = AuditTrail::new(id);

        Ok(Self {
            id,
            descriptor,
            bubble: Arc::new(std::sync::RwLock::new(bubble)),
            lifecycle: Arc::new(std::sync::RwLock::new(lifecycle)),
            audit: Arc::new(std::sync::RwLock::new(audit)),
        })
    }

    /// Start the service
    pub fn start(&self) -> Result<()> {
        let mut lifecycle = self.lifecycle.write().unwrap();
        lifecycle.start()?;

        let mut audit = self.audit.write().unwrap();
        audit.record_event("service_started", None)?;

        Ok(())
    }

    /// Stop the service
    pub fn stop(&self) -> Result<()> {
        let mut lifecycle = self.lifecycle.write().unwrap();
        lifecycle.stop()?;

        let mut audit = self.audit.write().unwrap();
        audit.record_event("service_stopped", None)?;

        Ok(())
    }

    /// Check if service is active
    pub fn is_active(&self) -> bool {
        let lifecycle = self.lifecycle.read().unwrap();
        lifecycle.is_active()
    }

    /// Record activity
    pub fn record_activity(&self, activity: &str, proof: Option<Vec<u8>>) -> Result<()> {
        let mut audit = self.audit.write().unwrap();
        audit.record_event(activity, proof)
    }

    /// Get audit proof
    pub fn get_audit_proof(&self) -> Vec<u8> {
        let audit = self.audit.read().unwrap();
        audit.get_proof()
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_service_creation() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let service = EphemeralService::new(
            ServiceType::Marketplace,
            resonance,
            300,
        ).unwrap();

        assert_eq!(service.descriptor.resonance.psi, 1.0);
        assert!(!service.is_active());
    }

    #[test]
    fn test_service_lifecycle() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let service = EphemeralService::new(
            ServiceType::Voting,
            resonance,
            300,
        ).unwrap();

        service.start().unwrap();
        assert!(service.is_active());

        service.stop().unwrap();
        assert!(!service.is_active());
    }

    #[test]
    fn test_activity_recording() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let service = EphemeralService::new(
            ServiceType::Messaging,
            resonance,
            300,
        ).unwrap();

        service.start().unwrap();
        service.record_activity("message_sent", None).unwrap();

        let proof = service.get_audit_proof();
        assert!(!proof.is_empty());
    }
}
