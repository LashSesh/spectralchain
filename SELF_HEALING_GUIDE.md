# Self-Healing Systems Guide

## Overview

This guide explains how to build resilient, self-healing systems using the MEF resilience infrastructure. Self-healing systems automatically detect, diagnose, and recover from failures without human intervention.

## Core Concepts

### 1. Circuit Breaker Pattern

**Problem:** When a service fails, cascading failures can bring down the entire system as clients keep trying to call the failing service.

**Solution:** Circuit breakers automatically detect failures and stop making requests to failing services, giving them time to recover.

#### Circuit States

```
┌─────────┐
│         │  Failures < threshold
│ CLOSED  │◄────────────────────┐
│         │                     │
└────┬────┘                     │
     │                          │
     │ Failures ≥ threshold     │ Success ≥ threshold
     ▼                          │
┌─────────┐                     │
│         │  Timeout elapsed    │
│  OPEN   ├──────────────►┌─────┴─────┐
│         │                │           │
└─────────┘                │ HALF_OPEN │
   │                       │           │
   │ Request blocked       └───────────┘
   │                              │
   └──────────────────────────────┘
           Failure in half-open
```

#### Basic Usage

```rust
use mef_common::resilience::{CircuitBreaker, CircuitBreakerConfig};

// Create circuit breaker
let config = CircuitBreakerConfig {
    failure_threshold: 5,    // Open after 5 failures
    timeout_seconds: 60,      // Wait 60s before testing recovery
    success_threshold: 2,     // Need 2 successes to close
};

let breaker = CircuitBreaker::new("database", config);

// Use circuit breaker
let result = breaker.call(|| {
    database.query("SELECT * FROM users")
});

match result {
    Ok(users) => {
        // Success - circuit remains closed
    }
    Err(e) if e.to_string().contains("Circuit breaker is OPEN") => {
        // Circuit is open - service is down
        // Use fallback or return cached data
        use_cached_data()
    }
    Err(e) => {
        // Other error - circuit may open if threshold reached
    }
}
```

#### Advanced Configuration

```rust
// Aggressive: Fast failure detection
let aggressive_config = CircuitBreakerConfig {
    failure_threshold: 3,     // Open after just 3 failures
    timeout_seconds: 30,       // Retry after 30 seconds
    success_threshold: 1,      // Single success closes circuit
};

// Conservative: Tolerate more failures
let conservative_config = CircuitBreakerConfig {
    failure_threshold: 10,    // Tolerate 10 failures
    timeout_seconds: 300,      // Wait 5 minutes
    success_threshold: 5,      // Need 5 consecutive successes
};

// Custom for different services
let api_breaker = CircuitBreaker::new("external-api", aggressive_config);
let db_breaker = CircuitBreaker::new("database", conservative_config);
```

#### Monitoring Circuit State

```rust
use mef_common::resilience::CircuitState;

let state = breaker.state();
let failures = breaker.failure_count();

match state {
    CircuitState::Closed => {
        metrics.gauge("circuit.state", 0.0);  // Normal
    }
    CircuitState::HalfOpen => {
        metrics.gauge("circuit.state", 0.5);  // Testing
    }
    CircuitState::Open => {
        metrics.gauge("circuit.state", 1.0);  // Failing
        alert("Circuit breaker OPEN for {}", breaker.name);
    }
}
```

### 2. Health Checks

Health checks provide visibility into system component status.

#### Implementing Health Checks

```rust
use mef_common::resilience::{HealthChecker, HealthCheck, HealthStatus};
use mef_common::error::MefResult;

struct DatabaseHealthChecker {
    pool: DatabasePool,
}

impl HealthChecker for DatabaseHealthChecker {
    fn name(&self) -> &str {
        "database"
    }

    fn check_health(&self) -> MefResult<HealthCheck> {
        // Try simple query
        match self.pool.query("SELECT 1") {
            Ok(_) => {
                HealthCheck::healthy("database", "Connected")
            }
            Err(e) => {
                HealthCheck::unhealthy("database", format!("Query failed: {}", e))
            }
        }
    }
}

struct CacheHealthChecker {
    cache: RedisClient,
}

impl HealthChecker for CacheHealthChecker {
    fn name(&self) -> &str {
        "cache"
    }

    fn check_health(&self) -> MefResult<HealthCheck> {
        match self.cache.ping() {
            Ok(latency) if latency < 100 => {
                HealthCheck::healthy("cache", "Responsive")
            }
            Ok(latency) => {
                // Slow but working
                HealthCheck::degraded(
                    "cache",
                    format!("Slow response: {}ms", latency)
                )
            }
            Err(e) => {
                HealthCheck::unhealthy("cache", format!("Unreachable: {}", e))
            }
        }
    }
}
```

