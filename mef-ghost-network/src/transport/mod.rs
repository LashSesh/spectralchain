/*!
 * Network Transport Layer for Ghost Protocol
 *
 * Provides abstract transport interface for real network communication.
 * Currently supports libp2p-based P2P networking.
 *
 * # Architecture
 *
 * ```
 * Ghost Protocol
 *       ↓
 * Broadcasting/Discovery
 *       ↓
 * Transport Layer (this module)
 *       ↓
 * TCP/UDP/QUIC (libp2p)
 * ```
 *
 * # Usage
 *
 * ```rust,no_run
 * use mef_ghost_network::transport::{Transport, TransportConfig, Libp2pTransport};
 * use mef_ghost_network::packet::GhostPacket;
 *
 * # async fn example() -> anyhow::Result<()> {
 * // Create transport
 * let config = TransportConfig::default();
 * let mut transport = Libp2pTransport::new(config).await?;
 *
 * // Listen for connections
 * transport.listen("/ip4/0.0.0.0/tcp/9000".to_string()).await?;
 *
 * // Dial a peer
 * let peer_id = transport.dial("/ip4/127.0.0.1/tcp/9001".to_string()).await?;
 *
 * // Send packet
 * let packet = GhostPacket::default();
 * transport.send(peer_id, packet).await?;
 *
 * // Receive packet
 * let (from_peer, received_packet) = transport.receive().await?;
 * # Ok(())
 * # }
 * ```
 */

use crate::packet::GhostPacket;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod codec;
pub mod config;
pub mod libp2p_transport;
pub mod peer;

pub use codec::PacketCodec;
pub use config::TransportConfig;
pub use libp2p_transport::Libp2pTransport;
pub use peer::{PeerIdProvider, PeerInfo, PeerManager};

/// Peer identifier (abstraction over libp2p PeerId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    /// Create new random peer ID
    pub fn random() -> Self {
        use rand::Rng;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);
        Self(bytes)
    }

    /// Create from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..8]))
    }
}

impl Default for PeerId {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

/// Transport trait for network communication
///
/// Provides abstract interface for sending/receiving Ghost packets
/// over a real network. Implementations handle the underlying
/// networking stack (libp2p, TCP, UDP, QUIC, etc.)
#[async_trait]
pub trait Transport: Send + Sync + std::fmt::Debug {
    /// Listen on a network address
    ///
    /// # Arguments
    /// * `addr` - Multiaddr string (e.g., "/ip4/0.0.0.0/tcp/9000")
    ///
    /// # Returns
    /// * `Ok(())` if listening started successfully
    /// * `Err` if address is invalid or already in use
    async fn listen(&mut self, addr: String) -> Result<()>;

    /// Dial a peer at the given address
    ///
    /// # Arguments
    /// * `addr` - Multiaddr string (e.g., "/ip4/127.0.0.1/tcp/9001")
    ///
    /// # Returns
    /// * `Ok(PeerId)` with the connected peer's ID
    /// * `Err` if connection failed
    async fn dial(&mut self, addr: String) -> Result<PeerId>;

    /// Send packet to a specific peer
    ///
    /// # Arguments
    /// * `peer` - Destination peer ID
    /// * `packet` - Ghost packet to send
    ///
    /// # Returns
    /// * `Ok(())` if packet was sent
    /// * `Err` if peer is not connected or send failed
    async fn send(&mut self, peer: PeerId, packet: GhostPacket) -> Result<()>;

    /// Broadcast packet to all connected peers
    ///
    /// # Arguments
    /// * `packet` - Ghost packet to broadcast
    ///
    /// # Returns
    /// * `Ok(())` if packet was broadcast
    /// * `Err` if broadcast failed
    async fn broadcast(&mut self, packet: GhostPacket) -> Result<()>;

    /// Receive next packet from the network
    ///
    /// This is a blocking call that waits for the next packet.
    ///
    /// # Returns
    /// * `Ok((PeerId, GhostPacket))` with sender and packet
    /// * `Err` if receive failed or transport closed
    async fn receive(&mut self) -> Result<(PeerId, GhostPacket)>;

    /// Get list of currently connected peers
    ///
    /// # Returns
    /// * Vector of connected peer IDs
    fn peers(&self) -> Vec<PeerId>;

    /// Get local peer ID
    ///
    /// # Returns
    /// * This node's peer ID
    fn local_peer_id(&self) -> PeerId;

    /// Check if connected to a specific peer
    ///
    /// # Arguments
    /// * `peer` - Peer ID to check
    ///
    /// # Returns
    /// * `true` if connected, `false` otherwise
    fn is_connected(&self, peer: PeerId) -> bool {
        self.peers().contains(&peer)
    }

    /// Get number of connected peers
    ///
    /// # Returns
    /// * Number of active peer connections
    fn peer_count(&self) -> usize {
        self.peers().len()
    }

    /// Shutdown transport
    ///
    /// Closes all connections and stops listening.
    async fn shutdown(&mut self) -> Result<()>;
}

/// Transport statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransportStats {
    /// Total packets sent
    pub packets_sent: u64,

    /// Total packets received
    pub packets_received: u64,

    /// Total bytes sent
    pub bytes_sent: u64,

    /// Total bytes received
    pub bytes_received: u64,

    /// Number of successful dials
    pub dials_successful: u64,

    /// Number of failed dials
    pub dials_failed: u64,

    /// Number of connections accepted
    pub connections_accepted: u64,

    /// Number of connections closed
    pub connections_closed: u64,
}

impl TransportStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record packet sent
    pub fn record_sent(&mut self, packet_size: usize) {
        self.packets_sent += 1;
        self.bytes_sent += packet_size as u64;
    }

    /// Record packet received
    pub fn record_received(&mut self, packet_size: usize) {
        self.packets_received += 1;
        self.bytes_received += packet_size as u64;
    }

    /// Record successful dial
    pub fn record_dial_success(&mut self) {
        self.dials_successful += 1;
    }

    /// Record failed dial
    pub fn record_dial_failure(&mut self) {
        self.dials_failed += 1;
    }

    /// Record connection accepted
    pub fn record_connection_accepted(&mut self) {
        self.connections_accepted += 1;
    }

    /// Record connection closed
    pub fn record_connection_closed(&mut self) {
        self.connections_closed += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_id_creation() {
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();

        // Random IDs should be different
        assert_ne!(peer1, peer2);
    }

    #[test]
    fn test_peer_id_from_bytes() {
        let bytes = [42u8; 32];
        let peer = PeerId::from_bytes(bytes);
        assert_eq!(peer.as_bytes(), &bytes);
    }

    #[test]
    fn test_transport_stats() {
        let mut stats = TransportStats::new();

        stats.record_sent(100);
        stats.record_sent(200);
        stats.record_received(150);

        assert_eq!(stats.packets_sent, 2);
        assert_eq!(stats.packets_received, 1);
        assert_eq!(stats.bytes_sent, 300);
        assert_eq!(stats.bytes_received, 150);
    }
}
