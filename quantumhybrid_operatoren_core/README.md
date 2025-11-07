# Quantum-Hybrid Operators Core Library

Eine modulare, gut dokumentierte Bibliothek für quantenresonante Operatoren, die universell als Basistechnologie in neuen Projekten genutzt werden kann.

## 🎯 Überblick

Diese Bibliothek extrahiert und modularisiert alle quantenresonanten Operatoren und Mechaniken aus dem SpectralChain-Ökosystem in ein sauberes, wiederverwendbares Framework.

## 📦 Operatoren-Portfolio

### 1. **Masking Operator (M)**

**Mathematische Formel:**
```
M_{θ,σ}(m) = e^{iθ} U_σ m
```

**Beschreibung:**
- Permutation + Phasenrotation für addressless encryption
- Selbst-invers (Involution): `M(M(m, p), p) = m`
- Forward Secrecy mit ephemeren Schlüsseln

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;

let operator = MaskingOperator::new();
let params = MaskingParams::random();
let message = b"Secret message";

// Mask
let masked = operator.mask(message, &params).unwrap();

// Unmask
let unmasked = operator.unmask(&masked, &params).unwrap();
assert_eq!(unmasked, message);
```

**Use Cases:**
- Addressless encryption für Ghost Network
- Privacy-preserving message routing
- Stealth addressing

---

### 2. **Resonance Operator (R_ε)**

**Mathematische Formel:**
```
R_ε(ψ₁, ψ₂) = 1 if d(ψ₁, ψ₂) < ε, else 0
d(ψ₁, ψ₂) = √[(ψ₁-ψ₂)² + (ρ₁-ρ₂)² + (ω₁-ω₂)²]
```

**Beschreibung:**
- 3D-Tripolar-Zustand: (ψ, ρ, ω) Gabriel Cells
- Euclidean distance metric mit konfigurierbaren Epsilon-Fenstern
- Kollektiv-Resonanz für Gruppenentscheidungen

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;

let operator = ResonanceOperator::new();
let window = ResonanceWindow::standard();

let node_state = ResonanceState::new(1.0, 0.8, 0.5);
let packet_state = ResonanceState::new(1.05, 0.82, 0.53);

// Check resonance
let is_resonant = operator.is_resonant(&node_state, &packet_state, &window);

// Get resonance strength (0.0 - 1.0)
let strength = operator.resonance_strength(&node_state, &packet_state, &window);
```

**Use Cases:**
- Addressless routing im Ghost Network
- Consensus-Finding via resonance alignment
- Privacy-preserving node discovery
- Decentralized decision making

---

### 3. **DoubleKick (DK)**

**Mathematische Formel:**
```
DK(v) = v + α₁u₁ + α₂u₂
```

Wobei:
- `⟨u₁, u₂⟩ = 0` (orthogonal)
- `||u_i||₂ = 1` (unit vectors)
- `|α₁| + |α₂| ≤ η ≪ 1` (non-expansive)

**Beschreibung:**
- Local unsticking durch duale orthogonale Impulse
- Non-expansive: Lipschitz-Konstante ≈ 1 + η mit η ≪ 1

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

let dk = DoubleKick::new(0.05, -0.03);
let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
let result = dk.apply(&v);
```

**Use Cases:**
- Escape local minima in optimization
- Perturbation for exploring solution space
- Fixed-point iteration improvements

---

### 4. **Sweep (SW)**

**Mathematische Formel:**
```
SW(v) = g_τ(m(v)) · v
g_τ(x) = σ((x - τ)/β)
τ_t = τ₀ + 0.5(1 + cos(πt/T))Δτ
```

**Beschreibung:**
- Sigmoid gate mit cosine/linear schedule
- Threshold evolution über Zeit

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

let mut sweep = Sweep::new(0.5, 0.1, "cosine".to_string());
let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
let result = sweep.apply(&v);
```

**Use Cases:**
- Adaptive thresholding in signal processing
- Scheduled gating in neural networks
- Progressive filtering

---

### 5. **Pfadinvarianz (PI)**

**Mathematische Formel:**
```
PI(v) = (1/|Π|) Σ_{p∈Π} T_p(v)
```

**Beschreibung:**
- Path-equivalent permutation averaging
- Idempotent: `PI(PI(v)) = PI(v)`
- Non-expansive projection

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

let pi = Pfadinvarianz::new("lexicographic".to_string(), 1e-6);
let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
let result = pi.apply(&v);
```

**Use Cases:**
- Canonical ordering enforcement
- Path-independent computations
- Symmetry-preserving projections

---

### 6. **Weight-Transfer (WT)**

**Mathematische Formel:**
```
WT(v) = Σ_{ℓ∈L} w'_ℓ · P_ℓ(v)
w'_ℓ = (1-γ)w_ℓ + γw̃_ℓ
```

**Beschreibung:**
- Multi-scale convex combination (Micro, Meso, Macro)
- Adaptive weight redistribution

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

let mut wt = WeightTransfer::default();
let v = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
let result = wt.apply(&v);
```

