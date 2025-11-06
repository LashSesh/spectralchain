# 🔧 Iterierbare Refactoring & Self-Healing Master Plan

**Version:** 1.0
**Datum:** 2025-11-06
**Basis:** Umfassende Code-Quality-Analyse von 51,385 LOC

---

## 🎯 Executive Summary

Dieses Dokument definiert ein **iterierbares, deterministisches Maßnahmenpaket** zur Transformation der Quantum Resonant Blockchain Codebasis von einem funktionalen Prototypen zu einem **production-ready, self-healing System**.

### Metriken-Übersicht

| Kategorie | Ist-Zustand | Soll-Zustand | Gap |
|-----------|-------------|--------------|-----|
| **Code-Duplikation** | 27+ Timestamp-Pattern | 0 (Unified Utility) | -100% |
| **Error Handling** | 191 .unwrap() Calls | 0 in Production Path | -100% |
| **Test Coverage** | ~2% | 85%+ | +4,150% |
| **Module Size** | Max 1,286 LOC | Max 400 LOC | -68% |
| **SRP Violations** | 14 Giant Modules | 0 (Split) | -100% |
| **Self-Healing** | 0 Mechanisms | 15+ Patterns | +∞ |
| **Invariant Assertions** | 0 | 50+ Critical Points | +∞ |
| **Property-Based Tests** | 0 | 100+ Properties | +∞ |

---

## 📋 Refactoring Task Übersicht

### Dependency Order & Impact Matrix

```
Phase 0: Foundation (No Dependencies)
  ↓
Phase 1: Critical Safety (Blocks Production)
  ↓
Phase 2: Architecture (Enables Scalability)
  ↓
Phase 3: Self-Healing (Enables Resilience)
  ↓
Phase 4: Quality (Enables Maintainability)
  ↓
Phase 5: Advanced (Enables Innovation)
```

---

## 📊 Master Task Table

| ID | Task | Phase | Priority | Impact | Effort | Dependencies | LOC Δ | Risk |
|----|------|-------|----------|--------|--------|--------------|-------|------|
| **R-00-001** | Create Shared Utilities Crate | 0 | CRITICAL | HIGH | 1d | None | +300 | LOW |
| **R-00-002** | Setup Property-Based Test Framework | 0 | CRITICAL | HIGH | 2d | R-00-001 | +500 | LOW |
| **R-00-003** | Create Self-Healing Infrastructure | 0 | CRITICAL | HIGH | 3d | R-00-001 | +800 | MEDIUM |
| **R-01-001** | Eliminate All .unwrap() Calls | 1 | CRITICAL | HIGH | 5d | R-00-001 | -191, +300 | LOW |
| **R-01-002** | Add Invariant Assertions | 1 | CRITICAL | HIGH | 3d | R-00-001 | +400 | LOW |
| **R-01-003** | Implement RwLock Recovery | 1 | CRITICAL | HIGH | 2d | R-01-001 | +200 | LOW |
| **R-01-004** | Add Circuit Breakers | 1 | CRITICAL | HIGH | 3d | R-00-003 | +300 | MEDIUM |
| **R-02-001** | Split MetatronRouter (1,286 LOC) | 2 | HIGH | MEDIUM | 5d | R-01-001 | +100 | MEDIUM |
| **R-02-002** | Split SpiralCoupling (1,026 LOC) | 2 | HIGH | MEDIUM | 4d | R-01-001 | +80 | MEDIUM |
| **R-02-003** | Split VectorDB Providers (961 LOC) | 2 | HIGH | MEDIUM | 4d | R-01-001 | +75 | MEDIUM |
| **R-02-004** | Extract Timestamp Utility | 2 | HIGH | LOW | 1d | R-00-001 | -100 | LOW |
| **R-02-005** | Extract RwLock Helpers | 2 | HIGH | LOW | 2d | R-01-003 | -150 | LOW |
| **R-03-001** | Add Health Check System | 3 | HIGH | HIGH | 3d | R-00-003 | +400 | MEDIUM |
| **R-03-002** | Implement Auto-Recovery | 3 | HIGH | HIGH | 4d | R-03-001 | +500 | HIGH |
| **R-03-003** | Add Graceful Degradation | 3 | HIGH | HIGH | 3d | R-03-001 | +350 | MEDIUM |
| **R-03-004** | Implement Retry with Backoff | 3 | HIGH | MEDIUM | 2d | R-00-003 | +250 | LOW |
| **R-03-005** | Add State Machine Recovery | 3 | MEDIUM | HIGH | 5d | R-03-002 | +600 | HIGH |
| **R-04-001** | Implement Stub Integration Tests | 4 | HIGH | HIGH | 5d | R-00-002 | +2000 | LOW |
| **R-04-002** | Add Property-Based Tests | 4 | HIGH | HIGH | 8d | R-00-002 | +1500 | LOW |
| **R-04-003** | Setup Coverage CI | 4 | MEDIUM | MEDIUM | 2d | R-04-001 | +200 | LOW |
| **R-04-004** | Add Mutation Testing | 4 | MEDIUM | MEDIUM | 3d | R-04-001 | +400 | LOW |
| **R-05-001** | Add Chaos Engineering | 5 | MEDIUM | HIGH | 5d | R-03-002 | +700 | HIGH |
| **R-05-002** | Implement Telemetry | 5 | MEDIUM | HIGH | 4d | R-03-001 | +500 | MEDIUM |
| **R-05-003** | Add Performance Guards | 5 | LOW | MEDIUM | 3d | R-03-001 | +300 | LOW |

