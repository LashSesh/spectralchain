/*!
 * MEF Quantum Operations
 *
 * Quantenresonante Operatoren für das Ghost Network nach dem Blueprint
 * "Quantenresonante Spektralfeld-Blockchain" (Klemm, 2025).
 *
 * Implementiert:
 * - Masking Operator (M): M_{θ,σ}(m) = e^{iθ} U_σ m
 * - Resonance Operator (R_ε): Resonanzfenster-Prüfung
 * - Steganography Operator (T): T(m') = Embed(m', Carrier)
 * - Zero-Knowledge Operator (ZK): ZK(a, pk) = (Proof(Eigenschaft), masked a)
 */

pub mod error;
pub mod masking;
pub mod resonance;
pub mod steganography;
pub mod zk_proofs;

pub use error::{QuantumOpsError, Result};
pub use masking::{MaskingOperator, MaskingParams};
pub use resonance::{ResonanceOperator, ResonanceWindow};
pub use steganography::{CarrierType, SteganographyOperator};
pub use zk_proofs::{ZKProof, ZKProofOperator};

/// Quantenresonante Operator-Trait
///
/// Alle Operatoren implementieren dieses Trait für konsistente API
pub trait QuantumOperator {
    type Input;
    type Output;
    type Params;

    fn apply(&self, input: Self::Input, params: &Self::Params) -> Result<Self::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_ops_available() {
        // Basic smoke test
        assert!(true, "Quantum ops module loaded successfully");
    }
}
