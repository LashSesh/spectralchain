/*!
 * Quantum State Operator - 13-Dimensional Hilbert Space on Metatron Cube
 *
 * ## Mathematische Grundlagen
 *
 * Der Metatron Cube besitzt 13 kanonische Nodes. Ein Quantum State ist ein
 * 13-dimensionaler komplexer Vektor im Hilbert-Raum:
 *
 * ```text
 * |ψ⟩ = Σᵢ αᵢ|i⟩,  i ∈ {1, 2, ..., 13}
 * ```
 *
 * Wobei:
 * - `αᵢ ∈ ℂ`: Komplexe Amplitude für Node i
 * - `Σᵢ |αᵢ|² = 1`: Normalisierungsbedingung
 * - `|i⟩`: Basis-Zustand (Node i)
 *
 * ## Quantenoperationen
 *
 * ### Superposition
 * Ein Zustand kann in mehreren Basis-Zuständen gleichzeitig sein:
 * ```text
 * |ψ⟩ = (1/√2)(|1⟩ + |2⟩)  // Gleichgewichtige Superposition
 * ```
 *
 * ### Measurement
 * Messung kollabiert den Zustand mit Wahrscheinlichkeit P(i) = |αᵢ|²
 *
 * ### Unitary Evolution
 * Zustandsentwicklung durch unitäre Operatoren:
 * ```text
 * |ψ'⟩ = U|ψ⟩,  wobei U†U = UU† = I
 * ```
 *
 * ## Use Cases
 * - Post-symbolic cognition (Theory of Everything)
 * - Quantum-inspired consensus algorithms
 * - Entanglement across multiple cubes
 * - Symmetry-preserving transformations
 *
 * ## Beispiel
 * ```rust
 * use quantumhybrid_operatoren_core::operators::quantum_state::*;
 * use num_complex::Complex64;
 *
 * // Create superposition state
 * let amplitudes = vec![
 *     Complex64::new(1.0, 0.0),
 *     Complex64::new(1.0, 0.0),
 * ];
 * let state = QuantumState::new(amplitudes, true)?;
 *
 * // Apply unitary operator
 * let permutation = vec![2, 3, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
 * let operator = QuantumOperator::from_permutation(&permutation);
 * let new_state = state.apply(&operator)?;
 *
 * // Measure
 * let node = state.measure();
 * ```
 */

use crate::core::{QuantumOperator as QuantumOp, UnitaryOperator};
use anyhow::{anyhow, Result};
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Dimension des Metatron Cube Hilbert-Raums
pub const METATRON_DIMENSION: usize = 13;

/// Quantum State auf dem 13-dimensionalen Hilbert-Raum des Metatron Cube
///
/// Ein Quantum State ist ein normalisierter komplexer Vektor:
/// |ψ⟩ = Σᵢ αᵢ|i⟩ mit Σᵢ |αᵢ|² = 1
///
/// # Felder
/// * `amplitudes` - Komplexe Amplituden für die 13 Nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// Komplexe Amplituden für die 13 Nodes
    pub amplitudes: Array1<Complex64>,
}

impl QuantumState {
    /// Erstelle neuen Quantum State aus komplexen Amplituden
    ///
    /// # Arguments
    /// * `amplitudes` - Vektor von bis zu 13 komplexen Zahlen
    /// * `normalize` - Wenn true, wird der Zustand auf Norm 1 normalisiert
    ///
    /// # Returns
    /// Neuer QuantumState oder Fehler bei mehr als 13 Amplituden
    ///
    /// # Beispiel
    /// ```ignore
    /// use num_complex::Complex64;
    /// let amps = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 1.0)];
    /// let state = QuantumState::new(amps, true)?;
    /// ```
    pub fn new(amplitudes: Vec<Complex64>, normalize: bool) -> Result<Self> {
        let mut amps = amplitudes;

        if amps.len() > METATRON_DIMENSION {
            return Err(anyhow!(
                "QuantumState expects at most {} amplitudes",
                METATRON_DIMENSION
            ));
        }

        // Pad mit Nullen falls nötig
        while amps.len() < METATRON_DIMENSION {
            amps.push(Complex64::new(0.0, 0.0));
        }

        let mut state = QuantumState {
            amplitudes: Array1::from(amps),
        };

        if normalize {
            state.normalize();
        }

        Ok(state)
    }

    /// Erstelle Basis-Zustand |i⟩
    ///
    /// # Arguments
    /// * `node` - Node-Index (0-based, 0-12)
    ///
    /// # Beispiel
    /// ```ignore
    /// let state = QuantumState::basis_state(0)?; // |1⟩
    /// ```
    pub fn basis_state(node: usize) -> Result<Self> {
        if node >= METATRON_DIMENSION {
            return Err(anyhow!("Node index must be < {}", METATRON_DIMENSION));
        }

        let mut amps = vec![Complex64::new(0.0, 0.0); METATRON_DIMENSION];
        amps[node] = Complex64::new(1.0, 0.0);

        Self::new(amps, false)
    }

