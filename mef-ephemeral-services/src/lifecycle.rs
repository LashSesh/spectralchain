//! Lifecycle Management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    Created,
    Active,
    Stopped,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleEvent {
    pub timestamp: u64,
    pub state: LifecycleState,
}

pub struct LifecycleManager {
    id: Uuid,
    state: LifecycleState,
    duration: u64,
    started_at: Option<u64>,
}

impl LifecycleManager {
    pub fn new(id: Uuid, duration: u64) -> Self {
        Self {
            id,
            state: LifecycleState::Created,
            duration,
            started_at: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.state = LifecycleState::Active;
        self.started_at = Some(Self::now());
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.state = LifecycleState::Stopped;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == LifecycleState::Active
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
