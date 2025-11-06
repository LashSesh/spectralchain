# 📊 Refactoring Tasks - Dependency Order & Impact Matrix

## 🎯 Quick Reference

**Total Tasks:** 23
**Total Effort:** 85 developer-days (17 weeks)
**Critical Path:** 7 tasks (19 days)
**Phases:** 6 (Foundation → Advanced)

---

## 📋 Complete Task Table

| ID | Task | Phase | Pri | Impact | Effort | Dependencies | LOC Δ | Risk | Status |
|----|------|-------|-----|--------|--------|--------------|-------|------|--------|
| **R-00-001** | Create Shared Utilities Crate | 0 | 🔴 CRIT | 🔥🔥🔥 HIGH | 1d | - | +300 | 🟢 LOW | ⏳ READY |
| **R-00-002** | Setup Property-Based Test Framework | 0 | 🔴 CRIT | 🔥🔥🔥 HIGH | 2d | R-00-001 | +500 | 🟢 LOW | ⏳ BLOCKED |
| **R-00-003** | Create Self-Healing Infrastructure | 0 | 🔴 CRIT | 🔥🔥🔥 HIGH | 3d | R-00-001 | +800 | 🟡 MED | ⏳ BLOCKED |
| **R-01-001** | Eliminate All .unwrap() Calls | 1 | 🔴 CRIT | 🔥🔥🔥 HIGH | 5d | R-00-001 | -191/+300 | 🟢 LOW | ⏳ BLOCKED |
| **R-01-002** | Add Invariant Assertions | 1 | 🔴 CRIT | 🔥🔥🔥 HIGH | 3d | R-00-001 | +400 | 🟢 LOW | ⏳ BLOCKED |
| **R-01-003** | Implement RwLock Recovery | 1 | 🔴 CRIT | 🔥🔥🔥 HIGH | 2d | R-01-001 | +200 | 🟢 LOW | ⏳ BLOCKED |
| **R-01-004** | Add Circuit Breakers | 1 | 🔴 CRIT | 🔥🔥🔥 HIGH | 3d | R-00-003 | +300 | 🟡 MED | ⏳ BLOCKED |
| **R-02-001** | Split MetatronRouter (1,286 LOC) | 2 | 🟠 HIGH | 🔥🔥 MED | 5d | R-01-001 | +100 | 🟡 MED | ⏳ BLOCKED |
| **R-02-002** | Split SpiralCoupling (1,026 LOC) | 2 | 🟠 HIGH | 🔥🔥 MED | 4d | R-01-001 | +80 | 🟡 MED | ⏳ BLOCKED |
| **R-02-003** | Split VectorDB Providers (961 LOC) | 2 | 🟠 HIGH | 🔥🔥 MED | 4d | R-01-001 | +75 | 🟡 MED | ⏳ BLOCKED |
| **R-02-004** | Extract Timestamp Utility | 2 | 🟠 HIGH | 🔥 LOW | 1d | R-00-001 | -100 | 🟢 LOW | ⏳ BLOCKED |
| **R-02-005** | Extract RwLock Helpers | 2 | 🟠 HIGH | 🔥 LOW | 2d | R-01-003 | -150 | 🟢 LOW | ⏳ BLOCKED |
| **R-03-001** | Add Health Check System | 3 | 🟠 HIGH | 🔥🔥🔥 HIGH | 3d | R-00-003 | +400 | 🟡 MED | ⏳ BLOCKED |
| **R-03-002** | Implement Auto-Recovery | 3 | 🟠 HIGH | 🔥🔥🔥 HIGH | 4d | R-03-001 | +500 | 🔴 HIGH | ⏳ BLOCKED |
| **R-03-003** | Add Graceful Degradation | 3 | 🟠 HIGH | 🔥🔥🔥 HIGH | 3d | R-03-001 | +350 | 🟡 MED | ⏳ BLOCKED |
| **R-03-004** | Implement Retry with Backoff | 3 | 🟠 HIGH | 🔥🔥 MED | 2d | R-00-003 | +250 | 🟢 LOW | ⏳ BLOCKED |
| **R-03-005** | Add State Machine Recovery | 3 | 🟡 MED | 🔥🔥🔥 HIGH | 5d | R-03-002 | +600 | 🔴 HIGH | ⏳ BLOCKED |
| **R-04-001** | Implement Stub Integration Tests | 4 | 🟠 HIGH | 🔥🔥🔥 HIGH | 5d | R-00-002 | +2000 | 🟢 LOW | ⏳ BLOCKED |
| **R-04-002** | Add Property-Based Tests | 4 | 🟠 HIGH | 🔥🔥🔥 HIGH | 8d | R-00-002 | +1500 | 🟢 LOW | ⏳ BLOCKED |
| **R-04-003** | Setup Coverage CI | 4 | 🟡 MED | 🔥🔥 MED | 2d | R-04-001 | +200 | 🟢 LOW | ⏳ BLOCKED |
| **R-04-004** | Add Mutation Testing | 4 | 🟡 MED | 🔥🔥 MED | 3d | R-04-001 | +400 | 🟢 LOW | ⏳ BLOCKED |
| **R-05-001** | Add Chaos Engineering | 5 | 🟡 MED | 🔥🔥🔥 HIGH | 5d | R-03-002 | +700 | 🔴 HIGH | ⏳ BLOCKED |
| **R-05-002** | Implement Telemetry | 5 | 🟡 MED | 🔥🔥🔥 HIGH | 4d | R-03-001 | +500 | 🟡 MED | ⏳ BLOCKED |
| **R-05-003** | Add Performance Guards | 5 | 🔵 LOW | 🔥🔥 MED | 3d | R-03-001 | +300 | 🟢 LOW | ⏳ BLOCKED |

