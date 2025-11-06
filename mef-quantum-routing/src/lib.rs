/*!
 * MEF Quantum Routing - Quantum Random Walk Routing
 *
 * Implements quantum-inspired routing based on random walks where packets
 * navigate the network based on resonance-weighted probabilities rather than
 * fixed routing tables.
 *
 * # Principle
 *
 * Packets perform a quantum random walk on the resonance tensor field:
 *
 * ```text
 * P_next = f(Resonanz, Entropie, lokale Topologie)
 * ```
 *
 * Key Features:
 * - No fixed routing tables
 * - Probabilistic path selection based on resonance
 * - Quantum entropy for true randomness
 * - Self-organizing network topology
 * - Fault-tolerant routing (no single point of failure)
 *
 * # Integration
 *
 * Works with mef-ghost-network for addressless packet delivery:
 * - Ghost packets carry resonance states
 * - Routing decisions based on resonance similarity
 * - Adaptive to network topology changes
 */

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod random_walk;
pub mod entropy_source;
pub mod topology;

pub use random_walk::{QuantumRandomWalkRouter, RoutingDecision, RoutingStats};
pub use entropy_source::{EntropySource, QuantumEntropySource};
pub use topology::{NetworkTopology, NodeMetrics, TopologyView};
