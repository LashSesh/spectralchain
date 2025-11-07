//! Basic Usage Example für Quantum-Hybrid Operators
//!
//! Run: cargo run --example basic_usage

use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

fn main() -> anyhow::Result<()> {
    println!("=== Quantum-Hybrid Operators Core - Basic Usage ===\n");

    // 1. Masking Operator Demo
    println!("1. Masking Operator Demo");
    println!("   Formula: M_{{θ,σ}}(m) = e^{{iθ}} U_σ m");
    let masking = MaskingOperator::new();
    let params = MaskingParams::from_seed(b"example_seed");
    let message = b"Hello, Quantum World!";

    let masked = masking.mask(message, &params)?;
    println!("   Original: {:?}", String::from_utf8_lossy(message));
    println!("   Masked length: {} bytes", masked.len());

    let unmasked = masking.unmask(&masked, &params)?;
    println!("   Unmasked: {:?}", String::from_utf8_lossy(&unmasked));
    assert_eq!(unmasked, message);
    println!("   ✓ Roundtrip successful!\n");

    // 2. Resonance Operator Demo
    println!("2. Resonance Operator Demo");
    println!("   Formula: R_ε(ψ₁, ψ₂) = 1 if d(ψ₁, ψ₂) < ε");
    let resonance = ResonanceOperator::new();
    let window = ResonanceWindow::standard();

    let state1 = ResonanceState::new(1.0, 0.8, 0.5);
    let state2 = ResonanceState::new(1.05, 0.82, 0.53);
    let state3 = ResonanceState::new(2.0, 2.0, 2.0);

    let is_resonant = resonance.is_resonant(&state1, &state2, &window);
    let strength = resonance.resonance_strength(&state1, &state2, &window);

    println!("   State 1: (ψ={}, ρ={}, ω={})", state1.psi, state1.rho, state1.omega);
    println!("   State 2: (ψ={}, ρ={}, ω={})", state2.psi, state2.rho, state2.omega);
    println!("   Is resonant: {}", is_resonant);
    println!("   Resonance strength: {:.3}", strength);

    let is_resonant_3 = resonance.is_resonant(&state1, &state3, &window);
    println!("   State 3 resonant with State 1: {}\n", is_resonant_3);

    // 3. DoubleKick Operator Demo
    println!("3. DoubleKick Operator Demo");
    println!("   Formula: DK(v) = v + α₁u₁ + α₂u₂");
    let dk = DoubleKick::new(0.05, -0.03);
    let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let result = dk.apply(&v);

    println!("   Input:  {:?}", v.to_vec());
    println!("   Output: {:?}", result.to_vec());
    println!("   Impulse strength: {:.6}", dk.compute_impulse_strength(&v));
    println!("   Lipschitz constant: {:.3}\n", dk.lipschitz_constant());

    // 4. Sweep Operator Demo
    println!("4. Sweep Operator Demo");
    println!("   Formula: SW(v) = g_τ(m(v)) · v");
    let mut sweep = Sweep::new(0.5, 0.1, "cosine".to_string());
    let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let result = sweep.apply(&v);

    println!("   Input:  {:?}", v.to_vec());
    println!("   Output: {:?}", result.to_vec());
    println!("   Current tau: {:.3}\n", sweep.compute_schedule());

    // 5. Pfadinvarianz Operator Demo
    println!("5. Pfadinvarianz Operator Demo");
    println!("   Formula: PI(v) = (1/|Π|) Σ_{{p∈Π}} T_p(v)");
    let pi = Pfadinvarianz::new("lexicographic".to_string(), 1e-6);
    let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let result = pi.apply(&v);

    println!("   Input:  {:?}", v.to_vec());
    println!("   Output: {:?}", result.to_vec());
    println!("   Is contractive: {}\n", pi.is_contractive());

    // 6. Weight-Transfer Operator Demo
    println!("6. Weight-Transfer Operator Demo");
    println!("   Formula: WT(v) = Σ_{{ℓ∈L}} w'_ℓ · P_ℓ(v)");
    let mut wt = WeightTransfer::default();
    let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let result = wt.apply(&v);

    println!("   Input:  {:?}", v.to_vec());
    println!("   Output: {:?}", result.to_vec());
    println!("   Lipschitz constant: {:.3}\n", wt.lipschitz_constant());

    println!("=== All operators executed successfully! ===");

    Ok(())
}
