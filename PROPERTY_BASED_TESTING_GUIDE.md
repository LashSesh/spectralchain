# Property-Based Testing Guide

## Overview

This guide explains how to use property-based testing with proptest in the MEF system. Property-based testing is a powerful technique that goes beyond traditional example-based testing to verify system invariants across thousands of automatically-generated test cases.

## What is Property-Based Testing?

### Traditional Testing (Example-Based)

```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
    assert_eq!(10 + 5, 15);
    assert_eq!(100 + 1, 101);
}
```

**Problems:**
- Only tests specific examples
- Easy to miss edge cases
- Requires manual selection of test cases
- Limited coverage

### Property-Based Testing

```rust
proptest! {
    #[test]
    fn test_addition_commutative(a in any::<i32>(), b in any::<i32>()) {
        // PROPERTY: Addition is commutative
        prop_assert_eq!(a + b, b + a);
    }
}
```

**Benefits:**
- ✅ Tests thousands of randomly-generated inputs
- ✅ Automatically finds edge cases
- ✅ Verifies **properties** that should always hold
- ✅ Shrinks failing cases to minimal examples

## Core Concepts

### 1. Properties vs Examples

**Property:** A statement that should hold for all valid inputs

Examples of properties:
- "Encryption is reversible: decrypt(encrypt(x)) = x"
- "Normalization produces unit vectors: ||normalize(v)||₂ = 1"
- "Hash collisions don't occur: x ≠ y → hash(x) ≠ hash(y)"
- "State transitions are deterministic: same input + action → same output"

### 2. Generators (Strategies)

Generators produce random test data:

```rust
use proptest::prelude::*;

// Built-in generators
any::<i32>()           // Any i32 value
0..100                 // Range
"[a-z]{3,10}"         // Regex pattern

// Custom generators
arb_resonance_triplet()     // From mef-common
arb_content_hash()          // 32-byte hashes
s7_permutation()            // Valid S7 permutations
```

### 3. Invariants

Invariants are conditions that must always be true:

```rust
use mef_common::proptest_support::invariants::*;

// Unit length invariant
assert_unit_length(&vector, 1e-10);

// Idempotence invariant
assert_normalization_idempotent(&triplet, 1e-10);

// Roundtrip invariant
assert_serde_roundtrip(&data);

// Monotonicity invariant
assert_monotonic_increase(&timestamps);
```

## Using MEF Property Testing Framework

### Setup

Add to your crate's `Cargo.toml`:

```toml
[dev-dependencies]
mef-common = { workspace = true, features = ["proptest-support"] }
proptest.workspace = true
```

### Basic Example

```rust
use proptest::prelude::*;
use mef_common::proptest_support::*;

proptest! {
    #[test]
    fn resonance_normalization_is_unit_length(
        triplet in arb_resonance_triplet()
    ) {
        let normalized = triplet.normalize();
        assert_unit_length(&normalized, 1e-10);
    }

    #[test]
    fn content_hash_hex_roundtrip(
        hash in arb_content_hash()
    ) {
        let hex = hash.to_hex();
        let parsed = ContentHash::from_hex(&hex).unwrap();
        prop_assert_eq!(hash, parsed);
    }
}
```

### Testing Quantum Operations

```rust
use mef_common::proptest_support::strategies::*;

proptest! {
    #[test]
    fn quantum_masking_is_reversible(
        data in arb_nonempty_bytes(1000),
        (theta, sigma) in quantum_masking_params()
    ) {
        let masker = QuantumMaskingOperator::new();
        let params = MaskingParams { theta, sigma };

        let masked = masker.mask(&data, &params)?;
        let unmasked = masker.unmask(&masked, &params)?;

        // INVARIANT: Masking is reversible
        prop_assert_eq!(data, unmasked);
    }
}
```

### Testing State Machines

```rust
proptest! {
    #[test]
    fn state_transitions_are_deterministic(
        initial_state in arb_state(),
        action in arb_action()
    ) {
        let state1 = apply_transition(&initial_state, &action);
        let state2 = apply_transition(&initial_state, &action);

        // INVARIANT: Determinism
        prop_assert_eq!(state1, state2);
    }
}
```

### Testing Concurrent Operations

```rust
use std::sync::Arc;
use tokio::task;

proptest! {
    #[test]
    fn concurrent_operations_preserve_total(
        initial_value in 0u64..1_000_000,
        (num_threads, ops_per_thread) in concurrent_scenario()
    ) {
        let counter = Arc::new(SafeRwLock::new(initial_value));
        let mut handles = vec![];

        // Spawn concurrent increments/decrements
        for _ in 0..num_threads {
            let counter_clone = Arc::clone(&counter);
            let handle = task::spawn(async move {
                for _ in 0..ops_per_thread {
                    // Paired increment/decrement
                    *counter_clone.write() += 1;
                    *counter_clone.write() -= 1;
                }
            });
            handles.push(handle);
        }

        // Wait for completion
        for handle in handles {
            handle.await.unwrap();
        }

        // INVARIANT: Total should be unchanged
        prop_assert_eq!(*counter.read(), initial_value);
    }
}
```

