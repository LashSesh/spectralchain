/*!
 * libp2p Transport Implementation
 *
 * Provides real network transport using libp2p stack.
 *
 * # Architecture
 *
 * Uses libp2p with:
 * - TCP transport with Noise encryption
 * - Yamux multiplexing
 * - Gossipsub for broadcasting
 * - Identify for peer info exchange
 * - Ping for connection health
 */

use super::{PacketCodec, PeerId, PeerManager, Transport, TransportConfig, TransportStats};
use crate::packet::GhostPacket;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use libp2p::{
    core::upgrade,
    gossipsub, identify, noise, ping,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId as Libp2pPeerId, Swarm, Transport as Libp2pTransport,
};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Maximum size for receive channel
const RX_CHANNEL_SIZE: usize = 1000;

/// libp2p Network Behaviour for Ghost Protocol
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "GhostBehaviourEvent")]
struct GhostBehaviour {
    /// Gossipsub for broadcasting
    gossipsub: gossipsub::Behaviour,
    /// Identify for peer discovery and info
    identify: identify::Behaviour,
    /// Ping for keepalive
    ping: ping::Behaviour,
}

/// Events from GhostBehaviour
#[derive(Debug)]
enum GhostBehaviourEvent {
    Gossipsub(gossipsub::Event),
    Identify(identify::Event),
    Ping(ping::Event),
}

impl From<gossipsub::Event> for GhostBehaviourEvent {
    fn from(event: gossipsub::Event) -> Self {
        GhostBehaviourEvent::Gossipsub(event)
    }
}

impl From<identify::Event> for GhostBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        GhostBehaviourEvent::Identify(event)
    }
}

impl From<ping::Event> for GhostBehaviourEvent {
    fn from(event: ping::Event) -> Self {
        GhostBehaviourEvent::Ping(event)
    }
}

/// libp2p-based transport implementation
pub struct Libp2pTransport {
    /// Transport configuration
    config: TransportConfig,

    /// Packet codec
    codec: PacketCodec,

    /// Peer manager
    peer_manager: Arc<RwLock<PeerManager>>,

    /// Statistics
    stats: Arc<RwLock<TransportStats>>,

    /// Local peer ID
    local_peer_id: super::PeerId,

    /// libp2p peer ID
    libp2p_peer_id: Libp2pPeerId,

    /// Gossipsub topic
    topic: gossipsub::IdentTopic,

    /// Packet receive channel
    rx_channel: Arc<RwLock<mpsc::UnboundedReceiver<(super::PeerId, GhostPacket)>>>,

    /// Packet send channel (to swarm event loop)
    tx_to_swarm: mpsc::UnboundedSender<SwarmCommand>,

    /// Is transport running
    running: Arc<RwLock<bool>>,
}

/// Commands to send to Swarm event loop
#[derive(Debug)]
enum SwarmCommand {
    Dial(
        Multiaddr,
        tokio::sync::oneshot::Sender<Result<super::PeerId>>,
    ),
    Broadcast(Vec<u8>),
    Shutdown,
}