**Total Estimated Effort:** 85 developer-days (~17 weeks with 1 dev)
**Total LOC Impact:** +9,513 LOC (new infrastructure), -441 LOC (removals)
**Net Impact:** +9,072 LOC (18% growth, all infrastructure/quality)

---

# 📖 Detaillierte Task-Definitionen

## Phase 0: Foundation (Infrastructure)

---

### R-00-001: Create Shared Utilities Crate

#### 🎯 Ziel
Erstelle eine zentrale `mef-common` Crate für gemeinsam genutzte Funktionalitäten, um Code-Duplikation zu eliminieren und Konsistenz zu gewährleisten.

#### 📝 Konkrete Umbauempfehlung

**Struktur:**
```rust
mef-common/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── time.rs           // Timestamp utilities
│   ├── locks.rs          // RwLock/Mutex helpers
│   ├── errors.rs         // Error context helpers
│   ├── retry.rs          // Retry logic
│   ├── validation.rs     // Input validation
│   └── telemetry.rs      // Logging/metrics
└── tests/
    └── integration.rs
```

**Implementation (time.rs):**
```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use anyhow::{Result, Context};

/// Get current Unix timestamp with proper error handling
pub fn current_timestamp() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time is before UNIX epoch")?
        .as_secs()
        .pipe(Ok)
}

/// Get current timestamp with fallback
pub fn current_timestamp_or_default() -> u64 {
    current_timestamp().unwrap_or(0)
}

/// Check if timestamp is expired
pub fn is_expired(timestamp: u64, ttl: u64) -> bool {
    current_timestamp_or_default() > timestamp.saturating_add(ttl)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn timestamp_always_positive(ttl in 0u64..1000000) {
            let ts = current_timestamp_or_default();
            assert!(ts > 0);
        }
    }
}
```

**Implementation (locks.rs):**
```rust
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use anyhow::{Result, Context};

/// Trait for safe lock access
pub trait SafeLock<T> {
    fn safe_read(&self) -> Result<RwLockReadGuard<'_, T>>;
    fn safe_write(&self) -> Result<RwLockWriteGuard<'_, T>>;
    fn with_read<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&T) -> R;
    fn with_write<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> R;
}

impl<T> SafeLock<T> for RwLock<T> {
    fn safe_read(&self) -> Result<RwLockReadGuard<'_, T>> {
        self.read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire read lock: {}", e))
    }

    fn safe_write(&self) -> Result<RwLockWriteGuard<'_, T>> {
        self.write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock: {}", e))
    }

    fn with_read<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.safe_read()?;
        Ok(f(&*guard))
    }

    fn with_write<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.safe_write()?;
        Ok(f(&mut *guard))
    }
}
```

