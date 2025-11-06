//! Resilience and self-healing infrastructure
//!
//! Provides circuit breakers, health checks, retry logic, and auto-recovery
//! mechanisms for building fault-tolerant systems.

use crate::error::{MefError, MefResult};
use crate::time::current_timestamp;
use std::sync::Arc;
use std::time::Duration;
use parking_lot::RwLock;

// ============================================================================
// Circuit Breaker Pattern
// ============================================================================

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation - requests allowed
    Closed,
    /// Failing state - requests blocked
    Open,
    /// Testing recovery - limited requests allowed
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Duration to wait before transitioning to HalfOpen (seconds)
    pub timeout_seconds: u64,
    /// Number of successful requests in HalfOpen before closing
    pub success_threshold: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_seconds: 60,
            success_threshold: 2,
        }
    }
}

/// Circuit breaker for fault tolerance
///
/// Prevents cascading failures by automatically blocking requests to failing services.
///
/// # States
///
/// - **Closed**: Normal operation, all requests allowed
/// - **Open**: Service is failing, all requests blocked
/// - **HalfOpen**: Testing if service recovered, limited requests allowed
///
/// # Example
///
/// ```rust
/// use mef_common::resilience::{CircuitBreaker, CircuitBreakerConfig};
///
/// let config = CircuitBreakerConfig::default();
/// let breaker = CircuitBreaker::new("database", config);
///
/// // Execute operation through circuit breaker
/// let result = breaker.call(|| {
///     // Potentially failing operation
///     database_query()
/// });
///
/// match result {
///     Ok(data) => { /* Success */ },
///     Err(e) if e.to_string().contains("Circuit breaker is OPEN") => {
///         // Circuit is open, service is down
///     },
///     Err(e) => { /* Other error */ }
/// }
/// ```
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitState>>,
    failure_count: Arc<RwLock<usize>>,
    success_count: Arc<RwLock<usize>>,
    last_failure_time: Arc<RwLock<Option<u64>>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(name: impl Into<String>, config: CircuitBreakerConfig) -> Self {
        Self {
            name: name.into(),
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
        }
    }

    /// Execute a fallible operation through the circuit breaker
    pub fn call<F, T>(&self, operation: F) -> MefResult<T>
    where
        F: FnOnce() -> MefResult<T>,
    {
        // Check current state
        let current_state = *self.state.read();

        match current_state {
            CircuitState::Open => {
                // Check if timeout has elapsed
                if self.should_attempt_reset()? {
                    self.transition_to_half_open();
                } else {
                    return Err(MefError::other(format!(
                        "Circuit breaker '{}' is OPEN",
                        self.name
                    )));
                }
            }
            CircuitState::HalfOpen => {
                // In half-open state, allow limited testing
            }
            CircuitState::Closed => {
                // Normal operation
            }
        }

        // Execute the operation
        match operation() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure()?;
                Err(e)
            }
        }
    }

    /// Handle successful operation
    fn on_success(&self) {
        let current_state = *self.state.read();

        match current_state {
            CircuitState::HalfOpen => {
                let mut success_count = self.success_count.write();
                *success_count += 1;

                tracing::info!(
                    circuit_breaker = %self.name,
                    successes = *success_count,
                    threshold = self.config.success_threshold,
                    "Circuit breaker success in HalfOpen state"
                );

                if *success_count >= self.config.success_threshold {
                    self.transition_to_closed();
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                *self.failure_count.write() = 0;
            }
            CircuitState::Open => {
                // Should not happen, but reset if it does
                *self.failure_count.write() = 0;
            }
        }
    }

    /// Handle failed operation
    fn on_failure(&self) -> MefResult<()> {
        let current_state = *self.state.read();

        match current_state {
            CircuitState::HalfOpen => {
                // Failure in half-open state -> back to open
                tracing::warn!(
                    circuit_breaker = %self.name,
                    "Circuit breaker failed in HalfOpen, returning to Open"
                );
                self.transition_to_open()?;
            }
            CircuitState::Closed => {
                let mut failure_count = self.failure_count.write();
                *failure_count += 1;

                tracing::warn!(
                    circuit_breaker = %self.name,
                    failures = *failure_count,
                    threshold = self.config.failure_threshold,
                    "Circuit breaker failure"
                );

                if *failure_count >= self.config.failure_threshold {
                    drop(failure_count); // Release lock before transition
                    self.transition_to_open()?;
                }
            }
            CircuitState::Open => {
                // Already open, nothing to do
            }
        }

        Ok(())
    }

    /// Check if enough time has passed to attempt reset
    fn should_attempt_reset(&self) -> MefResult<bool> {
        let last_failure = self.last_failure_time.read();

        if let Some(failure_time) = *last_failure {
            let now = current_timestamp()?;
            let elapsed = now.saturating_sub(failure_time);
            Ok(elapsed >= self.config.timeout_seconds)
        } else {
            Ok(false)
        }
    }

    /// Transition to Closed state (normal operation)
    fn transition_to_closed(&self) {
        *self.state.write() = CircuitState::Closed;
        *self.failure_count.write() = 0;
        *self.success_count.write() = 0;

        tracing::info!(
            circuit_breaker = %self.name,
            "Circuit breaker transitioned to CLOSED"
        );
    }

    /// Transition to Open state (failing)
    fn transition_to_open(&self) -> MefResult<()> {
        *self.state.write() = CircuitState::Open;
        *self.success_count.write() = 0;
        *self.last_failure_time.write() = Some(current_timestamp()?);

        tracing::error!(
            circuit_breaker = %self.name,
            timeout_seconds = self.config.timeout_seconds,
            "Circuit breaker transitioned to OPEN"
        );

        Ok(())
    }

    /// Transition to HalfOpen state (testing recovery)
    fn transition_to_half_open(&self) {
        *self.state.write() = CircuitState::HalfOpen;
        *self.success_count.write() = 0;

        tracing::info!(
            circuit_breaker = %self.name,
            "Circuit breaker transitioned to HALF_OPEN"
        );
    }

    /// Get current state
    pub fn state(&self) -> CircuitState {
        *self.state.read()
    }

    /// Get current failure count
    pub fn failure_count(&self) -> usize {
        *self.failure_count.read()
    }

    /// Manually reset the circuit breaker
    pub fn reset(&self) {
        self.transition_to_closed();
    }
}