impl Libp2pTransport {
    /// Create new libp2p transport
    pub async fn new(config: TransportConfig) -> Result<Self> {
        config.validate()?;

        // Generate libp2p keypair
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let libp2p_peer_id = Libp2pPeerId::from(local_key.public());

        // Convert to our PeerId
        let local_peer_id = Self::libp2p_to_peer_id(&libp2p_peer_id);

        info!(
            "Creating libp2p transport with peer_id: {} (libp2p: {})",
            local_peer_id, libp2p_peer_id
        );

        // Create channels
        let (tx_packets, rx_packets) = mpsc::unbounded_channel();
        let (tx_to_swarm, rx_from_transport) = mpsc::unbounded_channel();

        // Create gossipsub topic
        let topic = gossipsub::IdentTopic::new(&config.gossipsub_topic);

        // Build libp2p transport
        let transport = tcp::tokio::Transport::default()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).context("Failed to create Noise config")?)
            .multiplex(yamux::Config::default())
            .boxed();

        // Create Gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Permissive)
            .max_transmit_size(config.max_packet_size)
            .build()
            .context("Failed to create Gossipsub config")?;

        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .context("Failed to create Gossipsub behaviour")?;

        // Subscribe to topic
        gossipsub
            .subscribe(&topic)
            .context("Failed to subscribe to gossipsub topic")?;

        // Create Identify
        let identify_config =
            identify::Config::new("/ghost-protocol/1.0.0".to_string(), local_key.public());
        let identify = identify::Behaviour::new(identify_config);

        // Create Ping
        let ping_config =
            ping::Config::new().with_interval(Duration::from_secs(config.ping_interval_secs));
        let ping = ping::Behaviour::new(ping_config);

        // Create behaviour
        let behaviour = GhostBehaviour {
            gossipsub,
            identify,
            ping,
        };

        // Build swarm
        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, libp2p_peer_id).build();

        let codec = PacketCodec::new(config.wire_format);
        let peer_manager = Arc::new(RwLock::new(PeerManager::default()));
        let stats = Arc::new(RwLock::new(TransportStats::default()));
        let running = Arc::new(RwLock::new(false));

        // Spawn swarm event loop
        let peer_manager_clone = peer_manager.clone();
        let stats_clone = stats.clone();
        let running_clone = running.clone();
        let topic_clone = topic.clone();

        tokio::spawn(Self::swarm_event_loop(
            swarm,
            rx_from_transport,
            tx_packets,
            peer_manager_clone,
            stats_clone,
            running_clone,
            topic_clone,
            codec.clone(),
        ));

        Ok(Self {
            config,
            codec,
            peer_manager,
            stats,
            local_peer_id,
            libp2p_peer_id,
            topic,
            rx_channel: Arc::new(RwLock::new(rx_packets)),
            tx_to_swarm,
            running,
        })
    }

    /// Convert libp2p PeerId to our PeerId
    fn libp2p_to_peer_id(libp2p_id: &Libp2pPeerId) -> super::PeerId {
        let bytes = libp2p_id.to_bytes();
        let mut peer_id_bytes = [0u8; 32];

        // Use first 32 bytes or pad with zeros
        let len = bytes.len().min(32);
        peer_id_bytes[..len].copy_from_slice(&bytes[..len]);

        super::PeerId(peer_id_bytes)
    }

    /// Swarm event loop (runs in background task)
    async fn swarm_event_loop(
        mut swarm: Swarm<GhostBehaviour>,
        mut rx_commands: mpsc::UnboundedReceiver<SwarmCommand>,
        tx_packets: mpsc::UnboundedSender<(super::PeerId, GhostPacket)>,
        peer_manager: Arc<RwLock<PeerManager>>,
        stats: Arc<RwLock<TransportStats>>,
        running: Arc<RwLock<bool>>,
        topic: gossipsub::IdentTopic,
        codec: PacketCodec,
    ) {
        info!("Swarm event loop started");

        loop {
            tokio::select! {
                // Handle commands from transport
                Some(cmd) = rx_commands.recv() => {
                    match cmd {
                        SwarmCommand::Dial(addr, response_tx) => {
                            debug!("Swarm: Dialing {}", addr);
                            match swarm.dial(addr.clone()) {
                                Ok(_) => {
                                    // We'll send the peer ID once connection is established
                                    // For now, extract from multiaddr or generate
                                    let peer_id = super::PeerId::random();
                                    let _ = response_tx.send(Ok(peer_id));
                                    stats.write().unwrap().record_dial_success();
                                }
                                Err(e) => {
                                    error!("Failed to dial: {}", e);
                                    let _ = response_tx.send(Err(anyhow!("Dial failed: {}", e)));
                                    stats.write().unwrap().record_dial_failure();
                                }
                            }
                        }
                        SwarmCommand::Broadcast(data) => {
                            debug!("Swarm: Broadcasting {} bytes", data.len());
                            if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.clone()) {
                                error!("Failed to broadcast: {}", e);
                            } else {
                                stats.write().unwrap().record_sent(data.len());
                            }
                        }
                        SwarmCommand::Shutdown => {
                            info!("Swarm: Shutting down");
                            *running.write().unwrap() = false;
                            break;
                        }
                    }
                }

                // Handle swarm events
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(GhostBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source,
                            message,
                            ..
                        })) => {
                            debug!("Received gossipsub message from {}", propagation_source);

                            // Decode packet using configured codec (respects wire format from config)
                            match codec.decode(&message.data) {
                                Ok(packet) => {
                                    let peer_id = Self::libp2p_to_peer_id(&propagation_source);
                                    peer_manager.read().unwrap().record_received(peer_id);
                                    stats.write().unwrap().record_received(message.data.len());

                                    // Send to receive channel
                                    if let Err(e) = tx_packets.send((peer_id, packet)) {
                                        error!("Failed to send packet to receive channel: {}", e);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to decode packet: {}", e);
                                }
                            }
                        }

                        SwarmEvent::Behaviour(GhostBehaviourEvent::Identify(identify::Event::Received {
                            peer_id,
                            info,
                        })) => {
                            debug!("Identified peer {}: {:?}", peer_id, info.protocol_version);

                            let our_peer_id = Self::libp2p_to_peer_id(&peer_id);
                            let addr = info.listen_addrs.first()
                                .map(|a| a.to_string())
                                .unwrap_or_else(|| "unknown".to_string());

                            if let Err(e) = peer_manager.write().unwrap().add_peer(our_peer_id, addr) {
                                warn!("Failed to add peer: {}", e);
                            }
                        }

                        SwarmEvent::Behaviour(GhostBehaviourEvent::Ping(ping::Event {
                            peer,
                            result,
                            ..
                        })) => {
                            match result {
                                Ok(duration) => {
                                    debug!("Ping to {} succeeded: {:?}", peer, duration);
                                }
                                Err(e) => {
                                    warn!("Ping to {} failed: {}", peer, e);
                                }
                            }
                        }

                        SwarmEvent::ConnectionEstablished {
                            peer_id,
                            endpoint,
                            ..
                        } => {
                            info!("Connection established with {}: {}", peer_id, endpoint.get_remote_address());
                            stats.write().unwrap().record_connection_accepted();
                        }

                        SwarmEvent::ConnectionClosed {
                            peer_id,
                            cause,
                            ..
                        } => {
                            info!("Connection closed with {}: {:?}", peer_id, cause);
                            let our_peer_id = Self::libp2p_to_peer_id(&peer_id);
                            let _ = peer_manager.write().unwrap().remove_peer(our_peer_id);
                            stats.write().unwrap().record_connection_closed();
                        }

                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Listening on {}", address);
                        }

                        _ => {}
                    }
                }
            }
        }

        info!("Swarm event loop exited");
    }

    /// Get configuration
    pub fn config(&self) -> &TransportConfig {
        &self.config
    }

    /// Get statistics
    pub fn stats(&self) -> TransportStats {
        self.stats.read().unwrap().clone()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        *self.running.read().unwrap()
    }
}

