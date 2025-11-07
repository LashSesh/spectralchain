/*!
 * MEF Ghost Network - Addressless Networking Protocol
 *
 * Implementation of the Ghost Networking Protocol from the Blueprint
 * "Quantenresonante Spektralfeld-Blockchain" (Seite 4).
 *
 * # Architecture
 *
 * The Ghost Network enables addressless communication based on resonance states:
 * - No fixed IP addresses or routing tables
 * - Packets routed via resonance matching (ψ, ρ, ω)
 * - Temporary discovery via resonance events
 * - Privacy-first design with decoy traffic
 * - Automatic channel dissolution
 *
 * # Protocol Flow (6 Steps)
 *
 * 1. **Create Transaction**: Node creates proof-transaction with action, ZK proof, and resonance state
 * 2. **Masking**: Apply M_{θ,σ} operator to mask transaction
 * 3. **Steganography**: Apply T operator to embed in carrier
 * 4. **Broadcast**: Send packet to field with resonance state
 * 5. **Reception**: Nodes check resonance R_ε(ψ_node, ψ_pkt), extract and verify
 * 6. **Commit**: Verified transactions commit to ledger
 *
 * # Modules
 *
 * - `packet`: Ghost packet structures and resonance states
 * - `protocol`: Core protocol flow implementation
 * - `broadcasting`: Addressless broadcasting engine
 * - `discovery`: Node discovery via temporary resonance events
 *
 * # Integration with Infinity Ledger
 *
 * The Ghost Network integrates seamlessly with the Infinity Ledger:
 * - Gabriel Cells provide resonance states (ψ, ρ, ω)
 * - MEF-Ledger stores proof-carrying commits
 * - TIC (Temporal Information Crystals) for temporal consistency
 * - HDAG for fork management
 *
 * # Example
 *
 * ```rust
 * use mef_ghost_network::{
 *     GhostProtocol, ProtocolConfig,
 *     BroadcastEngine, DiscoveryEngine,
 *     ResonanceState, NodeIdentity,
 * };
 *
 * // Create protocol
 * let protocol = GhostProtocol::default();
 *
 * // Create transaction
 * let sender = ResonanceState::new(1.0, 1.0, 1.0);
 * let target = ResonanceState::new(2.0, 2.0, 2.0);
 * let tx = protocol.create_transaction(
 *     sender,
 *     target,
 *     b"action data".to_vec(),
 * ).unwrap();
 *
 * // Broadcast
 * let broadcast = BroadcastEngine::default();
 * let resonance = ResonanceState::new(1.0, 1.0, 1.0);
 * broadcast.create_channel(resonance, 0.1, 300).unwrap();
 * ```
 *
 * # Security & Privacy
 *
 * - **Addressless Routing**: No fixed addresses or identities
 * - **Resonance-Based Discovery**: Nodes found via temporary resonance patterns
 * - **Decoy Traffic**: Constant background noise prevents traffic analysis
 * - **Automatic Dissolution**: Channels disappear after use
 * - **Zero-Knowledge Proofs**: Verify without revealing
 * - **Steganography**: Hide payloads in innocuous carriers
 */

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

// Core modules
pub mod broadcasting;
pub mod discovery;
pub mod packet;
pub mod protocol;

// Network transport layer (Phase 1-2 implemented)
pub mod transport;

// High-level integration API (Phase 3)
pub mod integration;

// Re-exports for convenience
pub use broadcasting::{BroadcastChannel, BroadcastEngine, BroadcastStats};
pub use discovery::{
    DiscoveredNode, DiscoveryBeacon, DiscoveryEngine, DiscoveryEvent, DiscoveryStats, EventType,
};
pub use integration::GhostNetworkNode;
pub use packet::{CarrierType, GhostPacket, GhostTransaction, NodeIdentity, ResonanceState};
pub use protocol::{GhostProtocol, MaskingParams, ProtocolConfig};
pub use transport::{
    Libp2pTransport, PacketCodec, PeerId, PeerInfo, PeerManager, Transport, TransportConfig,
};

use anyhow::Result;
use std::sync::Arc;

/// Ghost Network - High-level interface combining all components
///
/// This provides a unified interface for the Ghost Networking Protocol,
/// combining protocol flow, broadcasting, and discovery.
pub struct GhostNetwork {
    /// Protocol implementation
    pub protocol: Arc<GhostProtocol>,

    /// Broadcasting engine
    pub broadcast: Arc<BroadcastEngine>,

    /// Discovery engine
    pub discovery: Arc<DiscoveryEngine>,

    /// Node identity
    pub identity: Arc<std::sync::RwLock<NodeIdentity>>,
}