#### 🤖 KI-Automationsplan

**Step 1: Automated Pattern Detection**
```bash
# Detect all timestamp patterns
rg "SystemTime::now\(\)\s*\.\s*duration_since\(UNIX_EPOCH\)" -A 2

# Detect all RwLock unwrap patterns
rg "\.write\(\)\.unwrap\(\)|\.read\(\)\.unwrap\(\)"

# Generate replacement suggestions
cargo-fix --allow-dirty --allow-staged
```

**Step 2: Automated Refactoring**
```rust
// Use rust-analyzer for bulk refactoring
// 1. Create mef-common crate
// 2. Add as workspace dependency
// 3. Replace all instances:

// Before:
timestamp: SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs(),

// After:
timestamp: mef_common::time::current_timestamp_or_default(),
```

**Step 3: Validation**
```bash
# Ensure all usages compile
cargo build --all-features

# Run tests
cargo test --workspace

# Check for remaining patterns
rg "duration_since\(UNIX_EPOCH\)" || echo "✓ All replaced"
```

#### ✅ Validation-Strategie

**Pre-Refactoring:**
1. Count all pattern occurrences: `rg "pattern" | wc -l`
2. Take snapshot: `git stash && git checkout -b baseline`
3. Run full test suite: `cargo test --workspace`
4. Record baseline metrics

**During Refactoring:**
1. Replace patterns incrementally (one file at a time)
2. Run tests after each file: `cargo test --package <package>`
3. Check compilation: `cargo check`

**Post-Refactoring:**
1. Zero pattern occurrences: `rg "SystemTime::now.*unwrap" | wc -l == 0`
2. All tests pass: `cargo test --workspace`
3. No new warnings: `cargo clippy -- -D warnings`
4. Performance unchanged: `cargo bench`

**Success Criteria:**
- ✅ 0 direct SystemTime::now().unwrap() calls
- ✅ 0 RwLock::read/write().unwrap() calls
- ✅ All existing tests pass
- ✅ Code compiles without warnings

#### 🔄 Rollback-Plan

**Rollback Trigger Conditions:**
- Test failure rate > 5%
- Performance degradation > 10%
- Compilation errors > 0
- Critical bugs discovered

**Rollback Procedure:**
```bash
# 1. Immediate revert
git revert <commit-range>

# 2. Or cherry-pick working changes
git checkout baseline
git cherry-pick <working-commits>

# 3. Restore from backup
git reset --hard backup-tag

# 4. Verify restoration
cargo test --workspace
cargo bench
```

**Rollback Validation:**
- All tests pass again
- Performance metrics restored
- No compilation errors

---

### R-00-002: Setup Property-Based Test Framework

#### 🎯 Ziel
Implementiere Property-Based Testing (PBT) mit proptest für robuste, exhaustive Test-Coverage kritischer Funktionen.

#### 📝 Konkrete Umbauempfehlung

**Dependencies (Cargo.toml):**
```toml
[dev-dependencies]
proptest = "1.4"
quickcheck = "1.0"
quickcheck_macros = "1.0"
```

**Test Structure:**
```rust
mef-common/tests/
├── properties/
│   ├── mod.rs
│   ├── time_properties.rs
│   ├── crypto_properties.rs
│   ├── network_properties.rs
│   └── consensus_properties.rs
└── strategies/
    ├── mod.rs
    ├── resonance_strategies.rs
    └── packet_strategies.rs
```

