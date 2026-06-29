//! Base Framework
//! 
//! A high-performance, low-overhead base framework providing application monitoring,
//! middleware inspection, and external interface invocation capabilities.
//! 
//! # Main Features
//! 
//! - **Cross-platform compatibility**: Supports Windows, Linux, macOS
//! - **Dynamic link library**: Provides C ABI compatible FFI interfaces
//! - **Application monitoring**: Real-time monitoring of application status and performance metrics
//! - **Middleware inspection**: Configurable call inspection chain
//! - **Error handling**: Unified error types and error codes
//! - **Logging**: Multi-level, multi-format logging system
//! 
//! # Module Structure
//! 
//! - [`error`]: Error handling module
//! - [`logger`]: Logging module
//! - [`monitor`]: Application monitoring module
//! - [`middleware`]: Middleware inspection layer module
//! - [`interface`]: FFI interface definition layer
//! 
//! # Quick Start
//! 
//! ## Rust Usage
//! 
//! ```rust
//! use base_framework::prelude::*;
//! 
//! // Initialize framework
//! let config = FrameworkConfig::default();
//! let framework = Framework::new(config)?;
//! 
//! // Register application
//! let app = AppInfo::new("app001", "My App", "1.0.0");
//! framework.monitor().register_app(app)?;
//! 
//! // Start monitoring
//! framework.monitor().start_app("app001")?;
//! 
//! // Execute inspection
//! let ctx = CheckContext::new("caller001", "get_data");
//! let result = framework.middleware().check(&ctx)?;
//! 
//! if result.passed {
//!     println!("Inspection passed");
//! }
//! ```
//! 
//! ## C/C++ Usage
//! 
//! ```c
//! #include "base_framework.h"
//! 
//! int main() {
//!     // Initialize framework
//!     int result = framework_init(NULL);
//!     if (result != 0) {
//!         printf("Initialization failed: %d\n", result);
//!         return -1;
//!     }
//! 
//!     // Register application
//!     result = framework_register_app("app001", "My App", "1.0.0");
//!     if (result != 0) {
//!         printf("Application registration failed: %d\n", result);
//!         return -1;
//!     }
//! 
//!     // Start application
//!     result = framework_start_app("app001");
//!     if (result != 0) {
//!         printf("Application start failed: %d\n", result);
//!         return -1;
//!     }
//! 
//!     // Execute call inspection
//!     result = framework_check_call("caller001", "get_data", "{}");
//!     if (result == 0) {
//!         printf("Inspection passed, executing operation\n");
//!     } else {
//!         printf("Inspection failed: %d\n", result);
//!     }
//! 
//!     // Shutdown framework
//!     framework_shutdown();
//!     return 0;
//! }
//! ```
//! 
//! # Performance Characteristics
//! 
//! - Lock-free design: High-performance concurrency using `parking_lot`
//! - Zero-copy: FFI interfaces avoid unnecessary memory allocation
//! - Low latency: Optimized inspection chain execution path
//! - Configurable: Adjust buffer sizes and sampling rates as needed

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod error;
pub mod logger;
pub mod monitor;
pub mod middleware;
pub mod interface;
pub mod cli;
pub mod database;
pub mod dll_loader;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::{FrameworkError, FrameworkResult, ErrorCode};
    pub use crate::logger::{LoggerConfig, init_logger, init_default_logger};
    pub use crate::middleware::{
        CheckContext, CheckResult, Checker, MiddlewareChain,
        WhitelistChecker, RateLimiter, ParamValidator,
    };
    pub use crate::monitor::{
        Monitor, MonitorConfig, MonitorEvent, AppInfo, AppState, MetricStats,
    };
    pub use crate::database::{Database, CommandMapping};
    pub use crate::dll_loader::DllLoader;
}

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get framework version
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
