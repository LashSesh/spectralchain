/*!
 * Node Discovery via Temporary Resonance Events
 *
 * Implements privacy-preserving node discovery where nodes find each other
 * through temporary resonance patterns rather than fixed addresses.
 *
 * Key Features:
 * - No fixed node lists or directories
 * - Temporary resonance beacons
 * - Ephemeral discovery events
 * - Privacy-first design (no tracking)
 */

use crate::packet::{CarrierType, GhostPacket, NodeIdentity, ResonanceState};
use crate::transport::{PeerId, Transport};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Discovery beacon - temporary resonance announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryBeacon {
    /// Beacon ID (ephemeral)
    pub id: Uuid,

    /// Node's current resonance state
    pub resonance: ResonanceState,

    /// Timestamp of beacon creation
    pub timestamp: u64,

    /// Time-to-live in seconds
    pub ttl_seconds: u64,

    /// Optional capabilities/metadata
    pub capabilities: Option<Vec<String>>,

    /// Beacon signature (for authenticity)
    pub signature: Option<Vec<u8>>,
}

impl DiscoveryBeacon {
    /// Create new discovery beacon
    pub fn new(
        resonance: ResonanceState,
        ttl_seconds: u64,
        capabilities: Option<Vec<String>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            resonance,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds,
            capabilities,
            signature: None,
        }
    }

    /// Check if beacon is still valid
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now < self.timestamp + self.ttl_seconds
    }

    /// Get age in seconds
    pub fn age(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now.saturating_sub(self.timestamp)
    }

    /// Check if beacon matches resonance window
    pub fn matches_resonance(&self, target: &ResonanceState, epsilon: f64) -> bool {
        self.resonance.is_resonant_with(target, epsilon)
    }
}

/// Discovery event - temporary resonance pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryEvent {
    /// Event ID
    pub id: Uuid,

    /// Resonance pattern for this event
    pub resonance_pattern: Vec<ResonanceState>,

    /// Event timestamp
    pub timestamp: u64,

    /// Event duration (seconds)
    pub duration: u64,

    /// Participating node count
    pub node_count: usize,

    /// Event type
    pub event_type: EventType,
}

/// Type of discovery event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Random rendezvous event
    Rendezvous,

    /// Scheduled gathering
    Scheduled,

    /// Emergency discovery
    Emergency,

    /// Routine background discovery
    Background,
}

impl DiscoveryEvent {
    /// Create new discovery event
    pub fn new(
        resonance_pattern: Vec<ResonanceState>,
        duration: u64,
        event_type: EventType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            resonance_pattern,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            duration,
            node_count: 0,
            event_type,
        }
    }

    /// Check if event is currently active
    pub fn is_active(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let end_time = self.timestamp + self.duration;
        now >= self.timestamp && now < end_time
    }

    /// Get current resonance state for this event
    pub fn current_resonance(&self) -> Option<ResonanceState> {
        if !self.is_active() {
            return None;
        }

        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - self.timestamp;

        let index = ((elapsed as f64 / self.duration as f64) * self.resonance_pattern.len() as f64)
            as usize;

        self.resonance_pattern.get(index).copied()
    }
}

/// Discovered node information
#[derive(Debug, Clone)]
pub struct DiscoveredNode {
    /// Node identity
    pub identity: NodeIdentity,

    /// Discovery timestamp
    pub discovered_at: u64,

    /// Last seen timestamp
    pub last_seen: u64,

    /// Beacon that led to discovery
    pub beacon_id: Uuid,

    /// Capabilities
    pub capabilities: Option<Vec<String>>,
}

impl DiscoveredNode {
    /// Create new discovered node entry
    pub fn new(identity: NodeIdentity, beacon_id: Uuid, capabilities: Option<Vec<String>>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            identity,
            discovered_at: now,
            last_seen: now,
            beacon_id,
            capabilities,
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Check if node is recently active (within timeout)
    pub fn is_active(&self, timeout_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now < self.last_seen + timeout_seconds
    }
}

/// Discovery statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryStats {
    /// Total beacons sent
    pub beacons_sent: usize,