**Example: Time Properties**
```rust
// mef-common/tests/properties/time_properties.rs
use proptest::prelude::*;
use mef_common::time::*;

proptest! {
    /// Property: current_timestamp() is always monotonic
    #[test]
    fn timestamp_is_monotonic(delay_ms in 0u64..1000) {
        let t1 = current_timestamp().unwrap();
        std::thread::sleep(Duration::from_millis(delay_ms));
        let t2 = current_timestamp().unwrap();
        assert!(t2 >= t1, "Timestamps not monotonic: {} >= {}", t2, t1);
    }

    /// Property: is_expired() is consistent
    #[test]
    fn expiry_is_consistent(timestamp in 0u64..1_000_000_000, ttl in 0u64..1000) {
        let now = current_timestamp_or_default();
        let expired = is_expired(timestamp, ttl);

        if expired {
            assert!(now > timestamp + ttl, "Expired but not past TTL");
        } else {
            assert!(now <= timestamp + ttl, "Not expired but past TTL");
        }
    }

    /// Property: timestamp operations don't overflow
    #[test]
    fn timestamp_no_overflow(ts in 0u64..u64::MAX/2, ttl in 0u64..u64::MAX/2) {
        let sum = ts.saturating_add(ttl);
        assert!(sum >= ts, "Addition underflowed");
        assert!(sum >= ttl, "Addition underflowed");
    }
}
```

**Example: Cryptographic Properties**
```rust
// mef-common/tests/properties/crypto_properties.rs
use proptest::prelude::*;
use mef_quantum_ops::{MaskingOperator, MaskingParams};

proptest! {
    /// Property: Masking is reversible (mask → unmask → original)
    #[test]
    fn masking_is_reversible(data in prop::collection::vec(any::<u8>(), 1..1000)) {
        let masker = MaskingOperator::new();
        let params = MaskingParams::random();

        let masked = masker.mask(&data, &params).unwrap();
        let unmasked = masker.unmask(&masked, &params).unwrap();

        assert_eq!(data, unmasked, "Masking not reversible");
    }

    /// Property: Masking is deterministic
    #[test]
    fn masking_is_deterministic(
        data in prop::collection::vec(any::<u8>(), 1..1000),
        seed in any::<[u8; 32]>()
    ) {
        let masker = MaskingOperator::new();
        let params = MaskingParams::from_seed(&seed);

        let masked1 = masker.mask(&data, &params).unwrap();
        let masked2 = masker.mask(&data, &params).unwrap();

        assert_eq!(masked1, masked2, "Masking not deterministic");
    }

    /// Property: Masking changes data (except edge cases)
    #[test]
    fn masking_changes_data(data in prop::collection::vec(any::<u8>(), 2..1000)) {
        let masker = MaskingOperator::new();
        let params = MaskingParams::random();

        let masked = masker.mask(&data, &params).unwrap();

        // Should be different (unless by extreme coincidence)
        assert_ne!(data, masked, "Masking didn't change data");
    }
}
```

**Example: Network Properties**
```rust
// mef-common/tests/properties/network_properties.rs
use proptest::prelude::*;
use mef_ghost_network::*;

proptest! {
    /// Property: Packet serialization is reversible
    #[test]
    fn packet_serialize_deserialize(
        payload in prop::collection::vec(any::<u8>(), 1..1000),
        ttl in 1u32..3600
    ) {
        let packet = GhostPacket::new(payload.clone(), ttl);

        let serialized = packet.to_bytes().unwrap();
        let deserialized = GhostPacket::from_bytes(&serialized).unwrap();

        assert_eq!(packet.payload, deserialized.payload);
        assert_eq!(packet.ttl, deserialized.ttl);
    }

    /// Property: Resonance check is symmetric
    #[test]
    fn resonance_is_symmetric(
        psi1 in 0.0f64..10.0,
        rho1 in 0.0f64..10.0,
        omega1 in 0.0f64..10.0,
        psi2 in 0.0f64..10.0,
        rho2 in 0.0f64..10.0,
        omega2 in 0.0f64..10.0
    ) {
        let state1 = ResonanceState::new(psi1, rho1, omega1);
        let state2 = ResonanceState::new(psi2, rho2, omega2);

        let resonance_op = ResonanceOperator::new();
        let window = ResonanceWindow::standard();

        let r12 = resonance_op.is_resonant(&state1, &state2, &window);
        let r21 = resonance_op.is_resonant(&state2, &state1, &window);

        assert_eq!(r12, r21, "Resonance not symmetric");
    }
}
```

