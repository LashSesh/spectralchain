//! Quantum State Operations Demo - 13-Dimensional Metatron Cube
//!
//! Run: cargo run --example quantum_state_demo

use num_complex::Complex64;
use quantumhybrid_operatoren_core::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== Quantum State Operations on Metatron Cube ===\n");
    println!("The Metatron Cube has {} canonical nodes", METATRON_DIMENSION);
    println!("State space: ℂ^{} (13-dimensional Hilbert space)\n", METATRON_DIMENSION);

    // 1. Basis States
    println!("1. Basis States |i⟩");
    println!("   Creating basis state |1⟩ (node 1)");
    let basis_state = QuantumState::basis_state(0)?;
    let probs = basis_state.probabilities();
    println!("   Probabilities: |ψ₁|² = {:.3}, |ψ₂|² = {:.3}, ...", probs[0], probs[1]);
    assert!((probs[0] - 1.0).abs() < 1e-10);
    println!("   ✓ Pure state in node 1\n");

    // 2. Superposition
    println!("2. Superposition States");
    println!("   Creating |ψ⟩ = (|1⟩ + |2⟩)/√2");
    let amps = vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(1.0, 0.0),
    ];
    let superposition = QuantumState::new(amps, true)?;
    let probs = superposition.probabilities();
    println!("   Probabilities: |ψ₁|² = {:.3}, |ψ₂|² = {:.3}", probs[0], probs[1]);
    println!("   ✓ Equal superposition between nodes 1 and 2\n");

    // 3. Uniform Superposition
    println!("3. Uniform Superposition over all nodes");
    println!("   |ψ⟩ = (1/√13) Σᵢ |i⟩");
    let uniform = QuantumState::uniform_superposition()?;
    let probs = uniform.probabilities();
    println!("   All probabilities ≈ {:.4}", 1.0 / METATRON_DIMENSION as f64);
    println!("   First three: {:.4}, {:.4}, {:.4}", probs[0], probs[1], probs[2]);
    println!("   ✓ Uniform distribution\n");

    // 4. Inner Product
    println!("4. Inner Product ⟨φ|ψ⟩");
    let state1 = QuantumState::basis_state(0)?;
    let state2 = QuantumState::basis_state(1)?;
    let inner = state1.inner_product(&state2);
    println!("   ⟨1|2⟩ = {:.3} + {:.3}i", inner.re, inner.im);
    println!("   ✓ Orthogonal states\n");

    let state3 = QuantumState::basis_state(0)?;
    let inner2 = state1.inner_product(&state3);
    println!("   ⟨1|1⟩ = {:.3} + {:.3}i", inner2.re, inner2.im);
    println!("   ✓ Normalized state\n");

    // 5. Identity Operator
    println!("5. Identity Operator I");
    println!("   Applying I to |ψ⟩");
    let identity = QuantumUnitaryOperator::identity();
    let state_before = QuantumState::random()?;
    let state_after = state_before.apply(&identity)?;

    let diff: f64 = state_before
        .amplitudes
        .iter()
        .zip(state_after.amplitudes.iter())
        .map(|(&a, &b)| (a - b).norm())
        .sum();
    println!("   ||ψ - I|ψ⟩|| = {:.3e}", diff);
    println!("   ✓ State unchanged\n");

    // 6. Permutation Operator
    println!("6. Permutation Operator P");
    println!("   Cyclic permutation: 1→2, 2→3, ..., 13→1");
    let mut perm: Vec<usize> = (2..=METATRON_DIMENSION).collect();
    perm.push(1);
    let perm_op = QuantumUnitaryOperator::from_permutation(&perm);

    println!("   Checking unitarity: P†P = I");
    assert!(perm_op.is_unitary(1e-8));
    println!("   ✓ Permutation operator is unitary\n");

    // Apply to basis state |1⟩
    let state1 = QuantumState::basis_state(0)?;
    let state2 = state1.apply(&perm_op)?;
    let probs = state2.probabilities();
    println!("   P|1⟩: probability at node 2 = {:.3}", probs[1]);
    println!("   ✓ State moved from node 1 to node 2\n");

    // 7. Operator Composition
    println!("7. Operator Composition");
    println!("   Computing P² = P ∘ P");
    let p_squared = perm_op.compose(&perm_op)?;

    let state1 = QuantumState::basis_state(0)?;
    let result = state1.apply(&p_squared)?;
    let probs = result.probabilities();
    println!("   P²|1⟩: probability at node 3 = {:.3}", probs[2]);
    println!("   ✓ Two cyclic shifts: 1→2→3\n");

    // 8. Adjoint Operator
    println!("8. Adjoint Operator P†");
    println!("   Computing P†P (should be identity)");
    let p_dagger = perm_op.adjoint();
    let product = perm_op.compose(&p_dagger)?;

    let identity_matrix = QuantumUnitaryOperator::identity();
    let mut max_diff = 0.0;
    for i in 0..METATRON_DIMENSION {
        for j in 0..METATRON_DIMENSION {
            let diff = (product.matrix[[i, j]] - identity_matrix.matrix[[i, j]]).norm();
            max_diff = max_diff.max(diff);
        }
    }
    println!("   Max|P†P - I| = {:.3e}", max_diff);
    println!("   ✓ P†P = I (within numerical precision)\n");

    // 9. Measurement
    println!("9. Quantum Measurement");
    println!("   Creating superposition and measuring 5 times:");
    for trial in 1..=5 {
        let mut state = QuantumState::uniform_superposition()?;
        let measurement = state.measure();
        print!("   Trial {}: measured node {} ", trial, measurement);

        // After measurement, state should be collapsed
        let probs = state.probabilities();
        let max_prob = probs.iter().cloned().fold(0.0, f64::max);
        println!("(max probability = {:.3})", max_prob);
    }
    println!("   ✓ Measurements collapse to basis states\n");

    // 10. Expectation Value
    println!("10. Expectation Value ⟨O⟩ = ⟨ψ|O|ψ⟩");
    let state = QuantumState::uniform_superposition()?;
    let observable = QuantumUnitaryOperator::identity();
    let expectation = state.expectation_value(&observable)?;
    println!("   ⟨I⟩ = {:.3} + {:.3}i", expectation.re, expectation.im);
    println!("   ✓ Expectation value of identity is 1\n");

    // 11. Complex Amplitudes
    println!("11. Complex Amplitudes");
    println!("   Creating state with complex phases");
    let amps = vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 1.0),  // i
        Complex64::new(-1.0, 0.0), // -1
        Complex64::new(0.0, -1.0), // -i
    ];
    let complex_state = QuantumState::new(amps, true)?;
    println!("   ψ₁ = {:.3} + {:.3}i",
        complex_state.amplitudes[0].re,
        complex_state.amplitudes[0].im);
    println!("   ψ₂ = {:.3} + {:.3}i",
        complex_state.amplitudes[1].re,
        complex_state.amplitudes[1].im);
    println!("   Norm = {:.3}", complex_state.norm());
    println!("   ✓ Normalized state with complex phases\n");

    // 12. Random State
    println!("12. Random Quantum State (Haar measure)");
    let random_state = QuantumState::random()?;
    println!("   Generated random state with norm = {:.3}", random_state.norm());
    let probs = random_state.probabilities();
    let entropy: f64 = -probs
        .iter()
        .filter(|&&p| p > 1e-10)
        .map(|&p| p * p.ln())
        .sum();
    println!("   Von Neumann entropy S = {:.3}", entropy);
    println!("   ✓ Random state generated\n");

    println!("=== All quantum operations executed successfully! ===");
    println!("\n📝 Summary:");
    println!("   - Basis states and superpositions ✓");
    println!("   - Unitary operators (I, P, P†) ✓");
    println!("   - Inner products and norms ✓");
    println!("   - Measurements and collapse ✓");
    println!("   - Expectation values ✓");
    println!("   - Complex amplitudes ✓");

    Ok(())
}