**Legend:**
- **Pri:** 🔴 CRITICAL | 🟠 HIGH | 🟡 MEDIUM | 🔵 LOW
- **Impact:** 🔥🔥🔥 HIGH | 🔥🔥 MEDIUM | 🔥 LOW
- **Risk:** 🔴 HIGH | 🟡 MEDIUM | 🟢 LOW

---

## 🔗 Dependency Graph

```
Phase 0: Foundation
    R-00-001 (Shared Utilities) ─┬─→ R-00-002 (Property Tests)
                                  ├─→ R-00-003 (Self-Healing)
                                  ├─→ R-01-001 (Remove unwrap)
                                  ├─→ R-01-002 (Invariants)
                                  └─→ R-02-004 (Timestamp Util)

Phase 1: Critical Safety
    R-01-001 ─┬─→ R-01-003 (RwLock Recovery)
              ├─→ R-02-001 (Split MetatronRouter)
              ├─→ R-02-002 (Split SpiralCoupling)
              └─→ R-02-003 (Split VectorDB)

    R-01-003 ──→ R-02-005 (RwLock Helpers)

    R-00-003 ──→ R-01-004 (Circuit Breakers)

Phase 2: Architecture
    [Multiple splits run in parallel]
    R-02-001, R-02-002, R-02-003 (Independent)
    R-02-004, R-02-005 (Independent)

Phase 3: Self-Healing
    R-00-003 ─┬─→ R-03-001 (Health Checks)
              └─→ R-03-004 (Retry)

    R-03-001 ─┬─→ R-03-002 (Auto-Recovery)
              ├─→ R-03-003 (Graceful Degradation)
              ├─→ R-05-002 (Telemetry)
              └─→ R-05-003 (Perf Guards)

    R-03-002 ─┬─→ R-03-005 (State Machine Recovery)
              └─→ R-05-001 (Chaos Engineering)

Phase 4: Quality
    R-00-002 ─┬─→ R-04-001 (Integration Tests)
              └─→ R-04-002 (Property Tests)

    R-04-001 ─┬─→ R-04-003 (Coverage CI)
              └─→ R-04-004 (Mutation Testing)

Phase 5: Advanced
    [Already covered above]
```

---

## ⏱️ Critical Path Analysis

**Critical Path (Longest Dependency Chain):**
```
R-00-001 (1d)
  → R-00-003 (3d)
    → R-03-001 (3d)
      → R-03-002 (4d)
        → R-03-005 (5d)
          → R-05-001 (5d)

Total: 21 days
```

