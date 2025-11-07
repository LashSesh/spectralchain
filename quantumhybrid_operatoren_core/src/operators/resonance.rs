/*!
 * Resonance Operator (R_ε)
 *
 * ## Mathematische Formel
 * ```text
 * R_ε(ψ_node, ψ_pkt) = 1 if d(ψ_node, ψ_pkt) < ε, else 0
 * d(ψ₁, ψ₂) = √[(ψ₁-ψ₂)² + (ρ₁-ρ₂)² + (ω₁-ω₂)²]
 * ```
 *
 * Wobei:
 * - `ψ` (Psi): Semantic Density / Activation
 * - `ρ` (Rho): Coherence
 * - `ω` (Omega): Rhythm / Oscillation
 * - `ε` (Epsilon): Resonanzfenster-Breite
 *
 * ## Eigenschaften
 * - **Multidimensionale Resonanz**: 3D-Tripolar-Zustand (ψ, ρ, ω)
 * - **Adaptive Fenster**: Konfigurierbare Epsilon-Werte
 * - **Gewichtete Distanz**: Individuelle Gewichte für jede Dimension
 *
 * ## Erweiterte Funktionen
 * - Resonanzstärke (0.0 - 1.0)
 * - Kollektiv-Resonanz für Gruppenentscheidungen
 * - Resonante Node-Discovery
 *
 * ## Use Cases
 * - Addressless routing im Ghost Network
 * - Consensus-Finding via resonance alignment
 * - Privacy-preserving node discovery
 * - Decentralized decision making
 *
 * ## Beispiel
 * ```rust
 * use quantumhybrid_operatoren_core::operators::resonance::{
 *     ResonanceOperator, ResonanceState, ResonanceWindow
 * };
 *
 * let operator = ResonanceOperator::new();
 * let window = ResonanceWindow::standard();
 *
 * let node_state = ResonanceState::new(1.0, 0.8, 0.5);
 * let packet_state = ResonanceState::new(1.05, 0.82, 0.53);
 *
 * let is_resonant = operator.is_resonant(&node_state, &packet_state, &window);
 * let strength = operator.resonance_strength(&node_state, &packet_state, &window);
 * ```
 */

use crate::core::{euclidean_distance, QuantumOperator, ResonanceOperator as ResonanceOp};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Resonanzfenster-Parameter
///
/// Definiert die Breite des Resonanzfensters und optionale Gewichte
/// für multidimensionale Resonanz.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceWindow {
    /// Epsilon (ε) - Resonanzfenster-Breite
    /// Kleinere Werte = engeres Fenster = höhere Selektivität
    pub epsilon: f64,

    /// Gewichte für multidimensionale Resonanz [psi, rho, omega]
    /// Standard: [1.0, 1.0, 1.0] (gleichgewichtig)
    pub weights: [f64; 3],
}

impl ResonanceWindow {
    /// Erstelle neues Resonanzfenster mit Standard-Gewichten
    pub fn new(epsilon: f64) -> Self {
        Self {
            epsilon,
            weights: [1.0, 1.0, 1.0],
        }
    }

    /// Erstelle Resonanzfenster mit custom Gewichten
    pub fn with_weights(epsilon: f64, weights: [f64; 3]) -> Self {
        Self { epsilon, weights }
    }

    /// Standard-Resonanzfenster (ε = 0.1)
    pub fn standard() -> Self {
        Self::new(0.1)
    }

    /// Enges Resonanzfenster (ε = 0.01) - hohe Selektivität
    pub fn narrow() -> Self {
        Self::new(0.01)
    }

    /// Weites Resonanzfenster (ε = 0.5) - niedrige Selektivität
    pub fn wide() -> Self {
        Self::new(0.5)
    }

    /// Sehr enges Resonanzfenster (ε = 0.001) - extreme Selektivität
    pub fn ultra_narrow() -> Self {
        Self::new(0.001)
    }
}

impl Default for ResonanceWindow {
    fn default() -> Self {
        Self::standard()
    }
}

/// Resonanz-Zustand (ψ, ρ, ω)
///
/// Repräsentiert einen 3D-Tripolar-Zustand für Resonanz-Matching.
/// Entspricht Gabriel Cell State aus Infinity Ledger.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ResonanceState {
    /// Psi (ψ) - Semantic Density / Activation
    /// Typischer Bereich: [0.0, 1.0]
    pub psi: f64,

    /// Rho (ρ) - Coherence
    /// Typischer Bereich: [0.0, 1.0]
    pub rho: f64,

    /// Omega (ω) - Rhythm / Oscillation
    /// Typischer Bereich: [0.0, 1.0] oder [-π, π]
    pub omega: f64,
}

impl ResonanceState {
    /// Erstelle neuen Resonanz-Zustand
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    /// Konvertiere zu Vektor [ψ, ρ, ω]
    pub fn as_vector(&self) -> [f64; 3] {
        [self.psi, self.rho, self.omega]
    }

    /// Erstelle aus Vektor [ψ, ρ, ω]
    pub fn from_vector(v: [f64; 3]) -> Self {
        Self {
            psi: v[0],
            rho: v[1],
            omega: v[2],
        }
    }