    /// Total beacons received
    pub beacons_received: usize,

    /// Total nodes discovered
    pub nodes_discovered: usize,

    /// Total events participated
    pub events_participated: usize,

    /// Average discovery time (ms)
    pub avg_discovery_time_ms: u64,
}

/// Discovery engine for finding nodes via resonance
pub struct DiscoveryEngine {
    /// Active beacons (own and received)
    beacons: Arc<RwLock<HashMap<Uuid, DiscoveryBeacon>>>,

    /// Discovered nodes
    discovered_nodes: Arc<RwLock<HashMap<Uuid, DiscoveredNode>>>,

    /// Active discovery events
    events: Arc<RwLock<HashMap<Uuid, DiscoveryEvent>>>,

    /// Statistics
    stats: Arc<RwLock<DiscoveryStats>>,

    /// Node timeout (seconds)
    node_timeout: u64,

    /// Beacon TTL (seconds)
    beacon_ttl: u64,

    /// Discovery epsilon for resonance matching
    discovery_epsilon: f64,

    /// Optional network transport (None = local discovery only)
    transport: Option<Arc<Mutex<dyn Transport>>>,
}

impl DiscoveryEngine {
    /// Create new discovery engine (local only)
    pub fn new(node_timeout: u64, beacon_ttl: u64, discovery_epsilon: f64) -> Self {
        Self {
            beacons: Arc::new(RwLock::new(HashMap::new())),
            discovered_nodes: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DiscoveryStats::default())),
            node_timeout,
            beacon_ttl,
            discovery_epsilon,
            transport: None,
        }
    }

    /// Create with network transport
    pub fn with_transport(
        node_timeout: u64,
        beacon_ttl: u64,
        discovery_epsilon: f64,
        transport: Arc<Mutex<dyn Transport>>,
    ) -> Self {
        Self {
            beacons: Arc::new(RwLock::new(HashMap::new())),
            discovered_nodes: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DiscoveryStats::default())),
            node_timeout,
            beacon_ttl,
            discovery_epsilon,
            transport: Some(transport),
        }
    }

    /// Create with default settings (local only)
    pub fn default() -> Self {
        Self::new(
            300, // 5 minute node timeout
            120, // 2 minute beacon TTL
            0.2, // Discovery epsilon (wider than normal)
        )
    }

    /// Announce presence via beacon
    ///
    /// If transport is configured, broadcasts beacon to network.
    /// Otherwise, only stores locally.
    pub async fn announce(
        &self,
        identity: &NodeIdentity,
        capabilities: Option<Vec<String>>,
    ) -> Result<Uuid> {
        let beacon = DiscoveryBeacon::new(identity.resonance, self.beacon_ttl, capabilities);

        let beacon_id = beacon.id;

        // Store beacon locally
        let mut beacons = self
            .beacons
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on beacons: {}", e))?;
        beacons.insert(beacon_id, beacon.clone());
        drop(beacons); // Release lock

        // If we have transport, broadcast beacon to network
        if let Some(ref transport) = self.transport {
            // Serialize beacon to packet payload
            let beacon_bytes = serde_json::to_vec(&beacon).context("Failed to serialize beacon")?;

            // Create Ghost packet with beacon
            // For beacon announcements:
            // - target_resonance = our resonance (nodes searching for us)
            // - sender_resonance = our resonance (we're announcing ourselves)
            // - masked_payload = beacon data
            // - stego_carrier = same data (no steganography for beacons)
            let packet = GhostPacket::new(
                identity.resonance,   // target_resonance
                identity.resonance,   // sender_resonance
                beacon_bytes.clone(), // masked_payload
                beacon_bytes,         // stego_carrier
                CarrierType::Raw,     // carrier_type
                None,                 // zk_proof
            );

            // Broadcast via transport
            let mut t = transport.lock().await;
            t.broadcast(packet)
                .await
                .context("Failed to broadcast beacon via transport")?;
        }

        let mut stats = self
            .stats
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on stats: {}", e))?;
        stats.beacons_sent += 1;

        Ok(beacon_id)
    }

    /// Poll for beacons from network transport
    ///
    /// Receives beacon packets from transport and processes them.
    /// Call this periodically when using network transport.
    pub async fn poll_beacons(&self) -> Result<usize> {
        if let Some(ref transport) = self.transport {
            let mut t = transport.lock().await;
            let mut beacons_received = 0;

            // Try to receive multiple beacons (non-blocking)
            loop {
                match tokio::time::timeout(
                    std::time::Duration::from_millis(10), // Short timeout
                    t.receive(),
                )
                .await
                {
                    Ok(Ok((_peer_id, packet))) => {
                        // Try to deserialize beacon from packet payload
                        if let Ok(beacon) =
                            serde_json::from_slice::<DiscoveryBeacon>(&packet.masked_payload)
                        {
                            if self.receive_beacon(beacon).is_ok() {
                                beacons_received += 1;
                            }
                        }
                    }
                    _ => break, // Timeout or error - no more beacons
                }
            }

            Ok(beacons_received)
        } else {
            Ok(0) // No transport, nothing to poll
        }
    }

    /// Receive beacon from another node
    pub fn receive_beacon(&self, beacon: DiscoveryBeacon) -> Result<()> {
        if !beacon.is_valid() {
            warn!(
                event = "beacon_rejected",
                reason = "expired",
                beacon_id = %beacon.id,
                timestamp = beacon.timestamp,
                ttl = beacon.ttl_seconds,
                age = beacon.age(),
                "Security: Beacon rejected due to expiration"
            );
            anyhow::bail!("Beacon expired");
        }

        let beacon_id = beacon.id;

        // Store beacon
        let mut beacons = self
            .beacons
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on beacons: {}", e))?;
        beacons.insert(beacon_id, beacon.clone());
        drop(beacons); // Release lock

        // Create node identity from beacon
        let node_identity = NodeIdentity {
            id: beacon.id,
            resonance: beacon.resonance,
            last_update: beacon.timestamp,
            public_key: None,
        };

        // Add to discovered nodes
        let mut discovered = self.discovered_nodes.write().map_err(|e| {
            anyhow::anyhow!("Failed to acquire write lock on discovered_nodes: {}", e)
        })?;
        let is_new_node = if let Some(node) = discovered.get_mut(&node_identity.id) {
            node.update_last_seen();
            false
        } else {
            let node = DiscoveredNode::new(node_identity, beacon_id, beacon.capabilities);
            discovered.insert(node.identity.id, node);
            true
        };
        drop(discovered); // Release lock

        let mut stats = self
            .stats
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on stats: {}", e))?;
        if is_new_node {
            stats.nodes_discovered += 1;
            info!(
                event = "node_discovered",
                beacon_id = %beacon_id,
                node_id = %node_identity.id,
                resonance = ?(node_identity.resonance.psi, node_identity.resonance.rho, node_identity.resonance.omega),
                capabilities = ?beacon.capabilities,
                "New node discovered via beacon"
            );
        }
        stats.beacons_received += 1;

        Ok(())
    }

    /// Find nodes matching a resonance state
    pub fn find_nodes(&self, target_resonance: &ResonanceState) -> Vec<DiscoveredNode> {
        let discovered = self.discovered_nodes.read().unwrap_or_else(|e| {
            eprintln!("Warning: RwLock poisoned in find_nodes: {}", e);
            e.into_inner()
        });

        discovered
            .values()
            .filter(|node| {
                node.is_active(self.node_timeout)
                    && node
                        .identity
                        .resonance
                        .is_resonant_with(target_resonance, self.discovery_epsilon)
            })
            .cloned()
            .collect()
    }

    /// Find nodes with specific capabilities
    pub fn find_nodes_with_capabilities(&self, required_caps: &[String]) -> Vec<DiscoveredNode> {
        let discovered = self.discovered_nodes.read().unwrap_or_else(|e| {
            eprintln!(
                "Warning: RwLock poisoned in find_nodes_with_capabilities: {}",
                e
            );
            e.into_inner()
        });

        discovered
            .values()
            .filter(|node| {
                if !node.is_active(self.node_timeout) {
                    return false;
                }

                if let Some(ref caps) = node.capabilities {
                    required_caps.iter().all(|req| caps.contains(req))
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    /// Create discovery event
    pub fn create_event(
        &self,
        resonance_pattern: Vec<ResonanceState>,
        duration: u64,
        event_type: EventType,
    ) -> Result<Uuid> {
        let event = DiscoveryEvent::new(resonance_pattern, duration, event_type);
        let event_id = event.id;

        let mut events = self
            .events
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on events: {}", e))?;
        events.insert(event_id, event);

        Ok(event_id)
    }

    /// Participate in discovery event
    pub fn participate_in_event(&self, event_id: Uuid) -> Result<Option<ResonanceState>> {
        let events = self
            .events
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire read lock on events: {}", e))?;

        if let Some(event) = events.get(&event_id) {
            if event.is_active() {
                let current_resonance = event.current_resonance();
                drop(events); // Release read lock before acquiring write lock

                let mut stats = self
                    .stats
                    .write()
                    .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on stats: {}", e))?;
                stats.events_participated += 1;

                Ok(current_resonance)
            } else {
                Ok(None)
            }
        } else {
            anyhow::bail!("Event not found")
        }
    }

    /// Get all active nodes
    pub fn get_active_nodes(&self) -> Vec<DiscoveredNode> {
        let discovered = self.discovered_nodes.read().unwrap_or_else(|e| {
            eprintln!("Warning: RwLock poisoned in get_active_nodes: {}", e);
            e.into_inner()
        });

        discovered
            .values()
            .filter(|node| node.is_active(self.node_timeout))
            .cloned()
            .collect()
    }

    /// Cleanup expired beacons and inactive nodes
    pub fn cleanup(&self) -> Result<(usize, usize)> {
        // Cleanup expired beacons
        let mut beacons = self
            .beacons
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock on beacons: {}", e))?;
        let expired_beacons: Vec<Uuid> = beacons
            .iter()
            .filter(|(_, beacon)| !beacon.is_valid())
            .map(|(id, _)| *id)
            .collect();

        for id in expired_beacons.iter() {
            beacons.remove(id);
        }
        drop(beacons); // Release lock

        // Cleanup inactive nodes
        let mut discovered = self.discovered_nodes.write().map_err(|e| {
            anyhow::anyhow!("Failed to acquire write lock on discovered_nodes: {}", e)
        })?;
        let inactive_nodes: Vec<Uuid> = discovered
            .iter()
            .filter(|(_, node)| !node.is_active(self.node_timeout))
            .map(|(id, _)| *id)
            .collect();

        for id in inactive_nodes.iter() {
            discovered.remove(id);
        }

        Ok((expired_beacons.len(), inactive_nodes.len()))
    }

    /// Get statistics
    pub fn get_stats(&self) -> DiscoveryStats {
        self.stats
            .read()
            .unwrap_or_else(|e| {
                eprintln!("Warning: RwLock poisoned in get_stats: {}", e);
                e.into_inner()
            })
            .clone()
    }

    /// Get active node count
    pub fn active_node_count(&self) -> usize {
        self.get_active_nodes().len()
    }

    /// Get active beacon count
    pub fn active_beacon_count(&self) -> usize {
        let beacons = self.beacons.read().unwrap_or_else(|e| {
            eprintln!("Warning: RwLock poisoned in active_beacon_count: {}", e);
            e.into_inner()
        });
        beacons.values().filter(|b| b.is_valid()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beacon_creation() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let beacon = DiscoveryBeacon::new(resonance, 300, None);

        assert_eq!(beacon.resonance.psi, 1.0);
        assert!(beacon.is_valid());
        assert_eq!(beacon.age(), 0);
    }

    #[test]
    fn test_beacon_expiration() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut beacon = DiscoveryBeacon::new(resonance, 1, None);

        assert!(beacon.is_valid());

        // Simulate expiration
        beacon.timestamp -= 2;
        assert!(!beacon.is_valid());
    }

    #[tokio::test]
    async fn test_discovery_engine() {
        let engine = DiscoveryEngine::default();

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let identity = NodeIdentity::new(resonance, None);

        let beacon_id = engine.announce(&identity, None).await.unwrap();
        assert_eq!(engine.active_beacon_count(), 1);

        let stats = engine.get_stats();
        assert_eq!(stats.beacons_sent, 1);
    }

    #[test]
    fn test_receive_beacon() {
        let engine = DiscoveryEngine::default();

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let beacon = DiscoveryBeacon::new(resonance, 300, None);

        engine.receive_beacon(beacon).unwrap();

        assert_eq!(engine.active_node_count(), 1);

        let stats = engine.get_stats();
        assert_eq!(stats.beacons_received, 1);
        assert_eq!(stats.nodes_discovered, 1);
    }

    #[test]
    fn test_find_nodes_by_resonance() {
        let engine = DiscoveryEngine::default();

        // Add some nodes
        let resonance1 = ResonanceState::new(1.0, 1.0, 1.0);
        let beacon1 = DiscoveryBeacon::new(resonance1, 300, None);
        engine.receive_beacon(beacon1).unwrap();

        let resonance2 = ResonanceState::new(1.1, 1.1, 1.1);
        let beacon2 = DiscoveryBeacon::new(resonance2, 300, None);
        engine.receive_beacon(beacon2).unwrap();

        let resonance3 = ResonanceState::new(5.0, 5.0, 5.0);
        let beacon3 = DiscoveryBeacon::new(resonance3, 300, None);
        engine.receive_beacon(beacon3).unwrap();

        // Find nodes near (1.0, 1.0, 1.0)
        let target = ResonanceState::new(1.05, 1.05, 1.05);
        let found = engine.find_nodes(&target);

        assert_eq!(found.len(), 2); // Should find first two nodes
    }

    #[test]
    fn test_find_nodes_by_capabilities() {
        let engine = DiscoveryEngine::default();

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);

        let caps1 = Some(vec!["storage".to_string(), "compute".to_string()]);
        let beacon1 = DiscoveryBeacon::new(resonance, 300, caps1);
        engine.receive_beacon(beacon1).unwrap();

        let caps2 = Some(vec!["storage".to_string()]);
        let beacon2 = DiscoveryBeacon::new(resonance, 300, caps2);
        engine.receive_beacon(beacon2).unwrap();

        // Find nodes with storage capability
        let found = engine.find_nodes_with_capabilities(&["storage".to_string()]);
        assert_eq!(found.len(), 2);

        // Find nodes with both storage and compute
        let found =
            engine.find_nodes_with_capabilities(&["storage".to_string(), "compute".to_string()]);
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_discovery_event() {
        let pattern = vec![
            ResonanceState::new(1.0, 1.0, 1.0),
            ResonanceState::new(2.0, 2.0, 2.0),
        ];

        let event = DiscoveryEvent::new(pattern, 300, EventType::Rendezvous);

        assert!(event.is_active());
        assert!(event.current_resonance().is_some());
    }

    #[test]
    fn test_cleanup() {
        let engine = DiscoveryEngine::new(1, 1, 0.2); // 1 second timeout

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut beacon = DiscoveryBeacon::new(resonance, 1, None);

        // Expire beacon
        beacon.timestamp -= 10;

        engine.receive_beacon(beacon).unwrap();

        // Cleanup should remove expired beacon and inactive node
        let (beacons_removed, nodes_removed) = engine.cleanup().unwrap();
        assert!(beacons_removed > 0 || nodes_removed > 0);
    }

    #[test]
    fn test_discovered_node() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let identity = NodeIdentity::new(resonance, None);
        let beacon_id = Uuid::new_v4();

        let mut node = DiscoveredNode::new(identity, beacon_id, None);

        assert!(node.is_active(300));

        // Simulate aging
        node.last_seen -= 400;
        assert!(!node.is_active(300));
    }
}
