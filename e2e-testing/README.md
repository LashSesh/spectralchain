# SpectralChain E2E Testing & Simulation Suite

**Version:** 1.0.0
**Created:** 2025-11-06
**Status:** Production Ready

---

## 📋 Overview

Comprehensive end-to-end testing and simulation framework for the SpectralChain quantum-resonant blockchain system. This suite validates system behavior across production scenarios, edge cases, and chaos engineering experiments.

### Features

- ✅ **50+ Test Scenarios**: Production, edge cases, and chaos engineering
- ✅ **Network Simulation**: Multi-node network with configurable topologies
- ✅ **Fault Injection**: Comprehensive chaos engineering capabilities
- ✅ **Metrics Collection**: Real-time metrics with time-series analysis
- ✅ **Rich Visualizations**: Grafana dashboards, custom web UI
- ✅ **Automated Reports**: HTML, JSON, and PDF report generation
- ✅ **CI/CD Integration**: GitHub Actions workflows included

---

## 🗂️ Directory Structure

```
e2e-testing/
├── docs/                           # Documentation
│   ├── TEST_CATALOG.md             # Complete test catalog (50+ tests)
│   ├── CHAOS_ENGINEERING_PLAN.md   # Chaos engineering methodology
│   └── AUTOMATION_VISUALIZATION.md # Automation & visualization guide
│
├── schemas/                        # JSON schemas
│   └── test-definition.json        # Test definition schema
│
├── tests/                          # Test implementations
│   ├── production/                 # Production scenario tests
│   ├── edge-cases/                 # Edge case tests
│   └── chaos/                      # Chaos engineering tests
│
├── simulation/                     # Simulation tools
│   ├── network_simulator.rs        # Network topology simulation
│   ├── fault_injector.rs           # Fault injection engine
│   └── metrics_collector.rs        # Metrics collection
│
├── visualization/                  # Visualization tools
│   ├── dashboards/                 # Grafana dashboard configs
│   ├── web/                        # Custom web UI (React)
│   └── scripts/                    # Visualization scripts
│
├── config/                         # Configuration files
│   ├── prometheus.yml              # Prometheus config
│   ├── grafana/                    # Grafana configs
│   └── test-suite-config.toml      # Test suite configuration
│
├── reports/                        # Generated reports (git-ignored)
│
├── scripts/                        # Utility scripts
│   ├── run-tests.sh                # Test execution script
│   ├── generate-report.sh          # Report generation
│   └── setup-environment.sh        # Environment setup
│
├── docker-compose.yml              # Test environment stack
└── README.md                       # This file
```

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ with Tokio
- Docker & Docker Compose
- 8GB RAM minimum (16GB recommended)
- 20GB disk space

### Installation

```bash
# Clone repository (if not already done)
git clone https://github.com/LashSesh/spectralchain.git
cd spectralchain/e2e-testing

# Install dependencies
cargo build --release

# Start test environment
docker-compose up -d

# Wait for services to be ready
./scripts/wait-for-services.sh
```

### Running Tests

```bash
# Run all tests
./scripts/run-tests.sh all

# Run specific test category
./scripts/run-tests.sh production
./scripts/run-tests.sh edge-cases
./scripts/run-tests.sh chaos

# Run specific test
cargo test --test e2e_production -- PROD-001

# Run with detailed output
RUST_LOG=debug cargo test --test e2e_production -- --nocapture
```

### Viewing Results

```bash
# Generate HTML report
./scripts/generate-report.sh

# Open report in browser
open reports/index.html

# Access dashboards
# Grafana: http://localhost:3000 (admin/admin)
# Prometheus: http://localhost:9090
# Kibana: http://localhost:5601
# Jaeger: http://localhost:16686
```

---

## 📊 Test Categories

### Production Scenarios (PROD-XXX)

Normal operation tests validating core functionality:

- **PROD-001**: Basic Transaction Flow
- **PROD-002**: Multi-Hop Quantum Routing
- **PROD-003**: Ephemeral Service Lifecycle
- **PROD-004**: Fork Detection and MEF-Attractor Healing
- **PROD-005**: Key Rotation Transition
- **PROD-006**: Forward Secrecy Validation
- **PROD-007**: Adaptive Timestamp Windows

See [TEST_CATALOG.md](docs/TEST_CATALOG.md) for complete list.

### Edge Case Scenarios (EDGE-XXX)

Boundary conditions and exceptional scenarios:

- **EDGE-001**: Ghost Network Failover
- **EDGE-002**: Network Partitioning
- **EDGE-003**: Adversarial Traffic Injection
- **EDGE-004**: Fork Cascade Scenario
- **EDGE-005**: Self-Healing Under Byzantine Failures
- **EDGE-006**: Disappearing Services
- **EDGE-007**: Massive Peer Join/Leave
- **EDGE-008**: Decoy Event Storm
- **EDGE-009**: Privacy Stress Test

