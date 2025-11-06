/*!
 * Ghost Packet Structures
 *
 * Addressless packet structures for the Ghost Networking Protocol.
 * Packets are identified by resonance state, not addresses.
 *
 * Based on "Quantenresonante Spektralfeld-Blockchain" Blueprint (Seite 4)
 */

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Resonance State - Tripolar (ψ, ρ, ω) from Gabriel Cells
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResonanceState {
    /// Psi (ψ) - Semantic dimension
    pub psi: f64,
    /// Rho (ρ) - Energy dimension
    pub rho: f64,
    /// Omega (ω) - Frequency dimension
    pub omega: f64,
}

impl ResonanceState {
    /// Create new resonance state
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    /// Create zero resonance state
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Calculate Euclidean distance to another state
    pub fn distance_to(&self, other: &ResonanceState) -> f64 {
        let dpsi = self.psi - other.psi;
        let drho = self.rho - other.rho;
        let domega = self.omega - other.omega;
        (dpsi * dpsi + drho * drho + domega * domega).sqrt()
    }

    /// Check if within resonance window
    pub fn is_resonant_with(&self, other: &ResonanceState, epsilon: f64) -> bool {
        self.distance_to(other) < epsilon
    }
}

/// Ghost Packet - Core data structure for addressless communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostPacket {
    /// Unique packet ID
    pub id: Uuid,

    /// Timestamp of creation
    pub timestamp: u64,

    /// Target resonance state - determines who can receive this packet
    pub resonance: ResonanceState,

    /// Sender resonance state - needed for deriving masking parameters
    /// This is included unmasked to enable symmetric key derivation
    pub sender_resonance: ResonanceState,

    /// Masked payload (after M_{θ,σ} operator)
    #[serde(with = "serde_bytes")]
    pub masked_payload: Vec<u8>,

    /// Steganographic carrier (after T operator)
    /// Can be text, image data, or other carrier types
    #[serde(with = "serde_bytes")]
    pub stego_carrier: Vec<u8>,

    /// Carrier type identifier
    pub carrier_type: CarrierType,

    /// Zero-knowledge proof (optional)
    pub zk_proof: Option<Vec<u8>>,

    /// Time-to-live (hops remaining)
    pub ttl: u8,

    /// Packet hash for integrity
    pub hash: [u8; 32],
}

/// Type of steganographic carrier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CarrierType {
    /// Zero-width Unicode characters in text
    ZeroWidth,

    /// LSB steganography in image data
    ImageLSB,

    /// Audio carrier (placeholder)
    Audio,

    /// Raw binary carrier
    Raw,
}

impl GhostPacket {
    /// Create new ghost packet
    pub fn new(
        resonance: ResonanceState,
        sender_resonance: ResonanceState,
        masked_payload: Vec<u8>,
        stego_carrier: Vec<u8>,
        carrier_type: CarrierType,
        zk_proof: Option<Vec<u8>>,
    ) -> Self {
        let id = Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("System time error: {}", e))
            .expect("Failed to get system time");

        let mut packet = Self {
            id,
            timestamp: timestamp.as_secs(),
            resonance,
            sender_resonance,
            masked_payload,
            stego_carrier,
            carrier_type,
            zk_proof,
            ttl: 32, // Default TTL
            hash: [0u8; 32],
        };

        packet.hash = packet.compute_hash();
        packet
    }

    /// Compute packet hash for integrity verification
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(self.id.as_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.resonance.psi.to_le_bytes());
        hasher.update(self.resonance.rho.to_le_bytes());
        hasher.update(self.resonance.omega.to_le_bytes());
        hasher.update(self.sender_resonance.psi.to_le_bytes());
        hasher.update(self.sender_resonance.rho.to_le_bytes());
        hasher.update(self.sender_resonance.omega.to_le_bytes());
        hasher.update(&self.masked_payload);
        hasher.update(&self.stego_carrier);
        hasher.update(&[self.carrier_type as u8]);
        hasher.update(&[self.ttl]);

        if let Some(ref proof) = self.zk_proof {
            hasher.update(proof);
        }

        hasher.finalize().into()
    }

    /// Verify packet integrity
    pub fn verify_integrity(&self) -> bool {
        self.hash == self.compute_hash()
    }

    /// Decrement TTL and return whether packet is still alive
    pub fn decrement_ttl(&mut self) -> bool {
        if self.ttl > 0 {
            self.ttl -= 1;
            // Recompute hash after TTL change
            self.hash = self.compute_hash();
            true
        } else {
            false
        }
    }

    /// Check if packet matches node's resonance state
    pub fn matches_resonance(&self, node_state: &ResonanceState, epsilon: f64) -> bool {
        self.resonance.is_resonant_with(node_state, epsilon)
    }

    /// Get packet size in bytes (approximate)
    pub fn size(&self) -> usize {
        16 + // UUID
        8 +  // timestamp
        24 + // resonance (3 x f64)
        24 + // sender_resonance (3 x f64)
        self.masked_payload.len() +
        self.stego_carrier.len() +
        1 +  // carrier_type
        1 +  // ttl
        32 + // hash
        self.zk_proof.as_ref().map_or(0, |p| p.len())
    }
}

