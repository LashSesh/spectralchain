# Network Transport Layer - Implementation Plan

**Date:** 2025-11-06
**Priority:** 🚨 CRITICAL (NET-001)
**Estimated Effort:** 30-40 hours
**Status:** Planning → Implementation

---

## Executive Summary

Implement real network transport for Ghost Network. Currently all networking is in-memory only. This is the **#1 blocker** for production use.

**Goal:** Enable nodes to communicate over real TCP/UDP networks using libp2p.

---

## Current State

### What Exists ✅
- Ghost Protocol (6-step flow) - fully implemented
- Addressless Broadcasting - works in-memory
- Discovery Engine - works in-memory
- Quantum Routing - algorithm complete
- libp2p dependency - declared but unused

### What's Missing ❌
- Actual TCP/UDP/QUIC transport
- Network packet serialization
- Peer connection management
- Listen/Dial network operations
- Network error handling

---

## Architecture

### Layer Stack

```
┌─────────────────────────────────────────┐
│      Ghost Protocol (protocol.rs)       │  ← Already exists
├─────────────────────────────────────────┤
│    Broadcasting/Discovery (*.rs)        │  ← Already exists
├─────────────────────────────────────────┤
│     Transport Layer (NEW)               │  ← WE ADD THIS
│  - libp2p Integration                   │
│  - TCP/UDP/QUIC                         │
│  - Peer Management                      │
│  - Packet Serialization                 │
└─────────────────────────────────────────┘
```

### New Module: `mef-ghost-network/src/transport/`

```
mef-ghost-network/src/transport/
├── mod.rs                  # Transport trait & API
├── libp2p_transport.rs     # libp2p implementation
├── peer.rs                 # Peer management
├── codec.rs                # Packet serialization
└── config.rs               # Transport configuration
```

---

## Implementation Plan

### Phase 1: Transport Trait & Codec (6-8 hours)

**Files to create:**
1. `src/transport/mod.rs` - Transport trait definition
2. `src/transport/codec.rs` - Packet serialization/deserialization
3. `src/transport/config.rs` - Configuration

**Key Types:**

```rust
/// Transport trait for network communication
pub trait Transport: Send + Sync {
    /// Listen on a network address
    async fn listen(&mut self, addr: String) -> Result<()>;

    /// Dial a peer
    async fn dial(&mut self, addr: String) -> Result<PeerId>;

    /// Send packet to peer
    async fn send(&mut self, peer: PeerId, packet: GhostPacket) -> Result<()>;

    /// Broadcast packet to all connected peers
    async fn broadcast(&mut self, packet: GhostPacket) -> Result<()>;

    /// Receive next packet
    async fn receive(&mut self) -> Result<(PeerId, GhostPacket)>;

    /// Get list of connected peers
    fn peers(&self) -> Vec<PeerId>;
}

/// Packet codec for wire format
pub struct PacketCodec;

impl PacketCodec {
    /// Serialize packet to bytes
    pub fn encode(packet: &GhostPacket) -> Result<Vec<u8>>;

    /// Deserialize packet from bytes
    pub fn decode(bytes: &[u8]) -> Result<GhostPacket>;
}
```

**Deliverables:**
- ✅ Transport trait definition
- ✅ Codec implementation (JSON/bincode)
- ✅ Configuration struct
- ✅ Basic tests

### Phase 2: libp2p Transport Implementation (10-12 hours)

**Files to create:**
1. `src/transport/libp2p_transport.rs` - Main implementation
2. `src/transport/peer.rs` - Peer management

**Key Components:**

```rust
pub struct Libp2pTransport {
    /// libp2p Swarm
    swarm: Swarm<GhostBehaviour>,

    /// Peer manager
    peers: Arc<RwLock<PeerManager>>,

    /// Packet receive queue
    rx_queue: Arc<RwLock<VecDeque<(PeerId, GhostPacket)>>>,

    /// Configuration
    config: TransportConfig,
}

impl Libp2pTransport {
    pub async fn new(config: TransportConfig) -> Result<Self>;

    pub async fn start(&mut self) -> Result<()>;

    // Implement Transport trait
}

/// libp2p Network Behaviour for Ghost Protocol
#[derive(NetworkBehaviour)]
pub struct GhostBehaviour {
    /// Gossipsub for broadcasting
    gossipsub: gossipsub::Behaviour,

    /// Identify for peer discovery
    identify: identify::Behaviour,

    /// Ping for keepalive
    ping: ping::Behaviour,
}
```

**libp2p Setup:**
- TCP transport with noise encryption
- Yamux multiplexing
- Gossipsub for broadcasting
- Identify protocol for peer info
- Ping for connection health

**Deliverables:**
- ✅ Libp2pTransport implementation
- ✅ GhostBehaviour for libp2p
- ✅ Peer connection management
- ✅ Basic send/receive working

### Phase 3: Integration with Existing Modules (8-10 hours)

**Files to modify:**
1. `src/broadcasting.rs` - Use Transport instead of in-memory
2. `src/discovery.rs` - Use Transport for beacon exchange
3. `src/lib.rs` - Add Transport to GhostNetwork
4. `src/protocol.rs` - Integrate with Transport

**Changes:**

