/*!
 * Peer Management
 *
 * Tracks connected peers and their metadata.
 */

use super::PeerId;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Peer connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: PeerId,

    /// Peer address (multiaddr string)
    pub address: String,

    /// Connection timestamp
    pub connected_at: u64,

    /// Last seen timestamp
    pub last_seen: u64,

    /// Total packets sent to this peer
    pub packets_sent: u64,

    /// Total packets received from this peer
    pub packets_received: u64,

    /// Connection quality (0.0-1.0)
    pub quality: f64,
}

impl PeerInfo {
    /// Create new peer info
    pub fn new(peer_id: PeerId, address: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            peer_id,
            address,
            connected_at: now,
            last_seen: now,
            packets_sent: 0,
            packets_received: 0,
            quality: 1.0,
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Record packet sent
    pub fn record_sent(&mut self) {
        self.packets_sent += 1;
        self.update_last_seen();
    }

    /// Record packet received
    pub fn record_received(&mut self) {
        self.packets_received += 1;
        self.update_last_seen();
    }

    /// Get connection duration in seconds
    pub fn connection_duration(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now.saturating_sub(self.connected_at)
    }

    /// Get time since last activity in seconds
    pub fn idle_time(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now.saturating_sub(self.last_seen)
    }
}

/// Peer manager for tracking connections
pub struct PeerManager {
    /// Connected peers
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,

    /// Maximum idle time before considering peer stale (seconds)
    max_idle_time: u64,
}

impl std::fmt::Debug for PeerManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerManager")
            .field("peers", &"<peer map>")
            .field("max_idle_time", &self.max_idle_time)
            .finish()
    }
}

impl PeerManager {
    /// Create new peer manager
    pub fn new(max_idle_time: u64) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            max_idle_time,
        }
    }

    /// Add a peer
    pub fn add_peer(&self, peer_id: PeerId, address: String) -> Result<()> {
        let mut peers = self.peers.write().unwrap();
        let info = PeerInfo::new(peer_id, address);
        peers.insert(peer_id, info);
        Ok(())
    }

    /// Remove a peer
    pub fn remove_peer(&self, peer_id: PeerId) -> Result<()> {
        let mut peers = self.peers.write().unwrap();
        peers.remove(&peer_id);
        Ok(())
    }

    /// Get peer info
    pub fn get_peer(&self, peer_id: PeerId) -> Option<PeerInfo> {
        let peers = self.peers.read().unwrap();
        peers.get(&peer_id).cloned()
    }

    /// Get all connected peers
    pub fn get_all_peers(&self) -> Vec<PeerId> {
        let peers = self.peers.read().unwrap();
        peers.keys().copied().collect()
    }

    /// Get peer count
    pub fn peer_count(&self) -> usize {
        let peers = self.peers.read().unwrap();
        peers.len()
    }

    /// Check if peer is connected
    pub fn is_connected(&self, peer_id: PeerId) -> bool {
        let peers = self.peers.read().unwrap();
        peers.contains_key(&peer_id)
    }

    /// Record packet sent to peer
    pub fn record_sent(&self, peer_id: PeerId) {
        let mut peers = self.peers.write().unwrap();
        if let Some(peer) = peers.get_mut(&peer_id) {
            peer.record_sent();
        }
    }

    /// Record packet received from peer
    pub fn record_received(&self, peer_id: PeerId) {
        let mut peers = self.peers.write().unwrap();
        if let Some(peer) = peers.get_mut(&peer_id) {
            peer.record_received();
        }
    }

    /// Clean up stale peers
    pub fn cleanup_stale_peers(&self) -> Vec<PeerId> {
        let mut peers = self.peers.write().unwrap();
        let mut stale = Vec::new();

        peers.retain(|peer_id, info| {
            if info.idle_time() > self.max_idle_time {
                stale.push(*peer_id);
                false
            } else {
                true
            }
        });

        stale
    }

    /// Get statistics
    pub fn stats(&self) -> PeerManagerStats {
        let peers = self.peers.read().unwrap();

        let mut total_sent = 0;
        let mut total_received = 0;
        let mut total_quality = 0.0;

        for peer in peers.values() {
            total_sent += peer.packets_sent;
            total_received += peer.packets_received;
            total_quality += peer.quality;
        }

        let count = peers.len();
        let avg_quality = if count > 0 {
            total_quality / count as f64
        } else {
            0.0
        };

        PeerManagerStats {
            peer_count: count,
            total_packets_sent: total_sent,
            total_packets_received: total_received,
            average_quality: avg_quality,
        }
    }
}