**Use Cases:**
- Multi-resolution signal processing
- Hierarchical feature extraction
- Scale-aware transformations

---

### 7. **Quantum State Operator (QS)**

**Mathematische Formel:**
```
|ψ⟩ = Σᵢ αᵢ|i⟩,  i ∈ {1, 2, ..., 13}
|ψ'⟩ = U|ψ⟩,  wobei U†U = I
```

**Beschreibung:**
- 13-dimensionaler Hilbert-Raum auf Metatron Cube
- Quantenmechanische Zustände und unitäre Operatoren
- Superposition, Messung, Verschränkung
- Normalisierung: Σᵢ |αᵢ|² = 1

**Anwendung:**
```rust
use quantumhybrid_operatoren_core::prelude::*;
use num_complex::Complex64;

// Create quantum state
let amps = vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)];
let state = QuantumState::new(amps, true)?;

// Apply unitary operator
let permutation = vec![2, 3, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
let operator = QuantumUnitaryOperator::from_permutation(&permutation);
let new_state = state.apply(&operator)?;

// Measure
let mut measurement_state = state.clone();
let node = measurement_state.measure();
```

**Use Cases:**
- Post-symbolic cognition (Theory of Everything)
- Quantum-inspired consensus algorithms
- Entanglement across multiple cubes
- Symmetry-preserving transformations

**Spezielle Features:**
- Basis states: `|i⟩` für jeden der 13 Nodes
- Uniform superposition: `|ψ⟩ = (1/√13) Σᵢ |i⟩`
- Permutation operators aus Symmetriegruppen
- Measurement collapse mit Wahrscheinlichkeit P(i) = |αᵢ|²
- Inner product: `⟨φ|ψ⟩`
- Expectation values: `⟨O⟩ = ⟨ψ|O|ψ⟩`

---

## 🏗️ Architektur

```
quantumhybrid_operatoren_core/
├── src/
│   ├── core/                  # Core trait definitions
│   │   ├── traits.rs          # QuantumOperator, InvertibleOperator, etc.
│   │   └── mod.rs
│   ├── operators/             # Individual operators
│   │   ├── masking.rs
│   │   ├── resonance.rs
│   │   ├── doublekick.rs
│   │   ├── sweep.rs
│   │   ├── pfadinvarianz.rs
│   │   ├── weight_transfer.rs
│   │   ├── quantum_state.rs
│   │   └── mod.rs
│   └── lib.rs                 # Main library
├── examples/
│   ├── basic_usage.rs
│   └── quantum_state_demo.rs
├── tests/
│   └── integration_tests.rs
├── docs/
│   ├── ARCHITECTURE.md
│   └── INTEGRATION.md
├── Cargo.toml
└── README.md
```

## 🔧 Installation

Füge zu deiner `Cargo.toml` hinzu:

```toml
[dependencies]
quantumhybrid_operatoren_core = { path = "../quantumhybrid_operatoren_core" }
```

Oder aus dem SpectralChain Workspace:

```toml
[dependencies]
quantumhybrid_operatoren_core = { version = "0.1.0" }
```

## 📚 Core Traits

### `QuantumOperator`
Haupt-Trait für alle Operatoren:
```rust
pub trait QuantumOperator: Send + Sync {
    type Input: Clone;
    type Output;
    type Params: Clone + Debug;

    fn apply(&self, input: Self::Input, params: &Self::Params) -> Result<Self::Output>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn formula(&self) -> &str;
}
```

### `InvertibleOperator`
Für invertierbare Operatoren:
```rust
pub trait InvertibleOperator: QuantumOperator {
    fn invert(&self, output: Self::Output, params: &Self::Params) -> Result<Self::Input>;
}
```

### `ContractiveOperator`
Für non-expansive Operatoren:
```rust
pub trait ContractiveOperator: QuantumOperator {
    fn lipschitz_constant(&self) -> f64;
    fn is_contractive(&self) -> bool;
}
```

### `IdempotentOperator`
Für idempotente Operatoren:
```rust
pub trait IdempotentOperator: QuantumOperator {
    fn is_idempotent(&self, input: &Self::Input, params: &Self::Params, tolerance: f64) -> Result<bool>;
}
```

## 🚀 Quick Start

