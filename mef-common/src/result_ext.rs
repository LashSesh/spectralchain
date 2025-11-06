//! Extension traits for Result types
//!
//! Provides convenient methods for working with Result types without unwrapping.

use crate::error::{MefError, MefResult};

/// Extension trait for Result types providing safer alternatives to unwrap
pub trait ResultExt<T, E> {
    /// Unwrap with a custom error message that provides context
    ///
    /// This is safer than unwrap() as it provides better error messages,
    /// but should still only be used where panicking is acceptable.
    fn expect_or_log(self, msg: &str) -> T;

    /// Convert any error type to MefError::Other
    fn map_to_mef_error(self) -> MefResult<T>
    where
        E: std::fmt::Display;

    /// Convert error with a custom message
    fn map_err_msg(self, msg: &str) -> MefResult<T>
    where
        E: std::fmt::Display;

    /// Log error and return default value
    fn unwrap_or_log_default(self) -> T
    where
        T: Default,
        E: std::fmt::Display;

    /// Log error and return provided default value
    fn unwrap_or_log(self, default: T) -> T
    where
        E: std::fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn expect_or_log(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => {
                tracing::error!("{}", msg);
                panic!("{}", msg)
            }
        }
    }

    fn map_to_mef_error(self) -> MefResult<T>
    where
        E: std::fmt::Display,
    {
        self.map_err(|e| MefError::other(e.to_string()))
    }

    fn map_err_msg(self, msg: &str) -> MefResult<T>
    where
        E: std::fmt::Display,
    {
        self.map_err(|e| MefError::other(format!("{}: {}", msg, e)))
    }

    fn unwrap_or_log_default(self) -> T
    where
        T: Default,
        E: std::fmt::Display,
    {
        match self {
            Ok(value) => value,
            Err(e) => {
                tracing::warn!("Error occurred, using default value: {}", e);
                T::default()
            }
        }
    }

    fn unwrap_or_log(self, default: T) -> T
    where
        E: std::fmt::Display,
    {
        match self {
            Ok(value) => value,
            Err(e) => {
                tracing::warn!("Error occurred, using fallback value: {}", e);
                default
            }
        }
    }
}

/// Extension trait for Option types
pub trait OptionExt<T> {
    /// Convert Option to Result with a custom error
    fn ok_or_mef_error(self, error: MefError) -> MefResult<T>;

    /// Convert Option to Result with a custom error message
    fn ok_or_msg(self, msg: &str) -> MefResult<T>;

    /// Unwrap or log and return default
    fn unwrap_or_log_default(self) -> T
    where
        T: Default;

    /// Unwrap or log and return provided default
    fn unwrap_or_log(self, default: T) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_mef_error(self, error: MefError) -> MefResult<T> {
        self.ok_or(error)
    }

    fn ok_or_msg(self, msg: &str) -> MefResult<T> {
        self.ok_or_else(|| MefError::other(msg.to_string()))
    }

    fn unwrap_or_log_default(self) -> T
    where
        T: Default,
    {
        match self {
            Some(value) => value,
            None => {
                tracing::warn!("None encountered, using default value");
                T::default()
            }
        }
    }

    fn unwrap_or_log(self, default: T) -> T {
        match self {
            Some(value) => value,
            None => {
                tracing::warn!("None encountered, using fallback value");
                default
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_mef_error() {
        let result: Result<i32, &str> = Err("test error");
        let mef_result = result.map_to_mef_error();
        assert!(mef_result.is_err());
        assert_eq!(mef_result.unwrap_err().to_string(), "test error");
    }

    #[test]
    fn test_map_err_msg() {
        let result: Result<i32, &str> = Err("original");
        let mef_result = result.map_err_msg("context");
        assert!(mef_result.is_err());
        let err_msg = mef_result.unwrap_err().to_string();
        assert!(err_msg.contains("context"));
        assert!(err_msg.contains("original"));
    }

    #[test]
    fn test_unwrap_or_log_default() {
        let ok_result: Result<i32, &str> = Ok(42);
        assert_eq!(ok_result.unwrap_or_log_default(), 42);

        let err_result: Result<i32, &str> = Err("error");
        assert_eq!(err_result.unwrap_or_log_default(), 0); // i32::default()
    }

    #[test]
    fn test_unwrap_or_log() {
        let ok_result: Result<i32, &str> = Ok(42);
        assert_eq!(ok_result.unwrap_or_log(100), 42);

        let err_result: Result<i32, &str> = Err("error");
        assert_eq!(err_result.unwrap_or_log(100), 100);
    }

    #[test]
    fn test_option_ok_or_msg() {
        let some_value: Option<i32> = Some(42);
        assert!(some_value.ok_or_msg("error").is_ok());

        let none_value: Option<i32> = None;
        let result = none_value.ok_or_msg("not found");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "not found");
    }

    #[test]
    fn test_option_unwrap_or_log_default() {
        let some_value: Option<i32> = Some(42);
        assert_eq!(some_value.unwrap_or_log_default(), 42);

        let none_value: Option<i32> = None;
        assert_eq!(none_value.unwrap_or_log_default(), 0);
    }
}