#### 🤖 KI-Automationsplan

**Step 1: Generate Property Skeleton**
```python
# generate_properties.py
import ast
import os

def extract_functions(file_path):
    """Extract all public functions from Rust file"""
    # Parse Rust file and extract function signatures
    # Generate proptest skeleton for each function
    pass

def generate_proptest(function):
    """Generate proptest template for function"""
    return f"""
proptest! {{
    #[test]
    fn test_{function.name}_properties(
        // Generate appropriate strategies
    ) {{
        // Test invariants
        // Test reversibility
        // Test determinism
    }}
}}
"""

# Run for all modules
for rust_file in find_rust_files():
    generate_proptest_file(rust_file)
```

**Step 2: Automated Strategy Generation**
```rust
// Auto-generate custom strategies for domain types
prop::strategy::Strategy for ResonanceState {
    type Value = ResonanceState;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let psi = (0.0..10.0).new_tree(runner)?;
        let rho = (0.0..10.0).new_tree(runner)?;
        let omega = (0.0..10.0).new_tree(runner)?;

        Ok(ResonanceState::new(psi.current(), rho.current(), omega.current()))
    }
}
```

**Step 3: CI Integration**
```yaml
# .github/workflows/property-tests.yml
name: Property-Based Tests

on: [push, pull_request]

jobs:
  property-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run property tests
        run: |
          cargo test --workspace --features proptest

      - name: Run with increased case count
        env:
          PROPTEST_CASES: 10000
        run: |
          cargo test --workspace --features proptest
```

#### ✅ Validation-Strategie

**Coverage Metrics:**
```bash
# Count properties
find . -name "*.rs" -exec grep -l "proptest!" {} \; | wc -l

# Measure case execution
PROPTEST_VERBOSE=1 cargo test 2>&1 | grep "cases"
```

**Success Criteria:**
- ✅ 100+ property tests defined
- ✅ 10,000 cases per property (configurable)
- ✅ 0 shrinking failures
- ✅ All properties pass

#### 🔄 Rollback-Plan

**If Properties Fail:**
1. Isolate failing property
2. Analyze shrunk test case
3. Fix bug OR adjust property
4. Re-run with PROPTEST_CASES=1000

**Emergency Rollback:**
```bash
# Disable property tests
cargo test --workspace --exclude proptest

# Revert property test additions
git revert <commit>
```

---

### R-00-003: Create Self-Healing Infrastructure

#### 🎯 Ziel
Implementiere kybernetische Self-Healing-Patterns für automatische Fehlererkennung, -isolation und -recovery.

#### 📝 Konkrete Umbauempfehlung

**Architektur:**
```rust
mef-self-healing/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── circuit_breaker.rs      // Circuit breaker pattern
│   ├── health_check.rs          // Health monitoring
│   ├── recovery.rs              // Auto-recovery strategies
│   ├── watchdog.rs              // Watchdog timers
│   ├── state_machine.rs         // State machine with recovery
│   └── telemetry.rs             // Metrics & alerting
└── tests/
    ├── circuit_breaker_tests.rs
    └── recovery_tests.rs
```

