//! Weight-Transfer (WT) Operator
//!
//! WT(v) = Σ_{ℓ∈L} w'_ℓ · P_ℓ(v)
//! w'_ℓ = (1-γ)w_ℓ + γw̃_ℓ
//!
//! Multi-scale convex combination across Micro, Meso, Macro levels

use crate::core::{ContractiveOperator, QuantumOperator};
use anyhow::Result;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScaleLevel {
    Micro,
    Meso,
    Macro,
}

impl ScaleLevel {
    fn as_str(&self) -> &str {
        match self {
            ScaleLevel::Micro => "micro",
            ScaleLevel::Meso => "meso",
            ScaleLevel::Macro => "macro",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightTransferParams {
    pub gamma: f64,
    pub levels: Vec<String>,
}

impl Default for WeightTransferParams {
    fn default() -> Self {
        Self {
            gamma: 0.1,
            levels: vec!["micro".to_string(), "meso".to_string(), "macro".to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeightTransfer {
    gamma: f64,
    levels: Vec<ScaleLevel>,
    weights: HashMap<ScaleLevel, f64>,
    target_weights: HashMap<ScaleLevel, f64>,
    projections: HashMap<ScaleLevel, Array2<f64>>,
}

impl WeightTransfer {
    pub fn new(gamma: f64, levels: Vec<ScaleLevel>) -> Self {
        assert!(
            gamma > 0.0 && gamma <= 0.5,
            "Gamma must be in (0, 0.5]"
        );

        let mut wt = Self {
            gamma,
            levels: levels.clone(),
            weights: HashMap::new(),
            target_weights: HashMap::new(),
            projections: HashMap::new(),
        };

        wt.initialize_weights();
        wt.initialize_projections();
        wt
    }

    fn initialize_weights(&mut self) {
        let n_levels = self.levels.len() as f64;
        for level in &self.levels {
            self.weights.insert(level.clone(), 1.0 / n_levels);
        }

        self.target_weights.insert(ScaleLevel::Micro, 0.25);
        self.target_weights.insert(ScaleLevel::Meso, 0.45);
        self.target_weights.insert(ScaleLevel::Macro, 0.30);
    }

    fn initialize_projections(&mut self) {
        for level in &self.levels {
            let projection = match level {
                ScaleLevel::Micro => {
                    Array2::from_diag(&Array1::from(vec![1.2, 0.8, 1.0, 0.9, 1.1]))
                }
                ScaleLevel::Meso => {
                    #[rustfmt::skip]
                    let data = vec![
                        0.7, 0.3, 0.0, 0.0, 0.0,
                        0.3, 0.7, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.6, 0.4, 0.0,
                        0.0, 0.0, 0.4, 0.6, 0.0,
                        0.0, 0.0, 0.0, 0.0, 1.0,
                    ];
                    Array2::from_shape_vec((5, 5), data).unwrap()
                }
                ScaleLevel::Macro => {
                    let mut p = Array2::from_elem((5, 5), 0.2);
                    for i in 0..5 {
                        p[[i, i]] = 0.4;
                    }
                    p
                }
            };
            self.projections.insert(level.clone(), projection);
        }
    }

    fn update_weights(&mut self) {
        let mut new_weights = HashMap::new();
        for level in &self.levels {
            let old_w = self.weights.get(level).copied().unwrap_or(0.0);
            let target_w = self.target_weights.get(level).copied().unwrap_or(old_w);
            let new_w = (1.0 - self.gamma) * old_w + self.gamma * target_w;
            new_weights.insert(level.clone(), new_w);
        }

        let total: f64 = new_weights.values().sum();
        self.weights = new_weights
            .into_iter()
            .map(|(k, v)| (k, v / total))
            .collect();
    }

    pub fn apply(&mut self, v: &Array1<f64>) -> Array1<f64> {
        self.update_weights();

        let mut result = Array1::zeros(v.len());
        for level in &self.levels {
            if let (Some(projection), Some(&weight)) =
                (self.projections.get(level), self.weights.get(level))
            {
                result += &(projection.dot(v) * weight);
            }
        }
        result
    }
}

impl Default for WeightTransfer {
    fn default() -> Self {
        Self::new(
            0.1,
            vec![ScaleLevel::Micro, ScaleLevel::Meso, ScaleLevel::Macro],
        )
    }
}

impl QuantumOperator for WeightTransfer {
    type Input = Array1<f64>;
    type Output = Array1<f64>;
    type Params = WeightTransferParams;

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        let mut wt = self.clone();
        Ok(wt.apply(&input))
    }

    fn name(&self) -> &str {
        "WeightTransfer"
    }

    fn description(&self) -> &str {
        "Multi-scale weight redistribution across Micro, Meso, Macro levels"
    }

    fn formula(&self) -> &str {
        "WT(v) = Σ_{ℓ∈L} w'_ℓ · P_ℓ(v)"
    }
}

impl ContractiveOperator for WeightTransfer {
    fn lipschitz_constant(&self) -> f64 {
        1.0 // Convex combination maintains non-expansiveness
    }
}
