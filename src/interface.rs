//! Interface Definition Layer (FFI)
//! 
//! Provides C ABI compatible interfaces for external programs (e.g., test.dll) to call
//! All interfaces use `extern "C"` to ensure cross-language compatibility

use crate::error::ErrorCode;
use crate::middleware::{CheckContext, MiddlewareChain, WhitelistChecker, RateLimiter, ParamValidator};
use crate::monitor::{Monitor, MonitorConfig, AppInfo};
use crate::logger::{LoggerConfig, init_logger};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::Once;

/// Global framework instance
static INIT: Once = Once::new();
static mut FRAMEWORK: Option<Arc<FrameworkInstance>> = None;

/// Framework instance
struct FrameworkInstance {
    monitor: Monitor,
    middleware: MiddlewareChain,
}

/// Initialize framework
/// 
/// # Parameters
/// - `config_json`: JSON format configuration string
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
/// 
/// # Example
/// ```c
/// const char* config = "{\"log_level\": \"info\"}";
/// int result = framework_init(config);
/// ```
#[no_mangle]
pub extern "C" fn framework_init(config_json: *const c_char) -> i32 {
    let result = std::panic::catch_unwind(|| {
        INIT.call_once(|| {
            // Parse configuration
            let config = if config_json.is_null() {
                LoggerConfig::default()
            } else {
                // Simplified processing, use default configuration
                LoggerConfig::default()
            };

            // Initialize logger
            if let Err(e) = init_logger(config) {
                log::error!("Failed to init logger: {}", e);
            }

            // Create monitor
            let monitor = Monitor::new(MonitorConfig::default());

            // Create middleware chain
            let middleware = MiddlewareChain::new();

            // Add default checkers
            let whitelist = WhitelistChecker::new();
            middleware.add_checker(Arc::new(whitelist));
            middleware.add_checker(Arc::new(RateLimiter::new(1000, 60000)));
            middleware.add_checker(Arc::new(ParamValidator::new()));

            let instance = Arc::new(FrameworkInstance { monitor, middleware });

            unsafe {
                FRAMEWORK = Some(instance);
            }

            log::info!("Framework initialized successfully");
        });

        ErrorCode::Success as i32
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Shutdown framework
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
#[no_mangle]
pub extern "C" fn framework_shutdown() -> i32 {
    let result = std::panic::catch_unwind(|| {
        unsafe {
            if let Some(instance) = FRAMEWORK.take() {
                drop(instance);
                log::info!("Framework shutdown");
            }
        }
        ErrorCode::Success as i32
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Register application
/// 
/// # Parameters
/// - `app_id`: Application ID (C string)
/// - `app_name`: Application name (C string)
/// - `version`: Version number (C string)
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
/// 
/// # Example
/// ```c
/// int result = framework_register_app("app001", "Test Application", "1.0.0");
/// ```
#[no_mangle]
pub extern "C" fn framework_register_app(
    app_id: *const c_char,
    app_name: *const c_char,
    version: *const c_char,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if app_id.is_null() || app_name.is_null() || version.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let id = unsafe { CStr::from_ptr(app_id) }.to_string_lossy();
        let name = unsafe { CStr::from_ptr(app_name) }.to_string_lossy();
        let ver = unsafe { CStr::from_ptr(version) }.to_string_lossy();

        let app = AppInfo::new(id.as_ref(), name.as_ref(), ver.as_ref());

        match instance.monitor.register_app(app) {
            Ok(_) => {
                log::info!("App registered: {}", id);
                ErrorCode::Success as i32
            }
            Err(e) => {
                log::error!("Failed to register app: {}", e);
                ErrorCode::MonitorFailed as i32
            }
        }
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Start application monitoring
/// 
/// # Parameters
/// - `app_id`: Application ID (C string)
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
#[no_mangle]
pub extern "C" fn framework_start_app(app_id: *const c_char) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if app_id.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let id = unsafe { CStr::from_ptr(app_id) }.to_string_lossy();

        match instance.monitor.start_app(&id) {
            Ok(_) => ErrorCode::Success as i32,
            Err(e) => {
                log::error!("Failed to start app: {}", e);
                ErrorCode::MonitorFailed as i32
            }
        }
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Stop application monitoring
/// 
/// # Parameters
/// - `app_id`: Application ID (C string)
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
#[no_mangle]
pub extern "C" fn framework_stop_app(app_id: *const c_char) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if app_id.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let id = unsafe { CStr::from_ptr(app_id) }.to_string_lossy();

        match instance.monitor.stop_app(&id) {
            Ok(_) => ErrorCode::Success as i32,
            Err(e) => {
                log::error!("Failed to stop app: {}", e);
                ErrorCode::MonitorFailed as i32
            }
        }
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Execute call inspection
/// 
/// # Parameters
/// - `caller_id`: Caller ID (C string)
/// - `interface_name`: Interface name (C string)
/// - `params_json`: Parameter JSON string (optional)
/// 
/// # Returns
/// - `0`: Inspection passed
/// - Negative: Inspection failed or error
/// 
/// # Example
/// ```c
/// int result = framework_check_call("caller001", "get_data", "{}");
/// if (result == 0) {
///     // Inspection passed, execute operation
/// }
/// ```
#[no_mangle]
pub extern "C" fn framework_check_call(
    caller_id: *const c_char,
    interface_name: *const c_char,
    params_json: *const c_char,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if caller_id.is_null() || interface_name.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let caller = unsafe { CStr::from_ptr(caller_id) }.to_string_lossy();
        let interface = unsafe { CStr::from_ptr(interface_name) }.to_string_lossy();

        let ctx = CheckContext::new(caller.as_ref(), interface.as_ref());

        // Parse parameters (simplified processing)
        if !params_json.is_null() {
            let _params = unsafe { CStr::from_ptr(params_json) }.to_string_lossy();
            // JSON parsing logic can be added here
        }

        match instance.middleware.check(&ctx) {
            Ok(check_result) => {
                if check_result.passed {
                    log::debug!(
                        "Check passed for {} -> {}: {} ({:.2}ms)",
                        caller,
                        interface,
                        check_result.message,
                        check_result.duration_ms
                    );
                    ErrorCode::Success as i32
                } else {
                    log::warn!(
                        "Check failed for {} -> {}: {}",
                        caller,
                        interface,
                        check_result.message
                    );
                    ErrorCode::MiddlewareCheckFailed as i32
                }
            }
            Err(e) => {
                log::error!("Check error: {}", e);
                ErrorCode::MiddlewareCheckFailed as i32
            }
        }
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Record performance metric
/// 
/// # Parameters
/// - `metric_name`: Metric name (C string)
/// - `value`: Metric value
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
#[no_mangle]
pub extern "C" fn framework_record_metric(
    metric_name: *const c_char,
    value: f64,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if metric_name.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let name = unsafe { CStr::from_ptr(metric_name) }.to_string_lossy();
        instance.monitor.record_metric(name.as_ref(), value);

        ErrorCode::Success as i32
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Record error
/// 
/// # Parameters
/// - `error_code`: Error code
/// - `message`: Error message (C string)
/// 
/// # Returns
/// - `0`: Success
/// - Negative: Error code
#[no_mangle]
pub extern "C" fn framework_record_error(error_code: i32, message: *const c_char) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if message.is_null() {
            return ErrorCode::InvalidParam as i32;
        }

        let instance = unsafe {
            match &FRAMEWORK {
                Some(fw) => fw,
                None => return ErrorCode::InitFailed as i32,
            }
        };

        let msg = unsafe { CStr::from_ptr(message) }.to_string_lossy();
        instance.monitor.record_error(error_code, msg.as_ref());

        ErrorCode::Success as i32
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

/// Get framework version
/// 
/// # Returns
/// Version string (caller needs to copy, do not free)
#[no_mangle]
pub extern "C" fn framework_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

/// Free string
/// 
/// # Parameters
/// - `ptr`: String pointer to free
#[no_mangle]
pub extern "C" fn framework_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Health check
/// 
/// # Returns
/// - `0`: Framework is healthy
/// - Negative: Framework anomaly
#[no_mangle]
pub extern "C" fn framework_health_check() -> i32 {
    let result = std::panic::catch_unwind(|| {
        unsafe {
            if FRAMEWORK.is_some() {
                ErrorCode::Success as i32
            } else {
                ErrorCode::InitFailed as i32
            }
        }
    });

    result.unwrap_or(ErrorCode::InternalError as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_init_shutdown() {
        let result = framework_init(ptr::null());
        assert_eq!(result, 0);

        let result = framework_health_check();
        assert_eq!(result, 0);

        let result = framework_shutdown();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ffi_register_app() {
        framework_init(ptr::null());

        let app_id = CString::new("test_app").unwrap();
        let app_name = CString::new("Test App").unwrap();
        let version = CString::new("1.0.0").unwrap();

        let result = framework_register_app(
            app_id.as_ptr(),
            app_name.as_ptr(),
            version.as_ptr(),
        );
        assert_eq!(result, 0);

        framework_shutdown();
    }

    #[test]
    fn test_ffi_null_params() {
        framework_init(ptr::null());

        let result = framework_register_app(ptr::null(), ptr::null(), ptr::null());
        assert_eq!(result, ErrorCode::InvalidParam as i32);

        framework_shutdown();
    }
}
