/*!
 * Quantum-Hybrid Operators Core Library
 *
 * Eine modulare, gut dokumentierte Bibliothek für quantenresonante Operatoren,
 * die universell als Basistechnologie in neuen Projekten genutzt werden kann.
 *
 * # Überblick
 *
 * Diese Bibliothek bietet eine Sammlung von mathematischen und kryptographischen
 * Operatoren für Quantum-Hybrid Systeme:
 *
 * ## Core Operatoren
 *
 * ### 1. Masking Operator (M)
 * - **Formel**: `M_{θ,σ}(m) = e^{iθ} U_σ m`
 * - **Use Case**: Addressless encryption für Ghost Network
 * - **Eigenschaft**: Selbst-invers (Involution)
 *
 * ### 2. Resonance Operator (R_ε)
 * - **Formel**: `R_ε(ψ₁, ψ₂) = 1 if d(ψ₁, ψ₂) < ε, else 0`
 * - **Use Case**: Resonanz-basiertes Routing und Node Discovery
 * - **Eigenschaft**: 3D-Tripolar-Zustand (ψ, ρ, ω)
 *
 * ### 3. DoubleKick (DK)
 * - **Formel**: `DK(v) = v + α₁u₁ + α₂u₂`
 * - **Use Case**: Local unsticking in Optimierung
 * - **Eigenschaft**: Non-expansive, orthogonale Impulse
 *
 * ### 4. Sweep (SW)
 * - **Formel**: `SW(v) = g_τ(m(v)) · v`
 * - **Use Case**: Threshold sweeping mit Schedule
 * - **Eigenschaft**: Sigmoid gate, cosine schedule
 *
 * ### 5. Pfadinvarianz (PI)
 * - **Formel**: `PI(v) = (1/|Π|) Σ_{p∈Π} T_p(v)`
 * - **Use Case**: Path-equivalence projection
 * - **Eigenschaft**: Idempotent, non-expansive
 *
 * ### 6. Weight-Transfer (WT)
 * - **Formel**: `WT(v) = Σ_{ℓ∈L} w'_ℓ · P_ℓ(v)`
 * - **Use Case**: Multi-scale weight redistribution
 * - **Eigenschaft**: Convex combination
 *
 * # Beispiel
 *
 * ```rust
 * use quantumhybrid_operatoren_core::prelude::*;
 *
 * // Masking Operator
 * let masking = MaskingOperator::new();
 * let params = MaskingParams::random();
 * let message = b"Secret message";
 * let masked = masking.mask(message, &params).unwrap();
 * let unmasked = masking.unmask(&masked, &params).unwrap();
 *
 * // Resonance Operator
 * let resonance = ResonanceOperator::new();
 * let state1 = ResonanceState::new(1.0, 0.8, 0.5);
 * let state2 = ResonanceState::new(1.05, 0.82, 0.53);
 * let window = ResonanceWindow::standard();
 * let is_resonant = resonance.is_resonant(&state1, &state2, &window);
 * ```
 *
 * # Architektur
 *
 * Die Bibliothek ist modular aufgebaut:
 *
 * - **core**: Trait-Definitionen und gemeinsame Funktionalität
 * - **operators**: Individuelle Operator-Implementierungen
 * - **examples**: Verwendungsbeispiele
 * - **integration**: Integrationshilfen für externe Projekte
 *
 * # Features
 *
 * - **std** (default): Standard library support
 * - **no_std**: No standard library (für embedded systems)
 */

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod core;
pub mod operators;

/// Prelude - Re-export der wichtigsten Typen
pub mod prelude {
    // Core Traits
    pub use crate::core::{
        ComposableOperator, ContractiveOperator, IdempotentOperator, InvertibleOperator,
        OperatorError, QuantumOperator, ResonanceOperator, StatefulOperator, UnitaryOperator,
    };

    // Operators
    pub use crate::operators::{
        DoubleKick, DoubleKickInfo, DoubleKickParams, MaskingOperator, MaskingParams,
        Pfadinvarianz, PfadinvarianzParams, QuantumState, QuantumStateParams,
        QuantumUnitaryOperator, ResonanceInput, ResonanceOperator, ResonanceState,
        ResonanceWindow, ScaleLevel, Sweep, SweepParams, WeightTransfer, WeightTransferParams,
        METATRON_DIMENSION,
    };
}

// Re-export wichtiger Typen auf oberster Ebene
pub use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        // Smoke test um sicherzustellen, dass die Bibliothek kompiliert
        let _masking = MaskingOperator::new();
        let _resonance = ResonanceOperator::new();
        let _dk = DoubleKick::default();
        let _sweep = Sweep::default();
        let _pi = Pfadinvarianz::default();
        let _wt = WeightTransfer::default();
        let _qop = QuantumUnitaryOperator::identity();
        let _qstate = QuantumState::basis_state(0).unwrap();
    }
}