    /// Erstelle gleichgewichtige Superposition über alle Nodes
    ///
    /// |ψ⟩ = (1/√13) Σᵢ |i⟩
    pub fn uniform_superposition() -> Result<Self> {
        let amplitude = Complex64::new(1.0 / (METATRON_DIMENSION as f64).sqrt(), 0.0);
        let amps = vec![amplitude; METATRON_DIMENSION];
        Self::new(amps, false)
    }

    /// Erstelle zufälligen Zustand (Haar-Maß)
    pub fn random() -> Result<Self> {
        let mut rng = thread_rng();
        let amps: Vec<Complex64> = (0..METATRON_DIMENSION)
            .map(|_| Complex64::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)))
            .collect();
        Self::new(amps, true)
    }

    /// Normalisiere den Zustand auf Einheitsnorm
    ///
    /// Nach Normalisierung gilt: ⟨ψ|ψ⟩ = Σᵢ |αᵢ|² = 1
    pub fn normalize(&mut self) {
        let norm = self.norm();
        if norm == 0.0 {
            // Null-Zustand -> setze auf |0⟩
            self.amplitudes.fill(Complex64::new(0.0, 0.0));
            self.amplitudes[0] = Complex64::new(1.0, 0.0);
        } else {
            self.amplitudes.mapv_inplace(|x| x / norm);
        }
    }

    /// Berechne L2-Norm des Zustands
    ///
    /// ||ψ|| = √(Σᵢ |αᵢ|²)
    pub fn norm(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|&c| c.norm_sqr())
            .sum::<f64>()
            .sqrt()
    }

    /// Berechne inneres Produkt ⟨φ|ψ⟩
    ///
    /// ⟨φ|ψ⟩ = Σᵢ φᵢ* ψᵢ
    ///
    /// # Arguments
    /// * `other` - Der andere Quantum State
    ///
    /// # Returns
    /// Komplexes inneres Produkt
    pub fn inner_product(&self, other: &QuantumState) -> Complex64 {
        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .map(|(&a, &b)| a.conj() * b)
            .sum()
    }

    /// Wende Quantum Operator auf diesen Zustand an
    ///
    /// |ψ'⟩ = U|ψ⟩
    ///
    /// # Arguments
    /// * `operator` - Der anzuwendende Operator
    ///
    /// # Returns
    /// Neuer transformierter Zustand
    pub fn apply(&self, operator: &QuantumUnitaryOperator) -> Result<Self> {
        if operator.matrix.shape() != [METATRON_DIMENSION, METATRON_DIMENSION] {
            return Err(anyhow!(
                "Operator must be {}×{} to act on QuantumState",
                METATRON_DIMENSION,
                METATRON_DIMENSION
            ));
        }

        let new_amplitudes = operator.matrix.dot(&self.amplitudes);

        Ok(QuantumState {
            amplitudes: new_amplitudes,
        })
    }

    /// Berechne Wahrscheinlichkeitsverteilung |ψᵢ|²
    ///
    /// # Returns
    /// Vektor mit 13 Wahrscheinlichkeiten (summiert zu 1)
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Führe projektive Messung in Computational Basis durch
    ///
    /// Misst den Zustand und kollabiert ihn auf einen Basis-Zustand.
    /// Die Wahrscheinlichkeit für Node i ist P(i) = |αᵢ|².
    ///
    /// # Returns
    /// Index des gemessenen Nodes (1-based: 1-13)
    ///
    /// # Side Effects
    /// Kollabiert den Zustand auf den gemessenen Basis-Zustand
    pub fn measure(&mut self) -> usize {
        let probs = self.probabilities();
        let mut rng = thread_rng();

        // Sample aus Wahrscheinlichkeitsverteilung
        let mut cumulative = 0.0;
        let random_value: f64 = rng.gen();

        for (idx, &prob) in probs.iter().enumerate() {
            cumulative += prob;
            if random_value < cumulative {
                // Kollabiere auf Basis-Zustand |idx⟩
                self.amplitudes.fill(Complex64::new(0.0, 0.0));
                self.amplitudes[idx] = Complex64::new(1.0, 0.0);
                return idx + 1; // 1-based
            }
        }

        // Fallback (sollte nicht erreicht werden)
        self.amplitudes.fill(Complex64::new(0.0, 0.0));
        self.amplitudes[METATRON_DIMENSION - 1] = Complex64::new(1.0, 0.0);
        METATRON_DIMENSION
    }

    /// Berechne Erwartungswert eines Observables
    ///
    /// ⟨O⟩ = ⟨ψ|O|ψ⟩
    pub fn expectation_value(&self, observable: &QuantumUnitaryOperator) -> Result<Complex64> {
        let o_psi = self.apply(observable)?;
        Ok(self.inner_product(&o_psi))
    }

    /// Konvertiere zu Array
    pub fn as_array(&self) -> Array1<Complex64> {
        self.amplitudes.clone()
    }

    /// Prüfe ob Zustand normalisiert ist
    pub fn is_normalized(&self, tolerance: f64) -> bool {
        (self.norm() - 1.0).abs() < tolerance
    }
}

