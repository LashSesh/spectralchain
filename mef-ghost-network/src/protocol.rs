/*!
 * Ghost Network Protocol - 6-Step Flow Implementation
 *
 * Implements the full protocol flow from the Blueprint (Seite 4):
 *
 * 1. Node creates proof-transaction: a, ZK(a, pk), ψ
 * 2. Masking: m' = M_{θ,σ}(a)
 * 3. Steganography: t = T(m')
 * 4. Broadcast to field: t, ψ
 * 5. Reception: Node checks R_ε(ψ_node, ψ); if yes: a* = M⁻¹_{θ,σ}(T⁻¹(t)), verify ZK
 * 6. Commit to ledger: B_new = Block(a*, ZK, ...)
 */

use crate::packet::{CarrierType, GhostPacket, GhostTransaction, ResonanceState};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zeroize::{Zeroize, ZeroizeOnDrop};

// Import quantum operators from mef-quantum-ops
// Note: These would be actual imports in production
// For now, we define interfaces that match mef-quantum-ops

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    /// Resonance window epsilon for matching
    pub resonance_epsilon: f64,

    /// Default TTL for packets
    pub default_ttl: u8,

    /// Maximum packet size
    pub max_packet_size: usize,

    /// Default carrier type
    pub default_carrier_type: CarrierType,

    /// Enable zero-knowledge proofs
    pub enable_zk_proofs: bool,

    /// Enable steganography
    pub enable_steganography: bool,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            resonance_epsilon: 0.1,
            default_ttl: 32,
            max_packet_size: 1024 * 1024, // 1 MB
            default_carrier_type: CarrierType::Raw,
            enable_zk_proofs: true,
            enable_steganography: true,
        }
    }
}

/// Masking parameters (from mef-quantum-ops)
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct MaskingParams {
    /// Seed for deterministic permutation
    #[serde(with = "serde_bytes")]
    pub seed: Vec<u8>,

    /// Phase rotation parameter
    #[serde(with = "serde_bytes")]
    pub phase: Vec<u8>,
}

impl MaskingParams {
    /// Create from seed
    pub fn from_seed(seed: &[u8]) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(b"phase");
        let phase = hasher.finalize().to_vec();

        Self {
            seed: seed.to_vec(),
            phase,
        }
    }

    /// Create random parameters
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let seed: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Self::from_seed(&seed)
    }

    /// Derive masking parameters from resonance states
    ///
    /// This allows both sender and receiver to compute the same parameters
    /// based on their shared resonance context. The derivation is deterministic
    /// and symmetric, enabling addressless key agreement.
    ///
    /// # Arguments
    /// * `sender` - Sender's resonance state
    /// * `target` - Target resonance state
    ///
    /// # Returns
    /// * Derived masking parameters that both parties can compute
    pub fn from_resonance(sender: &ResonanceState, target: &ResonanceState) -> Self {
        use sha2::{Digest, Sha256};

        // Create deterministic seed from resonance states
        let mut hasher = Sha256::new();
        hasher.update(b"ghost_network_masking_v1");
        hasher.update(sender.psi.to_le_bytes());
        hasher.update(sender.rho.to_le_bytes());
        hasher.update(sender.omega.to_le_bytes());
        hasher.update(target.psi.to_le_bytes());
        hasher.update(target.rho.to_le_bytes());
        hasher.update(target.omega.to_le_bytes());

        let seed = hasher.finalize().to_vec();
        Self::from_seed(&seed)
    }
}

/// Ghost Protocol - Core protocol implementation
pub struct GhostProtocol {
    config: ProtocolConfig,
}

impl GhostProtocol {
    /// Create new protocol instance
    pub fn new(config: ProtocolConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(ProtocolConfig::default())
    }

    /// Validate timestamp safety
    ///
    /// Checks that a timestamp is:
    /// 1. Not in the future (with 60s tolerance for clock skew)
    /// 2. Not too old (within 24 hours for security)
    /// 3. Not zero or invalid
    ///
    /// # Returns
    /// * `Ok(())` if timestamp is valid
    /// * `Err` with description if invalid
    fn validate_timestamp(&self, timestamp: u64) -> Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        if timestamp == 0 {
            anyhow::bail!("Timestamp cannot be zero");
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("System time error: {}", e))?
            .as_secs();

        // Allow 60 second clock skew tolerance
        const CLOCK_SKEW_TOLERANCE: u64 = 60;
        if timestamp > now + CLOCK_SKEW_TOLERANCE {
            anyhow::bail!("Timestamp is too far in the future: {} > {}", timestamp, now);
        }

