/*!
 * Packet Codec for Wire Format
 *
 * Handles serialization/deserialization of Ghost packets for network transmission.
 * Supports multiple wire formats:
 * - JSON (human-readable, debugging)
 * - Bincode (compact, production)
 */

use crate::packet::GhostPacket;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Wire format for packet serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WireFormat {
    /// JSON format (human-readable, larger size)
    Json,
    /// Bincode format (binary, compact)
    Bincode,
}

impl Default for WireFormat {
    fn default() -> Self {
        Self::Bincode // Use compact format by default
    }
}

/// Packet codec for serialization/deserialization
#[derive(Clone)]
pub struct PacketCodec {
    format: WireFormat,
}

impl std::fmt::Debug for PacketCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketCodec")
            .field("format", &self.format)
            .finish()
    }
}

impl PacketCodec {
    /// Create new codec with specified format
    pub fn new(format: WireFormat) -> Self {
        Self { format }
    }

    /// Create codec with JSON format
    pub fn json() -> Self {
        Self::new(WireFormat::Json)
    }

    /// Create codec with Bincode format
    pub fn bincode() -> Self {
        Self::new(WireFormat::Bincode)
    }

    /// Encode packet to bytes
    ///
    /// # Arguments
    /// * `packet` - Ghost packet to encode
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` with serialized bytes
    /// * `Err` if serialization failed
    pub fn encode(&self, packet: &GhostPacket) -> Result<Vec<u8>> {
        match self.format {
            WireFormat::Json => {
                serde_json::to_vec(packet).context("Failed to serialize packet as JSON")
            }
            WireFormat::Bincode => {
                bincode::serialize(packet).context("Failed to serialize packet as Bincode")
            }
        }
    }

    /// Decode packet from bytes
    ///
    /// # Arguments
    /// * `bytes` - Serialized packet bytes
    ///
    /// # Returns
    /// * `Ok(GhostPacket)` with deserialized packet
    /// * `Err` if deserialization failed
    pub fn decode(&self, bytes: &[u8]) -> Result<GhostPacket> {
        match self.format {
            WireFormat::Json => {
                serde_json::from_slice(bytes).context("Failed to deserialize packet from JSON")
            }
            WireFormat::Bincode => {
                bincode::deserialize(bytes).context("Failed to deserialize packet from Bincode")
            }
        }
    }

    /// Get wire format
    pub fn format(&self) -> WireFormat {
        self.format
    }

    /// Estimate encoded size (approximate)
    ///
    /// # Arguments
    /// * `packet` - Packet to estimate size for
    ///
    /// # Returns
    /// * Estimated size in bytes
    pub fn estimate_size(&self, _packet: &GhostPacket) -> usize {
        // Rough estimate based on format
        match self.format {
            WireFormat::Json => {
                // JSON is larger due to field names and formatting
                std::mem::size_of::<GhostPacket>() * 3
            }
            WireFormat::Bincode => {
                // Bincode is more compact
                std::mem::size_of::<GhostPacket>()
            }
        }
    }
}

impl Default for PacketCodec {
    fn default() -> Self {
        Self::bincode()
    }
}

/// Encode packet with default codec (Bincode)
pub fn encode(packet: &GhostPacket) -> Result<Vec<u8>> {
    PacketCodec::default().encode(packet)
}

/// Decode packet with default codec (Bincode)
pub fn decode(bytes: &[u8]) -> Result<GhostPacket> {
    PacketCodec::default().decode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::{GhostTransaction, NodeIdentity, ResonanceState};
    use uuid::Uuid;

    fn create_test_packet() -> GhostPacket {
        use crate::packet::CarrierType;

        GhostPacket::new(
            ResonanceState::new(2.0, 2.0, 2.0),      // resonance
            ResonanceState::new(1.0, 1.0, 1.0),      // sender_resonance
            b"test payload".to_vec(),                 // masked_payload
            vec![1, 2, 3, 4, 5],                      // stego_carrier
            CarrierType::Raw,                         // carrier_type
            Some(b"test proof".to_vec()),             // zk_proof
        )
    }

    #[test]
    fn test_json_codec_roundtrip() {
        let codec = PacketCodec::json();
        let packet = create_test_packet();

        let encoded = codec.encode(&packet).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(packet.id, decoded.id);
        assert_eq!(packet.timestamp, decoded.timestamp);
        assert_eq!(packet.masked_payload, decoded.masked_payload);
        assert_eq!(packet.stego_carrier, decoded.stego_carrier);
    }

    #[test]
    fn test_bincode_codec_roundtrip() {
        let codec = PacketCodec::bincode();
        let packet = create_test_packet();

        let encoded = codec.encode(&packet).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(packet.id, decoded.id);
        assert_eq!(packet.timestamp, decoded.timestamp);
        assert_eq!(packet.masked_payload, decoded.masked_payload);
        assert_eq!(packet.stego_carrier, decoded.stego_carrier);
    }

    #[test]
    fn test_bincode_is_smaller_than_json() {
        let packet = create_test_packet();

        let json_codec = PacketCodec::json();
        let bincode_codec = PacketCodec::bincode();

        let json_bytes = json_codec.encode(&packet).unwrap();
        let bincode_bytes = bincode_codec.encode(&packet).unwrap();

        // Bincode should be more compact
        assert!(bincode_bytes.len() < json_bytes.len());
        println!(
            "JSON: {} bytes, Bincode: {} bytes",
            json_bytes.len(),
            bincode_bytes.len()
        );
    }

    #[test]
    fn test_default_codec() {
        let packet = create_test_packet();

        let encoded = encode(&packet).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(packet.id, decoded.id);
    }

    #[test]
    fn test_estimate_size() {
        let packet = create_test_packet();
        let codec = PacketCodec::bincode();

        let estimated = codec.estimate_size(&packet);
        let actual = codec.encode(&packet).unwrap().len();

        // Estimate should be in the right ballpark (within 3x)
        assert!(actual <= estimated * 3);
    }
}
