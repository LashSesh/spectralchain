//! Safe concurrency primitives and patterns
//!
//! Provides safe wrappers around std::sync primitives that handle poisoning
//! and provide better error messages. Eliminates 66+ unsafe RwLock unwrap calls.

use crate::error::{MefError, MefResult};
use parking_lot::{RwLock as ParkingLotRwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

/// A safe wrapper around RwLock that never panics
///
/// Uses parking_lot::RwLock which doesn't poison, making it safer for production use.
///
/// # Example
///
/// ```
/// use mef_common::concurrency::SafeRwLock;
///
/// let lock = SafeRwLock::new(42);
///
/// // Reading
/// {
///     let value = lock.read();
///     assert_eq!(*value, 42);
/// }
///
/// // Writing
/// {
///     let mut value = lock.write();
///     *value = 100;
/// }
///
/// assert_eq!(*lock.read(), 100);
/// ```
#[derive(Debug)]
pub struct SafeRwLock<T> {
    inner: ParkingLotRwLock<T>,
}

impl<T> SafeRwLock<T> {
    /// Create a new SafeRwLock
    pub fn new(value: T) -> Self {
        Self {
            inner: ParkingLotRwLock::new(value),
        }
    }

    /// Create a new SafeRwLock wrapped in Arc for sharing
    pub fn new_arc(value: T) -> Arc<Self> {
        Arc::new(Self::new(value))
    }

    /// Acquire a read lock
    ///
    /// This will block until the lock is available. parking_lot locks
    /// don't poison, so this is always safe.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read()
    }

    /// Acquire a write lock
    ///
    /// This will block until the lock is available.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write()
    }

    /// Try to acquire a read lock without blocking
    ///
    /// Returns None if the lock cannot be acquired immediately.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        self.inner.try_read()
    }

    /// Try to acquire a write lock without blocking
    ///
    /// Returns None if the lock cannot be acquired immediately.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        self.inner.try_write()
    }

    /// Get a mutable reference to the inner value
    ///
    /// This is safe because we have exclusive access to self.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consume the lock and return the inner value
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: Clone> SafeRwLock<T> {
    /// Clone the inner value
    pub fn clone_inner(&self) -> T {
        self.read().clone()
    }
}

impl<T: Default> Default for SafeRwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Extension trait for std::sync::RwLock to provide safe operations
///
/// This allows gradual migration from std::sync::RwLock to SafeRwLock.
pub trait SafeRwLockExt<T> {
    /// Safely read from RwLock, converting poison errors to MefError
    fn safe_read(&self) -> MefResult<std::sync::RwLockReadGuard<'_, T>>;

    /// Safely write to RwLock, converting poison errors to MefError
    fn safe_write(&self) -> MefResult<std::sync::RwLockWriteGuard<'_, T>>;
}

impl<T> SafeRwLockExt<T> for std::sync::RwLock<T> {
    fn safe_read(&self) -> MefResult<std::sync::RwLockReadGuard<'_, T>> {
        self.read()
            .map_err(|e| MefError::concurrency(format!("RwLock poisoned during read: {}", e)))
    }

    fn safe_write(&self) -> MefResult<std::sync::RwLockWriteGuard<'_, T>> {
        self.write()
            .map_err(|e| MefError::concurrency(format!("RwLock poisoned during write: {}", e)))
    }
}

/// Retry a fallible operation with exponential backoff
///
/// # Arguments
///
/// * `max_attempts` - Maximum number of retry attempts
/// * `initial_delay_ms` - Initial delay in milliseconds (doubles each retry)
/// * `operation` - The operation to retry
///
/// # Example
///
/// ```no_run
/// use mef_common::concurrency::retry_with_backoff;
/// use mef_common::error::MefResult;
///
/// async fn flaky_operation() -> MefResult<i32> {
///     // Simulated network operation that might fail
///     Ok(42)
/// }
///
/// # tokio_test::block_on(async {
/// let result = retry_with_backoff(3, 100, || async {
///     flaky_operation().await
/// }).await;
/// # });
/// ```
pub async fn retry_with_backoff<F, Fut, T>(
    max_attempts: usize,
    initial_delay_ms: u64,
    mut operation: F,
) -> MefResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = MefResult<T>>,
{
    let mut attempts = 0;
    let mut delay_ms = initial_delay_ms;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts >= max_attempts => {
                return Err(MefError::other(format!(
                    "Operation failed after {} attempts: {}",
                    max_attempts, e
                )));
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                delay_ms *= 2; // Exponential backoff
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_rwlock_basic() {
        let lock = SafeRwLock::new(42);

        // Read
        assert_eq!(*lock.read(), 42);

        // Write
        *lock.write() = 100;
        assert_eq!(*lock.read(), 100);
    }

    #[test]
    fn test_safe_rwlock_concurrent() {
        use std::sync::Arc;
        use std::thread;

        let lock = Arc::new(SafeRwLock::new(0));
        let mut handles = vec![];

        // Spawn 10 threads that each increment the counter
        for _ in 0..10 {
            let lock_clone = Arc::clone(&lock);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut value = lock_clone.write();
                    *value += 1;
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*lock.read(), 1000);
    }

    #[test]
    fn test_try_read_write() {
        let lock = SafeRwLock::new(42);

        // Should be able to acquire read lock
        let read_guard = lock.try_read();
        assert!(read_guard.is_some());

        // Should not be able to acquire write lock while read is held
        let write_guard = lock.try_write();
        assert!(write_guard.is_none());

        drop(read_guard);

        // Now should be able to acquire write lock
        let write_guard = lock.try_write();
        assert!(write_guard.is_some());
    }

    #[test]
    fn test_clone_inner() {
        let lock = SafeRwLock::new(vec![1, 2, 3]);
        let cloned = lock.clone_inner();
        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_retry_with_backoff_success() {
        let mut attempts = 0;

        let result = retry_with_backoff(3, 10, || {
            attempts += 1;
            async move {
                if attempts < 2 {
                    Err(MefError::other("temporary failure"))
                } else {
                    Ok(42)
                }
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 2);
    }

    #[tokio::test]
    async fn test_retry_with_backoff_failure() {
        let result = retry_with_backoff(3, 10, || async {
            Err::<i32, _>(MefError::other("permanent failure"))
        })
        .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("after 3 attempts"));
    }
}