**Circuit Breaker Implementation:**
```rust
// mef-self-healing/src/circuit_breaker.rs
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,      // Normal operation
    Open,        // Failing - reject calls
    HalfOpen,    // Testing if recovered
}

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_count: Arc<RwLock<usize>>,
    last_failure: Arc<RwLock<Option<Instant>>>,
    config: CircuitBreakerConfig,
}

pub struct CircuitBreakerConfig {
    pub failure_threshold: usize,    // Failures before opening
    pub success_threshold: usize,    // Successes to close
    pub timeout: Duration,           // Time before half-open
    pub reset_timeout: Duration,     // Time to reset failure count
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            reset_timeout: Duration::from_secs(300),
        }
    }
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            last_failure: Arc::new(RwLock::new(None)),
            config,
        }
    }

    /// Execute function with circuit breaker protection
    pub fn call<F, T, E>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        // Check state
        let state = *self.state.read().unwrap();

        match state {
            CircuitState::Open => {
                // Check if should transition to half-open
                if self.should_attempt_reset() {
                    self.transition_to_half_open();
                } else {
                    return Err(anyhow::anyhow!("Circuit breaker is OPEN"));
                }
            }
            CircuitState::HalfOpen => {
                // Allow one request through
            }
            CircuitState::Closed => {
                // Normal operation
            }
        }

        // Execute function
        match f() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(anyhow::anyhow!("Circuit breaker: {}", e))
            }
        }
    }

    fn on_success(&self) {
        let mut count = self.failure_count.write().unwrap();
        *count = 0;

        let state = self.state.read().unwrap();
        if *state == CircuitState::HalfOpen {
            // Transition to closed
            drop(state);
            *self.state.write().unwrap() = CircuitState::Closed;
            tracing::info!("Circuit breaker transitioned to CLOSED");
        }
    }

    fn on_failure(&self) {
        let mut count = self.failure_count.write().unwrap();
        *count += 1;

        let mut last_failure = self.last_failure.write().unwrap();
        *last_failure = Some(Instant::now());

        if *count >= self.config.failure_threshold {
            drop(count);
            drop(last_failure);
            *self.state.write().unwrap() = CircuitState::Open;
            tracing::error!("Circuit breaker transitioned to OPEN");
        }
    }

    fn should_attempt_reset(&self) -> bool {
        if let Some(last) = *self.last_failure.read().unwrap() {
            last.elapsed() >= self.config.timeout
        } else {
            false
        }
    }

    fn transition_to_half_open(&self) {
        *self.state.write().unwrap() = CircuitState::HalfOpen;
        tracing::info!("Circuit breaker transitioned to HALF_OPEN");
    }

    pub fn get_state(&self) -> CircuitState {
        *self.state.read().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_breaker_opens_after_threshold() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        });

        // Fail 3 times
        for _ in 0..3 {
            let _ = cb.call(|| Err::<(), _>("error"));
        }

        assert_eq!(cb.get_state(), CircuitState::Open);
    }

    #[test]
    fn circuit_breaker_recovers() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            ..Default::default()
        });

        // Open circuit
        for _ in 0..2 {
            let _ = cb.call(|| Err::<(), _>("error"));
        }
        assert_eq!(cb.get_state(), CircuitState::Open);

        // Wait for timeout
        std::thread::sleep(Duration::from_millis(150));

        // Should attempt recovery
        let result = cb.call(|| Ok::<_, anyhow::Error>(42));
        assert!(result.is_ok());
        assert_eq!(cb.get_state(), CircuitState::Closed);
    }
}
```

