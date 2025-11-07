//! Complex proptest strategies for MEF testing
//!
//! Provides composite strategies for testing complex scenarios and invariants.

use proptest::prelude::*;

/// Strategy for generating valid quantum masking parameters
///
/// Ensures theta ∈ [0, 2π] and sigma ∈ [0, 1]
pub fn quantum_masking_params() -> impl Strategy<Value = (f64, f64)> {
    (0.0..=std::f64::consts::TAU, 0.0..=1.0).prop_map(|(theta, sigma)| (theta, sigma))
}

/// Strategy for generating valid permutations of length n
///
/// Generates all valid permutations of [0..n)
///
/// # Example
///
/// ```rust
/// use proptest::prelude::*;
/// use mef_common::proptest_support::strategies::permutation;
///
/// proptest! {
///     #[test]
///     fn test_permutation_has_all_elements(perm in permutation(7)) {
///         // S7 permutation should have all elements 0..7
///         prop_assert_eq!(perm.len(), 7);
///         for i in 0..7 {
///             prop_assert!(perm.contains(&i));
///         }
///     }
/// }
/// ```
pub fn permutation(n: usize) -> impl Strategy<Value = Vec<usize>> {
    Just(()).prop_perturb(move |_, mut rng| {
        let mut perm: Vec<usize> = (0..n).collect();
        // Fisher-Yates shuffle
        for i in (1..n).rev() {
            let j = rng.random_range(0..=i);
            perm.swap(i, j);
        }
        perm
    })
}

/// Strategy for generating valid S7 permutations (Metatron routing)
pub fn s7_permutation() -> impl Strategy<Value = Vec<usize>> {
    permutation(7)
}

/// Strategy for generating sequences of operations
///
/// Useful for testing state machines and workflows
pub fn operation_sequence<T: Clone + std::fmt::Debug>(
    op: impl Strategy<Value = T>,
    min_len: usize,
    max_len: usize,
) -> impl Strategy<Value = Vec<T>> {
    prop::collection::vec(op, min_len..=max_len)
}

/// Strategy for generating concurrent scenarios
///
/// Generates (num_threads, ops_per_thread) pairs
pub fn concurrent_scenario() -> impl Strategy<Value = (usize, usize)> {
    (1..=16usize, 1..=100usize) // 1-16 threads, 1-100 ops each
}

/// Strategy for generating network partition scenarios
///
/// Returns (total_nodes, partition_sizes)
pub fn network_partition(max_nodes: usize) -> impl Strategy<Value = (usize, Vec<usize>)> {
    (2usize..=max_nodes).prop_flat_map(|total| {
        let num_partitions = 2usize..=4usize;
        num_partitions.prop_flat_map(move |n_parts| {
            // Generate partition sizes that sum to total
            partition_sizes(total, n_parts).prop_map(move |sizes| (total, sizes))
        })
    })
}

fn partition_sizes(total: usize, num_partitions: usize) -> impl Strategy<Value = Vec<usize>> {
    Just(()).prop_perturb(move |_, mut rng| {
        let mut sizes = vec![1; num_partitions]; // Each partition has at least 1 node
        let remaining = total - num_partitions;

        // Distribute remaining nodes
        for _ in 0..remaining {
            let idx = rng.random_range(0..num_partitions);
            sizes[idx] += 1;
        }

        sizes
    })
}

/// Strategy for generating time-based scenarios
///
/// Returns (start_time, events: Vec<(offset, data)>)
pub fn time_series<T: Clone + std::fmt::Debug>(
    event_data: impl Strategy<Value = T> + Clone,
    num_events: usize,
) -> impl Strategy<Value = (u64, Vec<(u64, T)>)> {
    let start = 1577836800u64..=1893456000u64; // 2020-2030

    start.prop_flat_map(move |start_time| {
        let events = prop::collection::vec(
            (0u64..=86400, event_data.clone()), // Offsets within 24 hours
            num_events,
        );

        events.prop_map(move |mut evs| {
            // Sort by timestamp
            evs.sort_by_key(|(offset, _)| *offset);
            (start_time, evs)
        })
    })
}

/// Strategy for generating Byzantine fault scenarios
///
/// Returns (total_nodes, byzantine_count) where byzantine_count < total/3
pub fn byzantine_scenario(max_nodes: usize) -> impl Strategy<Value = (usize, usize)> {
    (4..=max_nodes).prop_flat_map(|total| {
        let max_byzantine = (total - 1) / 3; // Byzantine tolerance: f < n/3
        (0..=max_byzantine).prop_map(move |byzantine| (total, byzantine))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn test_quantum_masking_params_in_range((theta, sigma) in quantum_masking_params()) {
            prop_assert!((0.0..=std::f64::consts::TAU).contains(&theta));
            prop_assert!((0.0..=1.0).contains(&sigma));
        }

        #[test]
        fn test_permutation_is_valid(perm in permutation(10)) {
            prop_assert_eq!(perm.len(), 10);
            for i in 0..10 {
                prop_assert!(perm.contains(&i), "missing {}", i);
            }
            // No duplicates
            let mut sorted = perm.clone();
            sorted.sort();
            prop_assert_eq!(sorted, (0..10).collect::<Vec<_>>());
        }

        #[test]
        fn test_s7_permutation_has_7_elements(perm in s7_permutation()) {
            prop_assert_eq!(perm.len(), 7);
            for i in 0..7 {
                prop_assert!(perm.contains(&i));
            }
        }

        #[test]
        fn test_concurrent_scenario_reasonable((threads, ops) in concurrent_scenario()) {
            prop_assert!((1..=16).contains(&threads));
            prop_assert!((1..=100).contains(&ops));
        }

        #[test]
        fn test_network_partition_sums_correctly((total, sizes) in network_partition(20)) {
            let sum: usize = sizes.iter().sum();
            prop_assert_eq!(sum, total);
            // Each partition has at least one node
            for &size in &sizes {
                prop_assert!(size >= 1);
            }
        }

        #[test]
        fn test_time_series_sorted((start, events) in time_series(any::<u8>(), 10)) {
            prop_assert!(start >= 1577836800);
            prop_assert_eq!(events.len(), 10);
            // Events should be sorted by timestamp
            for i in 1..events.len() {
                prop_assert!(events[i].0 >= events[i-1].0);
            }
        }

        #[test]
        fn test_byzantine_scenario_respects_bound((total, byzantine) in byzantine_scenario(30)) {
            prop_assert!(total >= 4);
            prop_assert!(byzantine * 3 < total); // f < n/3
        }
    }
}
