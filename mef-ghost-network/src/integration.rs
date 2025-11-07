/*!
 * Ghost Network Integration - High-Level API
 *
 * Provides a complete Ghost Network node that integrates:
 * - Transport layer (libp2p)
 * - Broadcasting (resonance-based routing)
 * - Discovery (temporary beacons)
 * - Ghost Protocol (6-step flow)
 */

use crate::broadcasting::BroadcastEngine;
use crate::discovery::DiscoveryEngine;
use crate::packet::{GhostPacket, GhostTransaction, NodeIdentity, ResonanceState};
use crate::protocol::{GhostProtocol, MaskingParams, ProtocolConfig};
use crate::transport::{Libp2pTransport, PeerId, Transport, TransportConfig};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// Wrapper to enable passing Arc<Mutex<Libp2pTransport>> as Arc<Mutex<dyn Transport>>
///
/// This wrapper is necessary because Rust cannot unsize through Mutex<T> to Mutex<dyn Trait>.
/// We wrap the Arc<Mutex<Libp2pTransport>> and implement Transport by delegating all methods.
struct TransportWrapper {
    inner: Arc<Mutex<Libp2pTransport>>,
}

impl TransportWrapper {
    fn new(transport: Arc<Mutex<Libp2pTransport>>) -> Self {
        Self { inner: transport }
    }
}

#[async_trait]
impl Transport for TransportWrapper {
    async fn listen(&mut self, addr: String) -> Result<()> {
        self.inner.lock().await.listen(addr).await
    }

    async fn dial(&mut self, addr: String) -> Result<PeerId> {
        self.inner.lock().await.dial(addr).await
    }

    async fn send(&mut self, peer: PeerId, packet: GhostPacket) -> Result<()> {
        self.inner.lock().await.send(peer, packet).await
    }

    async fn broadcast(&mut self, packet: GhostPacket) -> Result<()> {
        self.inner.lock().await.broadcast(packet).await
    }

    async fn receive(&mut self) -> Result<(PeerId, GhostPacket)> {
        self.inner.lock().await.receive().await
    }

    fn peers(&self) -> Vec<PeerId> {
        // For non-async methods, we need to use try_lock or block_on
        // Since we're in a sync context, we'll use try_lock with fallback
        self.inner
            .try_lock()
            .map(|guard| guard.peers())
            .unwrap_or_default()
    }

    fn local_peer_id(&self) -> PeerId {
        self.inner
            .try_lock()
            .map(|guard| guard.local_peer_id())
            .unwrap_or_default()
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.inner.lock().await.shutdown().await
    }
}

/// Complete Ghost Network node
///
/// Combines transport, broadcasting, discovery, and protocol into a single interface.
#[derive(Debug)]
pub struct GhostNetworkNode {
    /// Node identity (resonance state)
    identity: NodeIdentity,

    /// Network transport
    transport: Arc<Mutex<Libp2pTransport>>,

    /// Broadcast engine
    broadcast: Arc<BroadcastEngine>,

    /// Discovery engine
    discovery: Arc<DiscoveryEngine>,

    /// Ghost protocol
    protocol: Arc<GhostProtocol>,

    /// Main broadcast channel ID
    #[allow(dead_code)]
    main_channel_id: uuid::Uuid,
}

