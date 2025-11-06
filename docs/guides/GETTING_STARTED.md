# Getting Started with SpectralChain

**⏱️ Time**: 15 minutes | **Level**: Beginner

---

## Welcome to SpectralChain!

This guide will help you get started with SpectralChain / Infinity Ledger, whether you're an absolute beginner or an experienced developer.

---

## Table of Contents

1. [What is SpectralChain?](#what-is-spectralchain)
2. [Installation](#installation)
3. [Your First Operation](#your-first-operation)
4. [Understanding the Workflow](#understanding-the-workflow)
5. [Next Steps](#next-steps)

---

## What is SpectralChain?

SpectralChain is a **proof-carrying vector ledger engine** that provides:

- 🔐 **Cryptographic Proofs**: Every operation has mathematical proof of integrity
- 📊 **Immutable Ledger**: Blockchain-style audit trail that can't be tampered with
- 🔍 **Vector Search**: Fast similarity search for AI embeddings and data
- 👻 **Privacy Features**: Ghost network for anonymous communication
- 🔒 **Zero-Knowledge Proofs**: Prove things without revealing secrets

### Think of it as...

- A **vector database** + blockchain + privacy layer
- A **ledger** that can search and prove data integrity
- A **privacy-preserving** system for sensitive applications

---

## Installation

### Option 1: Quick Install (Recommended)

```bash
# Clone the repository
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/resources_dev/infinityledger

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build everything
cargo build --release

# Install CLI
sudo cp target/release/mef /usr/local/bin/

# Verify installation
mef --version
```

**Expected output**: `mef 2.0.0`

### Option 2: Docker (Coming Soon)

```bash
docker pull spectralchain/infinityledger:latest
docker run -p 8000:8000 spectralchain/infinityledger
```

---

## Your First Operation

Let's ingest data, process it, and verify the ledger!

### Step 1: Start the API Server

```bash
# In one terminal
cd spectralchain/resources_dev/infinityledger
cargo run --release --package mef-api
```

You should see:
```
🚀 SpectralChain API server starting...
✓ Server listening on http://0.0.0.0:8000
```

### Step 2: Create Sample Data

```bash
# In another terminal
echo "Hello from SpectralChain!" > my_first_data.txt
```

### Step 3: Ingest Data

```bash
mef ingest my_first_data.txt
```

**Output**:
```json
{
  "snapshot_id": "snap_a1b2c3d4e5f6",
  "phase": "ingested",
  "por": {
    "hash": "sha256:abc123...",
    "signature": "sig_xyz..."
  },
  "timestamp": "2025-11-06T12:34:56Z"
}
```

💾 **Save your snapshot_id** - you'll need it next!

### Step 4: Process Snapshot

```bash
# Replace with your actual snapshot_id
mef process --commit snap_a1b2c3d4e5f6
```

**Output**:
```json
{
  "tic_id": "tic_9z8y7x6w5v4u",
  "converged": true,
  "iterations": 42,
  "final_eigenvalue": 0.999987,
  "committed": true
}
```

🎉 **Congratulations!** You've created your first Temporal Information Crystal (TIC) and committed it to the ledger!

### Step 5: Verify Ledger

```bash
mef audit
```

**Output**:
```json
{
  "valid": true,
  "total_blocks": 1,
  "checked_blocks": 1,
  "first_invalid_block": null
}
```

✅ **Your ledger is valid!** The hash chain is intact.

---

## Understanding the Workflow

### What Just Happened?

1. **Ingest** (`mef ingest`):
   - Took your data
   - Created a deterministic spiral snapshot
   - Generated Proof-of-Resonance (PoR)
   - Stored snapshot with unique ID

2. **Process** (`mef process`):
   - Loaded the snapshot
   - Ran Solve-Coagula quantum processing
   - Found the eigenvalue fixpoint
   - Crystallized a TIC (Temporal Information Crystal)
   - Committed TIC to the ledger

3. **Audit** (`mef audit`):
   - Verified the entire hash chain
   - Checked every block's integrity
   - Confirmed no tampering

### The Data Flow

```
Input Data
    ↓
[Spiral Snapshot] ← Proof-of-Resonance
    ↓
[Solve-Coagula Processing] ← Quantum Operators
    ↓
[TIC Crystal] ← Eigenvalue Convergence
    ↓
[Immutable Ledger] ← Hash-Chained Blocks
    ↓
[Audit Trail] ← Verification
```

---

## Key Concepts Explained Simply

### 🌀 Spiral Snapshot
Think of it as a "fingerprint" of your data. Same input + same seed = same snapshot (deterministic).

### 🔮 TIC (Temporal Information Crystal)
A processed, converged representation of your snapshot. Like a "final form" after quantum processing.

### 📖 Ledger
A blockchain-style chain of blocks. Each block contains a TIC and is linked to the previous block via hash.

### ✅ Proof-of-Resonance (PoR)
A cryptographic proof that says "this snapshot is valid and hasn't been tampered with."

### 🧮 Eigenvalue Convergence
The processing algorithm finds a stable "fixpoint" - like finding the solution to an equation.

---

## Common Tasks

### Deterministic Processing (Same Output Every Time)

```bash
# Using a seed ensures reproducibility
mef ingest --seed my-seed-123 data.txt

# Running again with same seed produces identical snapshot_id
mef ingest --seed my-seed-123 data.txt
```

### Processing Without API Server

```bash
# Use --local flag to process without API
mef ingest --local data.txt
mef process --local snap_123
```

### JSON Data

```bash
# Create JSON file
cat > data.json <<EOF
{
  "message": "Hello, SpectralChain!",
  "timestamp": "2025-11-06T12:00:00Z"
}
EOF

# Ingest JSON
mef ingest --type json data.json
```

### Batch Processing

```bash
# Process multiple files
for file in data/*.txt; do
  echo "Processing $file..."
  SNAPSHOT=$(mef ingest "$file" | jq -r '.snapshot_id')
  mef process --commit "$SNAPSHOT"
done

# Verify ledger
mef audit
```

---

## Next Steps

### 🚀 Explore More Features

1. **Try the API directly**:
   - [API Quickstart](../quickstart/API_QUICKSTART.md)
   - [OpenAPI Specification](../api/openapi.yaml)

2. **Learn about Privacy Features**:
   - [Ghost Network](../features/GHOST_NETWORK.md)
   - [Quantum Operators](../features/QUANTUM_OPERATORS.md)
   - [Zero-Knowledge Proofs](../features/ZK_PROOFS.md)

3. **Build an Application**:
   - [Ghost Voting System Tutorial](../tutorials/GHOST_VOTING.md)
   - [Ephemeral Marketplace Tutorial](../tutorials/EPHEMERAL_MARKETPLACE.md)

### 📚 Dive Deeper

- [CLI User Guide](../cli/USER_GUIDE.md) - Complete command reference
- [Architecture Overview](../architecture/QUANTUM_RESONANT_ARCHITECTURE.md) - How it all works
- [FAQ](../FAQ.md) - Common questions answered

### 💡 Real-World Examples

Check out complete examples:
- [examples/ghost-voting-system/](../../examples/ghost-voting-system/)
- [examples/ephemeral-marketplace/](../../examples/ephemeral-marketplace/)
- [examples/privacy-messaging/](../../examples/privacy-messaging/)

---

## Troubleshooting

### "Command not found: mef"

```bash
# Add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Or reinstall
sudo cp target/release/mef /usr/local/bin/
```

### "Connection refused"

```bash
# Make sure API server is running
ps aux | grep mef-api

# Or use local mode
mef ingest --local data.txt
```

### Need Help?

- 📖 [Troubleshooting Guide](../TROUBLESHOOTING.md)
- 💬 [Ask on GitHub Discussions](https://github.com/LashSesh/spectralchain/discussions)
- 🐛 [Report an Issue](https://github.com/LashSesh/spectralchain/issues)

---

## Summary

You now know how to:
- ✅ Install SpectralChain
- ✅ Ingest data and create snapshots
- ✅ Process snapshots to TICs
- ✅ Commit to the immutable ledger
- ✅ Verify ledger integrity

**Ready to build something amazing?**

Check out the [Tutorials](../tutorials/) or dive into the [Complete Documentation](../INDEX.md).

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
