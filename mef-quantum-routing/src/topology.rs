/*!
 * Network Topology Management
 *
 * Manages the local view of network topology for routing decisions.
 * Topology is learned organically through packet observation rather than
 * explicit routing protocols.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Resonance state (from mef-ghost-network)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResonanceState {
    /// Psi dimension
    pub psi: f64,
    /// Rho dimension
    pub rho: f64,
    /// Omega dimension
    pub omega: f64,
}

impl ResonanceState {
    /// Create new resonance state
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    /// Calculate distance to another state
    pub fn distance_to(&self, other: &ResonanceState) -> f64 {
        let dpsi = self.psi - other.psi;
        let drho = self.rho - other.rho;
        let domega = self.omega - other.omega;
        (dpsi * dpsi + drho * drho + domega * domega).sqrt()
    }
}

/// Node metrics for routing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    /// Node ID
    pub node_id: Uuid,

    /// Node resonance state
    pub resonance: ResonanceState,

    /// Last seen timestamp
    pub last_seen: u64,

    /// Successful packet deliveries through this node
    pub success_count: usize,

    /// Failed packet deliveries
    pub failure_count: usize,

    /// Average latency (milliseconds)
    pub avg_latency_ms: f64,

    /// Link quality (0.0 - 1.0)
    pub link_quality: f64,

    /// Hop distance estimate
    pub hop_distance: usize,
}

impl NodeMetrics {
    /// Create new node metrics
    pub fn new(node_id: Uuid, resonance: ResonanceState) -> Self {
        Self {
            node_id,
            resonance,
            last_seen: Self::current_timestamp(),
            success_count: 0,
            failure_count: 0,
            avg_latency_ms: 0.0,
            link_quality: 1.0,
            hop_distance: 1,
        }
    }

    /// Update metrics after successful delivery
    pub fn record_success(&mut self, latency_ms: f64) {
        self.success_count += 1;
        self.last_seen = Self::current_timestamp();

        // Update average latency (exponential moving average)
        let alpha = 0.2;
        self.avg_latency_ms = alpha * latency_ms + (1.0 - alpha) * self.avg_latency_ms;

        // Update link quality
        self.update_link_quality();
    }

    /// Record delivery failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_seen = Self::current_timestamp();
        self.update_link_quality();
    }

    /// Update link quality based on success/failure ratio
    fn update_link_quality(&mut self) {
        let total = self.success_count + self.failure_count;
        if total > 0 {
            self.link_quality = self.success_count as f64 / total as f64;
        }
    }

    /// Check if node is recently active
    pub fn is_active(&self, timeout_seconds: u64) -> bool {
        let now = Self::current_timestamp();
        now < self.last_seen + timeout_seconds
    }

    /// Get current timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Calculate routing score for this node
    pub fn routing_score(&self, target_resonance: &ResonanceState) -> f64 {
        // Score based on:
        // 1. Resonance similarity to target (higher = better)
        // 2. Link quality (higher = better)
        // 3. Latency (lower = better)
        // 4. Hop distance (lower = better)

        let resonance_distance = self.resonance.distance_to(target_resonance);
        let resonance_score = 1.0 / (1.0 + resonance_distance);

        let latency_score = 1.0 / (1.0 + self.avg_latency_ms / 100.0);
        let hop_score = 1.0 / (1.0 + self.hop_distance as f64);

        // Weighted combination
        0.5 * resonance_score + 0.3 * self.link_quality + 0.1 * latency_score + 0.1 * hop_score
    }
}

/// Network topology view
#[derive(Debug, Clone)]
pub struct NetworkTopology {
    /// Known nodes and their metrics
    nodes: HashMap<Uuid, NodeMetrics>,

    /// Node timeout (seconds)
    node_timeout: u64,

    /// Maximum nodes to track
    max_nodes: usize,
}

impl NetworkTopology {
    /// Create new network topology
    pub fn new(node_timeout: u64, max_nodes: usize) -> Self {
        Self {
            nodes: HashMap::new(),
            node_timeout,
            max_nodes,
        }
    }

    /// Create with default settings
    pub fn default() -> Self {
        Self::new(
            300,  // 5 minute timeout
            1000, // Track up to 1000 nodes
        )
    }

    /// Add or update node
    pub fn add_node(&mut self, node_id: Uuid, resonance: ResonanceState) {
        if let Some(metrics) = self.nodes.get_mut(&node_id) {
            // Update existing node
            metrics.last_seen = NodeMetrics::current_timestamp();
            metrics.resonance = resonance;
        } else {
            // Add new node
            if self.nodes.len() >= self.max_nodes {
                // Remove oldest inactive node
                self.cleanup_old_nodes();
            }

            self.nodes
                .insert(node_id, NodeMetrics::new(node_id, resonance));
        }
    }

    /// Record successful packet delivery through node
    pub fn record_success(&mut self, node_id: Uuid, latency_ms: f64) {
        if let Some(metrics) = self.nodes.get_mut(&node_id) {
            metrics.record_success(latency_ms);
        }
    }

    /// Record delivery failure
    pub fn record_failure(&mut self, node_id: Uuid) {
        if let Some(metrics) = self.nodes.get_mut(&node_id) {
            metrics.record_failure();
        }
    }

    /// Get node metrics
    pub fn get_node(&self, node_id: &Uuid) -> Option<&NodeMetrics> {
        self.nodes.get(node_id)
    }

    /// Get all active nodes
    pub fn get_active_nodes(&self) -> Vec<&NodeMetrics> {
        self.nodes
            .values()
            .filter(|n| n.is_active(self.node_timeout))
            .collect()
    }

    /// Find best next hop for target resonance
    pub fn find_best_hop(&self, target_resonance: &ResonanceState) -> Option<Uuid> {
        self.get_active_nodes()
            .iter()
            .max_by(|a, b| {
                let score_a = a.routing_score(target_resonance);
                let score_b = b.routing_score(target_resonance);
                score_a.partial_cmp(&score_b).unwrap()
            })
            .map(|n| n.node_id)
    }

    /// Find nodes within resonance window
    pub fn find_resonant_nodes(&self, target: &ResonanceState, epsilon: f64) -> Vec<&NodeMetrics> {
        self.get_active_nodes()
            .into_iter()
            .filter(|n| n.resonance.distance_to(target) < epsilon)
            .collect()
    }

    /// Get routing scores for all active nodes
    pub fn get_routing_scores(&self, target_resonance: &ResonanceState) -> Vec<(Uuid, f64)> {
        self.get_active_nodes()
            .iter()
            .map(|n| (n.node_id, n.routing_score(target_resonance)))
            .collect()
    }

    /// Cleanup inactive nodes
    pub fn cleanup_old_nodes(&mut self) -> usize {
        let to_remove: Vec<Uuid> = self
            .nodes
            .iter()
            .filter(|(_, n)| !n.is_active(self.node_timeout))
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove.iter() {
            self.nodes.remove(id);
        }

        to_remove.len()
    }

    /// Get total node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get active node count
    pub fn active_node_count(&self) -> usize {
        self.get_active_nodes().len()
    }
}

/// Topology view for routing decisions
pub struct TopologyView<'a> {
    /// Reference to topology
    topology: &'a NetworkTopology,

    /// Target resonance
    target_resonance: ResonanceState,

    /// Cached routing scores
    scores: Vec<(Uuid, f64)>,
}

impl<'a> TopologyView<'a> {
    /// Create new topology view
    pub fn new(topology: &'a NetworkTopology, target_resonance: ResonanceState) -> Self {
        let scores = topology.get_routing_scores(&target_resonance);
        Self {
            topology,
            target_resonance,
            scores,
        }
    }

    /// Get candidate nodes for routing
    pub fn get_candidates(&self) -> Vec<(Uuid, f64)> {
        self.scores.clone()
    }

    /// Get best candidate
    pub fn get_best_candidate(&self) -> Option<(Uuid, f64)> {
        self.scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .copied()
    }

    /// Get top N candidates
    pub fn get_top_candidates(&self, n: usize) -> Vec<(Uuid, f64)> {
        let mut sorted = self.scores.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_metrics() {
        let node_id = Uuid::new_v4();
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let mut metrics = NodeMetrics::new(node_id, resonance);

        assert_eq!(metrics.success_count, 0);
        assert_eq!(metrics.failure_count, 0);

        metrics.record_success(100.0);
        assert_eq!(metrics.success_count, 1);
        assert_eq!(metrics.avg_latency_ms, 20.0); // alpha * 100.0

        metrics.record_failure();
        assert_eq!(metrics.failure_count, 1);
        assert_eq!(metrics.link_quality, 0.5);
    }

    #[test]
    fn test_routing_score() {
        let node_id = Uuid::new_v4();
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);
        let metrics = NodeMetrics::new(node_id, resonance);

        let target = ResonanceState::new(1.0, 1.0, 1.0);
        let score = metrics.routing_score(&target);

        // Perfect match should give high score
        assert!(score > 0.8);
    }

    #[test]
    fn test_network_topology() {
        let mut topology = NetworkTopology::default();

        let node_id = Uuid::new_v4();
        let resonance = ResonanceState::new(1.0, 1.0, 1.0);

        topology.add_node(node_id, resonance);
        assert_eq!(topology.node_count(), 1);

        let node = topology.get_node(&node_id).unwrap();
        assert_eq!(node.node_id, node_id);
    }

    #[test]
    fn test_find_best_hop() {
        let mut topology = NetworkTopology::default();

        // Add nodes with different resonances
        let node1 = Uuid::new_v4();
        let node2 = Uuid::new_v4();

        topology.add_node(node1, ResonanceState::new(1.0, 1.0, 1.0));
        topology.add_node(node2, ResonanceState::new(5.0, 5.0, 5.0));

        // Target close to node1
        let target = ResonanceState::new(1.1, 1.1, 1.1);
        let best = topology.find_best_hop(&target).unwrap();

        assert_eq!(best, node1);
    }

    #[test]
    fn test_find_resonant_nodes() {
        let mut topology = NetworkTopology::default();

        let node1 = Uuid::new_v4();
        let node2 = Uuid::new_v4();

        topology.add_node(node1, ResonanceState::new(1.0, 1.0, 1.0));
        topology.add_node(node2, ResonanceState::new(5.0, 5.0, 5.0));

        let target = ResonanceState::new(1.05, 1.05, 1.05);
        let resonant = topology.find_resonant_nodes(&target, 0.2);

        assert_eq!(resonant.len(), 1);
        assert_eq!(resonant[0].node_id, node1);
    }

    #[test]
    fn test_topology_view() {
        let mut topology = NetworkTopology::default();

        let node_id = Uuid::new_v4();
        topology.add_node(node_id, ResonanceState::new(1.0, 1.0, 1.0));

        let target = ResonanceState::new(1.0, 1.0, 1.0);
        let view = TopologyView::new(&topology, target);

        let best = view.get_best_candidate();
        assert!(best.is_some());
        assert_eq!(best.unwrap().0, node_id);
    }

    #[test]
    fn test_cleanup() {
        let mut topology = NetworkTopology::new(0, 1000); // 0 second timeout

        let node_id = Uuid::new_v4();
        topology.add_node(node_id, ResonanceState::new(1.0, 1.0, 1.0));

        // Immediately expire
        std::thread::sleep(std::time::Duration::from_millis(10));

        let removed = topology.cleanup_old_nodes();
        assert_eq!(removed, 1);
        assert_eq!(topology.node_count(), 0);
    }
}
