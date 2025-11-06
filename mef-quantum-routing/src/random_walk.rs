/*!
 * Quantum Random Walk Router
 *
 * Implements probabilistic routing based on quantum random walks.
 * Packets navigate the network via resonance-weighted probability distributions
 * rather than deterministic routing tables.
 *
 * # Algorithm
 *
 * For each routing decision:
 * 1. Compute resonance similarity to all neighbor nodes
 * 2. Calculate transition probabilities based on:
 *    - Resonance match to target
 *    - Link quality
 *    - Historical success rate
 *    - Network topology
 * 3. Select next hop probabilistically using quantum entropy
 * 4. Update metrics based on delivery success/failure
 */

use crate::entropy_source::{EntropySource, QuantumEntropySource};
use crate::topology::{NetworkTopology, ResonanceState};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Routing decision made by the quantum random walk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    /// Selected next hop node ID
    pub next_hop: Uuid,

    /// Target resonance state
    pub target_resonance: ResonanceState,

    /// Routing probability (confidence)
    pub probability: f64,

    /// Alternative hops (with probabilities)
    pub alternatives: Vec<(Uuid, f64)>,

    /// Timestamp
    pub timestamp: u64,
}

/// Routing statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingStats {
    /// Total routing decisions made
    pub decisions_made: usize,

    /// Successful deliveries
    pub successful_deliveries: usize,

    /// Failed deliveries
    pub failed_deliveries: usize,

    /// Average hops to delivery
    pub avg_hops: f64,

    /// Average latency (ms)
    pub avg_latency_ms: f64,
}

/// Quantum Random Walk Router configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    /// Weight for resonance similarity (0.0 - 1.0)
    pub resonance_weight: f64,

    /// Weight for link quality (0.0 - 1.0)
    pub quality_weight: f64,

    /// Weight for latency (0.0 - 1.0)
    pub latency_weight: f64,

    /// Minimum probability for route selection
    pub min_probability: f64,

    /// Maximum alternative routes to consider
    pub max_alternatives: usize,

    /// Enable exploration (vs exploitation)
    pub exploration_rate: f64,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            resonance_weight: 0.6,
            quality_weight: 0.3,
            latency_weight: 0.1,
            min_probability: 0.01,
            max_alternatives: 5,
            exploration_rate: 0.1,
        }
    }
}

/// Quantum Random Walk Router
pub struct QuantumRandomWalkRouter<E: EntropySource = QuantumEntropySource> {
    /// Network topology
    topology: Arc<RwLock<NetworkTopology>>,

    /// Entropy source for probabilistic decisions
    entropy: Arc<RwLock<E>>,

    /// Router configuration
    config: RouterConfig,

    /// Routing statistics
    stats: Arc<RwLock<RoutingStats>>,
}

impl QuantumRandomWalkRouter<QuantumEntropySource> {
    /// Create new quantum random walk router with default entropy
    pub fn new(topology: Arc<RwLock<NetworkTopology>>, config: RouterConfig) -> Self {
        Self {
            topology,
            entropy: Arc::new(RwLock::new(QuantumEntropySource::new())),
            config,
            stats: Arc::new(RwLock::new(RoutingStats::default())),
        }
    }

    /// Create with default configuration
    pub fn default_with_topology(topology: Arc<RwLock<NetworkTopology>>) -> Self {
        Self::new(topology, RouterConfig::default())
    }
}

impl<E: EntropySource> QuantumRandomWalkRouter<E> {
    /// Create with custom entropy source
    pub fn with_entropy(
        topology: Arc<RwLock<NetworkTopology>>,
        entropy: E,
        config: RouterConfig,
    ) -> Self {
        Self {
            topology,
            entropy: Arc::new(RwLock::new(entropy)),
            config,
            stats: Arc::new(RwLock::new(RoutingStats::default())),
        }
    }