impl std::fmt::Debug for GhostNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GhostNetwork")
            .field("protocol", &"GhostProtocol")
            .field("broadcast", &"BroadcastEngine")
            .field("discovery", &"DiscoveryEngine")
            .field("identity", &"NodeIdentity")
            .finish()
    }
}

impl GhostNetwork {
    /// Create new ghost network with custom configuration
    pub fn new(protocol_config: ProtocolConfig, identity: NodeIdentity) -> Self {
        Self {
            protocol: Arc::new(GhostProtocol::new(protocol_config)),
            broadcast: Arc::new(BroadcastEngine::default()),
            discovery: Arc::new(DiscoveryEngine::default()),
            identity: Arc::new(std::sync::RwLock::new(identity)),
        }
    }

    /// Create with default configuration
    pub fn default_with_identity(identity: NodeIdentity) -> Self {
        Self::new(ProtocolConfig::default(), identity)
    }

    /// Create with random identity
    pub fn with_random_identity() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let resonance = ResonanceState::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        let identity = NodeIdentity::new(resonance, None);
        Self::default_with_identity(identity)
    }

    /// Announce presence to the network
    pub async fn announce(&self, capabilities: Option<Vec<String>>) -> Result<uuid::Uuid> {
        let identity = self
            .identity
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity read lock: {}", e))?;
        self.discovery.announce(&*identity, capabilities).await
    }

    /// Send transaction to target resonance
    pub async fn send_transaction(
        &self,
        target_resonance: ResonanceState,
        action: Vec<u8>,
    ) -> Result<uuid::Uuid> {
        let identity = self
            .identity
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity read lock: {}", e))?;

        // Step 1: Create transaction
        let tx = self
            .protocol
            .create_transaction(identity.resonance, target_resonance, action)?;

        // Step 2: Mask transaction with resonance-derived parameters
        // R-03-001: Uses current epoch for key rotation
        // R-03-002: Optionally adds forward secrecy
        let mut params = MaskingParams::from_resonance(&identity.resonance, &target_resonance);

        // R-03-002: Add forward secrecy if enabled
        let ephemeral_key = MaskingParams::generate_ephemeral_key();
        params = params.with_ephemeral_key(ephemeral_key);

        let masked = self.protocol.mask_transaction(&tx, &params)?;

        // Step 3: Embed in carrier
        let carrier = self.protocol.embed_transaction(&masked, CarrierType::Raw)?;

        // Step 4: Create packet with key epoch and ephemeral key
        let packet =
            self.protocol
                .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)?;

        // Step 5: Broadcast
        self.broadcast.broadcast(packet).await?;

        Ok(tx.id)
    }

    /// Receive pending transactions
    pub async fn receive_transactions(&self) -> Result<Vec<GhostTransaction>> {
        let identity = self
            .identity
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity read lock: {}", e))?;

        // Receive packets from broadcast
        let packets = self.broadcast.receive(&*identity).await?;

        let mut transactions = Vec::new();

        // Process each packet
        for packet in packets {
            // The protocol will automatically derive masking parameters
            // from the sender_resonance (in packet) and our resonance state
            if let Some(tx) = self.protocol.receive_packet(&packet, &identity.resonance)? {
                transactions.push(tx);
            }
        }

        Ok(transactions)
    }

    /// Find nodes matching target resonance
    pub fn find_nodes(&self, target_resonance: &ResonanceState) -> Vec<DiscoveredNode> {
        self.discovery.find_nodes(target_resonance)
    }

    /// Find nodes with specific capabilities
    pub fn find_nodes_with_capabilities(&self, capabilities: &[String]) -> Vec<DiscoveredNode> {
        self.discovery.find_nodes_with_capabilities(capabilities)
    }

    /// Update node resonance state
    pub fn update_resonance(&self, new_resonance: ResonanceState) -> Result<()> {
        let mut identity = self
            .identity
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity write lock: {}", e))?;
        identity.update_resonance(new_resonance);
        Ok(())
    }

    /// Regenerate ephemeral identity (for privacy)
    pub fn regenerate_identity(&self) -> Result<()> {
        let mut identity = self
            .identity
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity write lock: {}", e))?;
        identity.regenerate_id();
        Ok(())
    }

    /// Generate decoy traffic for privacy
    pub async fn generate_decoy_traffic(&self, count: usize) -> Result<()> {
        self.broadcast.generate_decoy_traffic(count).await
    }

    /// Cleanup expired channels and inactive nodes
    pub fn cleanup(&self) -> Result<()> {
        self.broadcast.cleanup_expired_channels()?;
        self.discovery.cleanup()?;
        Ok(())
    }

    /// Get network statistics
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            broadcast: self.broadcast.get_stats(),
            discovery: self.discovery.get_stats(),
            active_nodes: self.discovery.active_node_count(),
            active_channels: self.broadcast.active_channel_count(),
        }
    }

    /// Get current node identity
    pub fn get_identity(&self) -> Result<NodeIdentity> {
        let identity = self
            .identity
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire identity read lock: {}", e))?;
        Ok(identity.clone())
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Broadcasting statistics
    pub broadcast: BroadcastStats,

    /// Discovery statistics
    pub discovery: DiscoveryStats,

    /// Active node count
    pub active_nodes: usize,

    /// Active channel count
    pub active_channels: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_network_creation() {
        let network = GhostNetwork::with_random_identity();
        let identity = network.get_identity().unwrap();

        // Invariant: Identity should never be nil
        assert_ne!(
            identity.id,
            uuid::Uuid::nil(),
            "Identity UUID must not be nil"
        );

        // Invariant: Resonance values should be finite
        assert!(identity.resonance.psi.is_finite(), "Psi must be finite");
        assert!(identity.resonance.rho.is_finite(), "Rho must be finite");
        assert!(identity.resonance.omega.is_finite(), "Omega must be finite");
    }

    #[tokio::test]
    async fn test_announce_and_discover() {
        let network1 = GhostNetwork::with_random_identity();
        let network2 = GhostNetwork::with_random_identity();

        // Network 1 announces
        let beacon_id = network1
            .announce(Some(vec!["storage".to_string()]))
            .await
            .unwrap();
        assert_ne!(beacon_id, uuid::Uuid::nil(), "Beacon ID must not be nil");

        // Simulate beacon propagation to network2
        let resonance = network1.get_identity().unwrap().resonance;
        let beacon = DiscoveryBeacon::new(resonance, 300, Some(vec!["storage".to_string()]));
        network2.discovery.receive_beacon(beacon).unwrap();

        // Network 2 finds network 1
        let found = network2.find_nodes(&resonance);
        assert_eq!(found.len(), 1);
    }

    #[tokio::test]
    async fn test_send_and_receive() {
        let network = GhostNetwork::with_random_identity();

        let target = ResonanceState::new(2.0, 2.0, 2.0);
        let action = b"test transaction".to_vec();

        // Send transaction
        let tx_id = network.send_transaction(target, action).await.unwrap();
        assert_ne!(tx_id, uuid::Uuid::nil());

        let stats = network.get_stats();
        assert_eq!(stats.broadcast.packets_sent, 1);
    }

    #[test]
    fn test_regenerate_identity() {
        let network = GhostNetwork::with_random_identity();

        let old_id = network.get_identity().unwrap().id;
        network.regenerate_identity().unwrap();
        let new_id = network.get_identity().unwrap().id;

        assert_ne!(old_id, new_id);
        assert_ne!(new_id, uuid::Uuid::nil(), "New identity must not be nil");
    }

    #[test]
    fn test_update_resonance() {
        let network = GhostNetwork::with_random_identity();

        let new_resonance = ResonanceState::new(5.0, 5.0, 5.0);
        network.update_resonance(new_resonance).unwrap();

        let identity = network.get_identity().unwrap();
        assert_eq!(identity.resonance.psi, 5.0);
        assert_eq!(identity.resonance.rho, 5.0);
        assert_eq!(identity.resonance.omega, 5.0);
    }

    #[test]
    fn test_find_capabilities() {
        let network = GhostNetwork::with_random_identity();

        // Add a discovered node with capabilities
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let beacon = DiscoveryBeacon::new(
            resonance,
            300,
            Some(vec!["compute".to_string(), "storage".to_string()]),
        );
        network.discovery.receive_beacon(beacon).unwrap();

        // Find nodes with capabilities
        let found = network.find_nodes_with_capabilities(&["storage".to_string()]);
        assert_eq!(found.len(), 1);
    }

    #[tokio::test]
    async fn test_decoy_traffic() {
        let network = GhostNetwork::with_random_identity();

        network.generate_decoy_traffic(5).await.unwrap();

        let stats = network.get_stats();
        assert_eq!(stats.broadcast.decoy_packets, 5);
    }

    #[test]
    fn test_cleanup() {
        let network = GhostNetwork::with_random_identity();

        // Add some channels and nodes
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        network
            .broadcast
            .create_channel(resonance, 0.1, 300)
            .unwrap();

        // Cleanup should work without errors
        assert!(network.cleanup().is_ok());
    }

    #[tokio::test]
    async fn test_network_stats() {
        let network = GhostNetwork::with_random_identity();

        network.announce(None).await.unwrap();

        let stats = network.get_stats();
        assert_eq!(stats.discovery.beacons_sent, 1);
    }
}
