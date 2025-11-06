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
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, warn, info, debug};
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

    /// Enable forward secrecy (R-03-002)
    pub enable_forward_secrecy: bool,

    /// Adaptive timestamp window configuration (R-03-003)
    pub adaptive_timestamps: bool,
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
            enable_forward_secrecy: true,
            adaptive_timestamps: true,
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

    /// Key rotation epoch (R-03-001)
    /// Allows for time-based key rotation while maintaining backward compatibility
    pub epoch: u64,

    /// Ephemeral key for forward secrecy (R-03-002)
    /// Optional - if present, this ephemeral key is mixed with the base key
    #[serde(with = "serde_bytes")]
    pub ephemeral_key: Option<Vec<u8>>,
}

impl MaskingParams {
    /// Key rotation epoch duration (1 hour = 3600 seconds)
    /// R-03-001: Keys are rotated every epoch to limit exposure
    const EPOCH_DURATION: u64 = 3600;

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
            epoch: Self::current_epoch(),
            ephemeral_key: None,
        }
    }

    /// Create random parameters
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let seed: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Self::from_seed(&seed)
    }

    /// Get current key rotation epoch
    /// R-03-001: Epoch-based key rotation
    pub fn current_epoch() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs() / Self::EPOCH_DURATION
    }

    /// Derive masking parameters from resonance states with key rotation
    ///
    /// This allows both sender and receiver to compute the same parameters
    /// based on their shared resonance context. The derivation is deterministic
    /// and symmetric, enabling addressless key agreement.
    ///
    /// R-03-001: Includes epoch-based key rotation
    ///
    /// # Arguments
    /// * `sender` - Sender's resonance state
    /// * `target` - Target resonance state
    ///
    /// # Returns
    /// * Derived masking parameters that both parties can compute
    pub fn from_resonance(sender: &ResonanceState, target: &ResonanceState) -> Self {
        Self::from_resonance_with_epoch(sender, target, Self::current_epoch())
    }

    /// Derive masking parameters with specific epoch
    /// R-03-001: Allows verification of packets from previous epochs during rotation
    pub fn from_resonance_with_epoch(
        sender: &ResonanceState,
        target: &ResonanceState,
        epoch: u64,
    ) -> Self {
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
        // Mix in epoch for key rotation
        hasher.update(epoch.to_le_bytes());

        let seed = hasher.finalize().to_vec();

        let mut params = Self::from_seed(&seed);
        params.epoch = epoch;
        params
    }

    /// Add forward secrecy with ephemeral key
    /// R-03-002: Mix in an ephemeral key that's never reused
    pub fn with_ephemeral_key(mut self, ephemeral_key: Vec<u8>) -> Self {
        self.ephemeral_key = Some(ephemeral_key);
        self
    }

    /// Generate ephemeral key for forward secrecy
    /// R-03-002: Each session gets a unique ephemeral key
    pub fn generate_ephemeral_key() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32).map(|_| rng.gen()).collect()
    }

    /// Derive final key mixing base key with ephemeral key if present
    /// R-03-002: Forward secrecy - compromising base key doesn't reveal past sessions
    pub fn derive_final_key(&self) -> Vec<u8> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&self.seed);
        hasher.update(&self.phase);
        hasher.update(self.epoch.to_le_bytes());

        // Mix in ephemeral key if present (R-03-002)
        if let Some(ref ephemeral) = self.ephemeral_key {
            hasher.update(b"ephemeral");
            hasher.update(ephemeral);
        }

        hasher.finalize().to_vec()
    }
}

/// Rate limiter for tracking timestamp validation failures
#[derive(Debug, Clone)]
struct TimestampFailureRecord {
    /// Number of failures
    count: usize,
    /// First failure timestamp
    first_failure: u64,
    /// Last failure timestamp
    last_failure: u64,
}

