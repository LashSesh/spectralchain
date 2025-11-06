# Frequently Asked Questions (FAQ)

**Version**: 2.0.0
**Last Updated**: 2025-11-06

---

## Table of Contents

1. [General Questions](#general-questions)
2. [Getting Started](#getting-started)
3. [API Questions](#api-questions)
4. [CLI Questions](#cli-questions)
5. [Architecture & Concepts](#architecture--concepts)
6. [Performance](#performance)
7. [Security & Privacy](#security--privacy)
8. [Troubleshooting](#troubleshooting)
9. [Development](#development)

---

## General Questions

### What is SpectralChain / Infinity Ledger?

SpectralChain (Infinity Ledger) is a **proof-carrying vector ledger engine** that combines:
- **Cryptographic Proof-of-Resonance**: Mathematical proofs for data integrity
- **Immutable Ledger**: Hash-chained blockchain with temporal provenance
- **Vector Search**: High-performance HNSW and IVF-PQ indexing
- **Ghost Network**: Addressless, privacy-preserving networking
- **Quantum Operators**: Masking, steganography, zero-knowledge proofs

It's not a traditional vector database—it's a cryptographically-verifiable, audit-ready ledger with quantum resonance processing.

### How is this different from a vector database?

| Feature | Traditional Vector DB | SpectralChain |
|---------|----------------------|---------------|
| **Proofs** | None | Proof-of-Resonance for every operation |
| **Audit Trail** | Optional | Immutable hash-chained ledger |
| **Privacy** | Limited | Ghost network, steganography, ZK proofs |
| **Determinism** | Not guaranteed | Fully deterministic with seeds |
| **Temporal Crystals** | No | TIC fixpoint convergence |
| **Fork Healing** | No | Mandorla attractor-based resolution |

### What are the main use cases?

1. **Privacy-Preserving Systems**: Voting, messaging, marketplaces
2. **Audit-Ready Ledgers**: Regulatory compliance, financial records
3. **Zero-Knowledge Proofs**: Private computation and verification
4. **Vector Search with Provenance**: AI embeddings with audit trails
5. **Anonymous Networking**: Ghost network for addressless communication

### Is this production-ready?

✅ **Core Components**: Yes (mef-core, mef-ledger, mef-spiral)
⚠️ **Quantum Extensions**: Beta (ghost-network, quantum-ops, ephemeral-services)
🚧 **GUI**: Planned

See [Implementation Status](./IMPLEMENTATION_STATUS.md) for details.

---

## Getting Started

### What are the prerequisites?

**Minimum**:
- Rust 1.70+ (from [rustup.rs](https://rustup.rs))
- 4GB RAM
- 10GB disk space

**Recommended**:
- Rust 1.75+
- 8GB+ RAM
- SSD storage

### How do I install SpectralChain?

```bash
# Clone repository
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/resources_dev/infinityledger

# Build all components
cargo build --release

# Install CLI
sudo cp target/release/mef /usr/local/bin/

# Start API server
cargo run --release --package mef-api
```

See [Quickstart Guides](./quickstart/API_QUICKSTART.md) for detailed instructions.

### Do I need an API server to use SpectralChain?

**No!** The CLI supports local processing mode:

```bash
# No API server required
mef ingest --local data.txt
mef process --local snap_123
```

The API server is optional for remote/multi-user deployments.

### What languages are supported?

**Native**:
- Rust (SDK fully available)

**Planned**:
- Python bindings
- TypeScript/Node.js SDK
- Go SDK

See [SDK Documentation](./sdk/README.md).

---

## API Questions

### Where is the API documentation?

- [OpenAPI Specification](./api/openapi.yaml) - Machine-readable spec
- [API Quick Reference](./api/QUICK_REFERENCE.md) - Endpoint cheat sheet
- [API User Guide](./api/USER_GUIDE.md) - Detailed documentation
- [API Quickstart](./quickstart/API_QUICKSTART.md) - 5-minute tutorial

### How do I authenticate with the API?

```bash
# Set Bearer token
curl -H "Authorization: Bearer <your_token>" \
  http://localhost:8000/ingest

# Or via environment variable
export MEF_API_TOKEN=your_token
```

For development, disable authentication:
```bash
export AUTH_TOKEN_REQUIRED=false
cargo run --release --package mef-api
```

### What is the default API port?

**Default**: `8000`

Override with:
```bash
export MEF_API_PORT=8080
cargo run --release --package mef-api
```

### Are there rate limits?

**Default limits**:
- 1000 requests per minute
- 100 requests per second (burst)

Configure in `extension.yaml`:
```yaml
rate_limiting:
  enabled: true
  per_minute: 1000
  burst: 100
```

### How do I get a snapshot ID?

The `/ingest` endpoint returns a snapshot ID:

```bash
SNAPSHOT_ID=$(curl -X POST http://localhost:8000/ingest \
  -H "Content-Type: application/json" \
  -d '{"data": "hello", "data_type": "text"}' \
  | jq -r '.snapshot_id')

echo $SNAPSHOT_ID
# Output: snap_1a2b3c4d5e6f
```

---

## CLI Questions

### How do I install the CLI?

```bash
cargo build --release --package mef-cli
sudo cp target/release/mef /usr/local/bin/
```

Verify:
```bash
mef --version
```

See [CLI Quickstart](./quickstart/CLI_QUICKSTART.md).

### Where is the CLI configuration file?

**Default location**: `~/.config/mef/config.yaml`

Override with:
```bash
export MEF_CONFIG=/path/to/config.yaml
```

Example config:
```yaml
api_url: http://localhost:8000
api_token: your_token
default_seed: MEF_SEED_42
auto_commit: true
```

### How do I enable shell completion?

```bash
# Bash
eval "$(mef completions bash)"

# Zsh
eval "$(mef completions zsh)"

# Fish
mef completions fish > ~/.config/fish/completions/mef.fish
```

### Can I use the CLI without the API?

**Yes!** Use `--local` flag:

```bash
mef ingest --local data.txt
mef process --local snap_123
mef audit --local
```

---

## Architecture & Concepts

### What is Proof-of-Resonance (PoR)?

Proof-of-Resonance is a cryptographic proof that verifies:
1. **Data Integrity**: Hash-based verification
2. **Topological Coherence**: Spiral geometry validation
3. **Temporal Consistency**: Resonance score calculation

Every snapshot includes a PoR for auditability.

### What is a Temporal Information Crystal (TIC)?

A **TIC** is a deterministic fixpoint representation created by:
1. **Solve-Coagula**: Quantum processing algorithm
2. **Eigenvalue Convergence**: Iterative refinement to fixpoint
3. **XSwap Operations**: Quantum operator transformations

TICs are immutable and deterministically reproducible.

### What is the Ghost Network?

The **Ghost Network** provides:
- **Addressless Networking**: No IP addresses, route by resonance
- **Identity Rotation**: Regular identity regeneration for privacy
- **Decoy Traffic**: Privacy-preserving noise generation
- **Capability-Based Discovery**: Find nodes by what they do, not who they are

See [Ghost Network Documentation](./features/GHOST_NETWORK.md).

### What are Quantum Operators?

Quantum Operators provide:
- **Masking**: Reversible data obfuscation
- **Steganography**: Hidden data embedding (zero-width, LSB)
- **Resonance Matching**: Topological similarity calculation
- **ZK Proofs**: Zero-knowledge proof generation

See [Quantum Operators Documentation](./features/QUANTUM_OPERATORS.md).

### What is the Metatron Router?

The **Metatron Router** uses:
- **Sacred Geometry**: Metatron Cube-based routing
- **Symmetry Groups**: E8, D12, Platonic topologies
- **Dynamic Route Selection**: Optimal path calculation
- **Transformation Pipeline**: Composable operator chains

See [Metatron Routing Documentation](./features/METATRON_ROUTING.md).

### What is Fork Healing?

**Fork Healing** resolves ledger forks using:
- **Mandorla Attractor**: Geometric conflict resolution
- **Resonance Field**: Priority scoring based on coherence
- **Multiversum**: Alternative timeline management

Ensures eventual consistency without centralized authority.

---

## Performance

### What is the expected throughput?

**Benchmarks** (on modern hardware):
- **Ingestion**: 10,000+ records/second
- **Vector Search**: 1,000+ queries/second
- **Ledger Append**: 5,000+ blocks/second
- **PoR Validation**: 50,000+ proofs/second

See [Performance Benchmarks](./reference/BENCHMARKS.md).

### How do I optimize performance?

Enable optimizations in `optimization.yaml`:

```yaml
optimizations:
  kosmokrator:
    enabled: true  # Stability filtering
  orphan_array:
    enabled: true  # Parallel sharding
  chronokrator:
    enabled: true  # Adaptive routing
  mandorla_logic:
    enabled: true  # Query refinement
```

See [Performance Tuning Guide](./guides/PERFORMANCE_TUNING.md).

### What is the storage overhead?

**Per block**:
- Block header: ~256 bytes
- TIC data: ~1-10 KB (depends on input)
- PoR: ~512 bytes

**Total overhead**: ~10-15% vs raw data

### Can I run SpectralChain on low-end hardware?

**Yes**, but with limitations:
- Minimum 2GB RAM (4GB recommended)
- Reduce `max_items` in memory backends
- Use `inmemory` backend instead of FAISS/HNSW
- Disable optimizations

---

## Security & Privacy

### Is data encrypted at rest?

**By default**: No, data stored in plaintext.

**Enable encryption**:
1. Use `masking` operator before ingestion
2. Use `steganography` for hidden data
3. Configure disk encryption (LUKS, BitLocker)

### How secure is the Ghost Network?

**Security features**:
- ✅ No IP addresses exposed
- ✅ Identity rotation every N minutes
- ✅ Decoy traffic generation
- ✅ Resonance-based routing (hard to trace)

**Not protected against**:
- ❌ Traffic analysis (use Tor/VPN wrapper)
- ❌ Sybil attacks (reputation system in development)

### Are zero-knowledge proofs SNARK or STARK?

Current implementation uses **simplified ZK proofs**.

**Future plans**:
- SNARK support (Groth16, PLONK)
- STARK support (FRI-based)

See [Roadmap](./ROADMAP.md).

### How is the API token stored?

**CLI**: In config file or environment variable (plaintext)

**Best practices**:
- Use environment variables (not config file)
- Rotate tokens regularly
- Use secret management tools (Vault, AWS Secrets Manager)

---

## Troubleshooting

### "Connection refused" error

**Causes**:
1. API server not running
2. Wrong port
3. Firewall blocking

**Solutions**:
```bash
# Check server running
ps aux | grep mef-api

# Check port
netstat -tuln | grep 8000

# Use local mode
mef ingest --local data.txt
```

### "Snapshot not found" error

**Causes**:
1. Invalid snapshot ID
2. Snapshot expired (ephemeral mode)
3. Storage backend issue

**Solutions**:
```bash
# Verify snapshot ID format (snap_xxxxxxxxxxxx)
# Check storage directory
ls $MEF_STORE_DIR/snapshots/

# Try re-ingesting
mef ingest data.txt
```

### "Ledger integrity check failed"

**Causes**:
1. Corrupted block
2. Storage corruption
3. Bug in hash calculation

**Solutions**:
```bash
# Export audit trail
mef audit --export > audit.json

# Find first invalid block
cat audit.json | jq '.first_invalid_block'

# Report bug with audit trail
```

See [Troubleshooting Guide](./TROUBLESHOOTING.md) for more.

---

## Development

### How do I contribute?

1. Fork repository
2. Create feature branch
3. Make changes + add tests
4. Run test suite: `cargo test --workspace`
5. Format code: `cargo fmt --all`
6. Run clippy: `cargo clippy --all-targets`
7. Open pull request

See [Contributing Guide](../CONTRIBUTING.md).

### How do I run tests?

```bash
# All tests
cargo test --workspace

# Specific module
cargo test --package mef-ledger

# Integration tests
cargo test --workspace --test '*'

# E2E tests
cd e2e-testing
cargo test
```

### How do I add a new API endpoint?

1. Define route in `mef-api/src/routes/`
2. Add handler function
3. Add OpenAPI spec in `docs/api/openapi.yaml`
4. Add tests in route file
5. Update [API User Guide](./api/USER_GUIDE.md)

Example:
```rust
// mef-api/src/routes/my_feature.rs
pub async fn my_endpoint(
    State(state): State<AppState>,
    Json(req): Json<MyRequest>,
) -> Result<Json<MyResponse>, ApiError> {
    // Implementation
}
```

### How do I add a new CLI command?

1. Add command in `mef-cli/src/main.rs`
2. Implement handler
3. Add to completion script
4. Update [CLI User Guide](./cli/USER_GUIDE.md)

Example:
```rust
// mef-cli/src/main.rs
#[derive(Subcommand)]
enum Commands {
    MyCommand {
        #[arg(long)]
        option: String,
    },
}
```

### Where are the logs?

**Default location**: `$MEF_LOGS_DIR` or `./logs/`

**Configure**:
```bash
export MEF_LOGS_DIR=/var/log/mef
export RUST_LOG=debug
```

**View logs**:
```bash
tail -f logs/mef-api.log
```

---

## Still Have Questions?

- 📖 [Complete Documentation](./INDEX.md)
- 💬 [GitHub Discussions](https://github.com/LashSesh/spectralchain/discussions)
- 🐛 [Report Bug](https://github.com/LashSesh/spectralchain/issues/new?template=bug_report.md)
- 💡 [Request Feature](https://github.com/LashSesh/spectralchain/issues/new?template=feature_request.md)

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
