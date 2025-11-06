//! Time and timestamp utilities
//!
//! Provides safe, tested implementations of common time operations.
//! Eliminates 27+ duplications of timestamp patterns across the codebase.

use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in seconds since UNIX epoch
///
/// # Errors
///
/// Returns error if system time is before UNIX epoch (should never happen in practice)
///
/// # Example
///
/// ```
/// use mef_common::time::current_timestamp;
///
/// let now = current_timestamp().expect("Failed to get current time");
/// assert!(now > 0);
/// ```
pub fn current_timestamp() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time is before UNIX epoch")?
        .as_secs()
        .pipe(Ok)
}

/// Get current timestamp in milliseconds since UNIX epoch
///
/// # Errors
///
/// Returns error if system time is before UNIX epoch
///
/// # Example
///
/// ```
/// use mef_common::time::current_timestamp_millis;
///
/// let now_ms = current_timestamp_millis().expect("Failed to get current time");
/// assert!(now_ms > 0);
/// ```
pub fn current_timestamp_millis() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time is before UNIX epoch")?
        .as_millis()
        .try_into()
        .context("Timestamp overflow")
}

/// Get current timestamp, falling back to a default value on error
///
/// This should only be used in non-critical paths where timestamp accuracy
/// is not essential. For production code, prefer `current_timestamp()` with
/// proper error handling.
///
/// # Example
///
/// ```
/// use mef_common::time::current_timestamp_or_default;
///
/// let now = current_timestamp_or_default();
/// assert!(now > 0);
/// ```
pub fn current_timestamp_or_default() -> u64 {
    current_timestamp().unwrap_or(0)
}

/// Calculate time elapsed since a given timestamp
///
/// # Errors
///
/// Returns error if current time cannot be determined or if elapsed time
/// calculation overflows
pub fn elapsed_since(timestamp: u64) -> Result<u64> {
    let now = current_timestamp()?;
    now.checked_sub(timestamp)
        .context("Timestamp is in the future")
}

/// Check if a timestamp has expired based on TTL (time-to-live)
///
/// # Arguments
///
/// * `timestamp` - The starting timestamp in seconds
/// * `ttl` - Time-to-live in seconds
///
/// # Returns
///
/// Returns `Ok(true)` if the timestamp has expired, `Ok(false)` otherwise
///
/// # Example
///
/// ```
/// use mef_common::time::{current_timestamp, has_expired};
///
/// let now = current_timestamp().unwrap();
/// let expired = has_expired(now - 100, 50).unwrap();
/// assert!(expired);
///
/// let not_expired = has_expired(now - 10, 100).unwrap();
/// assert!(!not_expired);
/// ```
pub fn has_expired(timestamp: u64, ttl: u64) -> Result<bool> {
    let elapsed = elapsed_since(timestamp)?;
    Ok(elapsed >= ttl)
}

/// Trait extension for adding pipe functionality to types
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp().expect("Failed to get timestamp");
        assert!(ts > 1_600_000_000); // After Sept 2020
        assert!(ts < 2_000_000_000); // Before May 2033
    }

    #[test]
    fn test_current_timestamp_millis() {
        let ts_ms = current_timestamp_millis().expect("Failed to get timestamp");
        assert!(ts_ms > 1_600_000_000_000); // After Sept 2020
    }

    #[test]
    fn test_current_timestamp_or_default() {
        let ts = current_timestamp_or_default();
        assert!(ts > 0);
    }

    #[test]
    fn test_elapsed_since() {
        let now = current_timestamp().unwrap();
        let past = now - 100;

        let elapsed = elapsed_since(past).expect("Failed to calculate elapsed time");
        assert!(elapsed >= 100);
        assert!(elapsed < 110); // Allow for small timing variations
    }

    #[test]
    fn test_has_expired() {
        let now = current_timestamp().unwrap();

        // Expired case
        let past_expired = now - 100;
        assert!(has_expired(past_expired, 50).unwrap());

        // Not expired case
        let past_not_expired = now - 10;
        assert!(!has_expired(past_not_expired, 100).unwrap());
    }

    #[test]
    fn test_elapsed_since_future_timestamp() {
        let now = current_timestamp().unwrap();
        let future = now + 1000;

        let result = elapsed_since(future);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("future"));
    }
}
