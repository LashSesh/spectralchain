//! Service Registry

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::ResonanceState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    Voting,
    Marketplace,
    Messaging,
    Auction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor {
    pub id: Uuid,
    pub service_type: ServiceType,
    pub resonance: ResonanceState,
    pub created_at: u64,
}

pub struct ServiceRegistry;