// ============================================================================
// Health Check System
// ============================================================================

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Component is healthy
    Healthy,
    /// Component is degraded but operational
    Degraded,
    /// Component is unhealthy/failing
    Unhealthy,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub component: String,
    pub status: HealthStatus,
    pub message: String,
    pub timestamp: u64,
    pub details: Option<serde_json::Value>,
}

impl HealthCheck {
    /// Create a healthy check
    pub fn healthy(component: impl Into<String>, message: impl Into<String>) -> MefResult<Self> {
        Ok(Self {
            component: component.into(),
            status: HealthStatus::Healthy,
            message: message.into(),
            timestamp: current_timestamp()?,
            details: None,
        })
    }

    /// Create a degraded check
    pub fn degraded(component: impl Into<String>, message: impl Into<String>) -> MefResult<Self> {
        Ok(Self {
            component: component.into(),
            status: HealthStatus::Degraded,
            message: message.into(),
            timestamp: current_timestamp()?,
            details: None,
        })
    }

    /// Create an unhealthy check
    pub fn unhealthy(component: impl Into<String>, message: impl Into<String>) -> MefResult<Self> {
        Ok(Self {
            component: component.into(),
            status: HealthStatus::Unhealthy,
            message: message.into(),
            timestamp: current_timestamp()?,
            details: None,
        })
    }

    /// Add details to the health check
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Health checker trait
///
/// Implement this trait for components that need health monitoring
pub trait HealthChecker: Send + Sync {
    /// Check the health of the component
    fn check_health(&self) -> MefResult<HealthCheck>;

