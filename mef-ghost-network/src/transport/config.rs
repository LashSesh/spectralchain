/*!
 * Transport Configuration
 *
 * Configuration options for network transport layer.
 */

use super::codec::WireFormat;
use serde::{Deserialize, Serialize};

/// Transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Wire format for packet serialization
    pub wire_format: WireFormat,

    /// Maximum packet size in bytes
    pub max_packet_size: usize,

    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,

    /// Keep-alive interval in seconds
    pub keepalive_interval_secs: u64,

    /// Maximum number of concurrent connections
    pub max_connections: usize,

    /// Enable gossipsub protocol (for broadcasting)
    pub enable_gossipsub: bool,

    /// Gossipsub topic name
    pub gossipsub_topic: String,

    /// Enable identify protocol (for peer info exchange)
    pub enable_identify: bool,

    /// Enable ping protocol (for connection health)
    pub enable_ping: bool,

    /// Ping interval in seconds
    pub ping_interval_secs: u64,

    /// Enable TCP transport
    pub enable_tcp: bool,

    /// Enable QUIC transport
    pub enable_quic: bool,

    /// Local peer key (optional, auto-generated if None)
    pub local_key: Option<Vec<u8>>,
}

impl TransportConfig {
    /// Create new transport configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for local development/testing
    pub fn local() -> Self {
        Self {
            wire_format: WireFormat::Json, // Use JSON for debugging
            max_packet_size: 10 * 1024 * 1024, // 10 MB (generous for testing)
            connection_timeout_secs: 10,
            keepalive_interval_secs: 30,
            max_connections: 10, // Small network
            enable_gossipsub: true,
            gossipsub_topic: "ghost-protocol-local".to_string(),
            enable_identify: true,
            enable_ping: true,
            ping_interval_secs: 15,
            enable_tcp: true,
            enable_quic: false, // QUIC later
            local_key: None,
        }
    }

    /// Create configuration for production deployment
    pub fn production() -> Self {
        Self {
            wire_format: WireFormat::Bincode, // Compact
            max_packet_size: 1 * 1024 * 1024, // 1 MB
            connection_timeout_secs: 30,
            keepalive_interval_secs: 60,
            max_connections: 1000, // Large network
            enable_gossipsub: true,
            gossipsub_topic: "ghost-protocol".to_string(),
            enable_identify: true,
            enable_ping: true,
            ping_interval_secs: 30,
            enable_tcp: true,
            enable_quic: true, // Enable QUIC for production
            local_key: None,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.max_packet_size == 0 {
            anyhow::bail!("max_packet_size must be greater than 0");
        }

        if self.max_connections == 0 {
            anyhow::bail!("max_connections must be greater than 0");
        }

        if !self.enable_tcp && !self.enable_quic {
            anyhow::bail!("At least one transport (TCP or QUIC) must be enabled");
        }

        Ok(())
    }
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            wire_format: WireFormat::Bincode,
            max_packet_size: 2 * 1024 * 1024, // 2 MB default
            connection_timeout_secs: 30,
            keepalive_interval_secs: 60,
            max_connections: 100,
            enable_gossipsub: true,
            gossipsub_topic: "ghost-protocol".to_string(),
            enable_identify: true,
            enable_ping: true,
            ping_interval_secs: 30,
            enable_tcp: true,
            enable_quic: false, // QUIC opt-in
            local_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = TransportConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_local_config_is_valid() {
        let config = TransportConfig::local();
        assert!(config.validate().is_ok());
        assert_eq!(config.wire_format, WireFormat::Json);
    }

    #[test]
    fn test_production_config_is_valid() {
        let config = TransportConfig::production();
        assert!(config.validate().is_ok());
        assert_eq!(config.wire_format, WireFormat::Bincode);
        assert!(config.enable_quic);
    }

    #[test]
    fn test_invalid_config_zero_packet_size() {
        let config = TransportConfig {
            max_packet_size: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_config_no_transport() {
        let config = TransportConfig {
            enable_tcp: false,
            enable_quic: false,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
}