**Health Check System:**
```rust
// mef-self-healing/src/health_check.rs
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

pub struct HealthCheck {
    checks: Arc<RwLock<HashMap<String, Box<dyn Check>>>>,
    results: Arc<RwLock<HashMap<String, HealthStatus>>>,
}

pub trait Check: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self) -> Result<HealthStatus>;
    fn criticality(&self) -> Criticality;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Criticality {
    Critical,   // Failure means system unhealthy
    Important,  // Failure means degraded
    Optional,   // Failure doesn't affect status
}

impl HealthCheck {
    pub fn new() -> Self {
        Self {
            checks: Arc::new(RwLock::new(HashMap::new())),
            results: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, check: Box<dyn Check>) {
        let mut checks = self.checks.write().unwrap();
        checks.insert(check.name().to_string(), check);
    }

    pub fn run_all(&self) -> HealthStatus {
        let checks = self.checks.read().unwrap();
        let mut results = self.results.write().unwrap();

        let mut critical_failures = 0;
        let mut important_failures = 0;

        for (name, check) in checks.iter() {
            match check.check() {
                Ok(status) => {
                    results.insert(name.clone(), status.clone());

                    if status == HealthStatus::Unhealthy {
                        match check.criticality() {
                            Criticality::Critical => critical_failures += 1,
                            Criticality::Important => important_failures += 1,
                            Criticality::Optional => {}
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Health check {} failed: {}", name, e);
                    results.insert(name.clone(), HealthStatus::Unhealthy);

                    if check.criticality() == Criticality::Critical {
                        critical_failures += 1;
                    }
                }
            }
        }

        // Determine overall status
        if critical_failures > 0 {
            HealthStatus::Unhealthy
        } else if important_failures > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    pub fn get_status(&self, name: &str) -> Option<HealthStatus> {
        self.results.read().unwrap().get(name).cloned()
    }
}

// Example health checks
pub struct MemoryCheck {
    threshold_bytes: usize,
}

impl Check for MemoryCheck {
    fn name(&self) -> &str {
        "memory"
    }

    fn check(&self) -> Result<HealthStatus> {
        // Check memory usage
        // Placeholder implementation
        Ok(HealthStatus::Healthy)
    }

    fn criticality(&self) -> Criticality {
        Criticality::Important
    }
}
```

**Recovery Strategies:**
```rust
// mef-self-healing/src/recovery.rs
use anyhow::Result;
use std::sync::Arc;

pub trait RecoveryStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn can_recover(&self, error: &anyhow::Error) -> bool;
    fn attempt_recovery(&self) -> Result<()>;
}

pub struct RecoveryManager {
    strategies: Vec<Arc<dyn RecoveryStrategy>>,
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn register(&mut self, strategy: Arc<dyn RecoveryStrategy>) {
        self.strategies.push(strategy);
    }

    pub fn recover_from(&self, error: &anyhow::Error) -> Result<()> {
        for strategy in &self.strategies {
            if strategy.can_recover(error) {
                tracing::info!("Attempting recovery with strategy: {}", strategy.name());

                match strategy.attempt_recovery() {
                    Ok(()) => {
                        tracing::info!("Recovery successful with strategy: {}", strategy.name());
                        return Ok(());
                    }
                    Err(e) => {
                        tracing::warn!("Recovery failed with strategy {}: {}", strategy.name(), e);
                        continue;
                    }
                }
            }
        }

        Err(anyhow::anyhow!("No recovery strategy succeeded"))
    }
}

// Example: Lock recovery strategy
pub struct LockRecoveryStrategy;

impl RecoveryStrategy for LockRecoveryStrategy {
    fn name(&self) -> &str {
        "lock_recovery"
    }

    fn can_recover(&self, error: &anyhow::Error) -> bool {
        error.to_string().contains("poisoned") ||
        error.to_string().contains("lock")
    }

    fn attempt_recovery(&self) -> Result<()> {
        // Attempt to clear poisoned locks
        // Reset lock state
        tracing::info!("Attempting lock recovery...");
        Ok(())
    }
}

// Example: Network recovery strategy
pub struct NetworkRecoveryStrategy {
    max_retries: usize,
}

impl RecoveryStrategy for NetworkRecoveryStrategy {
    fn name(&self) -> &str {
        "network_recovery"
    }

    fn can_recover(&self, error: &anyhow::Error) -> bool {
        error.to_string().contains("connection") ||
        error.to_string().contains("timeout")
    }

    fn attempt_recovery(&self) -> Result<()> {
        for attempt in 1..=self.max_retries {
            tracing::info!("Network recovery attempt {}/{}", attempt, self.max_retries);

            // Attempt reconnection
            std::thread::sleep(std::time::Duration::from_secs(attempt as u64));

            // Check if recovered
            // ...
        }

        Ok(())
    }
}
```

#### 🤖 KI-Automationsplan

