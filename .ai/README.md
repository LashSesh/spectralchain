# .ai Directory - AI Agent Knowledge Base

**Version**: 1.0.0
**Purpose**: Centralized knowledge base and automation for AI agents

---

## 📁 Directory Structure

```
.ai/
├── README.md                       # This file
├── handover-automation.json        # Automation configuration
├── module-map.json                 # Module dependency map (auto-generated)
├── test-coverage.json              # Test coverage data (auto-generated)
├── performance-baseline.json       # Performance baseline (auto-generated)
├── health-score-history.json       # Historical health scores (auto-generated)
├── config/                         # Configuration files
│   ├── anti-outdating-config.json
│   └── blueprint-compliance-rules.json
└── reports/                        # Generated reports
    ├── daily-health-YYYYMMDD.json
    ├── dependency-audit-YYYYMMDD.json
    └── doc-sync-YYYYMMDD.json
```

---

## 🤖 For New AI Agents

### Quick Start

1. **Read the Master Prompt**:
   ```bash
   cat ../AI_HANDOVER_MASTER_PROMPT.md
   ```

2. **Run Self-Discovery**:
   ```bash
   ../scripts/ai-self-discovery.sh
   ```

3. **Check Current Health**:
   ```bash
   ../scripts/daily-health-check.sh
   ```

4. **Review Latest Reports**:
   ```bash
   ls -lt reports/ | head -5
   cat reports/$(ls -t reports/ | head -1)
   ```

---

## 📊 Auto-Generated Files

### module-map.json

Contains the complete module dependency graph. Generated on every commit.

**Example**:
```json
{
  "modules": {
    "mef-quantum-ops": {
      "dependencies": ["blake3", "chacha20poly1305", "halo2_proofs"],
      "dependents": ["mef-ghost-network"],
      "loc": 1433,
      "tests": 43
    }
  }
}
```

### test-coverage.json

Current test coverage statistics. Updated on every test run.

**Example**:
```json
{
  "overall": 95.0,
  "modules": {
    "mef-quantum-ops": 100.0,
    "mef-ghost-network": 96.0
  }
}
```

### performance-baseline.json

Performance benchmarks baseline. Updated weekly or on-demand.

**Example**:
```json
{
  "date": "2025-11-06",
  "benchmarks": {
    "masking_operation": {
      "mean": "1.2ms",
      "std_dev": "0.05ms"
    }
  }
}
```

---

## 🔄 Automation

### Scheduled Tasks

All tasks configured in `handover-automation.json`:

- **Daily**: Health checks (code quality, tests, security)
- **Weekly**: Dependency audits, performance baselines
- **Monthly**: Documentation sync, comprehensive reports
- **On-Commit**: Module map updates, blueprint compliance

### Manual Execution

Run any automation manually:

```bash
# Daily health check
../scripts/daily-health-check.sh

# Anti-outdating check
../scripts/anti-outdating-check.sh

# Codebase health scanner
../scripts/codebase-health-scanner.sh

# Blueprint compliance
../scripts/blueprint-sync-check.sh
```

---

## 🛡️ Innovation Safeguards

### 1. Anti-Outdating

Monitors for:
- Outdated Rust versions
- Outdated dependencies
- Security advisories
- Deprecated best practices

**Run**: `../scripts/anti-outdating-check.sh`

### 2. Health Scanner

Comprehensive health analysis:
- Code quality metrics
- Test coverage
- Documentation coverage
- Security audits
- Module complexity
- Technical debt

**Run**: `../scripts/codebase-health-scanner.sh`

### 3. Blueprint Sync

Verifies implementation matches original Blueprint:
- Operator implementations
- Protocol flows
- Mathematical properties
- Architecture principles

**Run**: `../scripts/blueprint-sync-check.sh`

---

## 📈 Health Monitoring

### Health Score Calculation

Overall health score (0-100) based on weighted metrics:

| Metric | Weight | Target |
|--------|--------|--------|
| Architecture | 20% | >95 |
| Code Quality | 20% | >95 |
| Testing | 15% | >90 |
| Documentation | 10% | >90 |
| Security | 15% | >95 |
| Performance | 10% | Baseline |
| Innovation | 10% | Active |

### Current Status

Check latest health score:
```bash
cat reports/$(ls -t reports/daily-health-* | head -1) | grep health_score
```

---

## 🔍 Self-Discovery

New AI agents should run the self-discovery process:

```bash
../scripts/ai-self-discovery.sh
```

This will:
1. Validate environment
2. Build and test codebase
3. Generate dependency graph
4. Create initial knowledge base
5. Identify current tasks

---

## 📚 Knowledge Base Updates

### Automatic Updates

The knowledge base is automatically updated on:
- **Git commits**: Module map, dependency graph
- **Test runs**: Coverage data
- **Benchmarks**: Performance baselines
- **PR merges**: Health score history

### Manual Updates

Force update the knowledge base:

```bash
# Update module map
../scripts/generate-module-map.sh

# Update test coverage
cargo test --workspace
../scripts/update-test-coverage.sh

# Update performance baseline
cargo bench --workspace
../scripts/update-performance-baseline.sh
```

---

## 🚨 Alert Thresholds

Alerts are triggered when:

| Condition | Threshold | Action |
|-----------|-----------|--------|
| Health score drop | >10 points | Email + GitHub issue |
| Test failures | Any | Immediate notification |
| Security vulnerability | Critical/High | Block PR + notify |
| Coverage drop | >5% | Warning notification |
| Performance regression | >10% | Create issue |

---

## 📞 Support

### For AI Agents

- **Documentation**: `../docs/`
- **Master Prompt**: `../AI_HANDOVER_MASTER_PROMPT.md`
- **Schema**: `../ai_handover_schema.json`
- **Scripts**: `../scripts/`

### For Humans

- Discord: #dev-discussion
- Email: dev@spectralchain.io
- GitHub Issues: Bug reports & questions

---

## ✅ Checklist for New Agents

Before starting work:

- [ ] Read AI_HANDOVER_MASTER_PROMPT.md
- [ ] Run ai-self-discovery.sh
- [ ] Review latest health report
- [ ] Check module-map.json
- [ ] Verify test coverage >90%
- [ ] Review current roadmap
- [ ] Identify assigned tasks

---

**Maintainer**: AI Infrastructure Team
**Last Updated**: 2025-11-06
**Next Review**: After Q4 2025 milestones