impl Default for PeerManager {
    fn default() -> Self {
        Self::new(300) // 5 minutes default idle timeout
    }
}

/// Peer manager statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerManagerStats {
    /// Number of connected peers
    pub peer_count: usize,
    /// Total packets sent to all peers
    pub total_packets_sent: u64,
    /// Total packets received from all peers
    pub total_packets_received: u64,
    /// Average quality score across all peers
    pub average_quality: f64,
}

/// Trait for providing peer IDs (abstraction over libp2p)
pub trait PeerIdProvider: Send + Sync {
    /// Get peer ID from libp2p peer ID
    fn to_peer_id(&self, libp2p_peer_id: &[u8]) -> PeerId;

    /// Get libp2p peer ID from our peer ID
    fn from_peer_id(&self, peer_id: PeerId) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_info_creation() {
        let peer_id = PeerId::random();
        let info = PeerInfo::new(peer_id, "/ip4/127.0.0.1/tcp/9000".to_string());

        assert_eq!(info.peer_id, peer_id);
        assert_eq!(info.packets_sent, 0);
        assert_eq!(info.packets_received, 0);
        assert_eq!(info.quality, 1.0);
    }

    #[test]
    fn test_peer_info_record_packets() {
        let peer_id = PeerId::random();
        let mut info = PeerInfo::new(peer_id, "/ip4/127.0.0.1/tcp/9000".to_string());

        info.record_sent();
        info.record_sent();
        info.record_received();

        assert_eq!(info.packets_sent, 2);
        assert_eq!(info.packets_received, 1);
    }

    #[test]
    fn test_peer_manager_add_remove() {
        let manager = PeerManager::default();
        let peer_id = PeerId::random();

        // Add peer
        manager
            .add_peer(peer_id, "/ip4/127.0.0.1/tcp/9000".to_string())
            .unwrap();
        assert!(manager.is_connected(peer_id));
        assert_eq!(manager.peer_count(), 1);

        // Remove peer
        manager.remove_peer(peer_id).unwrap();
        assert!(!manager.is_connected(peer_id));
        assert_eq!(manager.peer_count(), 0);
    }

    #[test]
    fn test_peer_manager_get_all() {
        let manager = PeerManager::default();
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();

        manager
            .add_peer(peer1, "/ip4/127.0.0.1/tcp/9000".to_string())
            .unwrap();
        manager
            .add_peer(peer2, "/ip4/127.0.0.1/tcp/9001".to_string())
            .unwrap();

        let all_peers = manager.get_all_peers();
        assert_eq!(all_peers.len(), 2);
        assert!(all_peers.contains(&peer1));
        assert!(all_peers.contains(&peer2));
    }

    #[test]
    fn test_peer_manager_stats() {
        let manager = PeerManager::default();
        let peer_id = PeerId::random();

        manager
            .add_peer(peer_id, "/ip4/127.0.0.1/tcp/9000".to_string())
            .unwrap();

        manager.record_sent(peer_id);
        manager.record_sent(peer_id);
        manager.record_received(peer_id);

        let stats = manager.stats();
        assert_eq!(stats.peer_count, 1);
        assert_eq!(stats.total_packets_sent, 2);
        assert_eq!(stats.total_packets_received, 1);
    }
}