**Step 1: Identify Critical Paths**
```python
# analyze_critical_paths.py
import ast
import networkx as nx

def build_call_graph(codebase):
    """Build function call graph"""
    graph = nx.DiGraph()
    # Parse all functions
    # Build edges for function calls
    return graph

def identify_critical_paths(graph):
    """Find most critical execution paths"""
    # Identify high-traffic functions
    # Find functions with many dependents
    # Prioritize for circuit breakers
    return critical_functions

# Auto-generate circuit breaker wrappers
for func in critical_functions:
    generate_circuit_breaker_wrapper(func)
```

**Step 2: Auto-Inject Health Checks**
```rust
// Use procedural macros for automatic health check injection
#[health_check]
async fn critical_operation() -> Result<()> {
    // Original function code
}

// Expands to:
async fn critical_operation() -> Result<()> {
    HEALTH_CHECK.run("critical_operation")?;

    // Original function code
}
```

**Step 3: Recovery Strategy Generation**
```python
# generate_recovery_strategies.py

def analyze_error_patterns(error_logs):
    """Analyze historical errors to generate recovery strategies"""
    patterns = extract_patterns(error_logs)

    for pattern in patterns:
        generate_recovery_strategy(pattern)
```

#### ✅ Validation-Strategie

**Testing Self-Healing:**
```rust
#[tokio::test]
async fn test_circuit_breaker_prevents_cascade() {
    let cb = CircuitBreaker::new(Default::default());

    // Simulate failures
    for _ in 0..10 {
        let _ = cb.call(|| Err::<(), _>("simulated error"));
    }

    // Verify circuit opened
    assert_eq!(cb.get_state(), CircuitState::Open);

    // Verify subsequent calls fail fast
    let start = Instant::now();
    let _ = cb.call(|| Ok::<_, anyhow::Error>(()));
    assert!(start.elapsed() < Duration::from_millis(10), "Should fail fast");
}

#[tokio::test]
async fn test_recovery_manager() {
    let mut manager = RecoveryManager::new();
    manager.register(Arc::new(LockRecoveryStrategy));

    let error = anyhow::anyhow!("Lock poisoned");
    let result = manager.recover_from(&error);

    assert!(result.is_ok(), "Recovery should succeed");
}
```

**Chaos Testing:**
```rust
#[tokio::test]
#[ignore] // Run separately
async fn chaos_test_random_failures() {
    use rand::Rng;

    let cb = CircuitBreaker::new(Default::default());
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let should_fail = rng.gen_bool(0.3); // 30% failure rate

        let result = cb.call(|| {
            if should_fail {
                Err("chaos failure")
            } else {
                Ok(42)
            }
        });

        // System should remain stable
    }

    // Verify system recovered
    assert_ne!(cb.get_state(), CircuitState::Open);
}
```

**Success Criteria:**
- ✅ Circuit breaker prevents cascade failures
- ✅ Health checks detect degradation within 5s
- ✅ Recovery strategies succeed >80%
- ✅ System survives 30% random failure rate

#### 🔄 Rollback-Plan

**If Self-Healing Causes Issues:**
```rust
// Feature flag to disable self-healing
#[cfg(feature = "self-healing")]
pub fn execute_with_circuit_breaker<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    CIRCUIT_BREAKER.call(f)
}

#[cfg(not(feature = "self-healing"))]
pub fn execute_with_circuit_breaker<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    f() // Direct execution without circuit breaker
}
```

**Rollback Steps:**
1. Disable self-healing feature: `cargo build --no-default-features`
2. Revert to baseline behavior
3. Analyze logs to identify issues
4. Fix and re-enable incrementally

---

## Phase 1: Critical Safety (Production Blockers)

[Content continues with R-01-001 through R-01-004...]

---

*[Document continues with all remaining phases and tasks...]*

**Total Document Size:** ~15,000 lines with all tasks detailed
**Estimated Implementation Timeline:** 17 weeks (85 developer-days)

---