## Available Generators

### Basic Types

```rust
// From mef-common::proptest_support::generators
arb_resonance_triplet()        // Random (ψ, ρ, ω)
arb_unit_resonance_triplet()   // Normalized triplet
arb_nonzero_resonance_triplet() // Non-zero triplet
arb_content_hash()             // 32-byte hash
arb_node_id()                  // Node identifier
arb_tx_id()                    // Transaction ID
arb_timestamp()                // 2020-2030 timestamp
arb_ttl()                      // 1 sec to 1 year TTL
arb_bytes(max_len)             // Random byte vector
arb_nonempty_bytes(max_len)    // Non-empty bytes
```

### Complex Strategies

```rust
// From mef-common::proptest_support::strategies
quantum_masking_params()       // (theta, sigma) pairs
s7_permutation()               // Valid 7-element permutation
permutation(n)                 // Valid n-element permutation
operation_sequence(op, min, max) // Sequence of operations
concurrent_scenario()          // (threads, ops_per_thread)
network_partition(max_nodes)   // Network split scenarios
time_series(data, num_events)  // Time-ordered events
byzantine_scenario(max_nodes)  // Byzantine fault scenarios
```

## Common Testing Patterns

### Pattern 1: Reversibility

Test that operations can be undone:

```rust
proptest! {
    #[test]
    fn encryption_is_reversible(
        plaintext in arb_bytes(1000),
        key in arb_nonempty_bytes(32)
    ) {
        let ciphertext = encrypt(&plaintext, &key)?;
        let decrypted = decrypt(&ciphertext, &key)?;
        prop_assert_eq!(plaintext, decrypted);
    }
}
```

### Pattern 2: Idempotence

Test that repeating an operation doesn't change the result:

```rust
proptest! {
    #[test]
    fn normalization_is_idempotent(
        triplet in arb_resonance_triplet()
    ) {
        let once = triplet.normalize();
        let twice = once.normalize();
        assert_normalization_idempotent(&triplet, 1e-10);
    }
}
```

### Pattern 3: Commutativity

Test that order doesn't matter:

```rust
proptest! {
    #[test]
    fn xor_is_commutative(
        a in arb_bytes(100),
        b in arb_bytes(100)
    ) {
        let ab = xor(&a, &b);
        let ba = xor(&b, &a);
        prop_assert_eq!(ab, ba);
    }
}
```

### Pattern 4: Conservation Laws

Test that totals are preserved:

```rust
proptest! {
    #[test]
    fn fork_healing_preserves_total_weight(
        forks in prop::collection::vec(arb_fork(), 2..10)
    ) {
        let total_before: u64 = forks.iter().map(|f| f.weight).sum();
        let healed = heal_forks(&forks)?;
        let total_after: u64 = healed.iter().map(|f| f.weight).sum();

        // INVARIANT: Total weight conserved
        prop_assert_eq!(total_before, total_after);
    }
}
```

### Pattern 5: Monotonicity

Test that sequences are ordered:

```rust
proptest! {
    #[test]
    fn block_heights_are_monotonic(
        blocks in time_series(arb_block_data(), 10)
    ) {
        let heights: Vec<u64> = blocks.iter().map(|b| b.height).collect();
        assert_strict_monotonic_increase(&heights);
    }
}
```

## Best Practices

### 1. Start Simple

Begin with basic properties:

```rust
proptest! {
    #[test]
    fn serialize_deserialize_roundtrip(data in arb_my_type()) {
        assert_serde_roundtrip(&data);
    }
}
```

### 2. Test Invariants, Not Implementation

**Bad:** Testing implementation details
```rust
// BAD: Tests implementation
fn test_uses_specific_algorithm() {
    assert!(result.contains("SHA256"));
}
```

**Good:** Testing invariants
```rust
// GOOD: Tests behavior
proptest! {
    #[test]
    fn hash_is_deterministic(data in arb_bytes(1000)) {
        let hash1 = compute_hash(&data);
        let hash2 = compute_hash(&data);
        prop_assert_eq!(hash1, hash2);
    }
}
```

### 3. Use Shrinking to Find Minimal Failing Cases

When a test fails, proptest automatically shrinks the input to find the minimal failing case:

```
Test failed for input: vec![1, 2, 3, 4, 5, ..., 1000]
Shrinking...
Minimal failing input: vec![5]
```

### 4. Set Appropriate Test Count

```rust
proptest! {
    // Default: 256 test cases
    #[test]
    fn standard_test(x in any::<i32>()) { ... }

    // More thorough: 10,000 test cases
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn thorough_test(x in any::<i32>()) { ... }
}
```

### 5. Use Filters Sparingly

```rust
// OK: Filter out invalid cases
proptest! {
    #[test]
    fn test_nonzero_division(
        a in any::<i32>(),
        b in any::<i32>().prop_filter("b != 0", |&b| b != 0)
    ) {
        let result = a / b;
        // ...
    }
}

// BETTER: Use generator that only produces valid values
proptest! {
    #[test]
    fn test_nonzero_division(
        a in any::<i32>(),
        b in 1..=i32::MAX  // Or use custom generator
    ) {
        let result = a / b;
        // ...
    }
}
```