/// Quantum Unitary Operator - Linearer Operator auf 13D Hilbert-Raum
///
/// Repräsentiert als 13×13 komplexe Matrix.
/// Für unitäre Operatoren gilt: U†U = UU† = I
#[derive(Debug, Clone)]
pub struct QuantumUnitaryOperator {
    /// 13×13 komplexe Matrix
    pub matrix: Array2<Complex64>,
}

impl QuantumUnitaryOperator {
    /// Erstelle neuen Quantum Operator aus Matrix
    ///
    /// # Arguments
    /// * `matrix` - 13×13 komplexe Matrix
    pub fn new(matrix: Array2<Complex64>) -> Result<Self> {
        if matrix.shape() != [METATRON_DIMENSION, METATRON_DIMENSION] {
            return Err(anyhow!(
                "QuantumUnitaryOperator matrix must be {}×{}",
                METATRON_DIMENSION,
                METATRON_DIMENSION
            ));
        }

        Ok(QuantumUnitaryOperator { matrix })
    }

    /// Erstelle Identitäts-Operator
    pub fn identity() -> Self {
        let matrix = Array2::eye(METATRON_DIMENSION).mapv(|x| Complex64::new(x, 0.0));
        QuantumUnitaryOperator { matrix }
    }

    /// Erstelle Permutations-Operator aus Permutation
    ///
    /// # Arguments
    /// * `sigma` - Permutation von (1..=13)
    ///
    /// # Beispiel
    /// ```ignore
    /// // Zyklische Permutation: 1→2, 2→3, ..., 13→1
    /// let perm = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 1];
    /// let op = QuantumUnitaryOperator::from_permutation(&perm);
    /// ```
    pub fn from_permutation(sigma: &[usize]) -> Self {
        let p = Self::permutation_matrix(sigma);
        let matrix = p.mapv(|x| Complex64::new(x, 0.0));
        QuantumUnitaryOperator { matrix }
    }

    /// Generiere Permutations-Matrix
    fn permutation_matrix(sigma: &[usize]) -> Array2<f64> {
        let n = METATRON_DIMENSION;
        let mut p = Array2::zeros((n, n));

        for (i, &s) in sigma.iter().enumerate().take(n) {
            if s > 0 && s <= n {
                p[[i, s - 1]] = 1.0; // 1-based zu 0-based
            }
        }

        p
    }

    /// Komponiere zwei Operatoren: C = A ∘ B
    ///
    /// (A ∘ B)|ψ⟩ = A(B|ψ⟩)
    pub fn compose(&self, other: &QuantumUnitaryOperator) -> Result<Self> {
        if self.matrix.shape() != [METATRON_DIMENSION, METATRON_DIMENSION]
            || other.matrix.shape() != [METATRON_DIMENSION, METATRON_DIMENSION]
        {
            return Err(anyhow!("Both operators must be {}×{}", METATRON_DIMENSION, METATRON_DIMENSION));
        }

        let result = self.matrix.dot(&other.matrix);
        Ok(QuantumUnitaryOperator { matrix: result })
    }

    /// Prüfe ob Operator unitär ist
    ///
    /// U ist unitär wenn U†U = UU† = I
    pub fn is_unitary(&self, atol: f64) -> bool {
        // Berechne U†
        let u_dagger = self.matrix.t().mapv(|x| x.conj());

        // Berechne U†U und UU†
        let product1 = u_dagger.dot(&self.matrix);
        let product2 = self.matrix.dot(&u_dagger);

        // Identitätsmatrix
        let identity = Array2::eye(METATRON_DIMENSION).mapv(|x| Complex64::new(x, 0.0));

        // Prüfe ob nahe an Identität
        let close1 = product1
            .iter()
            .zip(identity.iter())
            .all(|(&a, &b)| (a - b).norm() < atol);

        let close2 = product2
            .iter()
            .zip(identity.iter())
            .all(|(&a, &b)| (a - b).norm() < atol);

        close1 && close2
    }

