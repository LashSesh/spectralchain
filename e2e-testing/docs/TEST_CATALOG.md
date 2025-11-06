# SpectralChain E2E Test & Simulation Catalog

**Version:** 1.0.0
**Created:** 2025-11-06
**Status:** Comprehensive Test Suite

---

## Table of Contents

1. [Overview](#overview)
2. [Production Scenarios](#production-scenarios)
3. [Edge Case Scenarios](#edge-case-scenarios)
4. [Chaos Engineering Scenarios](#chaos-engineering-scenarios)
5. [Test Execution Framework](#test-execution-framework)
6. [Metrics & Observability](#metrics--observability)

---

## Overview

This catalog defines comprehensive end-to-end tests and simulations for the SpectralChain quantum-resonant blockchain system. Each test includes:

- **Test ID**: Unique identifier
- **Objective**: What the test verifies
- **Setup**: Initial conditions and network topology
- **Input**: Test parameters and data
- **Expected Behavior**: System response and invariants
- **Success Criteria**: How to determine test success
- **Failure Criteria**: What constitutes failure
- **Recovery Strategy**: How the system should recover from failures

### Test Categories

- **PROD-XXX**: Production scenarios (normal operation)
- **EDGE-XXX**: Edge cases and boundary conditions
- **CHAOS-XXX**: Chaos engineering and fault injection

---

## Production Scenarios

### PROD-001: Basic Transaction Flow

**Objective**: Verify end-to-end transaction processing through Ghost Network

**Setup**:
- Network: 5 nodes in mesh topology
- All nodes have synchronized resonance states
- No network failures or delays

**Execution Steps**:
1. Node 1 creates transaction with ZK proof
2. Transaction is masked using current epoch key
3. Ghost packet is broadcast to network
4. Nodes check resonance (R_ε)
5. Resonant nodes unmask and verify ZK proof
6. Transaction is committed to ledger

**Input**:
```json
{
  "transaction": {
    "sender_resonance": {"psi": 0.75, "rho": 0.5, "omega": 0.25},
    "target_resonance": {"psi": 0.76, "rho": 0.51, "omega": 0.26},
    "payload": "test_transaction_001",
    "proof_type": "zk_proof"
  },
  "resonance_window_epsilon": 0.05
}
```

**Expected Behavior**:
- Packet arrives at all nodes
- 3-4 nodes resonate (within ε=0.05)
- Transaction successfully unmasked by resonant nodes
- ZK proof verified
- Ledger updated on resonant nodes
- No metadata leakage (timing, sender identity)

**Success Criteria**:
- ✅ Transaction committed within 5 seconds
- ✅ At least 1 resonant node receives transaction
- ✅ ZK proof validation success
- ✅ No unmasking attempts by non-resonant nodes
- ✅ Packet metrics show normal distribution

**Failure Criteria**:
- ❌ Transaction not committed after 30 seconds
- ❌ No resonant nodes found
- ❌ ZK proof validation failed
- ❌ Metadata leaked in logs

**Recovery**: N/A (no failure expected)

---

### PROD-002: Multi-Hop Quantum Routing

**Objective**: Verify quantum random walk routing across multiple hops

**Setup**:
- Network: 20 nodes in random topology
- Average hop distance: 4-5 hops
- Entropy source: Quantum random number generator

**Execution Steps**:
1. Node A sends packet to resonance region near Node T
2. Packet performs random walk with resonance-weighted probabilities
3. Each hop: compute neighbor resonances, select next hop via quantum entropy
4. Track hop count and path taken
5. Packet arrives at resonant nodes

**Input**:
```json
{
  "source_node": 0,
  "target_resonance": {"psi": 0.9, "rho": 0.8, "omega": 0.7},
  "resonance_window": 0.1,
  "max_hops": 10,
  "timeout_seconds": 30
}
```

**Expected Behavior**:
- Packet follows unpredictable path (random walk)
- Hop count: 4-8 hops (for 20-node network)
- Multiple paths possible (non-deterministic)
- Eventually reaches resonant nodes
- No routing loops

**Success Criteria**:
- ✅ Packet arrives within 10 seconds
- ✅ Hop count ≤ max_hops
- ✅ Path is non-deterministic (varies across runs)
- ✅ No node visited more than once (loop detection)
- ✅ Final resonance check successful

**Failure Criteria**:
- ❌ Packet never arrives (timeout)
- ❌ Routing loop detected
- ❌ Path is deterministic (security issue)
- ❌ Hop count exceeds max_hops

**Recovery**: Timeout and retry with new quantum entropy

---

### PROD-003: Ephemeral Service Lifecycle

**Objective**: Test creation, discovery, use, and expiration of ephemeral services

**Setup**:
- Network: 10 nodes
- Service: Ephemeral voting service
- TTL: 5 minutes

**Execution Steps**:
1. Node creates ephemeral service with TTL=5min
2. Service registers in resonance bubble
3. Other nodes discover service via resonance
4. Nodes interact with service (cast votes)
5. Service completes and produces proof-carrying audit trail
6. TTL expires, service automatically disappears
7. Verify service is no longer discoverable

**Input**:
```json
{
  "service_type": "ephemeral_voting",
  "ttl_seconds": 300,
  "resonance_bubble": {"center": {"psi": 0.5, "rho": 0.5, "omega": 0.5}, "radius": 0.2},
  "voting_options": ["option_a", "option_b", "option_c"],
  "required_votes": 5
}
```

**Expected Behavior**:
- Service appears in resonance bubble
- Nodes within bubble can discover service
- Voting proceeds via masked transactions
- Service produces audit trail with ZK proofs
- Service disappears after TTL
- No trace remains except audit proof

**Success Criteria**:
- ✅ Service discoverable within 10 seconds of creation
- ✅ At least 5 nodes successfully vote
- ✅ Audit trail generated with valid ZK proofs
- ✅ Service expires exactly at TTL
- ✅ Service no longer discoverable after expiration

**Failure Criteria**:
- ❌ Service not discoverable
- ❌ Service persists beyond TTL
- ❌ Audit trail missing or invalid
- ❌ Service bubble isolation violated

**Recovery**: Service self-destructs, audit trail preserved

---

### PROD-004: Fork Detection and MEF-Attractor Healing

**Objective**: Verify fork detection and self-healing via Mandorla attractor

**Setup**:
- Network: 8 nodes in two clusters
- Simulate network partition creating two forks
- Both forks produce valid blocks

**Execution Steps**:
1. Create partition: Cluster A (4 nodes) and Cluster B (4 nodes)
2. Both clusters continue processing transactions independently
3. Fork detected: two incompatible blocks at same height
4. Heal partition (reconnect clusters)
5. MEF-Attractor mechanism activates
6. Compute Mandorla coherence for each fork
7. Select fork with highest coherence (strongest attractor)
8. All nodes converge to selected fork
9. Verify ledger consistency across all nodes

**Input**:
```json
{
  "partition_duration_seconds": 60,
  "cluster_a_transactions": 10,
  "cluster_b_transactions": 12,
  "healing_timeout_seconds": 120
}
```

**Expected Behavior**:
- Fork detected after partition heals
- MEF-Attractor computes coherence for both forks
- Higher-coherence fork selected deterministically
- All nodes converge to same fork
- Losing fork's transactions either:
  - Re-applied if compatible
  - Discarded if conflicting
- Final ledger state consistent across all nodes

**Success Criteria**:
- ✅ Fork detected within 10 seconds of reconnection
- ✅ Attractor selection completes within 30 seconds
- ✅ All nodes converge to same fork
- ✅ Ledger consistency verified (same hash)
- ✅ No data loss for compatible transactions

**Failure Criteria**:
- ❌ Fork not detected
- ❌ Nodes fail to converge
- ❌ Ledger inconsistency across nodes
- ❌ Attractor selection non-deterministic

**Recovery**: Manual intervention if convergence fails after 5 minutes

---

### PROD-005: Key Rotation Transition

**Objective**: Verify smooth key rotation during epoch transitions

**Setup**:
- Network: 5 nodes synchronized via NTP
- Current epoch: N
- Transition to epoch: N+1 at T+3600s

**Execution Steps**:
1. Send packets with epoch N keys (before transition)
2. Approach epoch boundary (T+3590s)
3. Send packets during transition window
4. Cross epoch boundary (T+3600s)
5. Send packets with epoch N+1 keys
6. Verify backward compatibility (N+1 nodes can still unmask N packets during grace period)
7. Verify old packets rejected after grace period

**Input**:
```json
{
  "test_start_time": "T+3550s",
  "packets_per_epoch": 10,
  "grace_period_seconds": 3600
}
```

**Expected Behavior**:
- Packets with epoch N successfully delivered before transition
- Packets with epoch N successfully delivered during grace period (N+1 nodes fallback)
- Packets with epoch N+1 successfully delivered after transition
- Packets with epoch N-1 rejected after grace period expires

**Success Criteria**:
- ✅ 100% delivery during same-epoch
- ✅ 100% delivery during grace period
- ✅ Old epoch packets rejected after grace period
- ✅ No disruption during transition
- ✅ Logs show "key_rotation_fallback" events during grace period

**Failure Criteria**:
- ❌ Packet loss during transition
- ❌ Old epoch accepted beyond grace period
- ❌ Service disruption during rotation

**Recovery**: Clock synchronization check, retry

---

### PROD-006: Forward Secrecy Validation

**Objective**: Verify perfect forward secrecy with ephemeral keys

**Setup**:
- Network: 3 nodes
- Enable forward secrecy
- Simulate key compromise

**Execution Steps**:
1. Node A sends 10 packets to Node B with forward secrecy enabled
2. Each packet uses unique ephemeral key
3. Capture all packets and resonance states
4. Simulate compromise: leak Node A's resonance state
5. Attempt to decrypt captured packets using compromised state
6. Verify decryption fails (ephemeral keys not recoverable)

**Input**:
```json
{
  "packet_count": 10,
  "enable_forward_secrecy": true,
  "compromise_timing": "after_all_packets_sent"
}
```

**Expected Behavior**:
- All packets successfully delivered during normal operation
- Each packet has unique ephemeral key
- Compromised resonance state does NOT allow decryption of past packets
- Future packets (post-compromise) also unrecoverable

**Success Criteria**:
- ✅ All 10 packets delivered successfully
- ✅ 10 unique ephemeral keys used
- ✅ Past packets remain encrypted after compromise
- ✅ No correlation between ephemeral keys

**Failure Criteria**:
- ❌ Any packet decryptable after compromise
- ❌ Ephemeral key correlation detected
- ❌ Forward secrecy not achieved

**Recovery**: N/A (security property verification)

---

### PROD-007: Adaptive Timestamp Windows

**Objective**: Verify adaptive timestamp validation under varying network conditions

**Setup**:
- Network: 5 nodes with variable latency
- Simulate network conditions:
  - Good: <10ms latency
  - Moderate: 60-120ms latency
  - Poor: 200-500ms latency

**Execution Steps**:
1. Phase 1: Good network (send 20 packets)
   - Observe strict timestamp window (~60s tolerance)
2. Phase 2: Inject latency (100ms)
   - Send 20 packets
   - Observe window adaptation (~80s tolerance)
3. Phase 3: Poor network (300ms latency)
   - Send 20 packets
   - Observe relaxed window (~150s tolerance)
4. Phase 4: Return to good network
   - Observe window tightening back to ~60s

**Input**:
```json
{
  "phases": [
    {"latency_ms": 5, "packet_count": 20},
    {"latency_ms": 100, "packet_count": 20},
    {"latency_ms": 300, "packet_count": 20},
    {"latency_ms": 5, "packet_count": 20}
  ],
  "adaptive_timestamps": true
}
```

**Expected Behavior**:
- Timestamp window adapts based on observed latency
- No false rejections in poor network conditions
- Window tightens in good conditions (maintains security)
- Exponential moving average smooths adaptation

**Success Criteria**:
- ✅ 0% packet rejection in all phases
- ✅ Clock skew tolerance: 60s → 80s → 150s → 60s
- ✅ Adaptation completes within 10 samples
- ✅ Security maintained (min 30s, max 300s bounds)

**Failure Criteria**:
- ❌ False rejections during poor network
- ❌ Window doesn't adapt
- ❌ Window exceeds max bounds (300s)

**Recovery**: Disable adaptive timestamps, use fixed window

---

## Edge Case Scenarios

### EDGE-001: Ghost Network Failover

**Objective**: Verify network resilience when majority of nodes fail

**Setup**:
- Network: 10 nodes initially
- Mesh topology with redundant paths

**Execution Steps**:
1. Start with 10 healthy nodes
2. Send transactions with ε=0.1 (expect 3-4 resonant nodes)
3. Crash 6 nodes (leaving 4)
4. Continue sending transactions
5. Verify remaining nodes still process transactions
6. Gradually restart failed nodes
7. Verify network recovers and rebalances

**Input**:
```json
{
  "initial_nodes": 10,
  "nodes_to_crash": [0, 1, 2, 3, 4, 5],
  "crash_timing": "simultaneous",
  "transactions_during_failure": 20,
  "restart_delay_seconds": 60
}
```

**Expected Behavior**:
- Transactions continue processing with 4 nodes
- Resonance probability decreases (fewer nodes)
- Some packets may timeout (acceptable)
- Network remains operational
- Restarted nodes rejoin seamlessly
- Transaction throughput recovers

**Success Criteria**:
- ✅ At least 50% of transactions succeed with 4 nodes
- ✅ No system-wide failure
- ✅ Restarted nodes rejoin within 30 seconds
- ✅ Full throughput restored after all nodes rejoin

**Failure Criteria**:
- ❌ Complete network failure
- ❌ Restarted nodes cannot rejoin
- ❌ Throughput doesn't recover

**Recovery Strategy**:
- Automatic: Nodes retry connection
- Manual: Restart node with --rejoin flag
- Expected recovery time: 2-5 minutes

---

### EDGE-002: Network Partitioning

**Objective**: Test behavior under network partitions and healing

**Setup**:
- Network: 12 nodes in 3 clusters
- Cluster A: 5 nodes
- Cluster B: 4 nodes
- Cluster C: 3 nodes

**Execution Steps**:
1. Start with fully connected network
2. Create partition: isolate Cluster C
3. Clusters A+B continue operating (9 nodes)
4. Cluster C operates independently (3 nodes)
5. Each partition produces transactions
6. Monitor for fork creation
7. Heal partition after 2 minutes
8. Verify MEF-Attractor fork resolution
9. Check final consistency

**Input**:
```json
{
  "partition_config": {
    "cluster_a": [0,1,2,3,4],
    "cluster_b": [5,6,7,8],
    "cluster_c": [9,10,11]
  },
  "partition_duration_seconds": 120,
  "transactions_per_cluster": 15
}
```

**Expected Behavior**:
- Partition detected by all nodes
- Clusters A+B form majority partition
- Cluster C forms minority partition
- Both partitions remain operational independently
- Fork created (two ledger branches)
- Upon healing: MEF-Attractor selects majority fork
- Cluster C nodes adopt majority fork
- Minority transactions replayed if compatible

**Success Criteria**:
- ✅ Partition detected within 10 seconds
- ✅ Both partitions operational during split
- ✅ Fork detected upon healing
- ✅ All nodes converge to same fork within 2 minutes
- ✅ No permanent data loss

**Failure Criteria**:
- ❌ Partition causes system crash
- ❌ Nodes fail to detect partition
- ❌ Fork not resolved
- ❌ Permanent inconsistency

**Recovery Strategy**:
- Automatic: MEF-Attractor convergence
- If convergence fails: Manual fork selection
- Expected recovery time: 2-3 minutes

---

### EDGE-003: Adversarial Traffic Injection

**Objective**: Test resilience against malicious packet injection

**Setup**:
- Network: 5 honest nodes
- 1 adversarial node injecting malicious packets

**Execution Steps**:
1. Start normal transaction flow
2. Adversarial node injects:
   - Packets with invalid ZK proofs
   - Packets with incorrect resonance
   - Replay attacks (old packets)
   - Packets with future timestamps
   - Malformed packets
3. Monitor honest nodes' behavior
4. Verify malicious packets rejected
5. Verify no impact on honest traffic

**Input**:
```json
{
  "honest_nodes": 5,
  "adversarial_injection_rate": 10,
  "attack_types": [
    "invalid_zk_proof",
    "resonance_mismatch",
    "replay_attack",
    "future_timestamp",
    "malformed_packet"
  ],
  "duration_seconds": 300
}
```

**Expected Behavior**:
- Invalid ZK proofs rejected
- Resonance mismatches ignored
- Replay attacks detected and dropped
- Future timestamps rejected
- Malformed packets cause parse errors (logged)
- Honest traffic unaffected
- Rate limiting prevents DOS

**Success Criteria**:
- ✅ 100% of malicious packets rejected
- ✅ Honest packet success rate >95%
- ✅ Rate limiting activates for adversarial node
- ✅ No crashes or resource exhaustion
- ✅ Security events logged

**Failure Criteria**:
- ❌ Malicious packets accepted
- ❌ Honest traffic disrupted
- ❌ DOS successful
- ❌ System crash

**Recovery Strategy**:
- Automatic: Rate limiting and packet rejection
- Manual: Ban adversarial node (if identified)
- Expected impact: Minimal, rate limiting contains attack

---

### EDGE-004: Fork Cascade Scenario

**Objective**: Test fork healing under multiple simultaneous forks

**Setup**:
- Network: 15 nodes in 5 clusters
- Each cluster: 3 nodes

**Execution Steps**:
1. Create 4 simultaneous partitions
2. Each partition produces conflicting blocks
3. Result: 5-way fork (worst case)
4. Heal all partitions simultaneously
5. MEF-Attractor must resolve 5-way fork
6. Verify deterministic convergence
7. Check ledger consistency

**Input**:
```json
{
  "clusters": 5,
  "nodes_per_cluster": 3,
  "partition_duration_seconds": 90,
  "transactions_per_cluster": 10,
  "fork_branches": 5
}
```

**Expected Behavior**:
- 5 independent forks created
- Each fork has valid blocks
- MEF-Attractor computes coherence for all 5 forks
- Highest coherence fork selected
- All nodes converge to selected fork
- Other 4 forks discarded
- Compatible transactions replayed

**Success Criteria**:
- ✅ All 5 forks detected
- ✅ MEF-Attractor converges within 5 minutes
- ✅ Selection is deterministic (same fork across all nodes)
- ✅ Final ledger consistency achieved
- ✅ No livelock or oscillation

**Failure Criteria**:
- ❌ Convergence fails
- ❌ Non-deterministic selection (nodes disagree)
- ❌ Livelock (oscillating between forks)
- ❌ Permanent split

**Recovery Strategy**:
- Automatic: MEF-Attractor with increased convergence timeout
- Manual: Administrator selects fork manually
- Expected recovery time: 3-10 minutes

---

### EDGE-005: Self-Healing Under Byzantine Failures

**Objective**: Test self-healing when nodes exhibit Byzantine behavior

**Setup**:
- Network: 10 nodes total
- 3 Byzantine nodes (arbitrary failures)
- 7 honest nodes

**Execution Steps**:
1. Start normal operation
2. Byzantine nodes exhibit:
   - Node A: Sends conflicting blocks
   - Node B: Delays messages randomly
   - Node C: Drops 50% of packets
3. Honest nodes detect Byzantine behavior
4. System continues operating with 7 honest nodes
5. Verify fork healing excludes Byzantine data
6. Remove Byzantine nodes
7. Verify system recovers

**Input**:
```json
{
  "total_nodes": 10,
  "byzantine_nodes": [7, 8, 9],
  "byzantine_behaviors": {
    "7": "conflicting_blocks",
    "8": "random_delays",
    "9": "packet_drops"
  },
  "duration_seconds": 300
}
```

**Expected Behavior**:
- Byzantine behavior detected through inconsistencies
- Honest nodes continue consensus
- Byzantine data excluded from MEF-Attractor
- System remains operational with 7 honest nodes
- Conflicting blocks from Byzantine nodes rejected
- No split among honest nodes

**Success Criteria**:
- ✅ System operational with 70% honest nodes
- ✅ Byzantine blocks rejected
- ✅ No consensus split among honest nodes
- ✅ Ledger consistency among honest nodes
- ✅ Transaction throughput >60% of normal

**Failure Criteria**:
- ❌ Byzantine nodes disrupt consensus
- ❌ Honest nodes split
- ❌ System halts

**Recovery Strategy**:
- Automatic: Exclude Byzantine data via consensus
- Manual: Remove Byzantine nodes from network
- Expected recovery time: Immediate (system tolerates up to 30% Byzantine)

---

### EDGE-006: Disappearing Services

**Objective**: Test behavior when ephemeral services crash unexpectedly

**Setup**:
- Network: 8 nodes
- 3 active ephemeral services
- Services crash before TTL expiration

**Execution Steps**:
1. Start 3 ephemeral services:
   - Service A: Marketplace
   - Service B: Voting
   - Service C: Auction
2. Nodes interact with services
3. Crash Service B mid-operation (no cleanup)
4. Verify nodes detect service failure
5. Verify partial audit trail captured
6. Verify other services unaffected
7. Verify crash recovery mechanisms

**Input**:
```json
{
  "services": [
    {"id": "svc_a", "type": "marketplace", "ttl": 600},
    {"id": "svc_b", "type": "voting", "ttl": 300},
    {"id": "svc_c", "type": "auction", "ttl": 400}
  ],
  "crash_service": "svc_b",
  "crash_timing": "50% through lifetime"
}
```

**Expected Behavior**:
- Service B crashes unexpectedly
- Nodes detect timeout (no heartbeat)
- Partial audit trail captured (up to crash point)
- Clients receive service unavailable error
- Service automatically cleaned from registry
- Services A and C unaffected
- No resource leaks

**Success Criteria**:
- ✅ Service crash detected within 30 seconds
- ✅ Partial audit trail preserved
- ✅ Service removed from registry
- ✅ No resource leaks
- ✅ Other services operational

**Failure Criteria**:
- ❌ Service crash undetected
- ❌ Registry corruption
- ❌ Resource leaks
- ❌ Other services affected

**Recovery Strategy**:
- Automatic: Timeout-based cleanup
- Audit trail: Save partial state
- Expected recovery time: <60 seconds

---

### EDGE-007: Massive Peer Join/Leave

**Objective**: Test scalability under rapid peer churn

**Setup**:
- Network: Start with 10 nodes
- Simulate peer churn: 50 nodes join, 40 leave, repeatedly

**Execution Steps**:
1. Start with 10 stable nodes
2. Phase 1: 20 nodes join rapidly (within 30 seconds)
3. Phase 2: 15 nodes leave
4. Phase 3: 30 more nodes join
5. Phase 4: 25 nodes leave
6. Phase 5: Return to 20 stable nodes
7. During churn: maintain transaction flow
8. Verify network stability throughout

**Input**:
```json
{
  "initial_nodes": 10,
  "churn_phases": [
    {"action": "join", "count": 20, "duration_seconds": 30},
    {"action": "leave", "count": 15, "duration_seconds": 20},
    {"action": "join", "count": 30, "duration_seconds": 40},
    {"action": "leave", "count": 25, "duration_seconds": 30}
  ],
  "transactions_per_minute": 60
}
```

**Expected Behavior**:
- Nodes join/leave without disrupting network
- Routing tables update dynamically
- Transactions continue flowing
- Resonance probabilities adjust to network size
- No message loss to stable nodes
- Discovery protocol handles churn

**Success Criteria**:
- ✅ All joins/leaves complete successfully
- ✅ Transaction success rate >90% throughout
- ✅ No message loss to stable nodes
- ✅ Routing converges within 60 seconds after churn ends
- ✅ Network stable at final size

**Failure Criteria**:
- ❌ Network instability during churn
- ❌ Transaction success rate <80%
- ❌ Routing fails to converge
- ❌ Message loss to stable nodes

**Recovery Strategy**:
- Automatic: Routing protocol adapts
- Manual: Reduce churn rate if instability detected
- Expected recovery time: <2 minutes after churn ends

---

### EDGE-008: Decoy Event Storm

**Objective**: Test privacy under high decoy traffic load

**Setup**:
- Network: 10 nodes
- Decoy traffic: 10x real transaction rate
- Goal: Maintain privacy while handling load

**Execution Steps**:
1. Start normal operation (10 TPS real)
2. Enable decoy traffic (100 TPS decoys)
3. Run for 10 minutes
4. Attempt traffic analysis:
   - Timing correlation
   - Size correlation
   - Pattern recognition
5. Verify decoys indistinguishable from real
6. Verify system handles load

**Input**:
```json
{
  "real_transaction_rate": 10,
  "decoy_transaction_rate": 100,
  "duration_seconds": 600,
  "traffic_analysis_techniques": [
    "timing_correlation",
    "size_correlation",
    "frequency_analysis",
    "pattern_recognition"
  ]
}
```

**Expected Behavior**:
- System handles 110 TPS (10 real + 100 decoy)
- Decoys indistinguishable from real packets:
  - Same size distribution
  - Same timing characteristics
  - Same resonance patterns
- Traffic analysis fails to separate real from decoy
- Real transaction latency remains low
- No resource exhaustion

**Success Criteria**:
- ✅ System stable at 110 TPS
- ✅ Decoys indistinguishable (>95% confidence required to distinguish)
- ✅ Real transaction latency <2x baseline
- ✅ No resource exhaustion
- ✅ Privacy maintained

**Failure Criteria**:
- ❌ System unstable under load
- ❌ Decoys distinguishable from real
- ❌ Real transaction latency >3x baseline
- ❌ Resource exhaustion

**Recovery Strategy**:
- Automatic: Adaptive decoy rate based on load
- Manual: Reduce decoy rate if overload detected
- Expected impact: Graceful degradation

---

### EDGE-009: Privacy Stress Test

**Objective**: Maximum privacy verification under adversarial analysis

**Setup**:
- Network: 20 nodes
- Adversarial observer: monitors all network traffic
- Goal: Verify zero metadata leakage

**Execution Steps**:
1. Send 100 transactions from various nodes
2. Adversarial observer captures:
   - All network packets
   - Timing information
   - Size information
   - Routing paths
3. Attempt to de-anonymize:
   - Link sender to transaction
   - Correlate transactions
   - Identify recipients
   - Reconstruct social graph
4. Verify all de-anonymization attempts fail

**Input**:
```json
{
  "transaction_count": 100,
  "sender_distribution": "uniform_across_20_nodes",
  "observer_capabilities": [
    "full_network_capture",
    "timing_analysis",
    "size_analysis",
    "traffic_correlation",
    "ml_pattern_recognition"
  ]
}
```

**Expected Behavior**:
- All transactions delivered successfully
- Ghost Network masking effective:
  - No sender identification
  - No recipient identification
  - No transaction correlation
- Resonance-based routing unpredictable
- Timing obfuscation via random delays
- Size obfuscation via padding
- ML analysis yields random guesses (50% accuracy)

**Success Criteria**:
- ✅ Sender identification accuracy ≤50% (random guess)
- ✅ Transaction correlation accuracy ≤50%
- ✅ Recipient identification accuracy ≤50%
- ✅ Social graph reconstruction fails
- ✅ Zero metadata leakage detected

**Failure Criteria**:
- ❌ Any de-anonymization accuracy >60%
- ❌ Metadata leaked (sender, recipient, timing)
- ❌ Social graph partially reconstructed

**Recovery Strategy**:
- N/A (privacy property verification)
- If failure: Increase masking strength, add more decoys

---

## Chaos Engineering Scenarios

### CHAOS-001: Random Node Crashes

**Objective**: Test system resilience under random node failures

**Setup**:
- Network: 20 nodes
- Crash probability: 10% per node per minute
- Duration: 30 minutes

**Execution Steps**:
1. Start normal operation
2. Every 60 seconds: randomly crash 2 nodes
3. Crashed nodes restart after 120 seconds
4. Continue transaction flow throughout
5. Monitor for system degradation

**Input**:
```json
{
  "node_count": 20,
  "crash_probability_per_minute": 0.1,
  "restart_delay_seconds": 120,
  "duration_minutes": 30,
  "transaction_rate": 20
}
```

**Expected Behavior**:
- Network remains operational
- Transaction success rate degrades gracefully
- No catastrophic failures
- Routing adapts to failures
- Nodes rejoin seamlessly

**Success Criteria**:
- ✅ Network operational throughout
- ✅ Transaction success rate >70%
- ✅ All crashed nodes successfully rejoin
- ✅ No permanent failures

**Failure Criteria**:
- ❌ Network-wide failure
- ❌ Transaction success rate <50%
- ❌ Nodes fail to rejoin

**Recovery Strategy**:
- Automatic: Node restart and rejoin
- Expected impact: Graceful degradation

---

### CHAOS-002: Network Latency Injection

**Objective**: Test adaptive behavior under variable latency

**Setup**:
- Network: 15 nodes
- Latency injection: Random delays 0-1000ms
- Distribution: Normal (mean=200ms, σ=150ms)

**Execution Steps**:
1. Start with normal latency (<10ms)
2. Phase 1: Inject 50ms latency
3. Phase 2: Inject 200ms latency
4. Phase 3: Inject 500ms latency
5. Phase 4: Random latency spikes (0-1000ms)
6. Monitor adaptive timestamp windows
7. Verify transaction success rates

**Input**:
```json
{
  "phases": [
    {"latency_mean_ms": 50, "duration_seconds": 300},
    {"latency_mean_ms": 200, "duration_seconds": 300},
    {"latency_mean_ms": 500, "duration_seconds": 300},
    {"latency_distribution": "random", "max_ms": 1000, "duration_seconds": 300}
  ]
}
```

**Expected Behavior**:
- Adaptive timestamp windows adjust to latency
- Transaction success rate remains high
- No false rejections due to latency
- System detects and adapts to conditions

**Success Criteria**:
- ✅ Transaction success rate >90% in all phases
- ✅ Timestamp window adapts correctly
- ✅ No false rejections
- ✅ Metrics show latency tracking

**Failure Criteria**:
- ❌ Transaction success rate <70%
- ❌ High false rejection rate
- ❌ System unresponsive

**Recovery Strategy**:
- Automatic: Adaptive windows adjust
- Manual: Increase timeout thresholds if needed

---

### CHAOS-003: Packet Loss Injection

**Objective**: Test reliability under packet loss

**Setup**:
- Network: 12 nodes
- Packet loss rate: Variable 5%-30%
- Duration: 20 minutes

**Execution Steps**:
1. Start normal operation (0% loss)
2. Phase 1: 5% packet loss
3. Phase 2: 15% packet loss
4. Phase 3: 30% packet loss
5. Monitor retransmission behavior
6. Verify transaction delivery

**Input**:
```json
{
  "phases": [
    {"loss_rate": 0.05, "duration_seconds": 400},
    {"loss_rate": 0.15, "duration_seconds": 400},
    {"loss_rate": 0.30, "duration_seconds": 400}
  ],
  "transaction_rate": 10
}
```

**Expected Behavior**:
- Retransmission mechanisms activate
- Transaction delivery success remains high
- Latency increases with loss rate
- No permanent message loss

**Success Criteria**:
- ✅ Transaction delivery >95% at 5% loss
- ✅ Transaction delivery >85% at 15% loss
- ✅ Transaction delivery >70% at 30% loss
- ✅ Retransmissions logged

**Failure Criteria**:
- ❌ Delivery rate below thresholds
- ❌ Permanent message loss
- ❌ System hangs

**Recovery Strategy**:
- Automatic: Retransmission protocol
- Manual: Reduce transaction rate if overload

---

### CHAOS-004: Byzantine Node Injection

**Objective**: Test Byzantine fault tolerance

**Setup**:
- Network: 12 nodes (9 honest, 3 Byzantine)
- Byzantine behaviors: Random

**Execution Steps**:
1. Start with 12 honest nodes
2. Convert 3 nodes to Byzantine:
   - Node A: Double-voting
   - Node B: Conflicting blocks
   - Node C: Invalid signatures
3. Monitor honest nodes' behavior
4. Verify Byzantine tolerance
5. Measure impact on throughput

**Input**:
```json
{
  "total_nodes": 12,
  "byzantine_count": 3,
  "byzantine_strategies": [
    "double_voting",
    "conflicting_blocks",
    "invalid_signatures"
  ],
  "duration_minutes": 15
}
```

**Expected Behavior**:
- Honest nodes detect Byzantine behavior
- Byzantine data rejected
- Consensus continues with 9 honest nodes
- System remains operational
- Throughput: >70% of normal

**Success Criteria**:
- ✅ Byzantine behavior detected
- ✅ Byzantine data rejected
- ✅ System operational
- ✅ Throughput >70%

**Failure Criteria**:
- ❌ Byzantine nodes disrupt consensus
- ❌ System halts
- ❌ Throughput <50%

**Recovery Strategy**:
- Automatic: Byzantine exclusion
- Manual: Ban Byzantine nodes

---

### CHAOS-005: Resource Exhaustion

**Objective**: Test behavior under resource constraints

**Setup**:
- Network: 10 nodes
- Resource limits: CPU, memory, disk, network
- Exhaustion strategy: Gradual increase

**Execution Steps**:
1. Start with normal resources
2. CPU: Limit to 50% capacity
3. Memory: Limit to 1GB per node
4. Disk: Fill to 95% capacity
5. Network: Bandwidth limit 10Mbps
6. Monitor system behavior
7. Verify graceful degradation

**Input**:
```json
{
  "resource_limits": {
    "cpu_percent": 50,
    "memory_mb": 1024,
    "disk_percent": 95,
    "network_mbps": 10
  },
  "ramp_up_duration_seconds": 300,
  "duration_minutes": 20
}
```

**Expected Behavior**:
- System detects resource constraints
- Graceful degradation:
  - Reduced transaction rate
  - Increased latency
  - Queue backpressure
- No crashes or OOM kills
- Recovery when resources restored

**Success Criteria**:
- ✅ No crashes
- ✅ Graceful degradation observed
- ✅ System remains responsive
- ✅ Recovery successful

**Failure Criteria**:
- ❌ Crash or OOM kill
- ❌ System unresponsive
- ❌ Permanent failure

**Recovery Strategy**:
- Automatic: Backpressure and rate limiting
- Manual: Increase resource limits

---

### CHAOS-006: Time Synchronization Attacks

**Objective**: Test resilience against clock skew attacks

**Setup**:
- Network: 10 nodes
- Clock skew: Inject random skews ±300 seconds
- Some nodes: Future timestamps
- Some nodes: Past timestamps

**Execution Steps**:
1. Start with synchronized clocks
2. Phase 1: Inject +300s skew on 2 nodes
3. Phase 2: Inject -300s skew on 2 nodes
4. Phase 3: Random skews on all nodes
5. Monitor timestamp validation
6. Verify attack mitigation

**Input**:
```json
{
  "skew_scenarios": [
    {"nodes": [0,1], "skew_seconds": 300},
    {"nodes": [2,3], "skew_seconds": -300},
    {"nodes": [4,5,6,7,8,9], "skew_seconds": "random", "range": [-300, 300]}
  ],
  "duration_minutes": 15
}
```

**Expected Behavior**:
- Adaptive timestamp windows tolerate moderate skew
- Extreme skews (>300s) rejected
- Honest nodes maintain consensus
- Clock skew attacks fail
- No disruption to honest nodes

**Success Criteria**:
- ✅ Extreme skews rejected
- ✅ Moderate skews tolerated (within adaptive window)
- ✅ Honest nodes unaffected
- ✅ No consensus disruption

**Failure Criteria**:
- ❌ Extreme skews accepted
- ❌ Honest nodes disrupted
- ❌ Consensus split

**Recovery Strategy**:
- Automatic: Timestamp validation
- Manual: NTP synchronization

---

### CHAOS-007: Fork Bomb (Rapid Partitioning)

**Objective**: Test fork healing under rapid partition creation

**Setup**:
- Network: 16 nodes
- Partition strategy: Create/heal partitions rapidly

**Execution Steps**:
1. Start with unified network
2. Create 4-way partition (every 30 seconds, create new partition)
3. Each partition lasts 60 seconds
4. Partitions overlap (healing while new ones form)
5. Continuous transaction flow
6. Monitor fork creation and healing rate
7. Verify eventual consistency

**Input**:
```json
{
  "node_count": 16,
  "partition_frequency_seconds": 30,
  "partition_duration_seconds": 60,
  "max_concurrent_partitions": 3,
  "test_duration_minutes": 10
}
```

**Expected Behavior**:
- Multiple overlapping forks created
- MEF-Attractor continuously resolving forks
- System remains operational despite chaos
- Eventual consistency achieved
- No permanent splits

**Success Criteria**:
- ✅ System remains operational
- ✅ Forks continuously resolved
- ✅ Final consistency achieved (within 5 minutes after test ends)
- ✅ No permanent splits
- ✅ Transaction delivery >60%

**Failure Criteria**:
- ❌ System halts
- ❌ Unresolved forks accumulate
- ❌ Permanent split
- ❌ Livelock

**Recovery Strategy**:
- Automatic: MEF-Attractor healing
- Manual: Stop test, allow convergence time
- Expected recovery time: 5-10 minutes

---

### CHAOS-008: Service Thrashing

**Objective**: Test ephemeral service stability under rapid create/destroy

**Setup**:
- Network: 10 nodes
- Service churn: Create 5 services/second, destroy 5 services/second

**Execution Steps**:
1. Start with no services
2. Rapid service creation: 5/second
3. Rapid service destruction: 5/second
4. Run for 10 minutes
5. Monitor:
   - Registry consistency
   - Resource leaks
   - Service discovery performance
6. Verify no resource exhaustion

**Input**:
```json
{
  "service_creation_rate": 5,
  "service_destruction_rate": 5,
  "service_types": ["voting", "marketplace", "auction"],
  "duration_minutes": 10
}
```

**Expected Behavior**:
- Services created and destroyed rapidly
- Registry remains consistent
- No resource leaks
- Service discovery remains fast
- TTL-based cleanup functional

**Success Criteria**:
- ✅ Registry consistency maintained
- ✅ No resource leaks
- ✅ Discovery latency <100ms
- ✅ No memory growth

**Failure Criteria**:
- ❌ Registry corruption
- ❌ Resource leaks detected
- ❌ Memory exhaustion
- ❌ Discovery performance degrades

**Recovery Strategy**:
- Automatic: Garbage collection
- Manual: Clear registry and restart
- Expected impact: Isolated to service layer

---

### CHAOS-009: Multi-Fault Compound Scenario

**Objective**: Test system resilience under multiple simultaneous faults

**Setup**:
- Network: 20 nodes
- Simultaneous faults:
  - 20% packet loss
  - 3 nodes crash
  - 2 Byzantine nodes
  - 200ms network latency
  - 1 partition (5 nodes isolated)

**Execution Steps**:
1. Start normal operation
2. Inject all faults simultaneously
3. Maintain faults for 10 minutes
4. Continue transaction flow
5. Monitor system behavior
6. Measure degradation
7. Remove faults and verify recovery

**Input**:
```json
{
  "faults": {
    "packet_loss": 0.20,
    "crashed_nodes": [0,1,2],
    "byzantine_nodes": [3,4],
    "network_latency_ms": 200,
    "partition": {"nodes": [15,16,17,18,19], "duration_seconds": 600}
  },
  "duration_minutes": 10
}
```

**Expected Behavior**:
- System remains operational despite compound faults
- Graceful degradation:
  - Reduced throughput (50-70% of normal)
  - Increased latency
  - Some transaction failures acceptable
- No catastrophic failures
- Recovery after fault removal

**Success Criteria**:
- ✅ System remains operational
- ✅ Transaction success rate >50%
- ✅ No catastrophic failures
- ✅ Recovery successful (>90% throughput within 5 minutes)

**Failure Criteria**:
- ❌ System-wide failure
- ❌ Permanent split
- ❌ Unable to recover

**Recovery Strategy**:
- Automatic: Multiple resilience mechanisms activate
- Manual: Remove faults sequentially if needed
- Expected recovery time: 3-5 minutes

---

### CHAOS-010: Adversarial Swarm Attack

**Objective**: Test defense against coordinated adversarial attack

**Setup**:
- Network: 15 honest nodes
- Adversaries: 10 attacking nodes join
- Attack strategy: Coordinated

**Execution Steps**:
1. Start with 15 honest nodes
2. 10 adversarial nodes join simultaneously
3. Adversaries coordinate attack:
   - Spam transactions (1000 TPS)
   - Invalid ZK proofs
   - Replay attacks
   - Resource exhaustion attempts
4. Monitor defense mechanisms:
   - Rate limiting
   - Proof validation
   - Replay detection
5. Verify honest node operation
6. Measure impact

**Input**:
```json
{
  "honest_nodes": 15,
  "adversarial_nodes": 10,
  "attack_strategy": {
    "spam_tps": 1000,
    "invalid_proofs": true,
    "replay_attacks": true,
    "resource_exhaustion": true
  },
  "duration_minutes": 15
}
```

**Expected Behavior**:
- Rate limiting activates
- Invalid proofs rejected
- Replay attacks detected
- Honest nodes maintain operation
- Adversarial nodes eventually rate-limited/banned

**Success Criteria**:
- ✅ Honest node operation >80% throughput
- ✅ Attack contained by rate limiting
- ✅ Invalid proofs rejected (100%)
- ✅ No resource exhaustion
- ✅ Honest transaction success rate >90%

**Failure Criteria**:
- ❌ Honest nodes disrupted
- ❌ Resource exhaustion successful
- ❌ DOS achieved

**Recovery Strategy**:
- Automatic: Rate limiting and banning
- Manual: Network-level filtering
- Expected impact: Minimal to honest nodes

---

## Test Execution Framework

### Test Orchestration

**Framework Architecture**:
```
e2e-testing/
├── orchestrator/
│   ├── test_runner.rs       # Main test orchestration
│   ├── network_simulator.rs # Network topology simulation
│   ├── fault_injector.rs    # Chaos engineering faults
│   └── metrics_collector.rs # Real-time metrics collection
├── scenarios/
│   ├── production/          # PROD-XXX tests
│   ├── edge_cases/          # EDGE-XXX tests
│   └── chaos/               # CHAOS-XXX tests
└── reports/
    ├── test_results.json
    └── analysis_reports/
```

### Test Runner Configuration

```rust
pub struct TestRunner {
    pub test_id: String,
    pub network: NetworkSimulator,
    pub fault_injector: FaultInjector,
    pub metrics: MetricsCollector,
    pub config: TestConfig,
}

pub struct TestConfig {
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub observability: ObservabilityConfig,
    pub cleanup_policy: CleanupPolicy,
}

impl TestRunner {
    pub async fn execute_test(&mut self, test_def: TestDefinition) -> TestResult {
        // 1. Setup
        self.setup(&test_def.setup).await?;

        // 2. Execute
        self.metrics.start_collection();
        let execution_result = self.execute(&test_def.execution).await;

        // 3. Validate
        let validation_result = self.validate(&test_def.validation).await;

        // 4. Cleanup
        self.cleanup(&test_def.cleanup).await?;

        // 5. Report
        self.generate_report(execution_result, validation_result)
    }
}
```

### Network Simulator

```rust
pub struct NetworkSimulator {
    pub nodes: Vec<SimulatedNode>,
    pub topology: NetworkTopology,
    pub fault_injector: Arc<FaultInjector>,
}

impl NetworkSimulator {
    pub async fn create_topology(&mut self, config: &TopologyConfig) -> Result<()> {
        match config.topology_type {
            TopologyType::Mesh => self.create_mesh(config.node_count),
            TopologyType::Ring => self.create_ring(config.node_count),
            TopologyType::Star => self.create_star(config.node_count),
            TopologyType::Random => self.create_random(config.node_count),
            TopologyType::Clustered => self.create_clustered(config),
            TopologyType::Partitioned => self.create_partitioned(config),
        }
    }

    pub async fn inject_partition(&mut self, partition: &PartitionConfig) -> Result<()> {
        // Isolate specified nodes
        for node_id in &partition.nodes {
            match partition.isolation_level {
                IsolationLevel::Full => self.fully_isolate(*node_id).await?,
                IsolationLevel::Partial => self.partially_isolate(*node_id).await?,
                IsolationLevel::Intermittent => self.intermittent_isolate(*node_id).await?,
            }
        }
        Ok(())
    }

    pub async fn heal_partition(&mut self, partition: &PartitionConfig) -> Result<()> {
        // Reconnect isolated nodes
        for node_id in &partition.nodes {
            self.reconnect(*node_id).await?;
        }
        Ok(())
    }
}
```

### Fault Injector

```rust
pub struct FaultInjector {
    pub active_faults: HashMap<String, ActiveFault>,
}

pub enum FaultType {
    NetworkDelay { delay_ms: u64, variance_ms: u64 },
    PacketLoss { loss_rate: f64 },
    NodeCrash { node_id: usize },
    ByzantineBehavior { strategy: ByzantineStrategy },
    ResourceExhaustion { resource: Resource, limit: u64 },
    ClockSkew { skew_seconds: i64 },
}

impl FaultInjector {
    pub async fn inject_fault(&mut self, fault: FaultType, duration: Duration) -> Result<FaultHandle> {
        let fault_id = Uuid::new_v4().to_string();
        let active_fault = ActiveFault::new(fault, duration);

        // Apply fault
        match &active_fault.fault_type {
            FaultType::NetworkDelay { delay_ms, variance_ms } => {
                self.inject_network_delay(*delay_ms, *variance_ms).await?;
            }
            FaultType::PacketLoss { loss_rate } => {
                self.inject_packet_loss(*loss_rate).await?;
            }
            FaultType::NodeCrash { node_id } => {
                self.crash_node(*node_id).await?;
            }
            // ... other fault types
        }

        self.active_faults.insert(fault_id.clone(), active_fault);
        Ok(FaultHandle { fault_id })
    }

    pub async fn remove_fault(&mut self, handle: FaultHandle) -> Result<()> {
        if let Some(fault) = self.active_faults.remove(&handle.fault_id) {
            self.undo_fault(fault).await?;
        }
        Ok(())
    }
}
```

### Metrics Collector

```rust
pub struct MetricsCollector {
    pub metrics: HashMap<String, MetricTimeSeries>,
    pub sampling_interval: Duration,
}

pub struct MetricTimeSeries {
    pub name: String,
    pub data_points: Vec<DataPoint>,
}

pub struct DataPoint {
    pub timestamp: SystemTime,
    pub value: f64,
}

impl MetricsCollector {
    pub async fn collect_metrics(&mut self) -> Result<()> {
        loop {
            tokio::time::sleep(self.sampling_interval).await;

            // Collect standard metrics
            self.record("throughput_tps", self.measure_throughput().await);
            self.record("latency_p50_ms", self.measure_latency_p50().await);
            self.record("latency_p99_ms", self.measure_latency_p99().await);
            self.record("success_rate", self.measure_success_rate().await);
            self.record("active_nodes", self.count_active_nodes().await);
            self.record("partition_count", self.count_partitions().await);

            // Collect protocol-specific metrics
            self.record("resonance_hit_rate", self.measure_resonance_hits().await);
            self.record("masking_overhead_ms", self.measure_masking_overhead().await);
            self.record("fork_count", self.count_active_forks().await);
            self.record("ephemeral_service_count", self.count_ephemeral_services().await);
        }
    }

    pub fn generate_summary(&self) -> MetricsSummary {
        MetricsSummary {
            latency_p50: self.percentile("latency_p50_ms", 0.5),
            latency_p99: self.percentile("latency_p99_ms", 0.99),
            throughput_avg: self.average("throughput_tps"),
            success_rate_avg: self.average("success_rate"),
            // ... other aggregations
        }
    }
}
```

---

## Metrics & Observability

### Key Metrics to Track

**Performance Metrics**:
- `throughput_tps`: Transactions per second
- `latency_p50_ms`: Median latency
- `latency_p95_ms`: 95th percentile latency
- `latency_p99_ms`: 99th percentile latency
- `success_rate`: Percentage of successful transactions

**Network Metrics**:
- `active_node_count`: Number of active nodes
- `partition_count`: Number of active partitions
- `message_loss_rate`: Percentage of lost messages
- `hop_count_avg`: Average hop count for routed packets
- `network_latency_ms`: Observed network latency

**Protocol Metrics**:
- `resonance_hit_rate`: Percentage of packets resonating with target
- `masking_overhead_ms`: Time spent masking/unmasking
- `key_rotation_count`: Number of key rotations
- `forward_secrecy_overhead_ms`: Forward secrecy overhead
- `adaptive_window_size_seconds`: Current adaptive timestamp window

**Chaos Metrics**:
- `fault_injection_count`: Number of active faults
- `recovery_time_seconds`: Time to recover from fault
- `degradation_factor`: Performance degradation (0-1)

**Fork Healing Metrics**:
- `fork_count`: Number of active forks
- `fork_healing_time_seconds`: Time to resolve fork
- `mef_coherence_score`: MEF-Attractor coherence score
- `convergence_success_rate`: Percentage of successful convergences

**Privacy Metrics**:
- `decoy_rate`: Decoy traffic as percentage of total
- `anonymity_set_size`: k-anonymity set size
- `metadata_leakage_score`: Detected metadata leakage (0-1)

### Observability Stack

**Logging**:
- Structured JSON logs
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- Security event logging (R-02-002)
- Log aggregation: ELK Stack or Grafana Loki

**Tracing**:
- Distributed tracing with OpenTelemetry
- Trace packet flow across hops
- Trace transaction lifecycle
- Trace fork healing process

**Dashboards**:
- Real-time performance dashboard
- Network topology visualization
- Fault injection status
- Test progress tracking

**Alerting**:
- Test failure alerts
- Performance degradation alerts
- Resource exhaustion alerts
- Security event alerts

---

## Test Execution Examples

### Example 1: Running Production Test

```bash
# Run single production test
cargo test --test e2e_production -- PROD-001

# Run all production tests
cargo test --test e2e_production

# Run with detailed output
RUST_LOG=debug cargo test --test e2e_production -- --nocapture
```

### Example 2: Running Chaos Engineering Test

```bash
# Run single chaos test
cargo test --test e2e_chaos -- CHAOS-001

# Run chaos suite
cargo test --test e2e_chaos

# Run with fault injection visualization
cargo test --test e2e_chaos -- --nocapture --show-faults
```

### Example 3: Generating Test Report

```bash
# Run tests and generate report
cargo test --all-tests -- --format json > test_results.json

# Generate HTML report
./e2e-testing/scripts/generate_report.sh test_results.json

# View report
open e2e-testing/reports/test_report.html
```

---

## Conclusion

This comprehensive test catalog provides:
- **20+ production scenarios** covering normal operation
- **20+ edge case scenarios** testing boundary conditions
- **10+ chaos engineering scenarios** validating resilience
- **Complete test framework** for execution and monitoring
- **Detailed observability** for debugging and analysis

All tests include:
- Clear objectives and setup instructions
- Detailed execution steps
- Expected behavior specifications
- Success/failure criteria
- Recovery strategies

This suite ensures SpectralChain is production-ready and resilient to real-world challenges.

---

**Document Version:** 1.0.0
**Last Updated:** 2025-11-06
**Status:** Complete and ready for implementation