```rust
// Before (in-memory)
impl BroadcastEngine {
    pub fn broadcast(&mut self, packet: GhostPacket) -> Result<()> {
        // In-memory buffer
        self.buffers.write().unwrap()...
    }
}

// After (real network)
impl BroadcastEngine {
    pub async fn broadcast(&mut self, packet: GhostPacket) -> Result<()> {
        // Use transport
        self.transport.broadcast(packet).await
    }
}
```

**Integration Points:**
- Broadcasting → Transport::broadcast()
- Discovery → Transport for beacon exchange
- Protocol → Transport for packet transmission
- GhostNetwork → Owns Transport instance

**Deliverables:**
- ✅ Broadcasting uses Transport
- ✅ Discovery uses Transport
- ✅ Protocol integrated
- ✅ GhostNetwork updated

### Phase 4: Testing & Validation (6-8 hours)

**Tests to create:**
1. Unit tests for Transport trait
2. Integration tests for libp2p
3. End-to-end Ghost Protocol tests
4. Multi-node communication tests

**Test Scenarios:**
```rust
#[tokio::test]
async fn test_two_nodes_communicate() {
    // Start node 1
    let mut node1 = create_node("127.0.0.1:9000").await;

    // Start node 2
    let mut node2 = create_node("127.0.0.1:9001").await;

    // Node 2 dials node 1
    node2.dial("127.0.0.1:9000").await.unwrap();

    // Node 1 broadcasts packet
    let packet = create_test_packet();
    node1.broadcast(packet.clone()).await.unwrap();

    // Node 2 receives packet
    let (peer, received) = node2.receive().await.unwrap();
    assert_eq!(received.id, packet.id);
}

#[tokio::test]
async fn test_resonance_routing() {
    // Create 5 nodes with different resonance states
    // Send packet with specific resonance
    // Verify only resonant nodes receive it
}

#[tokio::test]
async fn test_discovery_beacons() {
    // Start nodes
    // Send discovery beacons
    // Verify nodes discover each other
}
```

**Deliverables:**
- ✅ Unit tests passing
- ✅ Integration tests passing
- ✅ End-to-end tests passing
- ✅ Documentation updated

---

## Dependencies

### Already Available
- `libp2p` - P2P networking stack
- `tokio` - Async runtime
- `serde` - Serialization
- `serde_json` / `bincode` - Wire formats

### New Dependencies (if needed)
- None (all already in workspace)

---

## Risks & Mitigations

### Risk 1: libp2p Complexity
**Risk:** libp2p is complex, may take longer than estimated
**Mitigation:** Start with minimal setup (TCP + Gossipsub), expand later

### Risk 2: NAT Traversal
**Risk:** Nodes behind NAT can't connect
**Mitigation:** Phase 1 focuses on local/public IPs. NAT traversal is NET-002 (separate task)

### Risk 3: Breaking Changes to Existing API
**Risk:** Adding async may break existing code
**Mitigation:** Careful API design, add async gradually, keep compatibility layer

### Risk 4: Performance
**Risk:** Network overhead may be high
**Mitigation:** Profile and optimize in later phase, focus on correctness first

---

## Success Criteria

### Phase 1 ✅
- [ ] Transport trait compiles
- [ ] Codec can serialize/deserialize GhostPacket
- [ ] Basic tests pass

### Phase 2 ✅
- [ ] libp2p transport implementation compiles
- [ ] Can listen on TCP port
- [ ] Can dial another node
- [ ] Can send packet peer-to-peer

### Phase 3 ✅
- [ ] Broadcasting uses real network
- [ ] Discovery uses real network
- [ ] Ghost Protocol works end-to-end
- [ ] No breaking changes to existing tests (or update them)

### Phase 4 ✅
- [ ] All tests passing
- [ ] Multi-node communication works
- [ ] Resonance-based routing validated
- [ ] Documentation updated

### Final Acceptance ✅
- [ ] Two nodes can communicate over real TCP
- [ ] Ghost Protocol 6-step flow works end-to-end
- [ ] Addressless routing via resonance works
- [ ] Discovery beacons work
- [ ] Code is documented and tested

---

## Timeline

**Total Estimated:** 30-40 hours

- **Week 1 (16-20h):**
  - Phase 1: Transport trait & Codec (6-8h)
  - Phase 2: libp2p implementation (10-12h)

- **Week 2 (14-20h):**
  - Phase 3: Integration (8-10h)
  - Phase 4: Testing & Validation (6-8h)

**Checkpoints:**
- End of Day 3: Phase 1 complete
- End of Day 7: Phase 2 complete
- End of Day 10: Phase 3 complete
- End of Day 12: Phase 4 complete

---

## Next Steps

### Immediate (Today)
1. ✅ Create this plan
2. ✅ Commit plan to repo
3. 🚀 Create transport module structure
4. 🚀 Implement Transport trait
5. 🚀 Implement PacketCodec

### This Week
- Complete Phase 1 & 2
- Get basic libp2p working
- Test peer-to-peer communication

### Next Week
- Complete Phase 3 & 4
- Integration testing
- Documentation

---

## Notes

- This is the **#1 priority** for SpectralChain
- Without this, the system cannot function in a real network
- Keep changes minimal and focused
- Maintain backward compatibility where possible
- Add async gradually (some methods may need to become async)
- Document all breaking changes

---

**Status:** 📝 Plan Complete → Ready for Implementation
**Next Action:** Create `src/transport/` directory and start Phase 1
**Owner:** AI Development Agent
**Last Updated:** 2025-11-06