**Blocking Tasks (Must Complete First):**
1. R-00-001 - Blocks 6 other tasks
2. R-00-003 - Blocks 5 self-healing tasks
3. R-01-001 - Blocks 4 architecture tasks
4. R-03-001 - Blocks 3 advanced tasks

---

## 📊 Effort Distribution by Phase

| Phase | Tasks | Total Days | % of Total | Can Parallelize |
|-------|-------|------------|------------|-----------------|
| **Phase 0: Foundation** | 3 | 6 | 7% | No (Sequential) |
| **Phase 1: Critical** | 4 | 13 | 15% | Partially (2 parallel) |
| **Phase 2: Architecture** | 5 | 16 | 19% | Yes (4 parallel) |
| **Phase 3: Self-Healing** | 5 | 17 | 20% | Partially (3 parallel) |
| **Phase 4: Quality** | 4 | 18 | 21% | Partially (2 parallel) |
| **Phase 5: Advanced** | 3 | 12 | 14% | Yes (2 parallel) |
| **TOTAL** | **24** | **85** | **100%** | - |

**With Parallelization:**
- Sequential execution: 85 days (17 weeks)
- With 2 developers: ~55 days (11 weeks)
- With 3 developers: ~45 days (9 weeks)

---

## 🎯 Impact Analysis

### High Impact Tasks (Must Do)

| ID | Task | Impact Reason |
|----|------|---------------|
| R-00-001 | Shared Utilities | Blocks everything, eliminates 100+ duplications |
| R-01-001 | Remove .unwrap() | Eliminates 191 production panic points |
| R-01-004 | Circuit Breakers | Prevents cascade failures |
| R-03-002 | Auto-Recovery | Core self-healing capability |
| R-04-001 | Integration Tests | Currently only 17 stubs (0% real coverage) |
| R-04-002 | Property Tests | Exhaustive testing of critical functions |

### Medium Impact Tasks (Should Do)

| ID | Task | Impact Reason |
|----|------|---------------|
| R-02-001 | Split MetatronRouter | 1,286 LOC → 4 modules (maintainability) |
| R-02-002 | Split SpiralCoupling | 1,026 LOC → 3 modules |
| R-02-003 | Split VectorDB | 961 LOC → 5 modules |
| R-03-001 | Health Checks | Enables monitoring & alerting |
| R-05-001 | Chaos Engineering | Validates resilience |

### Low Impact Tasks (Nice to Have)

| ID | Task | Impact Reason |
|----|------|---------------|
| R-02-004 | Timestamp Utility | Quality of life (already functional) |
| R-02-005 | RwLock Helpers | Quality of life |
| R-04-003 | Coverage CI | Automation (manual possible) |
| R-05-003 | Performance Guards | Optimization (not blocking) |

---

## 🚦 Risk Assessment

### High Risk Tasks (Need Special Attention)

| ID | Task | Risk Factors | Mitigation |
|----|------|--------------|------------|
| R-03-002 | Auto-Recovery | Complex state management | Extensive testing, feature flag |
| R-03-005 | State Machine Recovery | Intricate state transitions | Formal verification, property tests |
| R-05-001 | Chaos Engineering | Could expose critical bugs | Isolated environment, gradual rollout |

### Medium Risk Tasks

| ID | Task | Risk Factors | Mitigation |
|----|------|--------------|------------|
| R-00-003 | Self-Healing Infra | New architecture pattern | Prototyping, incremental adoption |
| R-01-004 | Circuit Breakers | Behavioral changes | Feature flag, monitoring |
| R-02-001 | Split MetatronRouter | Large refactoring | Incremental extraction, tests |

### Low Risk Tasks (Safe to Execute)

- R-00-001, R-00-002 (Additive changes)
- R-01-001, R-01-002 (Error handling improvements)
- R-02-004, R-02-005 (Simple extractions)
- R-04-001, R-04-002 (Test additions)

---

## 📅 Suggested Implementation Timeline

### Weeks 1-2: Phase 0 (Foundation)
```
Week 1:
  [Dev 1] R-00-001: Shared Utilities (1d) → R-00-002: Property Tests (2d)
  [Dev 2] R-00-001 Complete → R-00-003: Self-Healing (3d)

Week 2:
  [Dev 1] Continue R-00-002
  [Dev 2] Complete R-00-003
```