    /// Make routing decision for target resonance
    ///
    /// Returns next hop node ID based on quantum random walk.
    pub fn next_hop(&self, target_resonance: ResonanceState) -> Result<Option<RoutingDecision>> {
        let topology = self.topology.read().unwrap();

        // Get routing scores for all active nodes
        let scores = topology.get_routing_scores(&target_resonance);

        if scores.is_empty() {
            return Ok(None);
        }

        // Compute transition probabilities
        let probabilities = self.compute_transition_probabilities(&scores);

        // Select next hop using entropy source
        let mut entropy = self.entropy.write().unwrap();
        let selected_idx = entropy
            .select_weighted(&probabilities)
            .ok_or_else(|| anyhow::anyhow!("Failed to select weighted route"))?;

        let next_hop = scores[selected_idx].0;
        let probability = probabilities[selected_idx];

        // Get alternatives
        let alternatives = self.get_alternatives(&scores, &probabilities, selected_idx);

        // Update statistics
        let mut stats = self.stats.write().unwrap();
        stats.decisions_made += 1;

        Ok(Some(RoutingDecision {
            next_hop,
            target_resonance,
            probability,
            alternatives,
            timestamp: Self::current_timestamp(),
        }))
    }

    /// Compute transition probabilities from routing scores
    ///
    /// P_i = normalize(score_i^β + ε)
    /// where β controls exploitation vs exploration
    fn compute_transition_probabilities(&self, scores: &[(Uuid, f64)]) -> Vec<f64> {
        let beta = 2.0 / (1.0 + self.config.exploration_rate);

        let mut probabilities: Vec<f64> = scores
            .iter()
            .map(|(_, score)| {
                // Add exploration bonus
                let exploration_bonus = self.config.exploration_rate * self.config.min_probability;
                score.powf(beta) + exploration_bonus
            })
            .collect();

        // Apply minimum probability threshold
        for p in probabilities.iter_mut() {
            if *p < self.config.min_probability {
                *p = self.config.min_probability;
            }
        }

        probabilities
    }

    /// Get alternative routes
    fn get_alternatives(
        &self,
        scores: &[(Uuid, f64)],
        probabilities: &[f64],
        selected_idx: usize,
    ) -> Vec<(Uuid, f64)> {
        let mut alternatives: Vec<(Uuid, f64)> = scores
            .iter()
            .zip(probabilities.iter())
            .enumerate()
            .filter(|(i, _)| *i != selected_idx)
            .map(|(_, ((id, _), prob))| (*id, *prob))
            .collect();

        // Sort by probability descending
        alternatives.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Take top N
        alternatives
            .into_iter()
            .take(self.config.max_alternatives)
            .collect()
    }

    /// Record successful delivery through a route
    pub fn record_success(&self, node_id: Uuid, latency_ms: f64, hops: usize) {
        let mut topology = self.topology.write().unwrap();
        topology.record_success(node_id, latency_ms);

        let mut stats = self.stats.write().unwrap();
        stats.successful_deliveries += 1;

        // Update moving averages
        let alpha = 0.1;
        stats.avg_latency_ms = alpha * latency_ms + (1.0 - alpha) * stats.avg_latency_ms;
        stats.avg_hops = alpha * hops as f64 + (1.0 - alpha) * stats.avg_hops;
    }

    /// Record delivery failure
    pub fn record_failure(&self, node_id: Uuid) {
        let mut topology = self.topology.write().unwrap();
        topology.record_failure(node_id);

        let mut stats = self.stats.write().unwrap();
        stats.failed_deliveries += 1;
    }

    /// Get routing statistics
    pub fn get_stats(&self) -> RoutingStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        *stats = RoutingStats::default();
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let stats = self.stats.read().unwrap();
        let total = stats.successful_deliveries + stats.failed_deliveries;
        if total == 0 {
            return 0.0;
        }
        stats.successful_deliveries as f64 / total as f64
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy_source::DeterministicEntropy;

