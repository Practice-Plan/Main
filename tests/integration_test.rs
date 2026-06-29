//! Integration tests

use base_framework::prelude::*;
use std::sync::Arc;

#[test]
fn test_full_lifecycle() {
    // Create monitor
    let monitor = Monitor::new(MonitorConfig::default());

    // Register application
    let app = AppInfo::new("integration_app", "Integration Test App", "1.0.0");
    assert!(monitor.register_app(app).is_ok());

    // Start application
    assert!(monitor.start_app("integration_app").is_ok());

    // Verify state
    let info = monitor.get_app_info("integration_app").unwrap();
    assert_eq!(info.state, AppState::Running);

    // Record metrics
    for i in 0..10 {
        monitor.record_metric("latency", (i as f64) * 10.0);
    }

    let stats = monitor.get_metric_stats("latency").unwrap();
    assert_eq!(stats.count, 10);

    // Stop application
    assert!(monitor.stop_app("integration_app").is_ok());
    let info = monitor.get_app_info("integration_app").unwrap();
    assert_eq!(info.state, AppState::Stopped);
}

#[test]
fn test_middleware_chain_integration() {
    let chain = MiddlewareChain::new();

    // Configure whitelist
    let whitelist = WhitelistChecker::new();
    whitelist.add_caller("trusted_caller");
    chain.add_checker(Arc::new(whitelist));

    // Configure rate limiting
    let rate_limiter = RateLimiter::new(100, 60000);
    chain.add_checker(Arc::new(rate_limiter));

    // Configure parameter validation
    let param_validator = ParamValidator::new();
    param_validator.add_required_param("sensitive_api", "auth_token");
    chain.add_checker(Arc::new(param_validator));

    // Test: whitelist passed + complete parameters
    let ctx = CheckContext::new("trusted_caller", "sensitive_api")
        .with_param("auth_token", "valid_token");
    let result = chain.check(&ctx).unwrap();
    assert!(result.passed);

    // Test: whitelist rejected
    let ctx = CheckContext::new("unknown_caller", "sensitive_api");
    let result = chain.check(&ctx).unwrap();
    assert!(!result.passed);

    // Test: missing parameters
    let ctx = CheckContext::new("trusted_caller", "sensitive_api");
    let result = chain.check(&ctx).unwrap();
    assert!(!result.passed);
}

#[test]
fn test_concurrent_access() {
    let monitor = Arc::new(Monitor::new(MonitorConfig::default()));
    let mut handles = vec![];

    // Concurrent application registration
    for i in 0..10 {
        let m = monitor.clone();
        let handle = std::thread::spawn(move || {
            let app = AppInfo::new(
                format!("app_{}", i),
                format!("App {}", i),
                "1.0.0",
            );
            m.register_app(app)
        });
        handles.push(handle);
    }

    for handle in handles {
        assert!(handle.join().unwrap().is_ok());
    }

    assert_eq!(monitor.list_apps().len(), 10);

    // Concurrent metric recording
    let mut handles = vec![];
    for i in 0..100 {
        let m = monitor.clone();
        let handle = std::thread::spawn(move || {
            m.record_metric("concurrent_metric", i as f64);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let stats = monitor.get_metric_stats("concurrent_metric").unwrap();
    assert_eq!(stats.count, 100);
}

#[test]
fn test_error_handling() {
    let monitor = Monitor::new(MonitorConfig::default());

    // Start non-existent application
    let result = monitor.start_app("nonexistent");
    assert!(result.is_err());

    // Duplicate registration
    let app = AppInfo::new("dup_app", "Dup App", "1.0.0");
    monitor.register_app(app.clone()).unwrap();
    let result = monitor.register_app(app);
    assert!(result.is_err());
}
