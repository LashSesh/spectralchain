//! Sweep (SW) Operator - Threshold sweeping with schedule
//!
//! SW(v) = g_τ(m(v)) · v
//! g_τ(x) = σ((x - τ)/β)
//!
//! Cosine schedule: τ_t = τ₀ + 0.5(1 + cos(πt/T))Δτ

use crate::core::{ContractiveOperator, QuantumOperator};
use anyhow::Result;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepParams {
    pub tau0: f64,
    pub beta: f64,
    pub schedule: String,
}

impl Default for SweepParams {
    fn default() -> Self {
        Self {
            tau0: 0.5,
            beta: 0.1,
            schedule: "cosine".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sweep {
    tau0: f64,
    beta: f64,
    schedule: String,
    iteration: usize,
    max_iterations: usize,
    delta_tau: f64,
}

impl Sweep {
    pub fn new(tau0: f64, beta: f64, schedule: String) -> Self {
        Self {
            tau0,
            beta,
            schedule,
            iteration: 0,
            max_iterations: 100,
            delta_tau: 0.3,
        }
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn gate_function(&self, x: f64, tau: f64) -> f64 {
        self.sigmoid((x - tau) / self.beta)
    }

    pub fn compute_schedule(&self) -> f64 {
        match self.schedule.as_str() {
            "cosine" => {
                let t = (self.iteration % self.max_iterations) as f64;
                let cap_t = self.max_iterations as f64;
                self.tau0 + 0.5 * (1.0 + (std::f64::consts::PI * t / cap_t).cos()) * self.delta_tau
            }
            "linear" => {
                let t = (self.iteration % self.max_iterations) as f64;
                let cap_t = self.max_iterations as f64;
                self.tau0 + (1.0 - t / cap_t) * self.delta_tau
            }
            _ => self.tau0,
        }
    }

    pub fn apply(&mut self, v: &Array1<f64>) -> Array1<f64> {
        let m_v = v.mean().unwrap_or(0.0);
        let tau = self.compute_schedule();
        let gate_value = self.gate_function(m_v, tau);
        self.iteration += 1;
        v * gate_value
    }
}

impl Default for Sweep {
    fn default() -> Self {
        Self::new(0.5, 0.1, "cosine".to_string())
    }
}

impl QuantumOperator for Sweep {
    type Input = Array1<f64>;
    type Output = Array1<f64>;
    type Params = SweepParams;

    fn apply(&self, input: Self::Input, _params: &Self::Params) -> Result<Self::Output> {
        let mut sweep = self.clone();
        Ok(sweep.apply(&input))
    }

    fn name(&self) -> &str {
        "Sweep"
    }

    fn description(&self) -> &str {
        "Threshold sweeping with sigmoid gate and cosine schedule"
    }

    fn formula(&self) -> &str {
        "SW(v) = g_τ(m(v)) · v"
    }
}

impl ContractiveOperator for Sweep {
    fn lipschitz_constant(&self) -> f64 {
        1.0 // Gate value is in [0, 1]
    }
}