    /// Berechne euklidische Distanz zu anderem Zustand
    ///
    /// d(ψ₁, ψ₂) = √[(ψ₁-ψ₂)² + (ρ₁-ρ₂)² + (ω₁-ω₂)²]
    pub fn distance(&self, other: &Self) -> f64 {
        euclidean_distance(&self.as_vector(), &other.as_vector())
    }

    /// Berechne gewichtete Distanz
    ///
    /// d_w(ψ₁, ψ₂) = √[w_ψ(ψ₁-ψ₂)² + w_ρ(ρ₁-ρ₂)² + w_ω(ω₁-ω₂)²]
    pub fn weighted_distance(&self, other: &Self, weights: &[f64; 3]) -> f64 {
        let dpsi = (self.psi - other.psi) * weights[0];
        let drho = (self.rho - other.rho) * weights[1];
        let domega = (self.omega - other.omega) * weights[2];
        (dpsi * dpsi + drho * drho + domega * domega).sqrt()
    }

    /// Erstelle Null-Zustand (0, 0, 0)
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Erstelle Einheits-Zustand (1, 1, 1)
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Erstelle zufälligen Zustand
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }
}

/// Resonance Operator
///
/// Implementiert R_ε(ψ_node, ψ_pkt) für Resonanz-basiertes Routing
/// und Entscheidungsfindung.
#[derive(Debug, Clone)]
pub struct ResonanceOperator;

impl ResonanceOperator {
    /// Erstelle neuen Resonance Operator
    pub fn new() -> Self {
        Self
    }

