//! Common type definitions and aliases
//!
//! Provides shared types used across the MEF system.

use serde::{Deserialize, Serialize};

/// Resonance triplet (ψ, ρ, ω) representing spectral signature
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResonanceTriplet {
    /// Phase alignment (psi)
    pub psi: f64,
    /// Resonance strength (rho)
    pub rho: f64,
    /// Oscillation frequency (omega)
    pub omega: f64,
}

impl ResonanceTriplet {
    /// Create a new resonance triplet
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    /// Create a zero resonance triplet
    pub fn zero() -> Self {
        Self {
            psi: 0.0,
            rho: 0.0,
            omega: 0.0,
        }
    }

    /// Compute the magnitude of the resonance triplet
    pub fn magnitude(&self) -> f64 {
        (self.psi.powi(2) + self.rho.powi(2) + self.omega.powi(2)).sqrt()
    }

    /// Normalize the resonance triplet to unit magnitude
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            return *self;
        }
        Self {
            psi: self.psi / mag,
            rho: self.rho / mag,
            omega: self.omega / mag,
        }
    }
}

impl Default for ResonanceTriplet {
    fn default() -> Self {
        Self::zero()
    }
}

/// Hash type for content addressing (32-byte SHA256)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    /// Create from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let bytes = hex::decode(s)?;
        if bytes.len() != 32 {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Node identifier in the network
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    /// Create a new node ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get as string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Transaction identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxId(pub String);

impl TxId {
    /// Create a new transaction ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get as string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp in seconds since UNIX epoch
pub type Timestamp = u64;

/// Time-to-live in seconds
pub type Ttl = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_triplet() {
        let triplet = ResonanceTriplet::new(3.0, 4.0, 0.0);
        assert_eq!(triplet.magnitude(), 5.0);

        let normalized = triplet.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
        assert_eq!(normalized.psi, 0.6);
        assert_eq!(normalized.rho, 0.8);
    }

    #[test]
    fn test_resonance_triplet_zero() {
        let zero = ResonanceTriplet::zero();
        assert_eq!(zero.magnitude(), 0.0);

        let normalized = zero.normalize();
        assert_eq!(normalized.magnitude(), 0.0);
    }

    #[test]
    fn test_content_hash_hex() {
        let hash = ContentHash::from_bytes([0x42; 32]);
        let hex = hash.to_hex();
        assert_eq!(hex.len(), 64);

        let parsed = ContentHash::from_hex(&hex).unwrap();
        assert_eq!(parsed, hash);
    }

    #[test]
    fn test_node_id() {
        let id = NodeId::new("node_123");
        assert_eq!(id.as_str(), "node_123");
        assert_eq!(id.to_string(), "node_123");

        let id2: NodeId = "node_456".into();
        assert_eq!(id2.as_str(), "node_456");
    }

    #[test]
    fn test_tx_id() {
        let tx = TxId::new("tx_abc123");
        assert_eq!(tx.as_str(), "tx_abc123");
        assert_eq!(tx.to_string(), "tx_abc123");
    }
}
