# SpectralChain Troubleshooting Guide

**Version**: 2.0.0
**Last Updated**: 2025-11-06

---

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [API Server Issues](#api-server-issues)
3. [CLI Issues](#cli-issues)
4. [Authentication & Authorization](#authentication--authorization)
5. [Data Processing Issues](#data-processing-issues)
6. [Ledger Issues](#ledger-issues)
7. [Performance Issues](#performance-issues)
8. [Network Issues](#network-issues)
9. [Storage Issues](#storage-issues)
10. [Common Error Codes](#common-error-codes)

---

## Installation Issues

### Rust Compilation Errors

#### Problem: "error: linking with `cc` failed"

**Symptoms**:
```
error: linking with `cc` failed: exit code: 1
```

**Solutions**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# macOS
xcode-select --install
brew install openssl

# Fedora/RHEL
sudo dnf install gcc gcc-c++ openssl-devel
```

#### Problem: "could not find `Cargo.toml`"

**Symptoms**:
```
error: could not find `Cargo.toml` in `/path/to/dir` or any parent directory
```

**Solution**:
```bash
# Ensure you're in correct directory
cd spectralchain/resources_dev/infinityledger

# Verify Cargo.toml exists
ls -la Cargo.toml
```

#### Problem: Out of memory during compilation

**Symptoms**:
```
error: could not compile `mef-core` due to 1 previous error
SIGKILL
```

**Solutions**:
```bash
# Reduce parallel jobs
cargo build --release -j 2

# Or use incremental compilation
export CARGO_INCREMENTAL=1
cargo build --release
```

### Dependency Issues

#### Problem: "failed to resolve: use of undeclared crate"

**Solution**:
```bash
# Update dependencies
cargo update

# Clean build cache
cargo clean

# Rebuild
cargo build --release
```

---

## API Server Issues

### Server Won't Start

#### Problem: "Address already in use"

**Symptoms**:
```
Error: Address already in use (os error 98)
```

**Solutions**:
```bash
# Find process using port 8000
lsof -i :8000
# or
netstat -tuln | grep 8000

# Kill existing process
kill -9 <PID>

# Or use different port
export MEF_API_PORT=8001
cargo run --release --package mef-api
```

#### Problem: "Permission denied (bind)"

**Symptoms**:
```
Error: Permission denied (os error 13)
```

**Solutions**:
```bash
# Use port > 1024 (no sudo needed)
export MEF_API_PORT=8000

# Or grant capability (Linux)
sudo setcap CAP_NET_BIND_SERVICE=+eip target/release/mef-api

# Or run with sudo (not recommended)
sudo cargo run --release --package mef-api
```

### Server Crashes

#### Problem: Server crashes with "Segmentation fault"

**Solutions**:
```bash
# Enable debug symbols
cargo build --profile=debug-release --package mef-api

# Run with backtrace
RUST_BACKTRACE=full cargo run --package mef-api

# Check logs
tail -f logs/mef-api.log
```

#### Problem: Server hangs or freezes

**Diagnostics**:
```bash
# Check CPU/memory
top -p $(pgrep mef-api)

# Check open files
lsof -p $(pgrep mef-api)

# Send SIGUSR1 for thread dump
kill -SIGUSR1 $(pgrep mef-api)
```

**Solutions**:
```bash
# Restart server
pkill mef-api
cargo run --release --package mef-api

# Enable request timeouts
export MEF_REQUEST_TIMEOUT=30
```

### Connection Issues

#### Problem: "Connection refused"

**Diagnostics**:
```bash
# Check server is running
ps aux | grep mef-api

# Test connectivity
curl http://localhost:8000/ping

# Check firewall
sudo ufw status
sudo iptables -L
```

**Solutions**:
```bash
# Start server
cargo run --release --package mef-api

# Disable firewall (temporarily)
sudo ufw allow 8000

# Bind to all interfaces
export MEF_API_HOST=0.0.0.0
```

---

## CLI Issues

### CLI Not Found

#### Problem: "mef: command not found"

**Solutions**:
```bash
# Check if binary exists
ls -la target/release/mef

# Add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Or copy to bin
sudo cp target/release/mef /usr/local/bin/

# Verify
which mef
mef --version
```

### Configuration Issues

#### Problem: "Config file not found"

**Symptoms**:
```
Error: No such file or directory (os error 2)
```

**Solutions**:
```bash
# Create config directory
mkdir -p ~/.config/mef

# Create default config
cat > ~/.config/mef/config.yaml <<EOF
api_url: http://localhost:8000
default_seed: MEF_SEED_42
EOF

# Verify
mef ping
```

#### Problem: "Invalid configuration"

**Diagnostics**:
```bash
# Validate YAML syntax
cat ~/.config/mef/config.yaml | yq eval

# Check for typos
mef --help
```

### CLI Hangs

#### Problem: CLI command hangs indefinitely

**Diagnostics**:
```bash
# Enable debug output
mef --log-level debug ingest data.txt

# Check API server
curl -v http://localhost:8000/ping

# Check network
netstat -an | grep 8000
```

**Solutions**:
```bash
# Use --local mode
mef ingest --local data.txt

# Increase timeout
export MEF_TIMEOUT=60

# Kill hanging process
pkill mef
```

---

## Authentication & Authorization

### Invalid Token

#### Problem: "Unauthorized" (401)

**Symptoms**:
```json
{
  "error": "UNAUTHORIZED",
  "message": "Missing or invalid authorization token"
}
```

**Solutions**:
```bash
# Set API token
export MEF_API_TOKEN=your_token_here

# Or in config
echo "api_token: your_token" >> ~/.config/mef/config.yaml

# Disable auth for development
export AUTH_TOKEN_REQUIRED=false
```

### Token Expired

#### Problem: "Token expired"

**Solutions**:
```bash
# Request new token
curl -X POST http://localhost:8000/auth/token \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "secret"}'

# Update config
export MEF_API_TOKEN=new_token
```

---

## Data Processing Issues

### Snapshot Not Found

#### Problem: "Snapshot not found" (404)

**Symptoms**:
```json
{
  "error": "NOT_FOUND",
  "message": "Snapshot not found: snap_123"
}
```

**Diagnostics**:
```bash
# Verify snapshot ID format
# Correct: snap_1a2b3c4d5e6f
# Incorrect: snapshot-123

# Check storage
ls $MEF_STORE_DIR/snapshots/

# List snapshots (future command)
mef list snapshots
```

**Solutions**:
```bash
# Re-ingest data
mef ingest data.txt

# Save snapshot ID
SNAPSHOT=$(mef ingest data.txt | jq -r '.snapshot_id')
echo $SNAPSHOT > snapshot_id.txt
```

### Processing Fails

#### Problem: "TIC convergence failed"

**Symptoms**:
```json
{
  "converged": false,
  "iterations": 1000,
  "final_eigenvalue": 0.95
}
```

**Solutions**:
```bash
# Increase max iterations
export MEF_MAX_ITERATIONS=10000

# Try different seed
mef ingest --seed alternative-seed data.txt

# Check input data quality
```

#### Problem: "Invalid data type"

**Symptoms**:
```json
{
  "error": "BAD_REQUEST",
  "message": "Invalid data_type: foo"
}
```

**Solutions**:
```bash
# Use valid data types: text, json, numeric, binary, raw
mef ingest --type json data.json

# Auto-detect (omit --type)
mef ingest data.txt
```

---

## Ledger Issues

### Integrity Check Fails

#### Problem: "Ledger integrity check failed"

**Symptoms**:
```json
{
  "valid": false,
  "first_invalid_block": 42
}
```

**Diagnostics**:
```bash
# Export audit trail
mef audit --export > audit.json

# Check specific block
cat audit.json | jq '.blocks[42]'

# Check storage
ls -la $MEF_LEDGER_DIR/
```

**Solutions**:
```bash
# Backup ledger
cp -r $MEF_LEDGER_DIR $MEF_LEDGER_DIR.backup

# Verify hash chain manually
# (Report bug if issue persists)

# Restore from backup if corrupted
rm -rf $MEF_LEDGER_DIR
cp -r $MEF_LEDGER_DIR.backup $MEF_LEDGER_DIR
```

### Block Append Fails

#### Problem: "Failed to append block"

**Symptoms**:
```
Error: Failed to append block: I/O error
```

**Diagnostics**:
```bash
# Check disk space
df -h $MEF_LEDGER_DIR

# Check permissions
ls -la $MEF_LEDGER_DIR

# Check inode usage
df -i $MEF_LEDGER_DIR
```

**Solutions**:
```bash
# Free disk space
rm -rf /tmp/*

# Fix permissions
chmod 755 $MEF_LEDGER_DIR

# Move to larger disk
export MEF_LEDGER_DIR=/mnt/large_disk/ledger
```

---

## Performance Issues

### Slow Ingestion

#### Problem: Ingestion takes too long

**Diagnostics**:
```bash
# Profile ingestion
time mef ingest large_file.bin

# Check system resources
top
iostat -x 1
```

**Solutions**:
```bash
# Enable optimizations
cat > config/optimization.yaml <<EOF
optimizations:
  kosmokrator: {enabled: true}
  orphan_array: {enabled: true}
  chronokrator: {enabled: true}
  mandorla_logic: {enabled: true}
EOF

# Use batch processing
for file in data/*.json; do
  mef ingest "$file" &
done
wait

# Increase batch size
export MEF_BATCH_SIZE=10000
```

### Slow Queries

#### Problem: Vector search is slow

**Diagnostics**:
```bash
# Check index status
curl http://localhost:8000/index/status

# Check collection size
curl http://localhost:8000/collections

# Profile query
time curl -X POST http://localhost:8000/search \
  -d '{"collection": "vecs", "query_vector": [...], "top_k": 10}'
```

**Solutions**:
```bash
# Build index
curl -X POST http://localhost:8000/index/build

# Use faster backend
cat > config/extension.yaml <<EOF
memory:
  backend: hnsw  # or optimized
EOF

# Reduce top_k
curl -X POST http://localhost:8000/search \
  -d '{"top_k": 5}'  # instead of 100
```

### High Memory Usage

#### Problem: Server uses too much memory

**Diagnostics**:
```bash
# Check memory
free -h
pmap $(pgrep mef-api)

# Check limits
ulimit -a
```

**Solutions**:
```bash
# Limit memory backend
cat > config/extension.yaml <<EOF
memory:
  backends:
    inmemory:
      max_items: 1000  # reduce from 10000
EOF

# Enable compression
export MEF_ENABLE_COMPRESSION=true

# Increase system swap
sudo swapon /swapfile
```

---

## Network Issues

### Ghost Network Issues

#### Problem: "Failed to announce to ghost network"

**Diagnostics**:
```bash
# Check ghost network logs
tail -f logs/ghost-network.log

# Check UDP ports
netstat -un | grep <ghost_port>
```

**Solutions**:
```bash
# Configure firewall for UDP
sudo ufw allow <ghost_port>/udp

# Use different port range
export GHOST_NETWORK_PORT_RANGE=20000-21000
```

#### Problem: "No nodes discovered"

**Solutions**:
```bash
# Wait for discovery period
sleep 30

# Check bootstrap nodes
export GHOST_NETWORK_BOOTSTRAP=node1.example.com:20000

# Increase TTL
export GHOST_NETWORK_TTL=10
```

---

## Storage Issues

### Disk Full

#### Problem: "No space left on device"

**Diagnostics**:
```bash
# Check disk space
df -h

# Find large files
du -sh $MEF_STORE_DIR/*
```

**Solutions**:
```bash
# Clean old snapshots
find $MEF_STORE_DIR/snapshots -mtime +30 -delete

# Move to larger disk
export MEF_STORE_DIR=/mnt/large_disk/mef
export MEF_LEDGER_DIR=/mnt/large_disk/ledger

# Enable compression
export MEF_ENABLE_COMPRESSION=true
```

### Corrupted Storage

#### Problem: "Failed to read snapshot"

**Diagnostics**:
```bash
# Check file integrity
file $MEF_STORE_DIR/snapshots/snap_*

# Check for corruption
dmesg | grep -i error
```

**Solutions**:
```bash
# Backup storage
tar -czf mef_backup.tar.gz $MEF_STORE_DIR

# Verify filesystem
sudo fsck /dev/sda1

# Restore from backup
tar -xzf mef_backup.tar.gz -C /
```

---

## Common Error Codes

### HTTP Error Codes

| Code | Error | Solution |
|------|-------|----------|
| 400 | Bad Request | Check request format, see OpenAPI spec |
| 401 | Unauthorized | Set API token: `export MEF_API_TOKEN=...` |
| 404 | Not Found | Verify resource ID (snapshot, TIC, block) |
| 422 | Unprocessable Entity | Validate input data format |
| 429 | Rate Limited | Wait or increase rate limits |
| 500 | Internal Error | Check logs, report bug if persistent |
| 503 | Service Unavailable | Wait for server to start, check health |

### Application Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| `INVALID_SNAPSHOT` | Snapshot not found | Re-ingest data |
| `CONVERGENCE_FAILED` | TIC processing failed | Increase iterations |
| `LEDGER_CORRUPTED` | Integrity check failed | Restore from backup |
| `STORAGE_ERROR` | File I/O error | Check disk space/permissions |
| `NETWORK_ERROR` | Ghost network issue | Check firewall/network |
| `AUTH_REQUIRED` | Missing token | Set `MEF_API_TOKEN` |

---

## Diagnostic Commands

### System Health Check

```bash
#!/bin/bash

echo "=== SpectralChain Health Check ==="

# 1. API Server
echo -e "\n1. API Server Status:"
curl -s http://localhost:8000/ping | jq '.' || echo "❌ API not responding"

# 2. Disk Space
echo -e "\n2. Disk Space:"
df -h | grep -E "Filesystem|/home"

# 3. Memory Usage
echo -e "\n3. Memory Usage:"
free -h

# 4. Process Status
echo -e "\n4. Process Status:"
ps aux | grep -E "mef-api|PID" | grep -v grep

# 5. Ledger Integrity
echo -e "\n5. Ledger Integrity:"
mef audit 2>/dev/null | jq '.' || echo "❌ Audit failed"

# 6. Configuration
echo -e "\n6. Configuration:"
echo "MEF_API_URL: ${MEF_API_URL:-http://localhost:8000}"
echo "MEF_CONFIG: ${MEF_CONFIG:-~/.config/mef/config.yaml}"
echo "MEF_STORE_DIR: ${MEF_STORE_DIR:-./store}"

echo -e "\n=== Health Check Complete ==="
```

Save as `health_check.sh`, make executable, and run:
```bash
chmod +x health_check.sh
./health_check.sh
```

---

## Getting Help

### Still Having Issues?

1. **Check Logs**:
   ```bash
   tail -f logs/mef-api.log
   tail -f logs/ghost-network.log
   ```

2. **Enable Debug Logging**:
   ```bash
   export RUST_LOG=debug
   export MEF_LOG_LEVEL=debug
   ```

3. **Search Existing Issues**:
   - [GitHub Issues](https://github.com/LashSesh/spectralchain/issues)

4. **Ask Community**:
   - [GitHub Discussions](https://github.com/LashSesh/spectralchain/discussions)

5. **Report Bug**:
   - [New Bug Report](https://github.com/LashSesh/spectralchain/issues/new?template=bug_report.md)

### Bug Report Template

Include:
- SpectralChain version: `mef --version`
- Operating system: `uname -a`
- Rust version: `rustc --version`
- Error message (full)
- Steps to reproduce
- Logs (if applicable)
- Configuration (redact secrets)

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
