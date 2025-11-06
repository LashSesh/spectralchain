# SpectralChain API Quickstart

**⏱️ Time**: 5 minutes | **Level**: Beginner

---

## Goal

Get up and running with the SpectralChain REST API in 5 minutes.

---

## Prerequisites

- API server running (or use local development server)
- `curl` or similar HTTP client
- API token (if authentication enabled)

---

## Step 1: Start the API Server (Development)

```bash
# Clone repository
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/resources_dev/infinityledger

# Build and run
cargo run --release --package mef-api
```

Server starts on `http://localhost:8000`

---

## Step 2: Health Check

```bash
curl http://localhost:8000/ping
```

**Expected Response**:
```json
{
  "status": "ok",
  "version": "2.0.0",
  "seed": "MEF_SEED_42",
  "timestamp": "2025-11-06T12:34:56Z"
}
```

✅ API is ready!

---

## Step 3: Ingest Data

```bash
curl -X POST http://localhost:8000/ingest \
  -H "Content-Type: application/json" \
  -d '{
    "data": "Hello, SpectralChain!",
    "data_type": "text",
    "seed": "my-first-seed"
  }'
```

**Response**:
```json
{
  "snapshot_id": "snap_1a2b3c4d5e6f",
  "phase": "ingested",
  "por": {
    "hash": "sha256:abc123...",
    "signature": "sig_xyz..."
  },
  "timestamp": "2025-11-06T12:35:00Z"
}
```

💾 Save the `snapshot_id` for next step!

---

## Step 4: Process Snapshot

```bash
curl -X POST http://localhost:8000/process \
  -H "Content-Type: application/json" \
  -d '{
    "snapshot_id": "snap_1a2b3c4d5e6f",
    "commit": true
  }'
```

**Response**:
```json
{
  "tic_id": "tic_9z8y7x6w5v4u",
  "converged": true,
  "iterations": 42,
  "final_eigenvalue": 0.999987,
  "committed": true
}
```

🎉 Your first TIC is created and committed to the ledger!

---

## Step 5: Audit Ledger

```bash
curl http://localhost:8000/audit
```

**Response**:
```json
{
  "valid": true,
  "total_blocks": 1,
  "checked_blocks": 1,
  "first_invalid_block": null
}
```

✅ Ledger integrity verified!

---

## Complete Example (One Script)

```bash
#!/bin/bash
API_URL="http://localhost:8000"

# 1. Health check
echo "1. Checking API health..."
curl -s "$API_URL/ping" | jq '.'

# 2. Ingest data
echo -e "\n2. Ingesting data..."
SNAPSHOT_ID=$(curl -s -X POST "$API_URL/ingest" \
  -H "Content-Type: application/json" \
  -d '{"data": "Hello, SpectralChain!", "data_type": "text", "seed": "quickstart"}' \
  | jq -r '.snapshot_id')

echo "Snapshot ID: $SNAPSHOT_ID"

# 3. Process snapshot
echo -e "\n3. Processing snapshot..."
TIC_ID=$(curl -s -X POST "$API_URL/process" \
  -H "Content-Type: application/json" \
  -d "{\"snapshot_id\": \"$SNAPSHOT_ID\", \"commit\": true}" \
  | jq -r '.tic_id')

echo "TIC ID: $TIC_ID"

# 4. Audit ledger
echo -e "\n4. Auditing ledger..."
curl -s "$API_URL/audit" | jq '.'

echo -e "\n✅ Quickstart complete!"
```

---

## With Authentication

If authentication is enabled:

```bash
export API_TOKEN="your_token_here"

curl -X POST http://localhost:8000/ingest \
  -H "Authorization: Bearer $API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"data": "Authenticated request", "data_type": "text"}'
```

---

## Next Steps

### 🚀 Explore More Endpoints

- [Vector Search](../api/USER_GUIDE.md#vector-search)
- [Zero-Knowledge Proofs](../api/USER_GUIDE.md#zero-knowledge)
- [Metatron Routing](../api/USER_GUIDE.md#metatron-routing)

### 📚 Deep Dive

- [Complete API Reference](../api/README.md)
- [OpenAPI Specification](../api/openapi.yaml)
- [Example Applications](../examples/README.md)

### 🛠️ Try Other Interfaces

- [CLI Quickstart](CLI_QUICKSTART.md)
- [Rust SDK Quickstart](RUST_SDK_QUICKSTART.md)
- [Docker Quickstart](DOCKER_QUICKSTART.md)

---

## Troubleshooting

### Connection Refused
```bash
# Check server is running
ps aux | grep mef-api

# Check port
netstat -tuln | grep 8000
```

### Authentication Error
```bash
# Disable auth for development
export AUTH_TOKEN_REQUIRED=false
cargo run --release --package mef-api
```

### Invalid Snapshot ID
- Snapshot IDs have format: `snap_<12_hex_chars>`
- Copy exact value from ingest response

---

**⏱️ Completed in under 5 minutes? Great!**

Need help? Check the [FAQ](../FAQ.md) or [Troubleshooting Guide](../TROUBLESHOOTING.md).