    /// Get component name
    fn name(&self) -> &str;
}

/// Aggregate health checker for multiple components
pub struct AggregateHealthChecker {
    checkers: Vec<Box<dyn HealthChecker>>,
}

impl AggregateHealthChecker {
    /// Create a new aggregate checker
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    /// Add a health checker
    pub fn add_checker(&mut self, checker: Box<dyn HealthChecker>) {
        self.checkers.push(checker);
    }

    /// Run all health checks
    pub fn check_all(&self) -> MefResult<Vec<HealthCheck>> {
        let mut results = Vec::new();

        for checker in &self.checkers {
            match checker.check_health() {
                Ok(check) => results.push(check),
                Err(e) => {
                    // If health check itself fails, report as unhealthy
                    results.push(HealthCheck::unhealthy(
                        checker.name(),
                        format!("Health check failed: {}", e),
                    )?);
                }
            }
        }

        Ok(results)
    }

    /// Get overall system health status
    pub fn overall_status(&self) -> MefResult<HealthStatus> {
        let checks = self.check_all()?;

        if checks.is_empty() {
            return Ok(HealthStatus::Healthy);
        }

        // Overall status is the worst individual status
        let mut has_degraded = false;

        for check in checks {
            match check.status {
                HealthStatus::Unhealthy => return Ok(HealthStatus::Unhealthy),
                HealthStatus::Degraded => has_degraded = true,
                HealthStatus::Healthy => {}
            }
        }

        if has_degraded {
            Ok(HealthStatus::Degraded)
        } else {
            Ok(HealthStatus::Healthy)
        }
    }
}

impl Default for AggregateHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Auto-Recovery Mechanisms
// ============================================================================

/// Auto-recovery configuration
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximum number of recovery attempts
    pub max_attempts: usize,
    /// Initial backoff delay in milliseconds
    pub initial_backoff_ms: u64,
    /// Maximum backoff delay in milliseconds
    pub max_backoff_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Execute operation with automatic retry and exponential backoff
///
/// # Example
///
/// ```no_run
/// use mef_common::resilience::{auto_recover, RecoveryConfig};
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = RecoveryConfig::default();
/// let result = auto_recover("fetch_data", config, || async {
///     fetch_remote_data().await
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn auto_recover<F, Fut, T>(
    operation_name: &str,
    config: RecoveryConfig,
    mut operation: F,
) -> MefResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = MefResult<T>>,
{
    let mut attempts = 0;
    let mut backoff_ms = config.initial_backoff_ms;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => {
                if attempts > 1 {
                    tracing::info!(
                        operation = operation_name,
                        attempts,
                        "Auto-recovery succeeded"
                    );
                }
                return Ok(result);
            }
            Err(e) => {
                if attempts >= config.max_attempts {
                    tracing::error!(
                        operation = operation_name,
                        attempts,
                        error = %e,
                        "Auto-recovery failed after maximum attempts"
                    );
                    return Err(MefError::other(format!(
                        "Operation '{}' failed after {} attempts: {}",
                        operation_name, config.max_attempts, e
                    )));
                }

                tracing::warn!(
                    operation = operation_name,
                    attempt = attempts,
                    max_attempts = config.max_attempts,
                    backoff_ms,
                    error = %e,
                    "Auto-recovery attempt failed, retrying"
                );

                // Wait before retrying
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;

                // Increase backoff for next iteration
                backoff_ms = ((backoff_ms as f64 * config.backoff_multiplier) as u64)
                    .min(config.max_backoff_ms);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_closed_to_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            timeout_seconds: 1,
            success_threshold: 2,
        };
        let breaker = CircuitBreaker::new("test", config);

        assert_eq!(breaker.state(), CircuitState::Closed);

        // Simulate failures
        for i in 0..3 {
            let result: MefResult<()> = breaker.call(|| Err(MefError::other("test error")));
            assert!(result.is_err());

            if i < 2 {
                assert_eq!(breaker.state(), CircuitState::Closed);
            } else {
                assert_eq!(breaker.state(), CircuitState::Open);
            }
        }
    }