#### Aggregate Health Monitoring

```rust
use mef_common::resilience::AggregateHealthChecker;

let mut health_checker = AggregateHealthChecker::new();

// Add component checkers
health_checker.add_checker(Box::new(DatabaseHealthChecker { pool }));
health_checker.add_checker(Box::new(CacheHealthChecker { cache }));
health_checker.add_checker(Box::new(APIHealthChecker { client }));

// Check all components
let checks = health_checker.check_all()?;
for check in &checks {
    tracing::info!(
        component = check.component,
        status = ?check.status,
        message = check.message,
        "Health check result"
    );
}

// Get overall status
let overall = health_checker.overall_status()?;
match overall {
    HealthStatus::Healthy => {
        // All systems operational
    }
    HealthStatus::Degraded => {
        // Some components degraded - may impact performance
        send_warning_notification()
    }
    HealthStatus::Unhealthy => {
        // Critical components failing
        send_critical_alert()
    }
}
```

#### Health Check Endpoints

```rust
// Axum HTTP endpoint
async fn health_endpoint(
    State(health_checker): State<Arc<AggregateHealthChecker>>
) -> impl IntoResponse {
    let checks = health_checker.check_all()
        .unwrap_or_else(|_| vec![]);

    let overall = health_checker.overall_status()
        .unwrap_or(HealthStatus::Unhealthy);

    let response = json!({
        "status": match overall {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
        },
        "components": checks.iter().map(|c| json!({
            "name": c.component,
            "status": format!("{:?}", c.status),
            "message": c.message,
            "timestamp": c.timestamp,
        })).collect::<Vec<_>>(),
    });

    let status_code = match overall {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (status_code, Json(response))
}
```

### 3. Auto-Recovery with Exponential Backoff

Automatically retry failed operations with increasing delays.

#### Basic Usage

```rust
use mef_common::resilience::{auto_recover, RecoveryConfig};

let config = RecoveryConfig {
    max_attempts: 3,
    initial_backoff_ms: 100,      // Start with 100ms
    max_backoff_ms: 10000,         // Max 10 seconds
    backoff_multiplier: 2.0,       // Double each time
};

let result = auto_recover("fetch_user_data", config, || async {
    api_client.get_user(user_id).await
}).await?;
```

#### Backoff Schedule Example

| Attempt | Backoff | Total Time |
|---------|---------|------------|
| 1       | 0ms     | 0ms        |
| 2       | 100ms   | 100ms      |
| 3       | 200ms   | 300ms      |
| 4       | 400ms   | 700ms      |
| 5       | 800ms   | 1500ms     |
| 6       | 1600ms  | 3100ms     |

#### Use Cases

```rust
// Network requests
let data = auto_recover("api_call", RecoveryConfig::default(), || async {
    reqwest::get("https://api.example.com/data").await
}).await?;

// Database operations
let user = auto_recover("db_query", RecoveryConfig::default(), || async {
    db.find_user(id).await
}).await?;

// File operations
let content = auto_recover("file_read", RecoveryConfig::default(), || async {
    tokio::fs::read_to_string(path).await.map_err(Into::into)
}).await?;
```

## Patterns and Best Practices

### Pattern 1: Circuit Breaker + Fallback

Combine circuit breakers with fallback strategies:

```rust
fn get_user_data(id: &str) -> MefResult<User> {
    let result = database_breaker.call(|| {
        database.get_user(id)
    });

    match result {
        Ok(user) => Ok(user),
        Err(e) if e.to_string().contains("OPEN") => {
            // Circuit open - use cache
            cache.get_user(id)
                .ok_or_else(|| MefError::not_found("User not in cache"))
        }
        Err(e) => Err(e),
    }
}
```

### Pattern 2: Health Check + Auto-Scaling

Use health checks to trigger auto-scaling:

```rust
async fn health_monitor_loop(checker: Arc<AggregateHealthChecker>) {
    loop {
        let status = checker.overall_status().unwrap_or(HealthStatus::Unhealthy);

        if status == HealthStatus::Unhealthy {
            // Trigger auto-scaling
            autoscaler.scale_up().await?;
        }

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
```

### Pattern 3: Layered Recovery

Combine multiple recovery strategies:

```rust
async fn resilient_api_call(endpoint: &str) -> MefResult<Response> {
    // Layer 1: Circuit breaker
    api_breaker.call(|| {
        // Layer 2: Auto-recovery with backoff
        auto_recover("api_request", RecoveryConfig::default(), || async {
            // Layer 3: Timeout protection
            tokio::time::timeout(
                Duration::from_secs(30),
                reqwest::get(endpoint)
            ).await
            .map_err(|_| MefError::timeout("API request timed out"))?
            .map_err(Into::into)
        })
    })
}
```

