# MEF CLI User Guide

**Version**: 2.0.0
**Last Updated**: 2025-11-06

---

## Overview

The `mef` command-line interface provides a powerful, user-friendly interface to interact with SpectralChain / Infinity Ledger. This guide covers all commands, options, and usage patterns for both beginners and expert users.

---

## Table of Contents

1. [Installation](#installation)
2. [Global Configuration](#global-configuration)
3. [Quick Reference](#quick-reference)
4. [Commands](#commands)
5. [Examples](#examples)
6. [Shell Completion](#shell-completion)
7. [Troubleshooting](#troubleshooting)

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/resources_dev/infinityledger

# Build the CLI
cargo build --release --package mef-cli

# Install to PATH
sudo cp target/release/mef /usr/local/bin/
```

### Verify Installation

```bash
mef --version
# Output: mef 2.0.0
```

---

## Global Configuration

### Configuration File

Create `~/.config/mef/config.yaml`:

```yaml
# API Server Configuration
api_url: http://localhost:8000
api_token: your_token_here

# Default Settings
default_seed: MEF_SEED_42
output_format: json  # json | yaml | table

# Processing Options
auto_commit: false
local_mode: false

# Logging
log_level: info  # trace | debug | info | warn | error
```

### Environment Variables

Override configuration with environment variables:

```bash
export MEF_CONFIG=~/.config/mef/config.yaml
export MEF_API_URL=http://localhost:8000
export MEF_API_TOKEN=your_token_here
export MEF_SEED=MEF_SEED_42
```

### Precedence Order

Configuration is resolved in this order (highest to lowest):
1. Command-line flags
2. Environment variables
3. Configuration file
4. Built-in defaults

---

## Quick Reference

### Most Common Commands

| Command | Description | Example |
|---------|-------------|---------|
| `mef ingest <file>` | Ingest file into MEF | `mef ingest data.txt` |
| `mef process <id>` | Process snapshot to TIC | `mef process snap_123` |
| `mef audit` | Audit ledger integrity | `mef audit` |
| `mef validate <id>` | Validate snapshot | `mef validate snap_123` |
| `mef ping` | Check API connectivity | `mef ping` |

### Command Structure

```
mef [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS] [ARGS]
```

Example:
```bash
mef --config prod.yaml ingest --type json --seed myseed data.json
```

---

## Commands

### `mef ingest`

Ingest data into the MEF-Core system, creating a deterministic spiral snapshot.

#### Usage

```bash
mef ingest [OPTIONS] <FILE_PATH>
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--type <TYPE>` | `-t` | Data type: text, json, numeric, binary, raw | Detected from file |
| `--seed <SEED>` | `-s` | Deterministic seed | `MEF_SEED` env var |
| `--local` | | Use local processing (no API) | false |

#### Examples

```bash
# Ingest text file
mef ingest document.txt

# Ingest JSON with custom seed
mef ingest --type json --seed my-seed-123 data.json

# Ingest using local processing
mef ingest --local large_file.bin

# Ingest with explicit type
mef ingest --type raw binary_data.dat
```

#### Output

```json
{
  "snapshot_id": "snap_1a2b3c4d5e6f",
  "phase": "ingested",
  "por": {
    "hash": "sha256:abc123...",
    "signature": "sig_xyz789..."
  },
  "timestamp": "2025-11-06T12:34:56Z"
}
```

---

### `mef process`

Process a spiral snapshot through Solve-Coagula to create a Temporal Information Crystal (TIC).

#### Usage

```bash
mef process [OPTIONS] <SNAPSHOT_ID>
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--commit` | | Auto-commit TIC to ledger | false |
| `--local` | | Use local processing | false |

#### Examples

```bash
# Process snapshot
mef process snap_1a2b3c4d5e6f

# Process and commit to ledger
mef process --commit snap_1a2b3c4d5e6f

# Local processing
mef process --local snap_1a2b3c4d5e6f
```

#### Output

```json
{
  "tic_id": "tic_9z8y7x6w5v4u",
  "converged": true,
  "iterations": 42,
  "final_eigenvalue": 0.999987,
  "committed": true
}
```

---

### `mef audit`

Audit the integrity of the hash-chained ledger.

#### Usage

```bash
mef audit [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--start <INDEX>` | `-s` | Start block index | 0 |
| `--export` | `-e` | Export full audit trail | false |
| `--local` | | Use local processing | false |

#### Examples

```bash
# Audit entire ledger
mef audit

# Audit from block 100
mef audit --start 100

# Export audit trail
mef audit --export > audit_trail.json
```

#### Output

```json
{
  "valid": true,
  "total_blocks": 1024,
  "checked_blocks": 1024,
  "first_invalid_block": null,
  "audit_time_ms": 234
}
```

---

### `mef validate`

Validate a snapshot using Proof-of-Resonance.

#### Usage

```bash
mef validate [OPTIONS] <SNAPSHOT_ID>
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--local` | | Use local processing | false |

#### Examples

```bash
# Validate snapshot
mef validate snap_1a2b3c4d5e6f
```

#### Output

```json
{
  "valid": true,
  "resonance_score": 0.987,
  "proof": {
    "hash": "sha256:def456...",
    "signature": "sig_uvw012..."
  }
}
```

---

### `mef export`

Export system data in various formats.

#### Usage

```bash
mef export [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--format <FORMAT>` | `-f` | Export format: json, audit | json |
| `--output <PATH>` | `-o` | Output file path | stdout |

#### Examples

```bash
# Export as JSON
mef export --format json --output system_state.json

# Export audit trail
mef export --format audit --output audit.json
```

---

### `mef embed`

Create spiral embedding (SPEC-002).

#### Usage

```bash
mef embed [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--in <FILE>` | | Input file | required |
| `--seed <SEED>` | `-s` | Deterministic seed | `MEF_SEED` |
| `--out <PATH>` | | Output path | stdout |

#### Examples

```bash
# Create embedding
mef embed --in data.txt --seed my-seed --out embedding.json
```

---

### `mef solve`

Calculate fixpoint (SPEC-002).

#### Usage

```bash
mef solve [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--snapshot <ID>` | | Snapshot ID | required |
| `--out <FILE>` | | Output TIC file | required |

#### Examples

```bash
# Solve fixpoint
mef solve --snapshot snap_123 --out tic.json
```

---

### `mef ledger append`

Append a block to the ledger.

#### Usage

```bash
mef ledger append --tic <TIC_ID> --snapshot <SNAPSHOT_ID>
```

#### Examples

```bash
mef ledger append --tic tic_123 --snapshot snap_456
```

---

### `mef ledger verify`

Verify ledger integrity (alias for `mef audit`).

#### Usage

```bash
mef ledger verify
```

---

### `mef ping`

Ping the API server to check connectivity.

#### Usage

```bash
mef ping [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--api-url <URL>` | | Override API URL | config value |

#### Examples

```bash
# Ping default server
mef ping

# Ping specific server
mef ping --api-url https://api.spectralchain.io
```

#### Output

```json
{
  "status": "ok",
  "version": "2.0.0",
  "seed": "MEF_SEED_42",
  "latency_ms": 23
}
```

---

## Examples

### Complete Workflow: Ingest → Process → Audit

```bash
# 1. Ingest data
SNAPSHOT_ID=$(mef ingest --type json data.json | jq -r '.snapshot_id')
echo "Snapshot ID: $SNAPSHOT_ID"

# 2. Process snapshot to TIC
TIC_ID=$(mef process --commit "$SNAPSHOT_ID" | jq -r '.tic_id')
echo "TIC ID: $TIC_ID"

# 3. Audit ledger
mef audit

# 4. Validate snapshot
mef validate "$SNAPSHOT_ID"
```

### Batch Processing

```bash
#!/bin/bash
# Process multiple files

for file in data/*.json; do
  echo "Processing $file..."

  # Ingest
  snapshot=$(mef ingest --type json "$file" | jq -r '.snapshot_id')

  # Process
  tic=$(mef process --commit "$snapshot" | jq -r '.tic_id')

  echo "  Snapshot: $snapshot"
  echo "  TIC: $tic"
done

# Final audit
echo "Auditing ledger..."
mef audit
```

### Using Custom Configuration

```bash
# Create production config
cat > prod-config.yaml <<EOF
api_url: https://api.spectralchain.io
api_token: \$PROD_API_TOKEN
default_seed: PROD_SEED_2025
auto_commit: true
log_level: warn
EOF

# Use production config
export MEF_CONFIG=prod-config.yaml
export PROD_API_TOKEN=your_production_token

mef ingest production_data.json
```

### Local Processing (No API)

```bash
# Process locally without API server
mef ingest --local data.txt
mef process --local snap_123
mef audit --local
```

---

## Shell Completion

### Bash

```bash
# Generate completion script
mef completions bash > /etc/bash_completion.d/mef

# Or add to ~/.bashrc
eval "$(mef completions bash)"
```

### Zsh

```bash
# Add to ~/.zshrc
eval "$(mef completions zsh)"
```

### Fish

```bash
# Generate completion
mef completions fish > ~/.config/fish/completions/mef.fish
```

### PowerShell

```powershell
# Add to profile
mef completions powershell | Out-String | Invoke-Expression
```

---

## Troubleshooting

### Command Not Found

```bash
# Check installation
which mef

# Add to PATH
export PATH="$PATH:/path/to/mef"
```

### Connection Refused

```bash
# Check API server is running
curl http://localhost:8000/ping

# Override API URL
mef --api-url http://localhost:8001 ping

# Use local processing
mef ingest --local data.txt
```

### Authentication Errors

```bash
# Set API token
export MEF_API_TOKEN=your_token_here

# Or in config file
echo "api_token: your_token_here" >> ~/.config/mef/config.yaml
```

### Snapshot Not Found

```bash
# List snapshots (future command)
mef list snapshots

# Validate snapshot ID format
# Correct: snap_1a2b3c4d5e6f
# Incorrect: snapshot-123
```

### JSON Parsing Errors

```bash
# Use jq for JSON output
mef ingest data.json | jq '.'

# Use --format for human-readable output
mef ingest data.json --format table
```

---

## Advanced Usage

### Scripting

```bash
#!/bin/bash
set -euo pipefail

# Error handling
trap 'echo "Error on line $LINENO"' ERR

# Function to ingest and process
ingest_and_process() {
  local file=$1
  local snapshot tic

  snapshot=$(mef ingest "$file" | jq -r '.snapshot_id')
  tic=$(mef process --commit "$snapshot" | jq -r '.tic_id')

  echo "$file -> $snapshot -> $tic"
}

# Process all files
for file in *.json; do
  ingest_and_process "$file"
done
```

### CI/CD Integration

```yaml
# .github/workflows/mef.yml
name: MEF Processing
on: [push]

jobs:
  process:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install MEF CLI
        run: |
          wget https://github.com/LashSesh/spectralchain/releases/download/v2.0.0/mef-linux-amd64
          chmod +x mef-linux-amd64
          sudo mv mef-linux-amd64 /usr/local/bin/mef

      - name: Process data
        env:
          MEF_API_URL: ${{ secrets.MEF_API_URL }}
          MEF_API_TOKEN: ${{ secrets.MEF_API_TOKEN }}
        run: |
          mef ingest data.json
          mef audit
```

---

## Related Documentation

- [API Reference](../api/README.md) - REST API documentation
- [SDK Documentation](../sdk/README.md) - Programmatic access
- [Configuration Guide](../configuration/README.md) - Detailed configuration
- [Troubleshooting](../TROUBLESHOOTING.md) - Common issues

---

## Getting Help

```bash
# Command help
mef --help
mef ingest --help
mef process --help

# Version information
mef --version

# Debug mode
mef --log-level debug ingest data.txt
```

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