### Weeks 3-5: Phase 1 (Critical Safety)
```
Week 3:
  [Dev 1] R-01-001: Remove .unwrap() (5d start)
  [Dev 2] R-01-002: Invariant Assertions (3d)

Week 4-5:
  [Dev 1] Continue R-01-001
  [Dev 2] R-01-003: RwLock Recovery (2d) → R-01-004: Circuit Breakers (3d)
```

### Weeks 6-9: Phase 2 (Architecture)
```
Week 6-7:
  [Dev 1] R-02-001: Split MetatronRouter (5d)
  [Dev 2] R-02-002: Split SpiralCoupling (4d) → R-02-004: Timestamp (1d)

Week 8-9:
  [Dev 1] Continue if needed
  [Dev 2] R-02-003: Split VectorDB (4d) → R-02-005: RwLock Helpers (2d)
```

### Weeks 10-13: Phase 3 (Self-Healing)
```
Week 10-11:
  [Dev 1] R-03-001: Health Checks (3d) → R-03-002: Auto-Recovery (4d start)
  [Dev 2] R-03-004: Retry Logic (2d) → R-03-003: Graceful Degradation (3d)

Week 12-13:
  [Dev 1] Complete R-03-002 → R-03-005: State Machine (5d)
  [Dev 2] Buffer/Support
```

### Weeks 14-17: Phase 4 (Quality)
```
Week 14-16:
  [Dev 1] R-04-001: Integration Tests (5d) → R-04-003: Coverage CI (2d)
  [Dev 2] R-04-002: Property Tests (8d)

Week 17:
  [Dev 1] R-04-004: Mutation Testing (3d)
  [Dev 2] Complete R-04-002
```

### Weeks 18-19: Phase 5 (Advanced)
```
Week 18-19:
  [Dev 1] R-05-001: Chaos Engineering (5d)
  [Dev 2] R-05-002: Telemetry (4d) → R-05-003: Perf Guards (3d)
```

---

## ✅ Success Criteria by Phase

### Phase 0: Foundation
- ✅ 0 timestamp pattern duplications (from 27+)
- ✅ 0 RwLock pattern duplications (from 66+)
- ✅ 100+ property tests defined
- ✅ Circuit breaker infrastructure ready

### Phase 1: Critical Safety
- ✅ 0 .unwrap() calls in production paths (from 191)
- ✅ 50+ invariant assertions added
- ✅ All RwLocks have recovery strategy
- ✅ 15+ circuit breakers deployed

### Phase 2: Architecture
- ✅ All modules <400 LOC (from max 1,286)
- ✅ 0 SRP violations (from 9)
- ✅ 100% shared utility adoption

### Phase 3: Self-Healing
- ✅ 20+ health checks operational
- ✅ 10+ recovery strategies
- ✅ Graceful degradation for all critical paths
- ✅ State machines with recovery

### Phase 4: Quality
- ✅ 85%+ test coverage (from ~2%)
- ✅ 150+ real integration tests (from 17 stubs)
- ✅ 100+ property tests running
- ✅ CI enforces coverage threshold

### Phase 5: Advanced
- ✅ Chaos tests pass
- ✅ Telemetry dashboard operational
- ✅ Performance guards prevent regression

---

## 🎬 Getting Started

### Step 1: Review & Approve Plan
1. Review this document with team
2. Adjust timeline based on available resources
3. Identify any missing tasks
4. Get stakeholder buy-in

### Step 2: Setup Infrastructure
1. Create tracking board (Jira/GitHub Projects)
2. Setup feature branches
3. Configure CI/CD for validation
4. Prepare rollback procedures

### Step 3: Execute Phase 0
1. Start with R-00-001 (Shared Utilities)
2. Follow dependency order
3. Validate at each step
4. Document learnings

### Step 4: Iterate
1. Complete each phase sequentially
2. Review metrics after each phase
3. Adjust plan as needed
4. Celebrate milestones! 🎉

---

**Last Updated:** 2025-11-06
**Status:** 📋 PLAN APPROVED, READY FOR EXECUTION
**Next Review:** After Phase 0 Completion

