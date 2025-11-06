/*!
 * Addressless Broadcasting via Resonance
 *
 * Implements broadcast mechanism where packets are routed based on
 * resonance state rather than network addresses.
 *
 * Key Features:
 * - No fixed addresses or routing tables
 * - Resonance-based packet propagation
 * - Decoy traffic for privacy
 * - Automatic channel dissolution
 */

use crate::packet::{GhostPacket, NodeIdentity, ResonanceState};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Broadcast channel - ephemeral, resonance-based
#[derive(Debug, Clone)]
pub struct BroadcastChannel {
    /// Channel ID (ephemeral)
    pub id: uuid::Uuid,

    /// Center resonance state for this channel
    pub resonance: ResonanceState,

    /// Resonance window (epsilon)
    pub epsilon: f64,

    /// Creation timestamp
    pub created_at: u64,

    /// Time-to-live in seconds
    pub ttl_seconds: u64,

    /// Whether this is a decoy channel
    pub is_decoy: bool,
}

impl BroadcastChannel {
    /// Create new broadcast channel
    pub fn new(resonance: ResonanceState, epsilon: f64, ttl_seconds: u64) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            resonance,
            epsilon,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds,
            is_decoy: false,
        }
    }

    /// Create decoy channel for privacy
    pub fn new_decoy(resonance: ResonanceState) -> Self {
        let mut channel = Self::new(resonance, 0.1, 300); // 5 minute TTL
        channel.is_decoy = true;
        channel
    }

    /// Check if channel is still alive
    pub fn is_alive(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now < self.created_at + self.ttl_seconds
    }

    /// Check if a packet matches this channel's resonance
    pub fn matches_packet(&self, packet: &GhostPacket) -> bool {
        self.resonance
            .is_resonant_with(&packet.resonance, self.epsilon)
    }

    /// Check if a node matches this channel's resonance
    pub fn matches_node(&self, node: &NodeIdentity) -> bool {
        self.resonance
            .is_resonant_with(&node.resonance, self.epsilon)
    }
}

/// Broadcast statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BroadcastStats {
    /// Total packets broadcast
    pub packets_sent: usize,

    /// Total packets received
    pub packets_received: usize,

    /// Total decoy packets generated
    pub decoy_packets: usize,

    /// Total channels created
    pub channels_created: usize,

    /// Total channels dissolved
    pub channels_dissolved: usize,

    /// Average resonance match rate
    pub avg_match_rate: f64,
}

/// Addressless broadcast engine
pub struct BroadcastEngine {
    /// Active broadcast channels
    channels: Arc<RwLock<HashMap<uuid::Uuid, BroadcastChannel>>>,

    /// Packet buffer for each channel
    buffers: Arc<RwLock<HashMap<uuid::Uuid, VecDeque<GhostPacket>>>>,

    /// Statistics
    stats: Arc<RwLock<BroadcastStats>>,

    /// Maximum packets per channel buffer
    max_buffer_size: usize,

    /// Decoy traffic generation rate (packets per second)
    decoy_rate: f64,

    /// Automatic channel cleanup interval (seconds)
    cleanup_interval: u64,
}