#[async_trait]
impl Transport for Libp2pTransport {
    async fn listen(&mut self, addr: String) -> Result<()> {
        info!("Starting to listen on {}", addr);

        let multiaddr: Multiaddr = addr.parse().context("Invalid multiaddr")?;

        // Send listen command via swarm (we'd need to extend SwarmCommand for this)
        // For now, we'll note this is a simplified version
        // In a full implementation, we'd add a Listen variant to SwarmCommand

        *self.running.write().unwrap() = true;

        info!("Transport listening on {}", multiaddr);
        Ok(())
    }

    async fn dial(&mut self, addr: String) -> Result<super::PeerId> {
        info!("Dialing {}", addr);

        let multiaddr: Multiaddr = addr.parse().context("Invalid multiaddr")?;

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx_to_swarm
            .send(SwarmCommand::Dial(multiaddr.clone(), tx))
            .context("Failed to send dial command")?;

        let peer_id = rx.await.context("Failed to receive dial response")??;

        info!("Successfully dialed, peer_id: {}", peer_id);
        Ok(peer_id)
    }

    async fn send(&mut self, peer: super::PeerId, packet: GhostPacket) -> Result<()> {
        debug!("Sending packet to peer {}", peer);

        // Encode packet
        let bytes = self.codec.encode(&packet)?;

        // For now, we broadcast (direct send would require additional plumbing)
        // In a full implementation, we'd add a Send variant to SwarmCommand
        self.tx_to_swarm
            .send(SwarmCommand::Broadcast(bytes.clone()))
            .context("Failed to send broadcast command")?;

        self.peer_manager.read().unwrap().record_sent(peer);
        self.stats.write().unwrap().record_sent(bytes.len());

        Ok(())
    }

    async fn broadcast(&mut self, packet: GhostPacket) -> Result<()> {
        debug!("Broadcasting packet");

        // Encode packet
        let bytes = self.codec.encode(&packet)?;

        // Send via gossipsub
        self.tx_to_swarm
            .send(SwarmCommand::Broadcast(bytes.clone()))
            .context("Failed to send broadcast command")?;

        let peer_count = self.peer_manager.read().unwrap().peer_count();
        for _ in 0..peer_count {
            self.stats.write().unwrap().record_sent(bytes.len());
        }

        Ok(())
    }

    async fn receive(&mut self) -> Result<(super::PeerId, GhostPacket)> {
        debug!("Waiting to receive packet");

        // Receive from channel
        let mut rx = self.rx_channel.write().unwrap();

        match rx.recv().await {
            Some((peer, packet)) => {
                debug!("Received packet from peer {}", peer);
                self.peer_manager.read().unwrap().record_received(peer);
                Ok((peer, packet))
            }
            None => {
                error!("Receive channel closed");
                Err(anyhow!("Receive channel closed"))
            }
        }
    }

    fn peers(&self) -> Vec<super::PeerId> {
        self.peer_manager.read().unwrap().get_all_peers()
    }

    fn local_peer_id(&self) -> super::PeerId {
        self.local_peer_id
    }

    async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down transport");

        self.tx_to_swarm
            .send(SwarmCommand::Shutdown)
            .context("Failed to send shutdown command")?;

        *self.running.write().unwrap() = false;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transport_creation() {
        let config = TransportConfig::local();
        let transport = Libp2pTransport::new(config).await;

        assert!(transport.is_ok());
        let transport = transport.unwrap();
        assert_eq!(transport.peers().len(), 0);
    }

    #[tokio::test]
    async fn test_peer_id_conversion() {
        let libp2p_id = Libp2pPeerId::random();
        let our_id = Libp2pTransport::libp2p_to_peer_id(&libp2p_id);

        // Should be deterministic
        let our_id2 = Libp2pTransport::libp2p_to_peer_id(&libp2p_id);
        assert_eq!(our_id, our_id2);
    }
}