impl GhostNetworkNode {
    /// Create a new Ghost Network node
    ///
    /// # Arguments
    /// * `resonance` - Node's resonance state (identity)
    /// * `transport_config` - Transport configuration
    /// * `protocol_config` - Protocol configuration
    ///
    /// # Returns
    /// * Fully initialized Ghost Network node ready for communication
    pub async fn new(
        resonance: ResonanceState,
        transport_config: TransportConfig,
        protocol_config: ProtocolConfig,
    ) -> Result<Self> {
        // Create node identity
        let identity = NodeIdentity::new(resonance, None);

        info!(
            event = "node_initializing",
            resonance = ?(resonance.psi, resonance.rho, resonance.omega),
            "Initializing Ghost Network node"
        );

        // Create libp2p transport
        let transport = Arc::new(Mutex::new(
            Libp2pTransport::new(transport_config)
                .await
                .context("Failed to create libp2p transport")?,
        ));

        // Create trait object wrapper for broadcast and discovery
        // We wrap the Arc<Mutex<Libp2pTransport>> in TransportWrapper which implements Transport
        // Then wrap that in Arc<Mutex<dyn Transport>> for sharing between engines
        let transport_trait: Arc<Mutex<dyn Transport>> =
            Arc::new(Mutex::new(TransportWrapper::new(transport.clone())));

        // Create broadcast engine with transport
        let broadcast = Arc::new(BroadcastEngine::with_transport(
            1000, // max_buffer_size
            10.0, // decoy_rate
            60,   // cleanup_interval
            transport_trait.clone(),
        ));

        // Create discovery engine with transport
        let discovery = Arc::new(DiscoveryEngine::with_transport(
            300, // node_timeout
            120, // beacon_ttl
            0.2, // discovery_epsilon
            transport_trait,
        ));

        // Create protocol
        let protocol = Arc::new(GhostProtocol::new(protocol_config));

        // Create main broadcast channel matching node's resonance
        let main_channel_id = broadcast.create_channel(
            resonance, 0.1,  // epsilon
            3600, // 1 hour TTL
        )?;

        info!(
            event = "node_initialized",
            peer_id = %transport.lock().await.local_peer_id(),
            channel_id = %main_channel_id,
            "Ghost Network node ready"
        );

        Ok(Self {
            identity,
            transport,
            broadcast,
            discovery,
            protocol,
            main_channel_id,
        })
    }

    /// Get node's peer ID
    pub async fn peer_id(&self) -> PeerId {
        self.transport.lock().await.local_peer_id()
    }

    /// Get node's resonance state
    pub fn resonance(&self) -> ResonanceState {
        self.identity.resonance
    }

    /// Listen on an address
    pub async fn listen(&mut self, addr: String) -> Result<()> {
        info!(event = "node_listening", addr = %addr, "Starting to listen");
        self.transport.lock().await.listen(addr).await
    }

    /// Connect to a peer
    pub async fn dial(&mut self, addr: String) -> Result<PeerId> {
        info!(event = "node_dialing", addr = %addr, "Connecting to peer");
        self.transport.lock().await.dial(addr).await
    }

    /// Announce presence to the network
    ///
    /// Broadcasts a discovery beacon so other nodes can find this node.
    pub async fn announce(&self, capabilities: Option<Vec<String>>) -> Result<uuid::Uuid> {
        info!(
            event = "node_announcing",
            capabilities = ?capabilities,
            "Announcing presence to network"
        );

        self.discovery.announce(&self.identity, capabilities).await
    }

    /// Poll for discovery beacons from network
    ///
    /// Receives and processes discovery beacons from other nodes.
    /// Call this periodically to discover new nodes.
    pub async fn poll_discovery(&self) -> Result<usize> {
        self.discovery.poll_beacons().await
    }

    /// Find nodes by resonance
    pub fn find_nodes(
        &self,
        target_resonance: &ResonanceState,
    ) -> Vec<crate::discovery::DiscoveredNode> {
        self.discovery.find_nodes(target_resonance)
    }

    /// Get all discovered nodes
    pub fn discovered_nodes(&self) -> Vec<crate::discovery::DiscoveredNode> {
        self.discovery.get_active_nodes()
    }

    /// Send a transaction to a target resonance
    ///
    /// Implements the full 6-step Ghost Protocol:
    /// 1. Create transaction with ZK proof
    /// 2. Mask with M_{θ,σ}
    /// 3. Embed in steganographic carrier
    /// 4. Broadcast to network
    /// 5. Resonance-based routing
    /// 6. Ready for ledger commit
    pub async fn send_transaction(
        &self,
        target_resonance: ResonanceState,
        action: Vec<u8>,
    ) -> Result<Vec<uuid::Uuid>> {
        info!(
            event = "transaction_sending",
            target_resonance = ?(target_resonance.psi, target_resonance.rho, target_resonance.omega),
            action_size = action.len(),
            "Sending transaction via Ghost Protocol"
        );

        // Step 1: Create transaction with ZK proof
        let transaction =
            self.protocol
                .create_transaction(self.identity.resonance, target_resonance, action)?;

        debug!(
            event = "transaction_created",
            tx_id = %transaction.id,
            "Transaction created"
        );

        // Step 2: Derive masking parameters from resonance states
        // Both sender and receiver can compute same params (addressless key agreement)
        let mut masking_params =
            MaskingParams::from_resonance(&self.identity.resonance, &target_resonance);

        // Add forward secrecy (R-03-002)
        if self.protocol.get_metrics().packets_accepted > 0 {
            let ephemeral_key = MaskingParams::generate_ephemeral_key();
            masking_params = masking_params.with_ephemeral_key(ephemeral_key);
        }

        // Step 2: Mask transaction
        let masked = self
            .protocol
            .mask_transaction(&transaction, &masking_params)?;

        debug!(event = "transaction_masked", "Transaction masked");

        // Step 3: Embed in steganographic carrier
        let carrier = self
            .protocol
            .embed_transaction(&masked, crate::packet::CarrierType::Raw)?;

        debug!(event = "transaction_embedded", "Transaction embedded");

        // Step 4: Create Ghost packet
        let packet = self.protocol.create_packet(
            &transaction,
            masked,
            carrier,
            crate::packet::CarrierType::Raw,
            &masking_params,
        )?;

        debug!(
            event = "packet_created",
            packet_id = %packet.id,
            "Ghost packet created"
        );

        // Step 5: Broadcast via resonance-based routing
        let matching_channels = self.broadcast.broadcast(packet).await?;

        info!(
            event = "transaction_broadcast",
            tx_id = %transaction.id,
            matching_channels = matching_channels.len(),
            "Transaction broadcast complete"
        );

        Ok(matching_channels)
    }