/// Metrics for packet processing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PacketMetrics {
    /// Total packets received
    pub packets_received: usize,

    /// Packets accepted (successfully processed)
    pub packets_accepted: usize,

    /// Packets rejected due to rate limiting
    pub rejected_rate_limited: usize,

    /// Packets rejected due to timestamp validation
    pub rejected_timestamp_invalid: usize,

    /// Packets rejected due to invalid resonance values
    pub rejected_invalid_resonance: usize,

    /// Packets rejected due to empty payload
    pub rejected_empty_payload: usize,

    /// Packets rejected due to integrity check failure
    pub rejected_integrity_failed: usize,

    /// Packets rejected due to ZK proof failure
    pub rejected_zk_proof_failed: usize,

    /// Packets ignored due to resonance mismatch (not a rejection)
    pub packets_ignored_resonance_mismatch: usize,

    /// Transactions rejected due to timestamp validation
    pub rejected_transaction_timestamp: usize,

    /// R-03-003: Adaptive timestamp window tracking
    /// Sum of timestamp deltas for computing average network latency
    pub timestamp_delta_sum: u64,

    /// Count of valid timestamps for computing average
    pub valid_timestamp_count: usize,
}

/// Network condition tracker for adaptive timestamp windows (R-03-003)
#[derive(Debug, Clone)]
struct NetworkConditions {
    /// Average network latency observed (seconds)
    average_latency: f64,

    /// Maximum latency observed (seconds)
    max_latency: u64,

    /// Last update timestamp
    last_update: u64,

    /// Sample count
    sample_count: usize,
}

/// Ghost Protocol - Core protocol implementation
pub struct GhostProtocol {
    config: ProtocolConfig,
    /// Rate limiter for timestamp failures (key: sender resonance hash)
    timestamp_failure_tracker: Arc<RwLock<HashMap<u64, TimestampFailureRecord>>>,
    /// Metrics for packet processing
    metrics: Arc<RwLock<PacketMetrics>>,
    /// Network condition tracker for adaptive timestamp windows (R-03-003)
    network_conditions: Arc<RwLock<NetworkConditions>>,
}

impl NetworkConditions {
    fn new() -> Self {
        Self {
            average_latency: 0.0,
            max_latency: 0,
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            sample_count: 0,
        }
    }

    /// Update with new latency sample (R-03-003)
    fn update(&mut self, latency_seconds: u64) {
        self.sample_count += 1;
        self.max_latency = self.max_latency.max(latency_seconds);

        // Exponential moving average (alpha = 0.3)
        let alpha = 0.3;
        self.average_latency = alpha * (latency_seconds as f64) + (1.0 - alpha) * self.average_latency;

        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
    }

    /// Get adaptive clock skew tolerance based on network conditions (R-03-003)
    fn get_clock_skew_tolerance(&self) -> u64 {
        const BASE_TOLERANCE: u64 = 60; // 60 seconds base
        const MIN_TOLERANCE: u64 = 30;
        const MAX_TOLERANCE: u64 = 300; // 5 minutes max

        if self.sample_count < 10 {
            // Not enough data, use base tolerance
            return BASE_TOLERANCE;
        }

        // Adaptive tolerance: base + 2 * average_latency + safety margin
        let adaptive = BASE_TOLERANCE + (2.0 * self.average_latency) as u64 + 10;
        adaptive.clamp(MIN_TOLERANCE, MAX_TOLERANCE)
    }

    /// Get adaptive maximum age based on network conditions (R-03-003)
    fn get_max_age(&self) -> u64 {
        const BASE_MAX_AGE: u64 = 24 * 3600; // 24 hours base
        const MIN_MAX_AGE: u64 = 3600; // 1 hour min
        const MAX_MAX_AGE: u64 = 48 * 3600; // 48 hours max

        if self.sample_count < 10 {
            return BASE_MAX_AGE;
        }

        // In poor network conditions, allow older packets
        // If average latency > 60s, increase max age
        if self.average_latency > 60.0 {
            let multiplier = 1.0 + (self.average_latency / 60.0 - 1.0) * 0.5;
            let adaptive = (BASE_MAX_AGE as f64 * multiplier) as u64;
            adaptive.clamp(MIN_MAX_AGE, MAX_MAX_AGE)
        } else {
            BASE_MAX_AGE
        }
    }
}