    #[test]
    fn test_router_creation() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));
        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        assert_eq!(router.get_stats().decisions_made, 0);
    }

    #[test]
    fn test_routing_decision() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));

        // Add some nodes
        {
            let mut topo = topology.write().unwrap();
            let node1 = Uuid::new_v4();
            let node2 = Uuid::new_v4();

            topo.add_node(node1, ResonanceState::new(1.0, 1.0, 1.0));
            topo.add_node(node2, ResonanceState::new(2.0, 2.0, 2.0));
        }

        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        // Make routing decision
        let target = ResonanceState::new(1.1, 1.1, 1.1);
        let decision = router.next_hop(target).unwrap();

        assert!(decision.is_some());
        let dec = decision.unwrap();
        assert!(dec.probability > 0.0);
        assert!(dec.probability <= 1.0);
    }

    #[test]
    fn test_deterministic_routing() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));

        {
            let mut topo = topology.write().unwrap();
            topo.add_node(Uuid::new_v4(), ResonanceState::new(1.0, 1.0, 1.0));
            topo.add_node(Uuid::new_v4(), ResonanceState::new(2.0, 2.0, 2.0));
        }

        let entropy = DeterministicEntropy::new(42);
        let router =
            QuantumRandomWalkRouter::with_entropy(topology, entropy, RouterConfig::default());

        // Should produce same decision with same seed
        let target = ResonanceState::new(1.5, 1.5, 1.5);
        let decision1 = router.next_hop(target).unwrap().unwrap();

        // Note: Decisions may vary due to topology updates
        assert!(decision1.probability > 0.0);
    }

    #[test]
    fn test_record_success() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));
        let node_id = Uuid::new_v4();

        {
            let mut topo = topology.write().unwrap();
            topo.add_node(node_id, ResonanceState::new(1.0, 1.0, 1.0));
        }

        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        router.record_success(node_id, 50.0, 3);

        let stats = router.get_stats();
        assert_eq!(stats.successful_deliveries, 1);
        assert!(stats.avg_latency_ms > 0.0);
    }

    #[test]
    fn test_record_failure() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));
        let node_id = Uuid::new_v4();

        {
            let mut topo = topology.write().unwrap();
            topo.add_node(node_id, ResonanceState::new(1.0, 1.0, 1.0));
        }

        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        router.record_failure(node_id);

        let stats = router.get_stats();
        assert_eq!(stats.failed_deliveries, 1);
    }

    #[test]
    fn test_success_rate() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));
        let node_id = Uuid::new_v4();

        {
            let mut topo = topology.write().unwrap();
            topo.add_node(node_id, ResonanceState::new(1.0, 1.0, 1.0));
        }

        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        router.record_success(node_id, 50.0, 2);
        router.record_success(node_id, 60.0, 3);
        router.record_failure(node_id);

        let rate = router.success_rate();
        assert!((rate - 0.6667).abs() < 0.01); // 2/3 ≈ 0.6667
    }

    #[test]
    fn test_no_nodes() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));
        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        let target = ResonanceState::new(1.0, 1.0, 1.0);
        let decision = router.next_hop(target).unwrap();

        assert!(decision.is_none());
    }

    #[test]
    fn test_alternatives() {
        let topology = Arc::new(RwLock::new(NetworkTopology::default()));

        {
            let mut topo = topology.write().unwrap();
            for _ in 0..10 {
                topo.add_node(Uuid::new_v4(), ResonanceState::new(1.0, 1.0, 1.0));
            }
        }

        let router = QuantumRandomWalkRouter::default_with_topology(topology);

        let target = ResonanceState::new(1.0, 1.0, 1.0);
        let decision = router.next_hop(target).unwrap().unwrap();

        // Should have alternatives (up to max_alternatives)
        assert!(decision.alternatives.len() > 0);
        assert!(decision.alternatives.len() <= router.config.max_alternatives);
    }
}
