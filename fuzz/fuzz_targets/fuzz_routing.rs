#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
struct NetworkTopology {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Arbitrary, Debug, Clone)]
struct Node {
    id: u32,
    entropy: [u8; 8],
}

#[derive(Arbitrary, Debug, Clone)]
struct Edge {
    from: u32,
    to: u32,
    weight: u8,
}

#[derive(Arbitrary, Debug)]
struct RoutingQuery {
    source: u32,
    destination: u32,
    entropy: Vec<u8>,
}

fuzz_target!(|input: (NetworkTopology, RoutingQuery)| {
    let (topology, query) = input;

    // Test quantum routing algorithm
    // Check for:
    // - No infinite loops
    // - Reasonable path lengths
    // - Entropy-based randomization

    if topology.nodes.is_empty() || query.entropy.len() < 8 {
        return;
    }

    // Limit network size to prevent timeout
    if topology.nodes.len() > 1000 || topology.edges.len() > 5000 {
        return;
    }

    if let Ok(path) = quantum_route(&topology, &query) {
        // Verify path properties
        // 1. Path is not empty if route exists
        assert!(!path.is_empty());

        // 2. Path length is reasonable
        assert!(path.len() <= 100);

        // 3. Path starts at source
        if let Some(&first) = path.first() {
            assert_eq!(first, query.source);
        }

        // 4. Path ends at destination
        if let Some(&last) = path.last() {
            assert_eq!(last, query.destination);
        }

        // 5. Path is connected (each hop is valid)
        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];
            assert!(is_edge_valid(&topology, from, to));
        }

        // 6. No routing loops
        let mut visited = std::collections::HashSet::new();
        for &node in &path {
            if visited.contains(&node) && node != query.destination {
                panic!("Routing loop detected!");
            }
            visited.insert(node);
        }
    }
});

fn quantum_route(topology: &NetworkTopology, query: &RoutingQuery) -> Result<Vec<u32>, ()> {
    // Simplified quantum routing implementation

    if topology.nodes.is_empty() {
        return Err(());
    }

    // Check if source and destination exist
    let source_exists = topology.nodes.iter().any(|n| n.id == query.source);
    let dest_exists = topology.nodes.iter().any(|n| n.id == query.destination);

    if !source_exists || !dest_exists {
        return Err(());
    }

    // Simple path finding with randomization
    let mut path = vec![query.source];
    let mut current = query.source;
    let mut visited = std::collections::HashSet::new();
    visited.insert(current);

    // Limit iterations to prevent infinite loops
    for _ in 0..100 {
        if current == query.destination {
            return Ok(path);
        }

        // Find neighbors
        let neighbors: Vec<u32> = topology.edges.iter()
            .filter(|e| e.from == current && !visited.contains(&e.to))
            .map(|e| e.to)
            .collect();

        if neighbors.is_empty() {
            return Err(());
        }

        // Select next hop using entropy
        let entropy_idx = query.entropy[path.len() % query.entropy.len()] as usize;
        let next = neighbors[entropy_idx % neighbors.len()];

        path.push(next);
        visited.insert(next);
        current = next;
    }

    Err(())
}

fn is_edge_valid(topology: &NetworkTopology, from: u32, to: u32) -> bool {
    topology.edges.iter().any(|e| e.from == from && e.to == to)
}