    #[test]
    fn test_circuit_breaker_blocks_when_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            timeout_seconds: 60,
            success_threshold: 2,
        };
        let breaker = CircuitBreaker::new("test", config);

        // Cause failure
        let _: MefResult<()> = breaker.call(|| Err(MefError::other("failure")));
        assert_eq!(breaker.state(), CircuitState::Open);

        // Should block subsequent calls
        let result: MefResult<()> = breaker.call(|| Ok(()));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("OPEN"));
    }

    #[test]
    fn test_circuit_breaker_half_open_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            timeout_seconds: 0, // Immediate transition
            success_threshold: 2,
        };
        let breaker = CircuitBreaker::new("test", config);

        // Cause failure -> Open
        let _: MefResult<()> = breaker.call(|| Err(MefError::other("failure")));
        assert_eq!(breaker.state(), CircuitState::Open);

        // Trigger half-open
        std::thread::sleep(Duration::from_millis(10));

        // First success in half-open
        let _: MefResult<()> = breaker.call(|| Ok(()));
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        // Second success -> Closed
        let _: MefResult<()> = breaker.call(|| Ok(()));
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn test_health_check_creation() -> MefResult<()> {
        let healthy = HealthCheck::healthy("database", "Connected")?;
        assert_eq!(healthy.status, HealthStatus::Healthy);

        let degraded = HealthCheck::degraded("cache", "Slow response")?;
        assert_eq!(degraded.status, HealthStatus::Degraded);

        let unhealthy = HealthCheck::unhealthy("api", "Timeout")?;
        assert_eq!(unhealthy.status, HealthStatus::Unhealthy);

        Ok(())
    }

    struct MockHealthChecker {
        name: String,
        status: HealthStatus,
    }

    impl HealthChecker for MockHealthChecker {
        fn check_health(&self) -> MefResult<HealthCheck> {
            match self.status {
                HealthStatus::Healthy => HealthCheck::healthy(&self.name, "OK"),
                HealthStatus::Degraded => HealthCheck::degraded(&self.name, "Degraded"),
                HealthStatus::Unhealthy => HealthCheck::unhealthy(&self.name, "Failed"),
            }
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_aggregate_health_checker() -> MefResult<()> {
        let mut aggregate = AggregateHealthChecker::new();

        aggregate.add_checker(Box::new(MockHealthChecker {
            name: "db".to_string(),
            status: HealthStatus::Healthy,
        }));

        aggregate.add_checker(Box::new(MockHealthChecker {
            name: "cache".to_string(),
            status: HealthStatus::Healthy,
        }));

        let checks = aggregate.check_all()?;
        assert_eq!(checks.len(), 2);
        assert_eq!(aggregate.overall_status()?, HealthStatus::Healthy);

        Ok(())
    }

    #[tokio::test]
    async fn test_auto_recover_success() -> MefResult<()> {
        let mut attempt = 0;

        let result = auto_recover(
            "test_op",
            RecoveryConfig {
                max_attempts: 3,
                initial_backoff_ms: 10,
                max_backoff_ms: 100,
                backoff_multiplier: 2.0,
            },
            || async {
                attempt += 1;
                if attempt < 2 {
                    Err(MefError::other("temporary failure"))
                } else {
                    Ok(42)
                }
            },
        )
        .await?;

        assert_eq!(result, 42);
        assert_eq!(attempt, 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_auto_recover_max_attempts() {
        let result: MefResult<i32> = auto_recover(
            "test_op",
            RecoveryConfig {
                max_attempts: 3,
                initial_backoff_ms: 10,
                max_backoff_ms: 100,
                backoff_multiplier: 2.0,
            },
            || async { Err(MefError::other("permanent failure")) },
        )
        .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("after 3 attempts"));
    }
}