impl BroadcastEngine {
    /// Create new broadcast engine
    pub fn new(max_buffer_size: usize, decoy_rate: f64, cleanup_interval: u64) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            buffers: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(BroadcastStats::default())),
            max_buffer_size,
            decoy_rate,
            cleanup_interval,
        }
    }

    /// Create with default settings
    pub fn default() -> Self {
        Self::new(
            1000,  // Max 1000 packets per channel
            10.0,  // 10 decoy packets per second
            60,    // Cleanup every 60 seconds
        )
    }

    /// Create new broadcast channel
    pub fn create_channel(
        &self,
        resonance: ResonanceState,
        epsilon: f64,
        ttl_seconds: u64,
    ) -> Result<uuid::Uuid> {
        let channel = BroadcastChannel::new(resonance, epsilon, ttl_seconds);
        let channel_id = channel.id;

        let mut channels = self.channels.write().unwrap();
        channels.insert(channel_id, channel);

        let mut buffers = self.buffers.write().unwrap();
        buffers.insert(channel_id, VecDeque::new());

        let mut stats = self.stats.write().unwrap();
        stats.channels_created += 1;

        Ok(channel_id)
    }

    /// Create decoy channel for privacy
    pub fn create_decoy_channel(&self, resonance: ResonanceState) -> Result<uuid::Uuid> {
        let channel = BroadcastChannel::new_decoy(resonance);
        let channel_id = channel.id;

        let mut channels = self.channels.write().unwrap();
        channels.insert(channel_id, channel);

        let mut buffers = self.buffers.write().unwrap();
        buffers.insert(channel_id, VecDeque::new());

        Ok(channel_id)
    }

    /// Broadcast packet to resonant channels
    ///
    /// Packet is routed to all channels with matching resonance.
    /// No addresses needed - purely resonance-based routing.
    pub fn broadcast(&self, packet: GhostPacket) -> Result<Vec<uuid::Uuid>> {
        let channels = self.channels.read().unwrap();
        let mut matching_channels = Vec::new();

        // Find all resonant channels
        for (channel_id, channel) in channels.iter() {
            if channel.is_alive() && channel.matches_packet(&packet) {
                matching_channels.push(*channel_id);
            }
        }

        // Add packet to matching channel buffers
        let mut buffers = self.buffers.write().unwrap();
        for channel_id in matching_channels.iter() {
            if let Some(buffer) = buffers.get_mut(channel_id) {
                // Enforce buffer size limit
                if buffer.len() >= self.max_buffer_size {
                    buffer.pop_front(); // Drop oldest packet
                }
                buffer.push_back(packet.clone());
            }
        }

        // Update statistics
        let mut stats = self.stats.write().unwrap();
        stats.packets_sent += 1;

        Ok(matching_channels)
    }

    /// Receive packets for a node based on its resonance
    ///
    /// Node receives all packets from channels matching its resonance state.
    pub fn receive(&self, node: &NodeIdentity) -> Result<Vec<GhostPacket>> {
        let channels = self.channels.read().unwrap();
        let mut received_packets = Vec::new();

        // Find matching channels
        let matching_channel_ids: Vec<uuid::Uuid> = channels
            .iter()
            .filter(|(_, channel)| channel.is_alive() && channel.matches_node(node))
            .map(|(id, _)| *id)
            .collect();

        // Collect packets from matching channels
        let mut buffers = self.buffers.write().unwrap();
        for channel_id in matching_channel_ids {
            if let Some(buffer) = buffers.get_mut(&channel_id) {
                // Take all packets from buffer
                while let Some(packet) = buffer.pop_front() {
                    // Double-check resonance match with node
                    if packet.matches_resonance(&node.resonance, 0.1) {
                        received_packets.push(packet);
                    }
                }
            }
        }

        // Update statistics
        let mut stats = self.stats.write().unwrap();
        stats.packets_received += received_packets.len();

        Ok(received_packets)
    }

    /// Generate decoy traffic for privacy
    ///
    /// Creates fake packets to maintain constant background noise,
    /// making traffic analysis more difficult.
    pub fn generate_decoy_traffic(&self, count: usize) -> Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Random resonance state
            let resonance = ResonanceState::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );

            // Create decoy channel
            self.create_decoy_channel(resonance)?;

            // Create decoy packet
            let packet = GhostPacket::new(
                resonance,
                vec![0u8; rng.gen_range(100..1000)], // Random size
                vec![0u8; rng.gen_range(100..1000)],
                crate::packet::CarrierType::Raw,
                None,
            );

            // Broadcast decoy packet
            self.broadcast(packet)?;
        }

        let mut stats = self.stats.write().unwrap();
        stats.decoy_packets += count;

        Ok(())
    }

    /// Cleanup expired channels (automatic dissolution)
    ///
    /// Removes channels that have exceeded their TTL.
    /// This implements "automatic channel dissolve" for privacy.
    pub fn cleanup_expired_channels(&self) -> Result<usize> {
        let mut channels = self.channels.write().unwrap();
        let mut buffers = self.buffers.write().unwrap();

        let mut dissolved_count = 0;

        // Find expired channels
        let expired_ids: Vec<uuid::Uuid> = channels
            .iter()
            .filter(|(_, channel)| !channel.is_alive())
            .map(|(id, _)| *id)
            .collect();

        // Remove expired channels and their buffers
        for id in expired_ids {
            channels.remove(&id);
            buffers.remove(&id);
            dissolved_count += 1;
        }

        // Update statistics
        let mut stats = self.stats.write().unwrap();
        stats.channels_dissolved += dissolved_count;

        Ok(dissolved_count)
    }

    /// Get all active channels
    pub fn get_active_channels(&self) -> Vec<BroadcastChannel> {
        let channels = self.channels.read().unwrap();
        channels
            .values()
            .filter(|c| c.is_alive())
            .cloned()
            .collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> BroadcastStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        *stats = BroadcastStats::default();
    }

    /// Get total active channels count
    pub fn active_channel_count(&self) -> usize {
        let channels = self.channels.read().unwrap();
        channels.values().filter(|c| c.is_alive()).count()
    }

    /// Get buffer size for a channel
    pub fn get_buffer_size(&self, channel_id: uuid::Uuid) -> Option<usize> {
        let buffers = self.buffers.read().unwrap();
        buffers.get(&channel_id).map(|b| b.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::CarrierType;

    #[test]
    fn test_channel_creation() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let channel = BroadcastChannel::new(resonance, 0.1, 300);

        assert_eq!(channel.resonance.psi, 1.0);
        assert_eq!(channel.epsilon, 0.1);
        assert!(channel.is_alive());
        assert!(!channel.is_decoy);
    }

    #[test]
    fn test_decoy_channel() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let channel = BroadcastChannel::new_decoy(resonance);

        assert!(channel.is_decoy);
    }

    #[test]
    fn test_channel_expiration() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut channel = BroadcastChannel::new(resonance, 0.1, 1); // 1 second TTL

        assert!(channel.is_alive());

        // Simulate time passing
        channel.created_at -= 2; // Subtract 2 seconds
        assert!(!channel.is_alive());
    }

    #[test]
    fn test_broadcast_engine() {
        let engine = BroadcastEngine::default();

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let channel_id = engine.create_channel(resonance, 0.1, 300).unwrap();

        assert_eq!(engine.active_channel_count(), 1);
        assert_eq!(engine.get_buffer_size(channel_id), Some(0));
    }

    #[test]
    fn test_broadcast_and_receive() {
        let engine = BroadcastEngine::default();

        // Create channel
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        engine.create_channel(resonance, 0.1, 300).unwrap();

        // Create and broadcast packet
        let packet = GhostPacket::new(
            resonance,
            b"test payload".to_vec(),
            b"test carrier".to_vec(),
            CarrierType::Raw,
            None,
        );

        let matching_channels = engine.broadcast(packet.clone()).unwrap();
        assert_eq!(matching_channels.len(), 1);

        // Create node with similar resonance
        let node = NodeIdentity::new(
            ResonanceState::new(1.05, 1.05, 1.05),
            None,
        );

        // Receive packets
        let received = engine.receive(&node).unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].id, packet.id);
    }

    #[test]
    fn test_non_resonant_packet_ignored() {
        let engine = BroadcastEngine::default();

        // Create channel with specific resonance
        let channel_resonance = ResonanceState::new(1.0, 1.0, 1.0);
        engine.create_channel(channel_resonance, 0.1, 300).unwrap();

        // Broadcast packet with very different resonance
        let packet = GhostPacket::new(
            ResonanceState::new(10.0, 10.0, 10.0),
            b"test".to_vec(),
            b"test".to_vec(),
            CarrierType::Raw,
            None,
        );

        let matching_channels = engine.broadcast(packet).unwrap();
        assert_eq!(matching_channels.len(), 0); // No matching channels
    }

    #[test]
    fn test_decoy_traffic() {
        let engine = BroadcastEngine::default();

        engine.generate_decoy_traffic(5).unwrap();

        let stats = engine.get_stats();
        assert_eq!(stats.decoy_packets, 5);
    }

    #[test]
    fn test_channel_cleanup() {
        let engine = BroadcastEngine::default();

        // Create channel with 1-second TTL
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let channel_id = engine.create_channel(resonance, 0.1, 0).unwrap();

        // Manually expire the channel
        {
            let mut channels = engine.channels.write().unwrap();
            if let Some(channel) = channels.get_mut(&channel_id) {
                channel.created_at -= 10; // Subtract 10 seconds
            }
        }

        // Cleanup
        let dissolved = engine.cleanup_expired_channels().unwrap();
        assert_eq!(dissolved, 1);
        assert_eq!(engine.active_channel_count(), 0);
    }

    #[test]
    fn test_buffer_overflow() {
        let engine = BroadcastEngine::new(10, 0.0, 60); // Max 10 packets

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let channel_id = engine.create_channel(resonance, 0.1, 300).unwrap();

        // Broadcast 15 packets (exceeds buffer)
        for _ in 0..15 {
            let packet = GhostPacket::new(
                resonance,
                b"test".to_vec(),
                b"test".to_vec(),
                CarrierType::Raw,
                None,
            );
            engine.broadcast(packet).unwrap();
        }

        // Buffer should be capped at 10
        assert_eq!(engine.get_buffer_size(channel_id), Some(10));
    }

    #[test]
    fn test_statistics() {
        let engine = BroadcastEngine::default();

        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        engine.create_channel(resonance, 0.1, 300).unwrap();

        // Broadcast packet
        let packet = GhostPacket::new(
            resonance,
            b"test".to_vec(),
            b"test".to_vec(),
            CarrierType::Raw,
            None,
        );
        engine.broadcast(packet).unwrap();

        let stats = engine.get_stats();
        assert_eq!(stats.packets_sent, 1);
        assert_eq!(stats.channels_created, 1);
    }
}