    /// Berechne adjungierte Matrix U†
    pub fn adjoint(&self) -> Self {
        let matrix = self.matrix.t().mapv(|x| x.conj());
        QuantumUnitaryOperator { matrix }
    }

    /// Berechne Trace Tr(U)
    pub fn trace(&self) -> Complex64 {
        self.matrix.diag().iter().sum()
    }
}

impl UnitaryOperator for QuantumUnitaryOperator {
    fn is_unitary(&self, tolerance: f64) -> bool {
        self.is_unitary(tolerance)
    }
}

/// Parameter für Quantum State Operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumStateParams {
    pub normalize: bool,
}

impl Default for QuantumStateParams {
    fn default() -> Self {
        Self { normalize: true }
    }
}

impl QuantumOp for QuantumUnitaryOperator {
    type Input = QuantumState;
    type Output = QuantumState;
    type Params = QuantumStateParams;

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        input.apply(self)
    }

    fn name(&self) -> &str {
        "QuantumUnitaryOperator"
    }

    fn description(&self) -> &str {
        "Unitary operator on 13-dimensional Metatron Cube Hilbert space"
    }

    fn formula(&self) -> &str {
        "|ψ'⟩ = U|ψ⟩"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_quantum_state() {
        let amps = vec![Complex64::new(1.0, 0.0)];
        let state = QuantumState::new(amps, true).unwrap();
        assert_eq!(state.amplitudes.len(), METATRON_DIMENSION);
        assert!(state.is_normalized(1e-10));
    }

    #[test]
    fn test_basis_state() {
        let state = QuantumState::basis_state(0).unwrap();
        assert_eq!(state.amplitudes[0], Complex64::new(1.0, 0.0));
        assert_eq!(state.amplitudes[1], Complex64::new(0.0, 0.0));
    }

    #[test]
    fn test_uniform_superposition() {
        let state = QuantumState::uniform_superposition().unwrap();
        let probs = state.probabilities();
        let expected = 1.0 / METATRON_DIMENSION as f64;
        for prob in probs {
            assert!((prob - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn test_normalization() {
        let amps = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
        ];
        let state = QuantumState::new(amps, true).unwrap();
        assert!((state.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inner_product() {
        let amps1 = vec![Complex64::new(1.0, 0.0)];
        let amps2 = vec![Complex64::new(1.0, 0.0)];

        let state1 = QuantumState::new(amps1, true).unwrap();
        let state2 = QuantumState::new(amps2, true).unwrap();

        let inner = state1.inner_product(&state2);
        assert!((inner - Complex64::new(1.0, 0.0)).norm() < 1e-10);
    }

    #[test]
    fn test_identity_operator() {
        let op = QuantumUnitaryOperator::identity();
        assert!(op.is_unitary(1e-8));

        let state = QuantumState::random().unwrap();
        let result = state.apply(&op).unwrap();

        for i in 0..METATRON_DIMENSION {
            assert!((state.amplitudes[i] - result.amplitudes[i]).norm() < 1e-10);
        }
    }

    #[test]
    fn test_permutation_operator() {
        let sigma: Vec<usize> = (1..=METATRON_DIMENSION).collect();
        let op = QuantumUnitaryOperator::from_permutation(&sigma);
        assert!(op.is_unitary(1e-8));
    }

    #[test]
    fn test_operator_composition() {
        let sigma: Vec<usize> = (1..=METATRON_DIMENSION).collect();
        let op1 = QuantumUnitaryOperator::from_permutation(&sigma);
        let op2 = QuantumUnitaryOperator::from_permutation(&sigma);

        let composed = op1.compose(&op2).unwrap();
        assert!(composed.is_unitary(1e-8));
    }

    #[test]
    fn test_measure() {
        let mut state = QuantumState::basis_state(0).unwrap();
        let measurement = state.measure();
        assert_eq!(measurement, 1); // Node 1 (1-based)
    }

    #[test]
    fn test_probabilities() {
        let state = QuantumState::basis_state(0).unwrap();
        let probs = state.probabilities();
        assert!((probs[0] - 1.0).abs() < 1e-10);
        assert!(probs[1].abs() < 1e-10);
    }

    #[test]
    fn test_adjoint() {
        let sigma: Vec<usize> = (1..=METATRON_DIMENSION).collect();
        let op = QuantumUnitaryOperator::from_permutation(&sigma);
        let adj = op.adjoint();

        let product = op.compose(&adj).unwrap();
        let identity = QuantumUnitaryOperator::identity();

        for i in 0..METATRON_DIMENSION {
            for j in 0..METATRON_DIMENSION {
                assert!((product.matrix[[i, j]] - identity.matrix[[i, j]]).norm() < 1e-10);
            }
        }
    }
}
