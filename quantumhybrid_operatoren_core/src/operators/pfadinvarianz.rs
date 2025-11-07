//! Pfadinvarianz (PI) Operator - Path-equivalence projection
//!
//! PI(v) = (1/|Π|) Σ_{p∈Π} T_p(v)
//!
//! Idempotent, non-expansive projection ensuring canonical ordering

use crate::core::{ContractiveOperator, IdempotentOperator, QuantumOperator};
use anyhow::Result;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfadinvarianzParams {
    pub canon: String,
    pub tolerance: f64,
}

impl Default for PfadinvarianzParams {
    fn default() -> Self {
        Self {
            canon: "lexicographic".to_string(),
            tolerance: 1e-6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pfadinvarianz {
    canon: String,
    tol: f64,
    permutations: Vec<Vec<usize>>,
}

impl Pfadinvarianz {
    pub fn new(canon: String, tol: f64) -> Self {
        let permutations = Self::initialize_permutations();
        Self {
            canon,
            tol,
            permutations,
        }
    }

    fn initialize_permutations() -> Vec<Vec<usize>> {
        vec![
            vec![0, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 0],
            vec![4, 0, 1, 2, 3],
            vec![0, 2, 4, 1, 3],
            vec![1, 3, 0, 4, 2],
            vec![4, 3, 2, 1, 0],
        ]
    }

    fn apply_permutation(&self, v: &Array1<f64>, perm: &[usize]) -> Array1<f64> {
        Array1::from(perm.iter().map(|&i| v[i]).collect::<Vec<_>>())
    }

    fn canonical_order(&self, vectors: &mut [Array1<f64>]) {
        match self.canon.as_str() {
            "lexicographic" => {
                vectors.sort_by(|a, b| {
                    for i in 0..a.len() {
                        match a[i].partial_cmp(&b[i]) {
                            Some(std::cmp::Ordering::Equal) => continue,
                            other => return other.unwrap(),
                        }
                    }
                    std::cmp::Ordering::Equal
                });
            }
            "norm" => {
                vectors.sort_by(|a, b| {
                    let norm_a = a.dot(a).sqrt();
                    let norm_b = b.dot(b).sqrt();
                    norm_a.partial_cmp(&norm_b).unwrap()
                });
            }
            _ => {}
        }
    }

    pub fn apply(&self, v: &Array1<f64>) -> Array1<f64> {
        let mut path_vectors: Vec<Array1<f64>> = self
            .permutations
            .iter()
            .map(|perm| self.apply_permutation(v, perm))
            .collect();

        self.canonical_order(&mut path_vectors);

        let n = path_vectors.len() as f64;
        let sum: Array1<f64> = path_vectors
            .iter()
            .fold(Array1::zeros(v.len()), |acc, v| acc + v);

        &sum / n
    }
}

impl Default for Pfadinvarianz {
    fn default() -> Self {
        Self::new("lexicographic".to_string(), 1e-6)
    }
}

impl QuantumOperator for Pfadinvarianz {
    type Input = Array1<f64>;
    type Output = Array1<f64>;
    type Params = PfadinvarianzParams;

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        Ok(self.apply(&input))
    }

    fn name(&self) -> &str {
        "Pfadinvarianz"
    }

    fn description(&self) -> &str {
        "Path-equivalence projection with canonical ordering"
    }

    fn formula(&self) -> &str {
        "PI(v) = (1/|Π|) Σ_{p∈Π} T_p(v)"
    }
}

impl ContractiveOperator for Pfadinvarianz {
    fn lipschitz_constant(&self) -> f64 {
        1.0
    }
}

impl IdempotentOperator for Pfadinvarianz {
    fn is_idempotent(
        &self,
        input: &Self::Input,
        _params: &Self::Params,
        tolerance: f64,
    ) -> Result<bool> {
        let pi_v = self.apply(input);
        let pi_pi_v = self.apply(&pi_v);
        let diff = &pi_pi_v - &pi_v;
        let distance = diff.dot(&diff).sqrt();
        Ok(distance < tolerance)
    }
}
