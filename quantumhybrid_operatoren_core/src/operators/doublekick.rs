/*!
 * DoubleKick (DK) Operator
 *
 * ## Mathematische Formel
 * ```text
 * DK(v) = v + α₁u₁ + α₂u₂
 * ```
 *
 * Wobei:
 * - `v`: Input-Vektor (5D)
 * - `α₁, α₂`: Impulse coefficients
 * - `u₁, u₂`: Orthogonale Einheitsvektoren (⟨u₁, u₂⟩ = 0, ||u_i||₂ = 1)
 * - Non-expansive: |α₁| + |α₂| ≤ η ≪ 1
 *
 * ## Eigenschaften
 * - **Local Unsticking**: Duale orthogonale Impulse ohne Expansion
 * - **Non-Expansive**: Lipschitz-Konstante ≈ 1 + η mit η ≪ 1
 * - **Orthogonalität**: u₁ ⟂ u₂ (Gram-Schmidt)
 *
 * ## Use Cases
 * - Escape local minima in optimization
 * - Perturbation for exploring solution space
 * - Fixed-point iteration improvements
 *
 * ## Beispiel
 * ```rust
 * use quantumhybrid_operatoren_core::operators::doublekick::DoubleKick;
 * use ndarray::Array1;
 *
 * let dk = DoubleKick::new(0.05, -0.03);
 * let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
 * let result = dk.apply(&v);
 * ```
 */

use crate::core::{ContractiveOperator, QuantumOperator};
use anyhow::Result;
use ndarray::Array1;
use rand::SeedableRng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};

/// DoubleKick operator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleKickInfo {
    pub alpha1: f64,
    pub alpha2: f64,
    pub eta: f64,
    pub u1_u2_orthogonal: bool,
    pub non_expansive: bool,
}

/// DoubleKick Operator: DK(v) = v + α₁u₁ + α₂u₂
///
/// Maintains non-expansiveness: |α₁| + |α₂| ≤ η with η ≪ 1
#[derive(Debug, Clone)]
pub struct DoubleKick {
    alpha1: f64,
    alpha2: f64,
    eta: f64,
    u1: Array1<f64>,
    u2: Array1<f64>,
}

impl DoubleKick {
    /// Erstelle neuen DoubleKick Operator
    ///
    /// # Arguments
    /// * `alpha1` - Erster Impuls-Koeffizient
    /// * `alpha2` - Zweiter Impuls-Koeffizient
    ///
    /// # Warnung
    /// Für non-expansiveness sollte |α₁| + |α₂| ≤ 0.1
    pub fn new(alpha1: f64, alpha2: f64) -> Self {
        let eta = alpha1.abs() + alpha2.abs();
        if eta > 0.1 {
            eprintln!(
                "Warning: DoubleKick η={} > 0.1, may affect contractivity",
                eta
            );
        }

        let (u1, u2) = Self::generate_orthogonal_vectors();

        Self {
            alpha1,
            alpha2,
            eta,
            u1,
            u2,
        }
    }

    /// Generiere zwei orthogonale Einheitsvektoren u₁ und u₂
    fn generate_orthogonal_vectors() -> (Array1<f64>, Array1<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let normal = StandardNormal;

        // Generate first unit vector
        let mut u1_data: Vec<f64> = (0..5).map(|_| normal.sample(&mut rng)).collect();
        let u1_norm: f64 = u1_data.iter().map(|&x| x * x).sum::<f64>().sqrt();
        u1_data.iter_mut().for_each(|x| *x /= u1_norm);
        let u1 = Array1::from(u1_data);

        // Generate second vector orthogonal to first (Gram-Schmidt)
        let u2_data: Vec<f64> = (0..5).map(|_| normal.sample(&mut rng)).collect();
        let mut u2 = Array1::from(u2_data);

        let dot_u2_u1 = u2.dot(&u1);
        u2 -= &(&u1 * dot_u2_u1);

        let u2_norm = u2.dot(&u2).sqrt();
        u2 /= u2_norm;

        // Verify orthogonality
        let dot_product = u1.dot(&u2);
        assert!(
            dot_product.abs() < 1e-10,
            "Vectors not orthogonal: ⟨u₁,u₂⟩ = {}",
            dot_product
        );

        (u1, u2)
    }