See [TEST_CATALOG.md](docs/TEST_CATALOG.md) for details.

### Chaos Engineering (CHAOS-XXX)

Fault injection and resilience testing:

- **CHAOS-001**: Random Node Crashes
- **CHAOS-002**: Network Latency Injection
- **CHAOS-003**: Packet Loss Injection
- **CHAOS-004**: Byzantine Node Injection
- **CHAOS-005**: Resource Exhaustion
- **CHAOS-006**: Time Synchronization Attacks
- **CHAOS-007**: Fork Bomb (Rapid Partitioning)
- **CHAOS-008**: Service Thrashing
- **CHAOS-009**: Multi-Fault Compound Scenario
- **CHAOS-010**: Adversarial Swarm Attack

See [CHAOS_ENGINEERING_PLAN.md](docs/CHAOS_ENGINEERING_PLAN.md) for methodology.

---

## 🔧 Configuration

### Test Suite Configuration

Edit `config/test-suite-config.toml`:

```toml
[test_runner]
parallel_tests = 4
timeout_seconds = 300
retry_max_attempts = 3
retry_backoff_strategy = "exponential"

[network_simulator]
default_node_count = 10
default_topology = "mesh"

[fault_injector]
chaos_intensity = 0.3  # 0.0 - 1.0
fault_duration_seconds = 60

[metrics]
sampling_interval_ms = 100
enable_prometheus = true
enable_tracing = true

[reporting]
output_format = ["html", "json", "pdf"]
output_dir = "./reports"
```

### Network Topologies

Supported topologies:
- `mesh`: Fully connected
- `ring`: Circular connections
- `star`: Central hub with spokes
- `random`: Random connections
- `clustered`: Multiple clusters
- `partitioned`: Isolated groups

### Fault Types

Available fault injections:
- `node_crash`: Simulate node failures
- `network_partition`: Split network into groups
- `network_delay`: Add latency
- `packet_loss`: Drop packets
- `resource_exhaustion`: Limit CPU/memory/disk
- `byzantine_behavior`: Malicious nodes
- `clock_skew`: Time desynchronization

---

## 📈 Metrics & Observability

### Key Metrics

**Performance**:
- `throughput_tps`: Transactions per second
- `latency_p50_ms`: Median latency
- `latency_p99_ms`: 99th percentile latency
- `success_rate`: Transaction success rate

**Network**:
- `active_node_count`: Active nodes
- `partition_count`: Network partitions
- `hop_count_avg`: Average routing hops

**Protocol**:
- `resonance_hit_rate`: Resonance success rate
- `fork_count`: Active forks
- `fork_healing_time_seconds`: Fork resolution time

**Chaos**:
- `chaos_fault_injection_count`: Active faults
- `recovery_time_seconds`: Recovery duration

### Dashboards

**Grafana Dashboards**:
- SpectralChain E2E Test Dashboard
- Network Topology Visualization
- Chaos Engineering Status
- Test Execution Timeline

Access: http://localhost:3000 (admin/admin)

**Custom Web UI**:
- Test Results Viewer
- Metrics Time Series Plots
- Network Graph Visualization

Access: http://localhost:8080

---

## 🤖 CI/CD Integration

### GitHub Actions

Automated test execution on:
- Every push to main/develop
- Pull requests
- Nightly at 2 AM UTC
- Manual workflow dispatch

### Workflows

```bash
# View workflow files
ls .github/workflows/

e2e-tests.yml           # Main E2E test workflow
chaos-engineering.yml   # Chaos engineering workflow
nightly-tests.yml       # Scheduled nightly tests
```

### Test Reports

Reports published to GitHub Pages:
https://[owner].github.io/spectralchain/test-reports/[run-number]/

---

## 🎮 Game Days

### What are Game Days?

Scheduled chaos engineering events for team practice and system validation.

**Schedule**: First Tuesday of each month, 2-4 hours

**Scenarios**:
1. Regional Outage (datacenter failure)
2. Cascading Failures
3. Byzantine Attack

