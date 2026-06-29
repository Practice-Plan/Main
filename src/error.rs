//! Error handling module
//! 
//! Provides unified error types and error handling mechanisms

use thiserror::Error;
use std::fmt;

/// Main framework error type
#[derive(Error, Debug)]
pub enum FrameworkError {
    /// Initialization error
    #[error("Initialization error: {0}")]
    InitError(String),

    /// Monitoring error
    #[error("Monitor error: {0}")]
    MonitorError(String),

    /// Middleware error
    #[error("Middleware error: {0}")]
    MiddlewareError(String),

    /// Interface call error
    #[error("Interface error: {0}")]
    InterfaceError(String),

    /// Configuration error
    #[error("Config error: {0}")]
    ConfigError(String),

    /// System error
    #[error("System error: {0}")]
    SystemError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Error code definitions
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// Success
    Success = 0,
    /// Initialization failed
    InitFailed = -1,
    /// Invalid parameter
    InvalidParam = -2,
    /// Monitoring failed
    MonitorFailed = -3,
    /// Middleware check failed
    MiddlewareCheckFailed = -4,
    /// Out of resources
    OutOfResource = -5,
    /// Timeout
    Timeout = -6,
    /// Not implemented
    NotImplemented = -7,
    /// Internal error
    InternalError = -99,
}

impl From<&FrameworkError> for ErrorCode {
    fn from(err: &FrameworkError) -> Self {
        match err {
            FrameworkError::InitError(_) => ErrorCode::InitFailed,
            FrameworkError::MonitorError(_) => ErrorCode::MonitorFailed,
            FrameworkError::MiddlewareError(_) => ErrorCode::MiddlewareCheckFailed,
            FrameworkError::InterfaceError(_) => ErrorCode::InvalidParam,
            FrameworkError::ConfigError(_) => ErrorCode::InitFailed,
            FrameworkError::SystemError(_) => ErrorCode::InternalError,
            FrameworkError::Unknown(_) => ErrorCode::InternalError,
        }
    }
}

impl From<ErrorCode> for i32 {
    fn from(code: ErrorCode) -> Self {
        code as i32
    }
}

/// Framework result type
pub type FrameworkResult<T> = Result<T, FrameworkError>;

/// Error context
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub code: ErrorCode,
    pub message: String,
    pub timestamp: i64,
    pub module: String,
}

impl ErrorContext {
    pub fn new(code: ErrorCode, message: impl Into<String>, module: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            timestamp: chrono::Utc::now().timestamp(),
            module: module.into(),
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - {} (code: {:?})",
            self.module, self.timestamp, self.message, self.code
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = FrameworkError::InitError("test error".to_string());
        assert_eq!(format!("{}", err), "Initialization error: test error");
    }

    #[test]
    fn test_error_code_mapping() {
        let err = FrameworkError::MonitorError("monitor failed".to_string());
        let code = ErrorCode::from(&err);
        assert_eq!(code, ErrorCode::MonitorFailed);
    }

    #[test]
    fn test_error_context() {
        let ctx = ErrorContext::new(ErrorCode::InitFailed, "test", "test_module");
        assert_eq!(ctx.code, ErrorCode::InitFailed);
        assert_eq!(ctx.module, "test_module");
    }
}