    /// Prüfe ob zwei Zustände resonant sind
    ///
    /// Gibt `true` zurück wenn d(ψ₁, ψ₂) < ε
    pub fn is_resonant(
        &self,
        node_state: &ResonanceState,
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> bool {
        let distance = node_state.weighted_distance(packet_state, &window.weights);
        distance < window.epsilon
    }

    /// Berechne Resonanzstärke
    ///
    /// Gibt einen Wert im Bereich [0.0, 1.0] zurück:
    /// - 0.0 = keine Resonanz (d ≥ ε)
    /// - 1.0 = perfekte Resonanz (d = 0)
    /// - Linear zwischen 0 und ε
    pub fn resonance_strength(
        &self,
        node_state: &ResonanceState,
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> f64 {
        let distance = node_state.weighted_distance(packet_state, &window.weights);
        if distance >= window.epsilon {
            0.0
        } else {
            1.0 - (distance / window.epsilon)
        }
    }

    /// Prüfe Kollektiv-Resonanz (Gruppen-Entscheidung)
    ///
    /// Gibt `true` zurück wenn mindestens `threshold` Prozent
    /// der Nodes resonant sind.
    ///
    /// # Arguments
    /// * `node_states` - Liste aller Node-Zustände
    /// * `packet_state` - Paket-Zustand zum Vergleich
    /// * `window` - Resonanzfenster
    /// * `threshold` - Schwellwert (0.0 - 1.0), z.B. 0.66 für 2/3 Mehrheit
    pub fn collective_resonance(
        &self,
        node_states: &[ResonanceState],
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
        threshold: f64,
    ) -> bool {
        if node_states.is_empty() {
            return false;
        }

        let resonant_count = node_states
            .iter()
            .filter(|state| self.is_resonant(state, packet_state, window))
            .count();

        let resonant_fraction = resonant_count as f64 / node_states.len() as f64;
        resonant_fraction >= threshold
    }

    /// Finde alle resonanten Nodes
    ///
    /// # Returns
    /// Liste der Node-IDs die mit dem Paket-Zustand resonant sind
    pub fn find_resonant_nodes(
        &self,
        node_states: &[(usize, ResonanceState)],
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> Vec<usize> {
        node_states
            .iter()
            .filter(|(_, state)| self.is_resonant(state, packet_state, window))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Berechne durchschnittliche Resonanzstärke über alle Nodes
    pub fn average_resonance(
        &self,
        node_states: &[ResonanceState],
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> f64 {
        if node_states.is_empty() {
            return 0.0;
        }

        let total: f64 = node_states
            .iter()
            .map(|state| self.resonance_strength(state, packet_state, window))
            .sum();

        total / node_states.len() as f64
    }

    /// Finde den Node mit der stärksten Resonanz
    pub fn find_strongest_resonance(
        &self,
        node_states: &[(usize, ResonanceState)],
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> Option<(usize, f64)> {
        node_states
            .iter()
            .map(|(id, state)| {
                let strength = self.resonance_strength(state, packet_state, window);
                (*id, strength)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    }
}

impl Default for ResonanceOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Input für Resonanz-Check
#[derive(Debug, Clone)]
pub struct ResonanceInput {
    pub node_state: ResonanceState,
    pub packet_state: ResonanceState,
}

impl QuantumOperator for ResonanceOperator {
    type Input = ResonanceInput;
    type Output = bool;
    type Params = ResonanceWindow;

    fn apply(&self, input: Self::Input, params: &Self::Params) -> Result<Self::Output> {
        Ok(self.is_resonant(&input.node_state, &input.packet_state, params))
    }

    fn name(&self) -> &str {
        "ResonanceOperator"
    }

    fn description(&self) -> &str {
        "Multidimensional resonance matching for addressless routing"
    }

    fn formula(&self) -> &str {
        "R_ε(ψ₁, ψ₂) = 1 if d(ψ₁, ψ₂) < ε, else 0"
    }
}

impl ResonanceOp for ResonanceOperator {
    type ResonanceState = ResonanceState;

    fn compute_resonance(
        &self,
        state1: &Self::ResonanceState,
        state2: &Self::ResonanceState,
    ) -> f64 {
        let window = ResonanceWindow::standard();
        self.resonance_strength(state1, state2, &window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_resonance() {
        let op = ResonanceOperator::new();
        let state = ResonanceState::new(1.0, 1.0, 1.0);
        let window = ResonanceWindow::standard();

        assert!(op.is_resonant(&state, &state, &window));
        assert_eq!(op.resonance_strength(&state, &state, &window), 1.0);
    }

    #[test]
    fn test_within_window() {
        let op = ResonanceOperator::new();
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(1.05, 1.02, 1.03);
        let window = ResonanceWindow::new(0.1);

        assert!(op.is_resonant(&state1, &state2, &window));
        assert!(op.resonance_strength(&state1, &state2, &window) > 0.0);
    }

    #[test]
    fn test_outside_window() {
        let op = ResonanceOperator::new();
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(2.0, 2.0, 2.0);
        let window = ResonanceWindow::new(0.1);

        assert!(!op.is_resonant(&state1, &state2, &window));
        assert_eq!(op.resonance_strength(&state1, &state2, &window), 0.0);
    }

    #[test]
    fn test_narrow_vs_wide_window() {
        let op = ResonanceOperator::new();
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(1.2, 1.1, 1.15);

        let narrow = ResonanceWindow::narrow();
        let wide = ResonanceWindow::wide();

        assert!(!op.is_resonant(&state1, &state2, &narrow));
        assert!(op.is_resonant(&state1, &state2, &wide));
    }

    #[test]
    fn test_weighted_resonance() {
        let op = ResonanceOperator::new();
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(1.0, 1.0, 2.0);

        // Weight omega heavily
        let window = ResonanceWindow::with_weights(0.5, [0.1, 0.1, 10.0]);

        assert!(!op.is_resonant(&state1, &state2, &window));
    }

    #[test]
    fn test_collective_resonance() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            ResonanceState::new(1.05, 1.02, 1.03),
            ResonanceState::new(1.03, 1.01, 1.02),
            ResonanceState::new(2.0, 2.0, 2.0),
        ];

        let window = ResonanceWindow::new(0.1);

        assert!(op.collective_resonance(&node_states, &packet_state, &window, 0.5));
        assert!(!op.collective_resonance(&node_states, &packet_state, &window, 0.75));
    }

    #[test]
    fn test_find_resonant_nodes() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            (0, ResonanceState::new(1.05, 1.02, 1.03)),
            (1, ResonanceState::new(2.0, 2.0, 2.0)),
            (2, ResonanceState::new(1.03, 1.01, 1.02)),
        ];

        let window = ResonanceWindow::new(0.1);
        let resonant = op.find_resonant_nodes(&node_states, &packet_state, &window);

        assert_eq!(resonant, vec![0, 2]);
    }

    #[test]
    fn test_average_resonance() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            ResonanceState::new(1.0, 1.0, 1.0), // Perfect
            ResonanceState::new(2.0, 2.0, 2.0), // None
        ];

        let window = ResonanceWindow::new(0.1);
        let avg = op.average_resonance(&node_states, &packet_state, &window);

        assert!((avg - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_find_strongest_resonance() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            (0, ResonanceState::new(1.05, 1.05, 1.05)),
            (1, ResonanceState::new(1.0, 1.0, 1.0)),
            (2, ResonanceState::new(1.1, 1.1, 1.1)),
        ];

        let window = ResonanceWindow::new(0.5);
        let (strongest_id, _strength) = op
            .find_strongest_resonance(&node_states, &packet_state, &window)
            .unwrap();

        assert_eq!(strongest_id, 1); // Perfect match
    }

    #[test]
    fn test_resonance_state_creation() {
        let state = ResonanceState::new(0.5, 0.8, 0.3);
        assert_eq!(state.psi, 0.5);
        assert_eq!(state.rho, 0.8);
        assert_eq!(state.omega, 0.3);

        let vec = state.as_vector();
        let state2 = ResonanceState::from_vector(vec);
        assert_eq!(state, state2);
    }

    #[test]
    fn test_quantum_operator_trait() {
        let op = ResonanceOperator::new();
        let input = ResonanceInput {
            node_state: ResonanceState::new(1.0, 1.0, 1.0),
            packet_state: ResonanceState::new(1.05, 1.02, 1.03),
        };
        let params = ResonanceWindow::standard();

        let result = op.apply(input, &params).unwrap();
        assert!(result);

        assert_eq!(op.name(), "ResonanceOperator");
        assert!(op.description().contains("resonance"));
        assert!(op.formula().contains("R_ε"));
    }
}