        // Reject packets older than 24 hours
        const MAX_AGE: u64 = 24 * 3600;
        if timestamp + MAX_AGE < now {
            anyhow::bail!("Timestamp is too old: {} < {}", timestamp, now - MAX_AGE);
        }

        Ok(())
    }

    /// Step 1: Create proof-transaction
    ///
    /// Creates a transaction with action, ZK proof, and target resonance state.
    ///
    /// # Arguments
    /// * `sender_resonance` - Current node's resonance state
    /// * `target_resonance` - Target resonance state for routing
    /// * `action` - Transaction action/payload
    ///
    /// # Returns
    /// * `GhostTransaction` with optional ZK proof
    pub fn create_transaction(
        &self,
        sender_resonance: ResonanceState,
        target_resonance: ResonanceState,
        action: Vec<u8>,
    ) -> Result<GhostTransaction> {
        // Validate action size
        if action.len() > self.config.max_packet_size {
            anyhow::bail!(
                "Action too large: {} > {}",
                action.len(),
                self.config.max_packet_size
            );
        }

        // Create ZK proof if enabled
        let zk_data = if self.config.enable_zk_proofs {
            Some(self.create_zk_proof(&action)?)
        } else {
            None
        };

        Ok(GhostTransaction::new(
            sender_resonance,
            target_resonance,
            action,
            zk_data,
        ))
    }

    /// Step 2: Mask transaction
    ///
    /// Applies masking operator M_{θ,σ}(m) to the transaction.
    ///
    /// # Arguments
    /// * `transaction` - Transaction to mask
    /// * `params` - Masking parameters (θ, σ)
    ///
    /// # Returns
    /// * Masked transaction bytes
    pub fn mask_transaction(
        &self,
        transaction: &GhostTransaction,
        params: &MaskingParams,
    ) -> Result<Vec<u8>> {
        let tx_bytes = transaction.to_bytes();

        // Apply masking operator: M_{θ,σ}(m) = e^{iθ} U_σ m
        let masked = self.apply_masking(&tx_bytes, params)?;

        Ok(masked)
    }

    /// Step 3: Embed in steganographic carrier
    ///
    /// Applies steganography operator T(m') to create hidden payload.
    ///
    /// # Arguments
    /// * `masked_data` - Masked transaction bytes
    /// * `carrier_type` - Type of carrier to use
    ///
    /// # Returns
    /// * Steganographic carrier with embedded data
    pub fn embed_transaction(
        &self,
        masked_data: &[u8],
        carrier_type: CarrierType,
    ) -> Result<Vec<u8>> {
        if !self.config.enable_steganography {
            // If steganography disabled, use raw carrier
            return Ok(masked_data.to_vec());
        }

        match carrier_type {
            CarrierType::ZeroWidth => self.embed_zero_width(masked_data),
            CarrierType::ImageLSB => self.embed_image_lsb(masked_data),
            CarrierType::Raw => Ok(masked_data.to_vec()),
            CarrierType::Audio => {
                // Placeholder - not implemented yet
                Ok(masked_data.to_vec())
            }
        }
    }

    /// Step 4: Broadcast packet to field
    ///
    /// Creates and broadcasts a ghost packet with resonance state.
    ///
    /// # Arguments
    /// * `transaction` - Original transaction
    /// * `masked_data` - Masked transaction
    /// * `stego_carrier` - Steganographic carrier
    /// * `carrier_type` - Type of carrier
    ///
    /// # Returns
    /// * Ghost packet ready for broadcast
    pub fn create_packet(
        &self,
        transaction: &GhostTransaction,
        masked_data: Vec<u8>,
        stego_carrier: Vec<u8>,
        carrier_type: CarrierType,
    ) -> Result<GhostPacket> {
        let packet = GhostPacket::new(
            transaction.target_resonance,
            transaction.sender_resonance,
            masked_data,
            stego_carrier,
            carrier_type,
            transaction.zk_data.clone(),
        );

        Ok(packet)
    }

    /// Step 5: Receive and process packet
    ///
    /// Checks resonance, extracts, unmasks, and verifies packet.
    /// The masking parameters are automatically derived from the sender and target resonance states.
    ///
    /// # Arguments
    /// * `packet` - Received ghost packet
    /// * `node_state` - Current node's resonance state
    ///
    /// # Returns
    /// * Recovered transaction if resonance matches, None otherwise
    pub fn receive_packet(
        &self,
        packet: &GhostPacket,
        node_state: &ResonanceState,
    ) -> Result<Option<GhostTransaction>> {
        // Runtime Invariant: Validate packet timestamp safety (R-01-003)
        self.validate_timestamp(packet.timestamp)
            .context("Packet timestamp validation failed")?;

        // Runtime Invariant: Resonance values must be finite (R-01-002)
        if !packet.resonance.psi.is_finite() ||
           !packet.resonance.rho.is_finite() ||
           !packet.resonance.omega.is_finite() {
            anyhow::bail!("Invalid packet: resonance values must be finite");
        }

        if !packet.sender_resonance.psi.is_finite() ||
           !packet.sender_resonance.rho.is_finite() ||
           !packet.sender_resonance.omega.is_finite() {
            anyhow::bail!("Invalid packet: sender resonance values must be finite");
        }

        // Runtime Invariant: Payload must not be empty (R-01-002)
        if packet.masked_payload.is_empty() {
            anyhow::bail!("Invalid packet: masked payload cannot be empty");
        }

        // Step 5a: Check resonance R_ε(ψ_node, ψ_pkt)
        if !packet.matches_resonance(node_state, self.config.resonance_epsilon) {
            // Not resonant - ignore packet
            return Ok(None);
        }

        // Step 5b: Verify packet integrity
        if !packet.verify_integrity() {
            anyhow::bail!("Packet integrity check failed");
        }

        // Step 5c: Derive masking parameters from resonance states
        // The receiver can compute the same params as the sender using:
        // sender_resonance (from packet) and target_resonance (node's own state)
        let masking_params = MaskingParams::from_resonance(&packet.sender_resonance, node_state);

        // Step 5d: Extract from steganographic carrier: a' = T⁻¹(t)
        let extracted = if self.config.enable_steganography {
            self.extract_from_carrier(&packet.stego_carrier, packet.carrier_type)?
        } else {
            packet.masked_payload.clone()
        };

        // Step 5e: Unmask: a* = M⁻¹_{θ,σ}(a')
        let unmasked = self.unmask_data(&extracted, &masking_params)?;

        // Step 5f: Deserialize transaction
        let transaction = GhostTransaction::from_bytes(&unmasked)
            .context("Failed to deserialize transaction")?;

        // Runtime Invariant: Validate transaction timestamp (R-01-003)
        self.validate_timestamp(transaction.timestamp)
            .context("Transaction timestamp validation failed")?;

        // Step 5g: Verify ZK proof if present
        if let Some(ref proof) = transaction.zk_data {
            if self.config.enable_zk_proofs {
                self.verify_zk_proof(&transaction.action, proof)?;
            }
        }

        Ok(Some(transaction))
    }

    /// Step 6: Commit to ledger (interface for integration)
    ///
    /// This would integrate with mef-ledger to commit the transaction.
    ///
    /// # Arguments
    /// * `transaction` - Verified transaction to commit
    ///
    /// # Returns
    /// * Block ID (placeholder - actual integration needed)
    pub fn commit_to_ledger(&self, transaction: &GhostTransaction) -> Result<Vec<u8>> {
        // TODO: Integrate with mef-ledger
        // For now, return transaction ID as placeholder
        Ok(transaction.id.as_bytes().to_vec())
    }

    // ==================== Private Helper Methods ====================

    /// Create ZK proof for action
    fn create_zk_proof(&self, action: &[u8]) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};

        // Simplified ZK proof - in production, use Halo2
        // Proof of knowledge: hash(action)
        let mut hasher = Sha256::new();
        hasher.update(action);
        hasher.update(b"zk_proof");

        Ok(hasher.finalize().to_vec())
    }

    /// Verify ZK proof
    fn verify_zk_proof(&self, action: &[u8], proof: &[u8]) -> Result<()> {
        use sha2::{Digest, Sha256};

        // Simplified verification
        let mut hasher = Sha256::new();
        hasher.update(action);
        hasher.update(b"zk_proof");

        let expected = hasher.finalize();

        if proof == expected.as_slice() {
            Ok(())
        } else {
            anyhow::bail!("ZK proof verification failed")
        }
    }

    /// Apply masking operator M_{θ,σ}(m)
    fn apply_masking(&self, data: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        // Simplified masking - XOR with derived key
        // In production, use full mef-quantum-ops implementation
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&params.seed);
        hasher.update(&params.phase);
        let key = hasher.finalize();

        let mut masked = data.to_vec();
        for (i, byte) in masked.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }

        Ok(masked)
    }

    /// Unmask data M⁻¹_{θ,σ}(m')
    fn unmask_data(&self, masked: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        // Masking is symmetric (XOR), so unmask = mask
        self.apply_masking(masked, params)
    }

    /// Embed data in zero-width characters
    fn embed_zero_width(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Create carrier text
        let mut carrier = String::from("Public message content. ");

        // Encode each byte as 8 zero-width characters
        for byte in data {
            for bit in (0..8).rev() {
                if (byte >> bit) & 1 == 1 {
                    carrier.push('\u{200C}'); // Zero Width Non-Joiner
                } else {
                    carrier.push('\u{200B}'); // Zero Width Space
                }
            }
        }

        carrier.push_str("End of public message.");
        Ok(carrier.into_bytes())
    }

    /// Extract data from zero-width characters
    fn extract_from_zero_width(&self, carrier: &[u8]) -> Result<Vec<u8>> {
        let text = String::from_utf8(carrier.to_vec())
            .context("Invalid UTF-8 in carrier")?;

        let mut bits = Vec::new();
        for ch in text.chars() {
            match ch {
                '\u{200B}' => bits.push(0),
                '\u{200C}' => bits.push(1),
                _ => {}
            }
        }

        // Convert bits to bytes
        let mut data = Vec::new();
        for chunk in bits.chunks(8) {
            if chunk.len() == 8 {
                let mut byte = 0u8;
                for (i, &bit) in chunk.iter().enumerate() {
                    byte |= bit << (7 - i);
                }
                data.push(byte);
            }
        }

        Ok(data)
    }

    /// Embed data in image LSB
    fn embed_image_lsb(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Create dummy image carrier (simplified)
        let image_size = (data.len() * 8) + 1024; // Need 8 pixels per byte
        let mut carrier = vec![0u8; image_size];

        // Embed data in LSB of each byte
        for (i, &byte) in data.iter().enumerate() {
            for bit in 0..8 {
                let carrier_idx = i * 8 + bit;
                if carrier_idx < carrier.len() {
                    carrier[carrier_idx] = (carrier[carrier_idx] & 0xFE) | ((byte >> bit) & 1);
                }
            }
        }

        Ok(carrier)
    }

    /// Extract data from image LSB
    fn extract_from_image_lsb(&self, carrier: &[u8]) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        for chunk in carrier.chunks(8) {
            let mut byte = 0u8;
            for (i, &pixel) in chunk.iter().enumerate() {
                byte |= (pixel & 1) << i;
            }
            data.push(byte);
        }

        Ok(data)
    }

    /// Extract from carrier based on type
    fn extract_from_carrier(&self, carrier: &[u8], carrier_type: CarrierType) -> Result<Vec<u8>> {
        match carrier_type {
            CarrierType::ZeroWidth => self.extract_from_zero_width(carrier),
            CarrierType::ImageLSB => self.extract_from_image_lsb(carrier),
            CarrierType::Raw => Ok(carrier.to_vec()),
            CarrierType::Audio => Ok(carrier.to_vec()), // Placeholder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let protocol = GhostProtocol::default();
        assert_eq!(protocol.config.resonance_epsilon, 0.1);
    }

    #[test]
    fn test_create_transaction() {
        let protocol = GhostProtocol::default();

        let sender = ResonanceState::new(1.0, 1.0, 1.0);
        let target = ResonanceState::new(2.0, 2.0, 2.0);
        let action = b"test action".to_vec();

        let tx = protocol
            .create_transaction(sender, target, action.clone())
            .unwrap();

        assert_eq!(tx.action, action);
        assert_eq!(tx.sender_resonance.psi, 1.0);
        assert_eq!(tx.target_resonance.psi, 2.0);
    }

    #[test]
    fn test_masking_roundtrip() {
        let protocol = GhostProtocol::default();
        let params = MaskingParams::from_seed(b"test_seed");

        let data = b"sensitive data";
        let masked = protocol.apply_masking(data, &params).unwrap();

        // Masked should be different
        assert_ne!(masked.as_slice(), data);

        // Unmask should recover original
        let unmasked = protocol.unmask_data(&masked, &params).unwrap();
        assert_eq!(unmasked.as_slice(), data);
    }

    #[test]
    fn test_zero_width_steganography() {
        let protocol = GhostProtocol::default();

        let data = b"secret";
        let carrier = protocol.embed_zero_width(data).unwrap();

        // Carrier should be larger than data
        assert!(carrier.len() > data.len());

        // Extract should recover original
        let extracted = protocol.extract_from_zero_width(&carrier).unwrap();
        assert_eq!(extracted, data);
    }

    #[test]
    fn test_full_protocol_flow() {
        let protocol = GhostProtocol::default();

        // Step 1: Create transaction
        let sender = ResonanceState::new(1.0, 1.0, 1.0);
        let target = ResonanceState::new(2.0, 2.0, 2.0);
        let action = b"transfer 100 tokens".to_vec();

        let tx = protocol
            .create_transaction(sender, target, action.clone())
            .unwrap();

        // Step 2: Mask with resonance-derived params
        let params = MaskingParams::from_resonance(&sender, &target);
        let masked = protocol.mask_transaction(&tx, &params).unwrap();

        // Step 3: Embed
        let carrier = protocol
            .embed_transaction(&masked, CarrierType::Raw)
            .unwrap();

        // Step 4: Create packet (now includes sender_resonance)
        let packet = protocol
            .create_packet(&tx, masked, carrier, CarrierType::Raw)
            .unwrap();

        // Step 5: Receive (matching resonance)
        // The receiver's resonance should be close to target
        let node_state = ResonanceState::new(2.05, 2.05, 2.05);
        let received = protocol
            .receive_packet(&packet, &node_state)
            .unwrap();

        assert!(received.is_some(), "Packet should be received");
        let recovered_tx = received.unwrap();
        assert_eq!(recovered_tx.action, action, "Action should match original");

        // Invariant: Transaction ID should be preserved
        assert_eq!(recovered_tx.id, tx.id, "Transaction ID must be preserved");
    }

    #[test]
    fn test_non_resonant_packet_ignored() {
        let protocol = GhostProtocol::default();

        let sender = ResonanceState::new(1.0, 1.0, 1.0);
        let target = ResonanceState::new(2.0, 2.0, 2.0);

        let tx = protocol
            .create_transaction(sender, target, b"test".to_vec())
            .unwrap();

        let params = MaskingParams::from_resonance(&sender, &target);
        let masked = protocol.mask_transaction(&tx, &params).unwrap();
        let carrier = masked.clone();

        let packet = protocol
            .create_packet(&tx, masked, carrier, CarrierType::Raw)
            .unwrap();

        // Node with very different resonance (outside epsilon window)
        let node_state = ResonanceState::new(10.0, 10.0, 10.0);

        let received = protocol
            .receive_packet(&packet, &node_state)
            .unwrap();

        // Should not receive packet due to resonance mismatch
        assert!(received.is_none(), "Non-resonant packet should be ignored");
    }

    #[test]
    fn test_zk_proof_verification() {
        let protocol = GhostProtocol::default();

        let action = b"test action";
        let proof = protocol.create_zk_proof(action).unwrap();

        // Valid proof should verify
        assert!(protocol.verify_zk_proof(action, &proof).is_ok());

        // Wrong action should fail
        assert!(protocol.verify_zk_proof(b"wrong action", &proof).is_err());
    }

    #[test]
    fn test_masking_params_from_resonance() {
        // Test that masking params are deterministically derived from resonance states
        let sender = ResonanceState::new(1.0, 2.0, 3.0);
        let target = ResonanceState::new(4.0, 5.0, 6.0);

        let params1 = MaskingParams::from_resonance(&sender, &target);
        let params2 = MaskingParams::from_resonance(&sender, &target);

        // Same inputs should produce same params
        assert_eq!(params1.seed, params2.seed, "Seeds should match");
        assert_eq!(params1.phase, params2.phase, "Phases should match");

        // Different inputs should produce different params
        let different_sender = ResonanceState::new(1.1, 2.0, 3.0);
        let params3 = MaskingParams::from_resonance(&different_sender, &target);
        assert_ne!(params1.seed, params3.seed, "Different senders should produce different seeds");
    }

    #[test]
    fn test_end_to_end_masking_with_resonance() {
        // Test the complete send/receive flow with resonance-derived params
        let protocol = GhostProtocol::default();

        let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);
        let action = b"secret message".to_vec();

        // Sender creates and encrypts transaction
        let tx = protocol
            .create_transaction(sender_resonance, target_resonance, action.clone())
            .unwrap();

        let sender_params = MaskingParams::from_resonance(&sender_resonance, &target_resonance);
        let masked = protocol.mask_transaction(&tx, &sender_params).unwrap();
        let carrier = protocol.embed_transaction(&masked, CarrierType::Raw).unwrap();
        let packet = protocol
            .create_packet(&tx, masked, carrier, CarrierType::Raw)
            .unwrap();

        // Receiver with matching resonance
        let receiver_resonance = ResonanceState::new(2.05, 2.05, 2.05); // Close to target
        let received = protocol
            .receive_packet(&packet, &receiver_resonance)
            .unwrap();

        // Should successfully decrypt and recover the message
        assert!(received.is_some(), "Receiver should decrypt packet");
        let recovered_tx = received.unwrap();
        assert_eq!(recovered_tx.action, action, "Recovered action should match original");
        assert_eq!(recovered_tx.id, tx.id, "Transaction ID should be preserved");
    }
}