```rust
use quantumhybrid_operatoren_core::prelude::*;

fn main() -> anyhow::Result<()> {
    // Masking Operator
    let masking = MaskingOperator::new();
    let params = MaskingParams::from_seed(b"my_seed");
    let message = b"Hello, Quantum World!";

    let masked = masking.mask(message, &params)?;
    let unmasked = masking.unmask(&masked, &params)?;
    assert_eq!(unmasked, message);

    // Resonance Operator
    let resonance = ResonanceOperator::new();
    let state1 = ResonanceState::new(1.0, 0.8, 0.5);
    let state2 = ResonanceState::new(1.05, 0.82, 0.53);
    let window = ResonanceWindow::standard();

    if resonance.is_resonant(&state1, &state2, &window) {
        println!("States are resonant!");
    }

    Ok(())
}
```

## 🔗 Integration in andere Projekte

### Blockchain
```rust
use quantumhybrid_operatoren_core::prelude::*;

// Use resonance for consensus
let operator = ResonanceOperator::new();
let node_states: Vec<ResonanceState> = get_network_states();
let proposal_state = get_proposal_state();

let consensus = operator.collective_resonance(
    &node_states,
    &proposal_state,
    &ResonanceWindow::standard(),
    0.66  // 2/3 majority
);
```

### Privacy Network
```rust
use quantumhybrid_operatoren_core::prelude::*;

// Addressless routing
let masking = MaskingOperator::new();
let params = MaskingParams::ephemeral(current_epoch());
let packet = masking.mask(data, &params)?;
```

### Machine Learning
```rust
use quantumhybrid_operatoren_core::prelude::*;
use ndarray::Array1;

// Multi-scale feature extraction
let mut wt = WeightTransfer::default();
let features = Array1::from(raw_features);
let transformed = wt.apply(&features);
```

## 📖 Dokumentation

Vollständige Dokumentation:
```bash
cargo doc --open
```

Siehe auch:
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Detaillierte Architektur
- [INTEGRATION.md](docs/INTEGRATION.md) - Integrationsleitfaden

## 🧪 Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# Mit Coverage
cargo tarpaulin --out Html
```

## 🔬 Benchmarks

```bash
cargo bench
```

## 📋 Features

- ✅ **Vollständig dokumentiert**: Jeder Operator mit Formeln, Beispielen und Use Cases
- ✅ **Typ-sicher**: Starkes Rust-Typ-System
- ✅ **Getestet**: Umfangreiche Unit- und Integration-Tests
- ✅ **Modular**: Jeder Operator einzeln verwendbar
- ✅ **Performant**: Zero-cost abstractions
- ✅ **Sicher**: Memory-safe, thread-safe

## 🛠️ Entwicklung

```bash
# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy

# Documentation
cargo doc --no-deps --open
```

## 📄 Lizenz

MIT OR Apache-2.0

## 🤝 Beiträge

Contributions sind willkommen! Siehe CONTRIBUTING.md für Details.

## 🔍 Konzeptionelle Lücken und TODOs

### Zukünftige Operatoren
- [ ] **Steganography Operator (T)**: Zero-width Unicode + LSB
- [ ] **Zero-Knowledge Proof Operator (ZK)**: Schnorr, Range Proofs
- [x] **Quantum State Operator**: 13-dimensional Hilbert space (Metatron Cube) ✅
- [ ] **Mandorla Attractor**: Fork resolution via coherence scoring

### Verbesserungen
- [ ] GPU-Beschleunigung für Matrix-Operationen
- [ ] SIMD-Optimierung
- [ ] No-std Support für embedded systems
- [ ] Python bindings via PyO3
- [ ] WebAssembly Support

### Dokumentation
- [ ] Tutorial-Serie
- [ ] Video-Demos
- [ ] Interaktive Jupyter Notebooks
- [ ] API-Referenz-Website

## 📊 Beispiel-Metriken

| Operator | Lipschitz | Invertierbar | Idempotent | Dimension |
|----------|-----------|--------------|------------|-----------|
| Masking (M) | ~1.0 | ✅ | ❌ | Variable |
| Resonance (R) | 1.0 | ❌ | ❌ | 3 |
| DoubleKick (DK) | 1.0 + η | ❌ | ❌ | 5 |
| Sweep (SW) | 1.0 | ❌ | ❌ | 5 |
| Pfadinvarianz (PI) | 1.0 | ❌ | ✅ | 5 |
| WeightTransfer (WT) | 1.0 | ❌ | ❌ | 5 |
| QuantumState (QS) | 1.0 | ✅ (U†) | ❌ | 13 |

## 🌟 Highlights

> **Universell einsetzbar**: Diese Operatoren bilden die Grundlage für quantenresonante Systeme in Blockchain, KI, Privacy-Netzwerken und mehr.

> **Mathematisch fundiert**: Jeder Operator basiert auf rigoroser mathematischer Theorie mit bewiesenen Eigenschaften.

> **Production-ready**: Getestet, dokumentiert und optimiert für den Einsatz in kritischen Systemen.

---

**Built with ❤️ by the SpectralChain Team**