    /// Wende DoubleKick Operator an
    pub fn apply(&self, v: &Array1<f64>) -> Array1<f64> {
        v + &(&self.u1 * self.alpha1) + &(&self.u2 * self.alpha2)
    }

    /// Verifiziere non-expansiveness
    pub fn verify_non_expansive(&self) -> bool {
        self.eta <= 0.1
    }

    /// Hole Operator-Information
    pub fn get_info(&self) -> DoubleKickInfo {
        DoubleKickInfo {
            alpha1: self.alpha1,
            alpha2: self.alpha2,
            eta: self.eta,
            u1_u2_orthogonal: self.u1.dot(&self.u2).abs() < 1e-10,
            non_expansive: self.verify_non_expansive(),
        }
    }

    /// Berechne Impulsstärke
    pub fn compute_impulse_strength(&self, _v: &Array1<f64>) -> f64 {
        let impulse = &self.u1 * self.alpha1 + &self.u2 * self.alpha2;
        impulse.dot(&impulse).sqrt()
    }
}

impl Default for DoubleKick {
    fn default() -> Self {
        Self::new(0.05, -0.03)
    }
}

/// Parameter für DoubleKick Operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleKickParams {
    pub alpha1: f64,
    pub alpha2: f64,
}

impl QuantumOperator for DoubleKick {
    type Input = Array1<f64>;
    type Output = Array1<f64>;
    type Params = DoubleKickParams;

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        Ok(self.apply(&input))
    }

    fn name(&self) -> &str {
        "DoubleKick"
    }

    fn description(&self) -> &str {
        "Local unsticking through dual orthogonal impulse without expansion"
    }

    fn formula(&self) -> &str {
        "DK(v) = v + α₁u₁ + α₂u₂"
    }
}

impl ContractiveOperator for DoubleKick {
    fn lipschitz_constant(&self) -> f64 {
        1.0 + self.eta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_doublekick() {
        let dk = DoubleKick::new(0.05, -0.03);
        let info = dk.get_info();

        assert_eq!(info.alpha1, 0.05);
        assert_eq!(info.alpha2, -0.03);
        assert_eq!(info.eta, 0.08);
        assert!(info.u1_u2_orthogonal);
    }

    #[test]
    fn test_apply_doublekick() {
        let dk = DoubleKick::new(0.05, -0.03);
        let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = dk.apply(&v);

        assert_eq!(result.len(), 5);
        let diff = &result - &v;
        let diff_norm = diff.dot(&diff).sqrt();
        assert!(diff_norm < 0.2); // Small impulse
    }

    #[test]
    fn test_verify_non_expansive() {
        let dk = DoubleKick::new(0.05, -0.03);
        assert!(dk.verify_non_expansive());

        let dk_large = DoubleKick::new(0.1, 0.05);
        assert!(!dk_large.verify_non_expansive());
    }

    #[test]
    fn test_orthogonal_vectors() {
        let (u1, u2) = DoubleKick::generate_orthogonal_vectors();

        // Check unit vectors
        assert!((u1.dot(&u1) - 1.0).abs() < 1e-10);
        assert!((u2.dot(&u2) - 1.0).abs() < 1e-10);

        // Check orthogonality
        assert!(u1.dot(&u2).abs() < 1e-10);
    }

    #[test]
    fn test_lipschitz_constant() {
        let dk = DoubleKick::new(0.05, -0.03);
        let lipschitz = dk.lipschitz_constant();
        assert!((lipschitz - 1.08).abs() < 1e-10);
    }
}
