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
#[derive(Clone, Debug)]
pub struct PacketCodec {
    format: WireFormat,
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