/// Ghost Transaction - High-level transaction before masking/embedding
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct GhostTransaction {
    /// Transaction ID
    pub id: Uuid,

    /// Sender's resonance state (for routing, not identity)
    pub sender_resonance: ResonanceState,

    /// Target resonance state (addressless targeting)
    pub target_resonance: ResonanceState,

    /// Action/payload data
    #[serde(with = "serde_bytes")]
    pub action: Vec<u8>,

    /// Zero-knowledge proof data
    pub zk_data: Option<Vec<u8>>,

    /// Timestamp
    pub timestamp: u64,
}

impl GhostTransaction {
    /// Create new transaction
    pub fn new(
        sender_resonance: ResonanceState,
        target_resonance: ResonanceState,
        action: Vec<u8>,
        zk_data: Option<Vec<u8>>,
    ) -> Self {
        let id = Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            sender_resonance,
            target_resonance,
            action,
            zk_data,
            timestamp,
        }
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}

/// Network Node Identity (Resonance-based, not IP-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeIdentity {
    /// Node UUID (ephemeral, regenerated periodically)
    pub id: Uuid,

    /// Current resonance state
    pub resonance: ResonanceState,

    /// Timestamp of last resonance update
    pub last_update: u64,

    /// Public key for verification (optional)
    pub public_key: Option<Vec<u8>>,
}

impl NodeIdentity {
    /// Create new node identity
    pub fn new(resonance: ResonanceState, public_key: Option<Vec<u8>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            resonance,
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            public_key,
        }
    }

    /// Regenerate ephemeral ID (for privacy)
    pub fn regenerate_id(&mut self) {
        self.id = Uuid::new_v4();
        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Update resonance state
    pub fn update_resonance(&mut self, new_resonance: ResonanceState) {
        self.resonance = new_resonance;
        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_state_creation() {
        let state = ResonanceState::new(1.0, 2.0, 3.0);
        assert_eq!(state.psi, 1.0);
        assert_eq!(state.rho, 2.0);
        assert_eq!(state.omega, 3.0);
    }

    #[test]
    fn test_resonance_distance() {
        let state1 = ResonanceState::new(0.0, 0.0, 0.0);
        let state2 = ResonanceState::new(3.0, 4.0, 0.0);

        let distance = state1.distance_to(&state2);
        assert!((distance - 5.0).abs() < 1e-10); // 3-4-5 triangle
    }

    #[test]
    fn test_resonance_check() {
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(1.05, 1.05, 1.05);

        assert!(state1.is_resonant_with(&state2, 0.1));
        assert!(!state1.is_resonant_with(&state2, 0.01));
    }

    #[test]
    fn test_ghost_packet_creation() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let payload = b"test payload".to_vec();
        let carrier = b"test carrier".to_vec();

        let packet = GhostPacket::new(
            resonance,
            payload.clone(),
            carrier.clone(),
            CarrierType::Raw,
            None,
        );

        assert_eq!(packet.masked_payload, payload);
        assert_eq!(packet.stego_carrier, carrier);
        assert_eq!(packet.carrier_type, CarrierType::Raw);
        assert_eq!(packet.ttl, 32);
    }

    #[test]
    fn test_packet_integrity() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let packet = GhostPacket::new(
            resonance,
            b"payload".to_vec(),
            b"carrier".to_vec(),
            CarrierType::Raw,
            None,
        );

        assert!(packet.verify_integrity());

        // Modify packet
        let mut modified = packet.clone();
        modified.masked_payload.push(0xFF);

        assert!(!modified.verify_integrity());
    }

    #[test]
    fn test_ttl_decrement() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut packet = GhostPacket::new(
            resonance,
            b"payload".to_vec(),
            b"carrier".to_vec(),
            CarrierType::Raw,
            None,
        );

        packet.ttl = 2;

        assert!(packet.decrement_ttl());
        assert_eq!(packet.ttl, 1);

        assert!(packet.decrement_ttl());
        assert_eq!(packet.ttl, 0);

        assert!(!packet.decrement_ttl());
        assert_eq!(packet.ttl, 0);
    }

    #[test]
    fn test_packet_resonance_matching() {
        let packet_resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let packet = GhostPacket::new(
            packet_resonance,
            b"payload".to_vec(),
            b"carrier".to_vec(),
            CarrierType::Raw,
            None,
        );

        let node_state = ResonanceState::new(1.05, 1.05, 1.05);

        assert!(packet.matches_resonance(&node_state, 0.1));
        assert!(!packet.matches_resonance(&node_state, 0.01));
    }

    #[test]
    fn test_ghost_transaction() {
        let sender = ResonanceState::new(1.0, 1.0, 1.0);
        let target = ResonanceState::new(2.0, 2.0, 2.0);

        let tx = GhostTransaction::new(
            sender,
            target,
            b"action data".to_vec(),
            None,
        );

        // Test serialization roundtrip
        let bytes = tx.to_bytes();
        let recovered = GhostTransaction::from_bytes(&bytes).unwrap();

        assert_eq!(recovered.id, tx.id);
        assert_eq!(recovered.action, tx.action);
    }

    #[test]
    fn test_node_identity() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut node = NodeIdentity::new(resonance, None);

        let old_id = node.id;
        node.regenerate_id();

        assert_ne!(node.id, old_id);
    }

    #[test]
    fn test_packet_size() {
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let packet = GhostPacket::new(
            resonance,
            vec![0u8; 100],
            vec![0u8; 200],
            CarrierType::Raw,
            None,
        );

        let size = packet.size();
        assert!(size > 300); // At least payload + carrier
        assert!(size < 400); // Plus overhead
    }
}