    /// Receive transactions matching node's resonance
    ///
    /// Implements Ghost Protocol reception (Step 5):
    /// - Checks resonance match R_ε(ψ_node, ψ_pkt)
    /// - Extracts from carrier
    /// - Unmasks with M⁻¹_{θ,σ}
    /// - Verifies ZK proof
    /// - Returns validated transactions
    pub async fn receive_transactions(&self) -> Result<Vec<GhostTransaction>> {
        // Receive packets via resonance-based routing
        let packets = self.broadcast.receive(&self.identity).await?;

        debug!(
            event = "packets_received",
            count = packets.len(),
            "Received packets from network"
        );

        let mut transactions = Vec::new();

        // Process each packet through Ghost Protocol
        for packet in packets {
            match self
                .protocol
                .receive_packet(&packet, &self.identity.resonance)
            {
                Ok(Some(tx)) => {
                    info!(
                        event = "transaction_received",
                        tx_id = %tx.id,
                        "Transaction validated and accepted"
                    );
                    transactions.push(tx);
                }
                Ok(None) => {
                    // Resonance mismatch - packet ignored (normal)
                    debug!(
                        event = "packet_ignored",
                        packet_id = %packet.id,
                        "Packet resonance mismatch"
                    );
                }
                Err(e) => {
                    // Validation failed - security event
                    debug!(
                        event = "packet_rejected",
                        packet_id = %packet.id,
                        error = %e,
                        "Packet validation failed"
                    );
                }
            }
        }

        Ok(transactions)
    }

    /// Generate decoy traffic for privacy
    ///
    /// Creates fake packets to maintain constant background noise.
    pub async fn generate_decoy_traffic(&self, count: usize) -> Result<()> {
        self.broadcast.generate_decoy_traffic(count).await
    }

    /// Get broadcast statistics
    pub fn broadcast_stats(&self) -> crate::broadcasting::BroadcastStats {
        self.broadcast.get_stats()
    }

    /// Get discovery statistics
    pub fn discovery_stats(&self) -> crate::discovery::DiscoveryStats {
        self.discovery.get_stats()
    }

    /// Get protocol metrics
    pub fn protocol_metrics(&self) -> crate::protocol::PacketMetrics {
        self.protocol.get_metrics()
    }

    /// Shutdown the node gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        info!(
            event = "node_shutting_down",
            "Shutting down Ghost Network node"
        );
        self.transport.lock().await.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_creation() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let transport_config = TransportConfig::local();
        let protocol_config = ProtocolConfig::default();

        let node = GhostNetworkNode::new(resonance, transport_config, protocol_config)
            .await
            .unwrap();

        assert_eq!(node.resonance().psi, 1.0);
    }

    #[tokio::test]
    async fn test_node_announce_and_discover() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let transport_config = TransportConfig::local();
        let protocol_config = ProtocolConfig::default();

        let node = GhostNetworkNode::new(resonance, transport_config, protocol_config)
            .await
            .unwrap();

        // Announce presence
        let beacon_id = node.announce(Some(vec!["test".to_string()])).await.unwrap();

        assert!(beacon_id.to_string().len() > 0);

        let stats = node.discovery_stats();
        assert_eq!(stats.beacons_sent, 1);
    }
}