See [CHAOS_ENGINEERING_PLAN.md](docs/CHAOS_ENGINEERING_PLAN.md#game-days) for details.

### Running a Game Day

```bash
# Prepare for game day
./scripts/prepare-game-day.sh

# Execute game day scenario
./scripts/run-game-day.sh scenario-1

# Post-mortem analysis
./scripts/generate-game-day-report.sh
```

---

## 🐛 Troubleshooting

### Common Issues

**Test timeout**:
```bash
# Increase timeout in config
[test_runner]
timeout_seconds = 600
```

**Insufficient resources**:
```bash
# Reduce parallel tests
[test_runner]
parallel_tests = 2

# Or increase Docker resources
# Docker Desktop > Settings > Resources > Increase memory/CPU
```

**Port conflicts**:
```bash
# Check ports in use
lsof -i :3000  # Grafana
lsof -i :9090  # Prometheus

# Stop conflicting services or change ports in docker-compose.yml
```

**Services not ready**:
```bash
# Wait for services with timeout
./scripts/wait-for-services.sh --timeout 120

# Check service health
docker-compose ps
docker-compose logs [service-name]
```

---

## 📚 Documentation

### Core Documents

1. **[TEST_CATALOG.md](docs/TEST_CATALOG.md)**: Complete test catalog with 50+ tests
2. **[CHAOS_ENGINEERING_PLAN.md](docs/CHAOS_ENGINEERING_PLAN.md)**: Chaos engineering methodology
3. **[AUTOMATION_VISUALIZATION.md](docs/AUTOMATION_VISUALIZATION.md)**: Automation framework guide

### Test Definition Schema

JSON schema: [schemas/test-definition.json](schemas/test-definition.json)

Example test definition:
```json
{
  "test_id": "PROD-001",
  "category": "production",
  "name": "Basic Transaction Flow",
  "objective": "Verify end-to-end transaction processing",
  "setup": {
    "network_topology": {
      "node_count": 5,
      "topology_type": "mesh"
    }
  },
  "execution": {
    "actions": [
      {
        "step": 1,
        "description": "Send transaction",
        "timing": {"timeout": "30s"}
      }
    ]
  },
  "validation": {
    "success_criteria": [
      {
        "criterion": "Transaction committed within 5s",
        "validation_method": "metric_check"
      }
    ]
  }
}
```

---

## 🤝 Contributing

### Adding New Tests

1. **Define test** in JSON using schema
2. **Implement test** in appropriate category (production/edge-cases/chaos)
3. **Document test** in TEST_CATALOG.md
4. **Add to CI/CD** if needed
5. **Submit PR** with test results

### Test Development Workflow

```bash
# Create new test file
touch tests/production/test_prod_new.rs

# Implement test using framework
# (See existing tests for examples)

# Run test locally
cargo test --test test_prod_new -- --nocapture

# Verify test passes
# Generate report
./scripts/generate-report.sh

# Commit and push
git add .
git commit -m "Add PROD-XXX: New test scenario"
git push
```

---

## 📞 Support

### Getting Help

- **Documentation**: Read [docs/](docs/) for detailed guides
- **Issues**: https://github.com/LashSesh/spectralchain/issues
- **Discussions**: https://github.com/LashSesh/spectralchain/discussions

### Reporting Bugs

When reporting test failures, include:
- Test ID and category
- Full test output (with `--nocapture`)
- Environment details (OS, Docker version, resources)
- Reproduction steps
- Generated report (if available)

---

## 📄 License

MIT License - See [LICENSE](../LICENSE) file for details

---

## 🙏 Acknowledgments

Built for the SpectralChain quantum-resonant blockchain project, integrating:
- **Infinity Ledger**: Proof-carrying vector ledger engine
- **MEF Framework**: Mandorla Eigenstate Fractals
- **Quantum Resonance Protocol**: Ghost networking and routing

Special thanks to the chaos engineering community for methodologies and best practices.

---

## 📊 Test Statistics

**Current Test Coverage**:
- Production Scenarios: 7 tests
- Edge Case Scenarios: 9 tests
- Chaos Engineering: 10 tests
- **Total: 26+ comprehensive tests**

**Execution Time**:
- Production Suite: ~30 minutes
- Edge Case Suite: ~60 minutes
- Chaos Suite: ~90 minutes
- **Full Suite: ~3 hours**

**Resource Requirements**:
- CPU: 4-8 cores
- Memory: 8-16 GB
- Disk: 20 GB
- Network: 100+ Mbps (for multi-node simulation)

---

**Version:** 1.0.0
**Last Updated:** 2025-11-06
**Status:** Production Ready ✅

---

## 🚀 Next Steps

1. **Review Documentation**: Read TEST_CATALOG.md and CHAOS_ENGINEERING_PLAN.md
2. **Run Tests Locally**: Follow quick start guide
3. **Schedule Game Day**: Plan first chaos engineering game day
4. **Integrate CI/CD**: Enable GitHub Actions workflows
5. **Monitor & Improve**: Use metrics to continuously improve system resilience

**Happy Testing! 🎉**
