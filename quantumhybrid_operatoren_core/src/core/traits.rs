/*!
 * Core Traits für Quantum-Resonant Operators
 *
 * Dieses Modul definiert die grundlegenden Traits, die alle Operatoren
 * im Framework implementieren müssen.
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Haupt-Trait für Quantum-Resonant Operatoren
///
/// Jeder Operator transformiert einen Input zu einem Output unter Verwendung
/// von Parametern. Operatoren sollten deterministisch sein (gleiche Parameter
/// + Input → gleicher Output).
///
/// # Mathematische Notation
/// Ein Operator O transformiert: O: (I, P) → O
/// wobei I = Input, P = Parameter, O = Output
///
/// # Eigenschaften
/// - **Determinismus**: O(i, p) ist eindeutig für gegebene (i, p)
/// - **Komponierbarkeit**: Operatoren können komponiert werden: O₂(O₁(i, p₁), p₂)
/// - **Invertierbarkeit**: Manche Operatoren sind invertierbar: O⁻¹(O(i, p), p) = i
///
/// # Beispiel
/// ```ignore
/// use quantumhybrid_operatoren_core::core::QuantumOperator;
///
/// struct MyOperator;
///
/// impl QuantumOperator for MyOperator {
///     type Input = Vec<u8>;
///     type Output = Vec<u8>;
///     type Params = f64;
///
///     fn apply(&self, input: Self::Input, params: &Self::Params) -> Result<Self::Output> {
///         // Operator-Logik hier
///         Ok(input)
///     }
/// }
/// ```
pub trait QuantumOperator: Send + Sync {
    /// Input-Typ des Operators
    type Input: Clone;

    /// Output-Typ des Operators
    type Output;

    /// Parameter-Typ des Operators
    type Params: Clone + Debug;

    /// Wendet den Operator auf einen Input an
    ///
    /// # Arguments
    /// * `input` - Der Input-Wert
    /// * `params` - Die Operator-Parameter
    ///
    /// # Returns
    /// Der transformierte Output oder ein Fehler
    fn apply(&self, input: Self::Input, params: &Self::Params) -> Result<Self::Output>;

    /// Gibt den Namen des Operators zurück
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// Gibt eine Beschreibung des Operators zurück
    fn description(&self) -> &str {
        "No description available"
    }

    /// Gibt die mathematische Formel des Operators als String zurück
    fn formula(&self) -> &str {
        "No formula available"
    }
}

/// Trait für invertierbare Operatoren
///
/// Einige Operatoren sind invertierbar, d.h. es existiert eine Umkehrfunktion
/// die den ursprünglichen Input wiederherstellt.
///
/// # Mathematische Notation
/// O⁻¹(O(i, p), p) = i
///
/// # Beispiel
/// Der Masking Operator ist eine Involution: M(M(m, p), p) = m
pub trait InvertibleOperator: QuantumOperator {
    /// Invertiert die Operator-Transformation
    ///
    /// # Arguments
    /// * `output` - Der Output einer vorherigen `apply`-Operation
    /// * `params` - Die gleichen Parameter wie bei `apply`
    ///
    /// # Returns
    /// Der ursprüngliche Input oder ein Fehler
    fn invert(&self, output: Self::Output, params: &Self::Params) -> Result<Self::Input>;
}

/// Trait für unitäre Operatoren
///
/// Unitäre Operatoren erhalten die Norm: ||O(v)|| = ||v||
/// Dies ist wichtig für Quantenoperatoren auf Zustandsvektoren.
///
/// # Mathematische Notation
/// O ist unitär wenn: O†O = OO† = I
/// wobei O† die adjungierte Matrix ist
pub trait UnitaryOperator: QuantumOperator {
    /// Prüft ob der Operator unitär ist
    ///
    /// # Arguments
    /// * `tolerance` - Numerische Toleranz für die Prüfung
    ///
    /// # Returns
    /// true wenn unitär (innerhalb der Toleranz), sonst false
    fn is_unitary(&self, tolerance: f64) -> bool;
}

/// Trait für kontraktive Operatoren (non-expansive)
///
/// Kontraktive Operatoren verkleinern oder erhalten Distanzen:
/// d(O(x), O(y)) ≤ d(x, y)
///
/// # Mathematische Notation
/// Ein Operator O ist kontraktiv wenn:
/// ||O(x) - O(y)|| ≤ ||x - y|| für alle x, y
///
/// Lipschitz-Konstante L ≤ 1
pub trait ContractiveOperator: QuantumOperator {
    /// Berechnet die Lipschitz-Konstante des Operators
    ///
    /// # Returns
    /// Die Lipschitz-Konstante L (sollte ≤ 1.0 sein für kontraktive Operatoren)
    fn lipschitz_constant(&self) -> f64;

    /// Prüft ob der Operator kontraktiv ist
    fn is_contractive(&self) -> bool {
        self.lipschitz_constant() <= 1.0
    }
}

/// Trait für idempotente Operatoren
///
/// Idempotente Operatoren erfüllen: O(O(x)) = O(x)
/// Mehrfache Anwendung ändert das Ergebnis nicht.
///
/// # Mathematische Notation
/// O² = O (Projektor-Eigenschaft)
pub trait IdempotentOperator: QuantumOperator {
    /// Prüft ob der Operator idempotent ist
    ///
    /// # Arguments
    /// * `input` - Test-Input
    /// * `params` - Operator-Parameter
    /// * `tolerance` - Numerische Toleranz
    ///
    /// # Returns
    /// true wenn O(O(x)) ≈ O(x), sonst false
    fn is_idempotent(
        &self,
        input: &Self::Input,
        params: &Self::Params,
        tolerance: f64,
    ) -> Result<bool>;
}

/// Trait für komponierbare Operatoren
///
/// Ermöglicht die Komposition von Operatoren: O₂ ∘ O₁
pub trait ComposableOperator: QuantumOperator {
    /// Komponiert diesen Operator mit einem anderen
    ///
    /// # Arguments
    /// * `other` - Der andere Operator (wird zuerst angewendet)
    ///
    /// # Returns
    /// Ein neuer Operator der die Komposition repräsentiert
    fn compose<O>(&self, other: &O) -> Result<Box<dyn QuantumOperator<Input = O::Input, Output = Self::Output, Params = (O::Params, Self::Params)>>>
    where
        O: QuantumOperator<Output = Self::Input>;
}

/// Marker-Trait für Resonanz-basierte Operatoren
///
/// Diese Operatoren nutzen Resonanz-Mechanismen zur Entscheidungsfindung
pub trait ResonanceOperator: QuantumOperator {
    /// Resonanz-Zustandstyp
    type ResonanceState: Clone + Debug;

    /// Berechnet die Resonanzstärke zwischen zwei Zuständen
    ///
    /// # Returns
    /// Resonanzstärke im Bereich [0.0, 1.0]
    fn compute_resonance(
        &self,
        state1: &Self::ResonanceState,
        state2: &Self::ResonanceState,
    ) -> f64;
}

/// Statistiken über Operator-Ausführung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorStats {
    /// Anzahl der Ausführungen
    pub executions: u64,
    /// Durchschnittliche Ausführungszeit (Nanosekunden)
    pub avg_duration_ns: u64,
    /// Anzahl der Fehler
    pub errors: u64,
    /// Letzte Ausführungszeit
    pub last_execution: Option<u64>,
}

impl Default for OperatorStats {
    fn default() -> Self {
        Self {
            executions: 0,
            avg_duration_ns: 0,
            errors: 0,
            last_execution: None,
        }
    }
}

/// Trait für Operatoren mit Statistik-Tracking
pub trait StatefulOperator: QuantumOperator {
    /// Gibt die Ausführungsstatistiken zurück
    fn stats(&self) -> &OperatorStats;

    /// Setzt die Statistiken zurück
    fn reset_stats(&mut self);
}

/// Hilfsfunktion zum Berechnen der euklidischen Distanz
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Hilfsfunktion zum Normalisieren eines Vektors
pub fn normalize_vector(v: &mut [f64]) {
    let norm = v.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(euclidean_distance(&a, &b), 1.0);

        let c = vec![0.0, 0.0, 0.0];
        let d = vec![1.0, 1.0, 1.0];
        assert!((euclidean_distance(&c, &d) - 3.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_vector() {
        let mut v = vec![3.0, 4.0, 0.0];
        normalize_vector(&mut v);
        let norm = v.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_operator_stats_default() {
        let stats = OperatorStats::default();
        assert_eq!(stats.executions, 0);
        assert_eq!(stats.errors, 0);
    }
}