### Pattern 4: Graceful Degradation

Degrade functionality instead of failing completely:

```rust
async fn get_recommendations(user_id: &str) -> Vec<Recommendation> {
    let ml_breaker = CircuitBreaker::new("ml-service", config);

    match ml_breaker.call(|| ml_service.get_recommendations(user_id)) {
        Ok(recommendations) => recommendations,
        Err(_) => {
            // ML service down - use rule-based fallback
            tracing::warn!("ML service unavailable, using rule-based recommendations");
            rule_based_recommender.get_recommendations(user_id)
        }
    }
}
```

### Pattern 5: Self-Healing State Machines

Implement state machines that recover from invalid states:

```rust
struct SelfHealingStateMachine {
    state: Arc<SafeRwLock<State>>,
    health_checker: Arc<AggregateHealthChecker>,
}

impl SelfHealingStateMachine {
    async fn run(&self) {
        loop {
            // Check health
            let health = self.health_checker.overall_status().unwrap();

            if health == HealthStatus::Unhealthy {
                // Attempt recovery
                self.attempt_recovery().await;
            }

            // Normal operation
            self.process_events().await;

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn attempt_recovery(&self) {
        tracing::warn!("Attempting self-recovery");

        // Reset to safe state
        *self.state.write() = State::default();

        // Re-initialize components
        auto_recover("reinitialize", RecoveryConfig::default(), || async {
            self.initialize().await
        }).await.ok();
    }
}
```

## Testing Self-Healing Systems

### Testing Circuit Breakers

```rust
#[test]
fn test_circuit_breaker_opens_on_failures() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout_seconds: 1,
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new("test", config);

    // Simulate failures
    for _ in 0..3 {
        let _: MefResult<()> = breaker.call(|| Err(MefError::other("failure")));
    }

    // Circuit should be open
    assert_eq!(breaker.state(), CircuitState::Open);

    // Should block requests
    let result: MefResult<()> = breaker.call(|| Ok(()));
    assert!(result.is_err());
}
```

### Property-Based Testing for Resilience

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn circuit_breaker_never_panics(
        operations in prop::collection::vec(
            prop::bool::ANY,  // true = success, false = failure
            0..100
        )
    ) {
        let breaker = CircuitBreaker::new("test", CircuitBreakerConfig::default());

        for success in operations {
            let result: MefResult<()> = breaker.call(|| {
                if success {
                    Ok(())
                } else {
                    Err(MefError::other("test failure"))
                }
            });

            // Should never panic, always return a result
            let _ = result;
        }
    }
}
```

## Monitoring and Observability

### Metrics to Track

```rust
// Circuit breaker metrics
metrics.gauge("circuit_breaker.state", match state {
    CircuitState::Closed => 0.0,
    CircuitState::HalfOpen => 0.5,
    CircuitState::Open => 1.0,
});

metrics.counter("circuit_breaker.failures", failures as i64);
metrics.counter("circuit_breaker.state_transitions", 1);

// Health check metrics
metrics.gauge("health.overall", match status {
    HealthStatus::Healthy => 1.0,
    HealthStatus::Degraded => 0.5,
    HealthStatus::Unhealthy => 0.0,
});

// Recovery metrics
metrics.counter("recovery.attempts", 1);
metrics.counter("recovery.successes", 1);
metrics.histogram("recovery.duration_ms", duration);
```

### Logging Best Practices

```rust
// Circuit breaker events
tracing::error!(
    circuit_breaker = "database",
    failures = 5,
    threshold = 5,
    "Circuit breaker OPENED"
);

// Recovery attempts
tracing::warn!(
    operation = "fetch_user",
    attempt = 2,
    max_attempts = 3,
    backoff_ms = 200,
    "Auto-recovery retry"
);

// Health check results
tracing::info!(
    component = "cache",
    status = "degraded",
    latency_ms = 150,
    "Health check completed"
);
```

## Summary

Self-healing systems use three core mechanisms:

1. **Circuit Breakers**: Prevent cascading failures by blocking requests to failing services
2. **Health Checks**: Monitor component status for early detection of issues
3. **Auto-Recovery**: Automatically retry failed operations with backoff

Together, these create systems that:
- ✅ Detect failures automatically
- ✅ Isolate failing components
- ✅ Provide fallback functionality
- ✅ Recover without human intervention
- ✅ Maintain overall system stability

Use `mef-common::resilience` to build production-ready, fault-tolerant MEF systems.