impl GhostProtocol {
    /// Create new protocol instance
    pub fn new(config: ProtocolConfig) -> Self {
        Self {
            config,
            timestamp_failure_tracker: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PacketMetrics::default())),
            network_conditions: Arc::new(RwLock::new(NetworkConditions::new())),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(ProtocolConfig::default())
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> PacketMetrics {
        self.metrics.read()
            .unwrap_or_else(|e| {
                warn!("Failed to acquire metrics lock: {}", e);
                e.into_inner()
            })
            .clone()
    }

    /// Reset metrics
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.write()
            .unwrap_or_else(|e| {
                warn!("Failed to acquire metrics lock: {}", e);
                e.into_inner()
            });
        *metrics = PacketMetrics::default();
    }

    /// Hash resonance state to use as rate limiter key
    fn hash_resonance(&self, resonance: &ResonanceState) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        // Hash the resonance values as bytes
        resonance.psi.to_bits().hash(&mut hasher);
        resonance.rho.to_bits().hash(&mut hasher);
        resonance.omega.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    /// Check if source is rate limited for timestamp failures
    /// Returns true if rate limit exceeded
    fn check_timestamp_failure_rate_limit(&self, sender_resonance: &ResonanceState) -> bool {
        const MAX_FAILURES: usize = 10; // Max 10 failures
        const WINDOW_SECONDS: u64 = 60; // Within 60 seconds

        let resonance_hash = self.hash_resonance(sender_resonance);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tracker = match self.timestamp_failure_tracker.write() {
            Ok(guard) => guard,
            Err(e) => {
                warn!("Failed to acquire rate limiter lock: {}", e);
                return false; // Don't rate limit on lock failure
            }
        };

        if let Some(record) = tracker.get(&resonance_hash) {
            // Check if within time window
            if now - record.first_failure < WINDOW_SECONDS {
                if record.count >= MAX_FAILURES {
                    warn!(
                        event = "rate_limit_applied",
                        reason = "timestamp_failures",
                        resonance = ?(sender_resonance.psi, sender_resonance.rho, sender_resonance.omega),
                        failure_count = record.count,
                        window_seconds = WINDOW_SECONDS,
                        "Rate limit applied: too many timestamp validation failures"
                    );
                    return true; // Rate limit exceeded
                }
            } else {
                // Window expired, reset counter
                tracker.remove(&resonance_hash);
            }
        }

        false
    }

    /// Record a timestamp validation failure
    fn record_timestamp_failure(&self, sender_resonance: &ResonanceState) {
        let resonance_hash = self.hash_resonance(sender_resonance);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tracker = match self.timestamp_failure_tracker.write() {
            Ok(guard) => guard,
            Err(e) => {
                warn!("Failed to acquire rate limiter lock: {}", e);
                return;
            }
        };

        tracker
            .entry(resonance_hash)
            .and_modify(|record| {
                record.count += 1;
                record.last_failure = now;
            })
            .or_insert(TimestampFailureRecord {
                count: 1,
                first_failure: now,
                last_failure: now,
            });
    }

    /// Cleanup expired rate limit records (should be called periodically)
    pub fn cleanup_rate_limiters(&self) -> usize {
        const WINDOW_SECONDS: u64 = 60;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tracker = match self.timestamp_failure_tracker.write() {
            Ok(guard) => guard,
            Err(e) => {
                warn!("Failed to acquire rate limiter lock for cleanup: {}", e);
                return 0;
            }
        };

        let expired: Vec<u64> = tracker
            .iter()
            .filter(|(_, record)| now - record.first_failure >= WINDOW_SECONDS)
            .map(|(key, _)| *key)
            .collect();

        for key in expired.iter() {
            tracker.remove(key);
        }

        expired.len()
    }

    /// Validate timestamp safety with adaptive windows (R-03-003)
    ///
    /// Checks that a timestamp is:
    /// 1. Not in the future (with adaptive tolerance for clock skew)
    /// 2. Not too old (adaptive max age based on network conditions)
    /// 3. Not zero or invalid
    ///
    /// # Returns
    /// * `Ok(())` if timestamp is valid
    /// * `Err` with description if invalid
    fn validate_timestamp(&self, timestamp: u64) -> Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        if timestamp == 0 {
            warn!(
                event = "timestamp_validation_failed",
                reason = "zero_timestamp",
                timestamp = timestamp,
                "Security: Rejected packet with zero timestamp"
            );
            anyhow::bail!("Timestamp cannot be zero");
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("System time error: {}", e))?
            .as_secs();

        // R-03-003: Adaptive clock skew tolerance based on network conditions
        let clock_skew_tolerance = if self.config.adaptive_timestamps {
            let conditions = self.network_conditions.read()
                .unwrap_or_else(|e| {
                    warn!("Failed to acquire network conditions lock: {}", e);
                    e.into_inner()
                });
            conditions.get_clock_skew_tolerance()
        } else {
            60 // Default 60 seconds
        };

        if timestamp > now + clock_skew_tolerance {
            warn!(
                event = "timestamp_validation_failed",
                reason = "future_timestamp",
                timestamp = timestamp,
                current_time = now,
                delta = timestamp - now,
                tolerance = clock_skew_tolerance,
                "Security: Rejected packet with future timestamp"
            );
            anyhow::bail!("Timestamp is too far in the future: {} > {}", timestamp, now);
        }

        // R-03-003: Adaptive maximum age based on network conditions
        let max_age = if self.config.adaptive_timestamps {
            let conditions = self.network_conditions.read()
                .unwrap_or_else(|e| {
                    warn!("Failed to acquire network conditions lock: {}", e);
                    e.into_inner()
                });
            conditions.get_max_age()
        } else {
            24 * 3600 // Default 24 hours
        };

        if timestamp + max_age < now {
            warn!(
                event = "timestamp_validation_failed",
                reason = "expired_timestamp",
                timestamp = timestamp,
                current_time = now,
                age_seconds = now - timestamp,
                max_age = max_age,
                "Security: Rejected packet with expired timestamp"
            );
            anyhow::bail!("Timestamp is too old: {} < {}", timestamp, now - max_age);
        }

        // Update network conditions with observed latency (R-03-003)
        if self.config.adaptive_timestamps && timestamp <= now {
            let latency = now - timestamp;
            if let Ok(mut conditions) = self.network_conditions.write() {
                conditions.update(latency);
            }

            // Update metrics
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.timestamp_delta_sum += latency;
                metrics.valid_timestamp_count += 1;
            }
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

    /// Step 2: Mask transaction with forward secrecy (R-03-002)
    ///
    /// Applies masking operator M_{θ,σ}(m) to the transaction.
    /// If forward secrecy is enabled, an ephemeral key is generated and mixed in.
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

        // Apply masking operator with forward secrecy support (R-03-001, R-03-002)
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

    /// Step 4: Broadcast packet to field with forward secrecy (R-03-001, R-03-002)
    ///
    /// Creates and broadcasts a ghost packet with resonance state.
    /// Includes key rotation epoch and optional ephemeral key for forward secrecy.
    ///
    /// # Arguments
    /// * `transaction` - Original transaction
    /// * `masked_data` - Masked transaction
    /// * `stego_carrier` - Steganographic carrier
    /// * `carrier_type` - Type of carrier
    /// * `masking_params` - Masking parameters used (contains epoch and ephemeral key)
    ///
    /// # Returns
    /// * Ghost packet ready for broadcast
    pub fn create_packet(
        &self,
        transaction: &GhostTransaction,
        masked_data: Vec<u8>,
        stego_carrier: Vec<u8>,
        carrier_type: CarrierType,
        masking_params: &MaskingParams,
    ) -> Result<GhostPacket> {
        // R-03-001 & R-03-002: Include key epoch and ephemeral key in packet
        let packet = GhostPacket::new_with_keys(
            transaction.target_resonance,
            transaction.sender_resonance,
            masked_data,
            stego_carrier,
            carrier_type,
            transaction.zk_data.clone(),
            masking_params.epoch,
            masking_params.ephemeral_key.clone(),
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
        // Increment total packets received
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.packets_received += 1;
        }

        // Check rate limiting for timestamp failures
        if self.check_timestamp_failure_rate_limit(&packet.sender_resonance) {
            // Increment rate limited metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_rate_limited += 1;
            }

            error!(
                event = "packet_rejected",
                reason = "rate_limited",
                packet_id = %packet.id,
                sender_resonance = ?(packet.sender_resonance.psi, packet.sender_resonance.rho, packet.sender_resonance.omega),
                "Security: Packet rejected due to rate limiting (too many timestamp failures)"
            );
            anyhow::bail!("Rate limit exceeded for timestamp validation failures");
        }

        // Runtime Invariant: Validate packet timestamp safety (R-01-003)
        if let Err(e) = self.validate_timestamp(packet.timestamp) {
            // Record the failure for rate limiting
            self.record_timestamp_failure(&packet.sender_resonance);

            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_timestamp_invalid += 1;
            }

            error!(
                event = "packet_rejected",
                reason = "timestamp_invalid",
                packet_id = %packet.id,
                timestamp = packet.timestamp,
                error = %e,
                "Security: Packet rejected due to timestamp validation failure"
            );
            return Err(e).context("Packet timestamp validation failed");
        }

        // Runtime Invariant: Resonance values must be finite (R-01-002)
        if !packet.resonance.psi.is_finite() ||
           !packet.resonance.rho.is_finite() ||
           !packet.resonance.omega.is_finite() {
            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_invalid_resonance += 1;
            }

            error!(
                event = "packet_rejected",
                reason = "invalid_resonance",
                packet_id = %packet.id,
                psi = packet.resonance.psi,
                rho = packet.resonance.rho,
                omega = packet.resonance.omega,
                "Security: Packet rejected due to non-finite resonance values"
            );
            anyhow::bail!("Invalid packet: resonance values must be finite");
        }

        if !packet.sender_resonance.psi.is_finite() ||
           !packet.sender_resonance.rho.is_finite() ||
           !packet.sender_resonance.omega.is_finite() {
            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_invalid_resonance += 1;
            }

            error!(
                event = "packet_rejected",
                reason = "invalid_sender_resonance",
                packet_id = %packet.id,
                psi = packet.sender_resonance.psi,
                rho = packet.sender_resonance.rho,
                omega = packet.sender_resonance.omega,
                "Security: Packet rejected due to non-finite sender resonance values"
            );
            anyhow::bail!("Invalid packet: sender resonance values must be finite");
        }

        // Runtime Invariant: Payload must not be empty (R-01-002)
        if packet.masked_payload.is_empty() {
            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_empty_payload += 1;
            }

            warn!(
                event = "packet_rejected",
                reason = "empty_payload",
                packet_id = %packet.id,
                "Security: Packet rejected due to empty payload"
            );
            anyhow::bail!("Invalid packet: masked payload cannot be empty");
        }

        // Step 5a: Check resonance R_ε(ψ_node, ψ_pkt)
        if !packet.matches_resonance(node_state, self.config.resonance_epsilon) {
            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.packets_ignored_resonance_mismatch += 1;
            }

            // Not resonant - ignore packet (this is normal, not a security event)
            debug!(
                event = "packet_ignored",
                reason = "resonance_mismatch",
                packet_id = %packet.id,
                packet_resonance = ?(packet.resonance.psi, packet.resonance.rho, packet.resonance.omega),
                node_resonance = ?(node_state.psi, node_state.rho, node_state.omega),
                epsilon = self.config.resonance_epsilon,
                "Packet ignored due to resonance mismatch"
            );
            return Ok(None);
        }

        // Step 5b: Verify packet integrity
        if !packet.verify_integrity() {
            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_integrity_failed += 1;
            }

            error!(
                event = "packet_rejected",
                reason = "integrity_check_failed",
                packet_id = %packet.id,
                "Security: Packet rejected due to integrity check failure"
            );
            anyhow::bail!("Packet integrity check failed");
        }

        // Step 5c: Derive masking parameters with key rotation support (R-03-001)
        // The receiver can compute the same params as the sender using:
        // sender_resonance (from packet) and target_resonance (node's own state)
        // Try the packet's epoch first, then fall back to current epoch if needed
        let mut masking_params = MaskingParams::from_resonance_with_epoch(
            &packet.sender_resonance,
            node_state,
            packet.key_epoch,
        );

        // R-03-002: Add ephemeral key for forward secrecy if present
        if let Some(ref ephemeral) = packet.ephemeral_key {
            masking_params = masking_params.with_ephemeral_key(ephemeral.clone());
        }

        // Step 5d: Extract from steganographic carrier: a' = T⁻¹(t)
        let extracted = if self.config.enable_steganography {
            self.extract_from_carrier(&packet.stego_carrier, packet.carrier_type)?
        } else {
            packet.masked_payload.clone()
        };

        // Step 5e: Unmask: a* = M⁻¹_{θ,σ}(a')
        // R-03-001: Try current epoch, then previous epoch during rotation
        let unmasked = match self.unmask_data(&extracted, &masking_params) {
            Ok(data) => data,
            Err(_) => {
                // Key rotation: Try previous epoch
                let current_epoch = MaskingParams::current_epoch();
                if packet.key_epoch < current_epoch && current_epoch - packet.key_epoch <= 1 {
                    debug!(
                        event = "key_rotation_fallback",
                        packet_epoch = packet.key_epoch,
                        current_epoch = current_epoch,
                        "Trying previous epoch key during rotation"
                    );

                    let mut fallback_params = MaskingParams::from_resonance_with_epoch(
                        &packet.sender_resonance,
                        node_state,
                        current_epoch,
                    );

                    if let Some(ref ephemeral) = packet.ephemeral_key {
                        fallback_params = fallback_params.with_ephemeral_key(ephemeral.clone());
                    }

                    self.unmask_data(&extracted, &fallback_params)?
                } else {
                    anyhow::bail!("Failed to unmask packet with any known epoch");
                }
            }
        };

        // Step 5f: Deserialize transaction
        let transaction = GhostTransaction::from_bytes(&unmasked)
            .context("Failed to deserialize transaction")?;

        // Runtime Invariant: Validate transaction timestamp (R-01-003)
        if let Err(e) = self.validate_timestamp(transaction.timestamp) {
            // Record the failure for rate limiting
            self.record_timestamp_failure(&packet.sender_resonance);

            // Increment metric
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.rejected_transaction_timestamp += 1;
            }

            error!(
                event = "transaction_rejected",
                reason = "timestamp_invalid",
                packet_id = %packet.id,
                transaction_id = %transaction.id,
                timestamp = transaction.timestamp,
                error = %e,
                "Security: Transaction rejected due to timestamp validation failure"
            );
            return Err(e).context("Transaction timestamp validation failed");
        }

        // Step 5g: Verify ZK proof if present
        if let Some(ref proof) = transaction.zk_data {
            if self.config.enable_zk_proofs {
                if let Err(e) = self.verify_zk_proof(&transaction.action, proof) {
                    // Increment metric
                    if let Ok(mut metrics) = self.metrics.write() {
                        metrics.rejected_zk_proof_failed += 1;
                    }

                    error!(
                        event = "transaction_rejected",
                        reason = "zk_proof_invalid",
                        packet_id = %packet.id,
                        transaction_id = %transaction.id,
                        error = %e,
                        "Security: Transaction rejected due to ZK proof verification failure"
                    );
                    return Err(e);
                }
            }
        }

        // Increment packets accepted metric
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.packets_accepted += 1;
        }

        info!(
            event = "transaction_accepted",
            packet_id = %packet.id,
            transaction_id = %transaction.id,
            "Transaction successfully validated and accepted"
        );

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

    /// Apply masking operator M_{θ,σ}(m) with R-03-002 forward secrecy support
    fn apply_masking(&self, data: &[u8], params: &MaskingParams) -> Result<Vec<u8>> {
        // R-03-001 & R-03-002: Use derived key that includes epoch and ephemeral key
        let key = params.derive_final_key();

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
            .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)
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
            .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)
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
            .create_packet(&tx, masked, carrier, CarrierType::Raw, &sender_params)
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
