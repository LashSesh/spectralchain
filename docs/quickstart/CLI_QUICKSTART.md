# MEF CLI Quickstart

**⏱️ Time**: 5 minutes | **Level**: Beginner

---

## Goal

Get started with the MEF command-line interface in 5 minutes.

---

## Step 1: Install CLI

```bash
# Clone repository
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/resources_dev/infinityledger

# Build CLI
cargo build --release --package mef-cli

# Install to PATH
sudo cp target/release/mef /usr/local/bin/
```

Verify installation:
```bash
mef --version
# Output: mef 2.0.0
```

✅ CLI installed!

---

## Step 2: Start API Server (if not running)

```bash
# In another terminal
cargo run --release --package mef-api
```

Or configure CLI for local processing:
```bash
# Use --local flag to skip API
mef ingest --local data.txt
```

---

## Step 3: Ping Server

```bash
mef ping
```

**Output**:
```json
{
  "status": "ok",
  "version": "2.0.0",
  "seed": "MEF_SEED_42"
}
```

✅ Connected!

---

## Step 4: Ingest Data

Create a sample file:
```bash
echo "Hello, SpectralChain!" > hello.txt
```

Ingest it:
```bash
mef ingest hello.txt
```

**Output**:
```json
{
  "snapshot_id": "snap_1a2b3c4d5e6f",
  "phase": "ingested",
  "por": {
    "hash": "sha256:abc123...",
    "signature": "sig_xyz..."
  }
}
```

💾 Note your `snapshot_id`!

---

## Step 5: Process Snapshot

```bash
mef process --commit snap_1a2b3c4d5e6f
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

🎉 TIC created and committed!

---

## Step 6: Audit Ledger

```bash
mef audit
```

**Output**:
```json
{
  "valid": true,
  "total_blocks": 1,
  "checked_blocks": 1
}
```

✅ Ledger verified!

---

## Complete Workflow Script

```bash
#!/bin/bash

# Create test data
echo "SpectralChain CLI Quickstart" > test_data.txt

# Ingest
SNAPSHOT=$(mef ingest test_data.txt | jq -r '.snapshot_id')
echo "Snapshot: $SNAPSHOT"

# Process
TIC=$(mef process --commit "$SNAPSHOT" | jq -r '.tic_id')
echo "TIC: $TIC"

# Audit
mef audit

echo "✅ Complete!"
```

---

## With Custom Seed (Deterministic)

```bash
# Same input + same seed = same snapshot ID
mef ingest --seed my-seed-123 data.txt

# Run again
mef ingest --seed my-seed-123 data.txt

# Both produce identical snapshot_id!
```

---

## JSON Data

```bash
# Create JSON file
cat > data.json <<EOF
{
  "name": "SpectralChain",
  "type": "quantum-ledger",
  "features": ["proof-of-resonance", "ghost-network", "zk-proofs"]
}
EOF

# Ingest JSON
mef ingest --type json data.json
```

---

## Next Steps

### 🚀 Try More Commands

```bash
# Validate snapshot
mef validate snap_1a2b3c4d5e6f

# Export data
mef export --format json --output system.json

# Create embedding
mef embed --in data.txt --out embedding.json
```

### 📚 Learn More

- [Complete CLI Guide](../cli/USER_GUIDE.md)
- [Configuration Options](../cli/CONFIGURATION.md)
- [Shell Completion](../cli/SHELL_COMPLETION.md)

### 🛠️ Try Other Interfaces

- [API Quickstart](API_QUICKSTART.md)
- [Rust SDK Quickstart](RUST_SDK_QUICKSTART.md)

---

## Configuration (Optional)

```bash
# Create config file
mkdir -p ~/.config/mef
cat > ~/.config/mef/config.yaml <<EOF
api_url: http://localhost:8000
default_seed: MEF_SEED_42
auto_commit: true
output_format: json
EOF

# Set config path
export MEF_CONFIG=~/.config/mef/config.yaml
```

---

## Shell Completion (Optional)

```bash
# Bash
eval "$(mef completions bash)"

# Zsh
eval "$(mef completions zsh)"

# Fish
mef completions fish > ~/.config/fish/completions/mef.fish
```

Now tab completion works:
```bash
mef <TAB>
# Shows: audit  embed  export  ingest  ping  process  solve  validate
```

---

## Troubleshooting

### mef: command not found
```bash
# Add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Or copy to bin
sudo cp target/release/mef /usr/local/bin/
```

### Connection refused
```bash
# Check API server
curl http://localhost:8000/ping

# Or use local mode
mef ingest --local data.txt
```

---

**⏱️ Done in 5 minutes!**

Questions? Check [FAQ](../FAQ.md) or [CLI User Guide](../cli/USER_GUIDE.md).