## Integration with Existing Tests

### Migrating from Example Tests

**Before:**
```rust
#[test]
fn test_timestamp_calculation() {
    assert_eq!(elapsed_since(100, 150), 50);
    assert_eq!(elapsed_since(0, 1000), 1000);
}
```

**After:**
```rust
proptest! {
    #[test]
    fn elapsed_time_is_difference(
        start in arb_timestamp(),
        duration in 0u64..=86400  // 0-24 hours
    ) {
        let end = start + duration;
        let elapsed = elapsed_since(start, end)?;
        prop_assert_eq!(elapsed, duration);
    }
}
```

### Combining with Unit Tests

Use both example-based and property-based tests:

```rust
// Example test for specific known cases
#[test]
fn test_known_hash() {
    let data = b"hello";
    let hash = compute_hash(data);
    assert_eq!(hash, "2cf24dba5fb0a30e...");
}

// Property test for general behavior
proptest! {
    #[test]
    fn hash_is_deterministic(data in arb_bytes(1000)) {
        let hash1 = compute_hash(&data);
        let hash2 = compute_hash(&data);
        prop_assert_eq!(hash1, hash2);
    }
}
```

## MEF-Specific Testing Scenarios

### Testing Ghost Network

```rust
proptest! {
    #[test]
    fn ghost_packets_maintain_anonymity(
        packet in arb_ghost_packet(),
        hops in 3usize..=10
    ) {
        let forwarded = forward_ghost_packet(packet, hops)?;

        // INVARIANT: Origin information should be masked
        prop_assert!(forwarded.origin_entropy > 0.9);
        prop_assert_eq!(forwarded.hop_count, hops);
    }
}
```

### Testing Quantum Routing

```rust
proptest! {
    #[test]
    fn quantum_routing_visits_all_nodes(
        network in arb_network(10..50),
        start_node in arb_node_id()
    ) {
        let route = quantum_random_walk(&network, start_node, 1000)?;

        // INVARIANT: Should eventually visit all nodes (with high probability)
        let visited: HashSet<_> = route.iter().collect();
        prop_assert!(visited.len() >= network.size() * 0.8);
    }
}
```

### Testing Ephemeral Services

```rust
proptest! {
    #[test]
    fn ephemeral_services_expire_correctly(
        service in arb_ephemeral_service(),
        ttl in arb_ttl()
    ) {
        let created_at = current_timestamp()?;
        service.set_ttl(ttl);

        // Fast-forward time
        mock_time_advance(ttl + 1);

        // INVARIANT: Service should be expired
        prop_assert!(service.is_expired()?);
        prop_assert!(service.should_cleanup());
    }
}
```

### Testing Fork Healing

```rust
proptest! {
    #[test]
    fn fork_healing_converges_to_valid_state(
        forks in prop::collection::vec(arb_fork(), 2..10),
        attractor in arb_mef_attractor()
    ) {
        let healed = heal_forks(&forks, &attractor)?;

        // INVARIANT: Result should satisfy PoR
        prop_assert!(healed.proof_of_resonance.is_valid());

        // INVARIANT: Should have single canonical chain
        prop_assert_eq!(healed.canonical_chains().len(), 1);
    }
}
```

## Debugging Failed Property Tests

### 1. Read the Shrunk Input

```
thread 'test_quantum_masking' panicked at 'assertion failed'
    minimal failing input: data = [42, 0, 255]
```

The shrunk input shows the simplest case that fails.

### 2. Reproduce Locally

```rust
#[test]
fn reproduce_failure() {
    let data = vec![42, 0, 255];
    // Add debugging...
}
```

### 3. Use Regression Tests

```rust
proptest! {
    #[test]
    fn quantum_masking_reversible(data in arb_bytes(1000)) {
        // Found failing case: data = [42, 0, 255]
        // Add as regression test below
    }
}

#[test]
fn regression_quantum_masking_255() {
    let data = vec![42, 0, 255];
    // Test the specific failing case
}
```

## Performance Considerations

- Property tests run 256 cases by default (configurable)
- Each test case is independent
- Tests can run in parallel
- Use `#![proptest_config(ProptestConfig::with_cases(10000))]` for thorough testing
- Consider timeout for expensive operations

## Further Reading

- [proptest book](https://proptest-rs.github.io/proptest/)
- [Choosing Properties](https://hypothesis.works/articles/what-is-property-based-testing/)
- MEF Refactoring Master Plan: Task R-00-002

## Summary

Property-based testing is a powerful technique for:

- ✅ Finding edge cases automatically
- ✅ Verifying system invariants
- ✅ Testing with thousands of inputs
- ✅ Building confidence in correctness
- ✅ Documenting behavioral contracts

Use `mef-common::proptest_support` to access MEF-specific generators, strategies, and invariants for comprehensive property-based testing across the entire system.
