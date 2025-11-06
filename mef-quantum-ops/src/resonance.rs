/*!
 * Resonance Operator (R_ε)
 *
 * Blueprint Formel: R_ε(ψ_node, ψ_pkt) = 1 if |ψ_node - ψ_pkt| < ε, else 0
 *
 * Erweiterte Implementation:
 * - Unterstützt multidimensionale Resonanz (psi, rho, omega)
 * - Adaptive Resonanzfenster
 * - Kollektiv-Resonanz für Gruppenentscheidungen
 */

use crate::{QuantumOperator, QuantumOpsError, Result};
use serde::{Deserialize, Serialize};

/// Resonanzfenster-Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceWindow {
    /// Epsilon (ε) - Resonanzfenster-Breite
    pub epsilon: f64,
    /// Gewichte für multidimensionale Resonanz [psi, rho, omega]
    pub weights: [f64; 3],
}

impl ResonanceWindow {
    pub fn new(epsilon: f64) -> Self {
        Self {
            epsilon,
            weights: [1.0, 1.0, 1.0], // Gleichgewichtig
        }
    }

    pub fn with_weights(epsilon: f64, weights: [f64; 3]) -> Self {
        Self { epsilon, weights }
    }

    /// Standard-Resonanzfenster (wie im Blueprint)
    pub fn standard() -> Self {
        Self::new(0.1)
    }

    /// Enges Resonanzfenster (hohe Selektivität)
    pub fn narrow() -> Self {
        Self::new(0.01)
    }

    /// Weites Resonanzfenster (niedrige Selektivität)
    pub fn wide() -> Self {
        Self::new(0.5)
    }
}

impl Default for ResonanceWindow {
    fn default() -> Self {
        Self::standard()
    }
}

/// Resonanz-Zustand (ψ, ρ, ω)
///
/// Entspricht Gabriel Cell State aus Infinity Ledger
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResonanceState {
    /// Psi (ψ) - Semantic Density / Activation
    pub psi: f64,
    /// Rho (ρ) - Coherence
    pub rho: f64,
    /// Omega (ω) - Rhythm / Oscillation
    pub omega: f64,
}

impl ResonanceState {
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self { psi, rho, omega }
    }

    /// Als Vektor
    pub fn as_vector(&self) -> [f64; 3] {
        [self.psi, self.rho, self.omega]
    }

    /// Aus Vektor
    pub fn from_vector(v: [f64; 3]) -> Self {
        Self {
            psi: v[0],
            rho: v[1],
            omega: v[2],
        }
    }

    /// Euklidische Distanz zu anderem Zustand
    pub fn distance(&self, other: &Self) -> f64 {
        let dpsi = self.psi - other.psi;
        let drho = self.rho - other.rho;
        let domega = self.omega - other.omega;
        (dpsi * dpsi + drho * drho + domega * domega).sqrt()
    }

    /// Gewichtete Distanz
    pub fn weighted_distance(&self, other: &Self, weights: &[f64; 3]) -> f64 {
        let dpsi = (self.psi - other.psi) * weights[0];
        let drho = (self.rho - other.rho) * weights[1];
        let domega = (self.omega - other.omega) * weights[2];
        (dpsi * dpsi + drho * drho + domega * domega).sqrt()
    }
}

/// Resonance Operator
///
/// Implementiert R_ε(ψ_node, ψ_pkt)
pub struct ResonanceOperator;

impl ResonanceOperator {
    pub fn new() -> Self {
        Self
    }

    /// Prüfe ob zwei Zustände resonant sind
    pub fn is_resonant(
        &self,
        node_state: &ResonanceState,
        packet_state: &ResonanceState,
        window: &ResonanceWindow,
    ) -> bool {
        let distance = node_state.weighted_distance(packet_state, &window.weights);
        distance < window.epsilon
    }

    /// Berechne Resonanzstärke (0.0 = keine Resonanz, 1.0 = perfekte Resonanz)
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
    /// Gibt true zurück wenn mindestens `threshold` Prozent der Nodes resonant sind
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

        assert!(!op.is_resonant(&state1, &state2, &narrow), "Should not resonate with narrow window");
        assert!(op.is_resonant(&state1, &state2, &wide), "Should resonate with wide window");
    }

    #[test]
    fn test_weighted_resonance() {
        let op = ResonanceOperator::new();
        let state1 = ResonanceState::new(1.0, 1.0, 1.0);
        let state2 = ResonanceState::new(1.0, 1.0, 2.0); // Only omega differs

        // Weight omega heavily
        let window = ResonanceWindow::with_weights(0.5, [0.1, 0.1, 10.0]);

        assert!(!op.is_resonant(&state1, &state2, &window), "Omega difference should break resonance");
    }

    #[test]
    fn test_collective_resonance() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            ResonanceState::new(1.05, 1.02, 1.03), // Resonant
            ResonanceState::new(1.03, 1.01, 1.02), // Resonant
            ResonanceState::new(2.0, 2.0, 2.0),    // Not resonant
        ];

        let window = ResonanceWindow::new(0.1);

        // 2 out of 3 are resonant (66%)
        assert!(op.collective_resonance(&node_states, &packet_state, &window, 0.5));
        assert!(!op.collective_resonance(&node_states, &packet_state, &window, 0.75));
    }

    #[test]
    fn test_find_resonant_nodes() {
        let op = ResonanceOperator::new();
        let packet_state = ResonanceState::new(1.0, 1.0, 1.0);

        let node_states = vec![
            (0, ResonanceState::new(1.05, 1.02, 1.03)), // Resonant
            (1, ResonanceState::new(2.0, 2.0, 2.0)),    // Not resonant
            (2, ResonanceState::new(1.03, 1.01, 1.02)), // Resonant
        ];

        let window = ResonanceWindow::new(0.1);
        let resonant = op.find_resonant_nodes(&node_states, &packet_state, &window);

        assert_eq!(resonant, vec![0, 2]);
    }

    #[test]
    fn test_distance_calculation() {
        let state1 = ResonanceState::new(0.0, 0.0, 0.0);
        let state2 = ResonanceState::new(1.0, 0.0, 0.0);

        assert_eq!(state1.distance(&state2), 1.0);

        let state3 = ResonanceState::new(1.0, 1.0, 1.0);
        let dist = state1.distance(&state3);
        assert!((dist - 3.0_f64.sqrt()).abs() < 1e-10);
    }
}
