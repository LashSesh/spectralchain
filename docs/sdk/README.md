# SpectralChain SDK Documentation

**Version**: 2.0.0
**Last Updated**: 2025-11-06

---

## Overview

The SpectralChain SDK provides programmatic access to all SpectralChain / Infinity Ledger functionality. Currently available in Rust with planned support for Python, TypeScript, and Go.

---

## Table of Contents

1. [Rust SDK](#rust-sdk)
2. [Python SDK](#python-sdk-planned)
3. [TypeScript SDK](#typescript-sdk-planned)
4. [Go SDK](#go-sdk-planned)
5. [SDK Comparison](#sdk-comparison)
6. [Examples](#examples)

---

## Rust SDK

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mef-core = { path = "../mef-core" }
mef-spiral = { path = "../mef-spiral" }
mef-ledger = { path = "../mef-ledger" }
mef-tic = { path = "../mef-tic" }
mef-vector-db = { path = "../mef-vector-db" }
mef-quantum-ops = { path = "../../mef-quantum-ops" }
mef-ghost-network = { path = "../../mef-ghost-network" }
```

### Core Modules

#### mef-core

**Main Exports**:
```rust
use mef_core::{
    Pipeline,
    Cube,
    Mandorla,
    ResonanceTensor,
};
```

**Example**:
```rust
use mef_core::Pipeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline::new().await?;

    let input_data = vec![1.0, 2.0, 3.0];
    let result = pipeline.process(input_data).await?;

    println!("Result: {:?}", result);
    Ok(())
}
```

#### mef-spiral

**Main Exports**:
```rust
use mef_spiral::{
    SpiralSnapshot,
    ProofOfResonance,
    SpiralStorage,
};
```

**Example**:
```rust
use mef_spiral::{SpiralSnapshot, SpiralStorage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = SpiralStorage::new("./snapshots")?;

    // Create snapshot
    let snapshot = SpiralSnapshot::from_bytes(
        b"Hello, SpectralChain!",
        "my-seed",
        None, // default params
    )?;

    // Store snapshot
    storage.store(&snapshot).await?;

    // Retrieve snapshot
    let retrieved = storage.load(&snapshot.id).await?;

    println!("Snapshot ID: {}", retrieved.id);
    println!("PoR: {:?}", retrieved.proof_of_resonance);

    Ok(())
}
```

#### mef-ledger

**Main Exports**:
```rust
use mef_ledger::{
    MefLedger,
    Block,
    BlockHeader,
};
```

**Example**:
```rust
use mef_ledger::MefLedger;
use mef_tic::TIC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ledger = MefLedger::new("./ledger")?;

    // Create TIC (simplified)
    let tic = TIC {
        id: "tic_123".to_string(),
        snapshot_id: "snap_456".to_string(),
        eigenvalue: 0.999987,
        iterations: 42,
        converged: true,
        timestamp: chrono::Utc::now(),
    };

    // Append to ledger
    let block_index = ledger.append(tic).await?;

    // Verify integrity
    let audit_result = ledger.verify_integrity().await?;
    println!("Ledger valid: {}", audit_result.valid);

    // Get block
    let block = ledger.get_block(block_index).await?;
    println!("Block hash: {}", block.hash);

    Ok(())
}
```

#### mef-tic

**Main Exports**:
```rust
use mef_tic::{
    TICCrystallizer,
    TIC,
    ConvergenceInfo,
};
```

**Example**:
```rust
use mef_tic::TICCrystallizer;
use mef_spiral::SpiralSnapshot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crystallizer = TICCrystallizer::new()?;

    let snapshot = SpiralSnapshot::from_bytes(
        b"data",
        "seed",
        None,
    )?;

    // Crystallize TIC
    let tic = crystallizer.crystallize(&snapshot).await?;

    println!("TIC ID: {}", tic.id);
    println!("Converged: {}", tic.converged);
    println!("Eigenvalue: {}", tic.eigenvalue);
    println!("Iterations: {}", tic.iterations);

    Ok(())
}
```

#### mef-quantum-ops

**Main Exports**:
```rust
use mef_quantum_ops::{
    QuantumOperator,
    MaskingOperator,
    ResonanceOperator,
    SteganographyOperator,
    ZKProofOperator,
};
```

**Example**:
```rust
use mef_quantum_ops::{QuantumOperator, MaskingOperator, MaskingParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operator = MaskingOperator::new()?;

    let data = b"Sensitive data";
    let seed = "my-secret-seed";
    let params = MaskingParams {
        strength: 256,
        deterministic: true,
    };

    // Mask data
    let masked = operator.apply(data, &params)?;
    println!("Masked: {:?}", masked);

    // Unmask data
    let unmasked = operator.reverse(&masked, &params)?;
    println!("Unmasked: {:?}", String::from_utf8_lossy(&unmasked));

    Ok(())
}
```

#### mef-ghost-network

**Main Exports**:
```rust
use mef_ghost_network::{
    GhostNetwork,
    GhostProtocol,
    ResonanceState,
};
```

**Example**:
```rust
use mef_ghost_network::{GhostNetwork, ResonanceState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create ghost network node
    let network = GhostNetwork::new().await?;

    // Announce capabilities
    let node_id = network.announce(Some(vec![
        "data-processing".to_string(),
        "storage".to_string(),
    ])).await?;

    println!("Announced with ID: {}", node_id);

    // Find nodes with capability
    let resonance = ResonanceState::new(vec![1.0, 0.5, 0.3]);
    let nodes = network.find_nodes(&resonance).await?;

    println!("Found {} nodes", nodes.len());

    // Send transaction
    let tx_id = network.send_transaction(
        resonance,
        b"Hello, ghost network!".to_vec(),
    ).await?;

    println!("Transaction sent: {}", tx_id);

    // Receive transactions
    let transactions = network.receive_transactions().await?;
    for tx in transactions {
        println!("Received: {:?}", tx);
    }

    Ok(())
}
```

### Complete Example: Ingest → Process → Ledger

```rust
use mef_spiral::{SpiralSnapshot, SpiralStorage};
use mef_tic::TICCrystallizer;
use mef_ledger::MefLedger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let storage = SpiralStorage::new("./snapshots")?;
    let crystallizer = TICCrystallizer::new()?;
    let mut ledger = MefLedger::new("./ledger")?;

    // 1. Create and store snapshot
    let snapshot = SpiralSnapshot::from_bytes(
        b"Important data",
        "deterministic-seed-123",
        None,
    )?;

    storage.store(&snapshot).await?;
    println!("✓ Snapshot created: {}", snapshot.id);

    // 2. Process to TIC
    let tic = crystallizer.crystallize(&snapshot).await?;
    println!("✓ TIC crystallized: {}", tic.id);
    println!("  Converged: {}", tic.converged);
    println!("  Eigenvalue: {}", tic.eigenvalue);

    // 3. Append to ledger
    let block_index = ledger.append(tic).await?;
    println!("✓ Block appended: {}", block_index);

    // 4. Verify integrity
    let audit = ledger.verify_integrity().await?;
    println!("✓ Ledger integrity: {}", audit.valid);

    Ok(())
}
```

### Documentation

Generate Rust docs:
```bash
cargo doc --workspace --no-deps --open
```

---

## Python SDK (Planned)

### Installation (Future)

```bash
pip install spectralchain
```

### Example Usage (Future)

```python
from spectralchain import Pipeline, SpiralSnapshot, MefLedger

# Create pipeline
pipeline = Pipeline()

# Ingest data
snapshot = SpiralSnapshot.from_bytes(
    b"Hello, SpectralChain!",
    seed="my-seed"
)

# Process
tic = pipeline.process(snapshot)

# Append to ledger
ledger = MefLedger("./ledger")
block_index = ledger.append(tic)

print(f"Block: {block_index}")
```

**Status**: Planned for v2.1.0

---

## TypeScript SDK (Planned)

### Installation (Future)

```bash
npm install @spectralchain/sdk
```

### Example Usage (Future)

```typescript
import { Pipeline, SpiralSnapshot, MefLedger } from '@spectralchain/sdk';

// Create pipeline
const pipeline = new Pipeline();

// Ingest data
const snapshot = SpiralSnapshot.fromBytes(
  Buffer.from('Hello, SpectralChain!'),
  { seed: 'my-seed' }
);

// Process
const tic = await pipeline.process(snapshot);

// Append to ledger
const ledger = new MefLedger('./ledger');
const blockIndex = await ledger.append(tic);

console.log(`Block: ${blockIndex}`);
```

**Status**: Planned for v2.2.0

---

## Go SDK (Planned)

### Installation (Future)

```bash
go get github.com/LashSesh/spectralchain-go
```

### Example Usage (Future)

```go
package main

import (
    "fmt"
    "github.com/LashSesh/spectralchain-go"
)

func main() {
    // Create pipeline
    pipeline := spectralchain.NewPipeline()

    // Ingest data
    snapshot, _ := spectralchain.SpiralSnapshotFromBytes(
        []byte("Hello, SpectralChain!"),
        "my-seed",
    )

    // Process
    tic, _ := pipeline.Process(snapshot)

    // Append to ledger
    ledger, _ := spectralchain.NewMefLedger("./ledger")
    blockIndex, _ := ledger.Append(tic)

    fmt.Printf("Block: %d\n", blockIndex)
}
```

**Status**: Planned for v2.3.0

---

## SDK Comparison

| Feature | Rust | Python | TypeScript | Go |
|---------|------|--------|------------|-----|
| **Status** | ✅ Stable | 🔜 Planned | 🔜 Planned | 🔜 Planned |
| **Performance** | Excellent | Good | Good | Excellent |
| **Type Safety** | Strong | Dynamic | Strong | Strong |
| **Async/Await** | ✅ | ✅ | ✅ | ✅ |
| **Memory Safety** | ✅ | N/A | N/A | ✅ |
| **FFI Overhead** | None | Low | Low | None |
| **Learning Curve** | Steep | Easy | Medium | Medium |

---

## Examples

Full examples available in:
- [examples/ghost-voting-system/](../../examples/ghost-voting-system/)
- [examples/ephemeral-marketplace/](../../examples/ephemeral-marketplace/)
- [examples/privacy-messaging/](../../examples/privacy-messaging/)

---

## API Reference

Complete API reference:
- **Rust**: `cargo doc --workspace --no-deps --open`
- **Python**: (future) `pydoc spectralchain`
- **TypeScript**: (future) [docs.spectralchain.io/ts](https://docs.spectralchain.io/ts)
- **Go**: (future) [pkg.go.dev/spectralchain](https://pkg.go.dev/spectralchain)

---

## Related Documentation

- [API Reference](../api/README.md)
- [CLI User Guide](../cli/USER_GUIDE.md)
- [Quickstart Guides](../quickstart/API_QUICKSTART.md)
- [Architecture](../architecture/QUANTUM_RESONANT_ARCHITECTURE.md)

---

## Contributing

Want to add SDK support for another language?
See [Contributing Guide](../../CONTRIBUTING.md).

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
