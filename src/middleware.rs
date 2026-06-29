//! Middleware inspection layer module
//! 
//! Provides call inspection, permission verification, and request filtering functionality

use crate::error::FrameworkResult;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

/// Inspection context
#[derive(Debug, Clone)]
pub struct CheckContext {
    /// Caller ID
    pub caller_id: String,
    /// Target interface
    pub target_interface: String,
    /// Call parameters
    pub params: HashMap<String, String>,
    /// Timestamp
    pub timestamp: i64,
    /// Source IP (optional)
    pub source_ip: Option<String>,
}

impl CheckContext {
    pub fn new(caller_id: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            caller_id: caller_id.into(),
            target_interface: target.into(),
            params: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
            source_ip: None,
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn with_source_ip(mut self, ip: impl Into<String>) -> Self {
        self.source_ip = Some(ip.into());
        self
    }
}

/// Inspection result
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Whether passed
    pub passed: bool,
    /// Inspection message
    pub message: String,
    /// Inspection duration (milliseconds)
    pub duration_ms: f64,
}

impl CheckResult {
    pub fn success(message: impl Into<String>, duration_ms: f64) -> Self {
        Self {
            passed: true,
            message: message.into(),
            duration_ms,
        }
    }

    pub fn failure(message: impl Into<String>, duration_ms: f64) -> Self {
        Self {
            passed: false,
            message: message.into(),
            duration_ms,
        }
    }
}

/// Checker trait
pub trait Checker: Send + Sync {
    /// Checker name
    fn name(&self) -> &str;
    
    /// Execute check
    fn check(&self, ctx: &CheckContext) -> FrameworkResult<CheckResult>;
    
    /// Checker priority (lower number = higher priority)
    fn priority(&self) -> u32 {
        100
    }
}

/// Whitelist checker
pub struct WhitelistChecker {
    allowed_callers: RwLock<HashSet<String>>,
}

impl WhitelistChecker {
    pub fn new() -> Self {
        Self {
            allowed_callers: RwLock::new(HashSet::new()),
        }
    }

    pub fn add_caller(&self, caller_id: impl Into<String>) {
        self.allowed_callers.write().insert(caller_id.into());
    }

    pub fn remove_caller(&self, caller_id: &str) {
        self.allowed_callers.write().remove(caller_id);
    }
}

impl Default for WhitelistChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for WhitelistChecker {
    fn name(&self) -> &str {
        "whitelist_checker"
    }

    fn check(&self, ctx: &CheckContext) -> FrameworkResult<CheckResult> {
        let start = Instant::now();
        let allowed = self.allowed_callers.read().contains(&ctx.caller_id);
        let duration = start.elapsed().as_secs_f64() * 1000.0;

        if allowed {
            Ok(CheckResult::success("Caller is whitelisted", duration))
        } else {
            Ok(CheckResult::failure("Caller not in whitelist", duration))
        }
    }

    fn priority(&self) -> u32 {
        10
    }
}

/// Rate limiter
pub struct RateLimiter {
    max_calls: usize,
    window_ms: u64,
    call_records: RwLock<HashMap<String, Vec<i64>>>,
}

impl RateLimiter {
    pub fn new(max_calls: usize, window_ms: u64) -> Self {
        Self {
            max_calls,
            window_ms,
            call_records: RwLock::new(HashMap::new()),
        }
    }

    fn cleanup_old_records(&self, caller_id: &str, now: i64) {
        let mut records = self.call_records.write();
        if let Some(calls) = records.get_mut(caller_id) {
            let cutoff = now - self.window_ms as i64;
            calls.retain(|&t| t > cutoff);
        }
    }
}

impl Checker for RateLimiter {
    fn name(&self) -> &str {
        "rate_limiter"
    }

    fn check(&self, ctx: &CheckContext) -> FrameworkResult<CheckResult> {
        let start = Instant::now();
        let now = ctx.timestamp;
        
        self.cleanup_old_records(&ctx.caller_id, now);
        
        let mut records = self.call_records.write();
        let calls = records.entry(ctx.caller_id.clone()).or_insert_with(Vec::new);
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        
        if calls.len() >= self.max_calls {
            Ok(CheckResult::failure(
                format!("Rate limit exceeded: {} calls per {} ms", self.max_calls, self.window_ms),
                duration,
            ))
        } else {
            calls.push(now);
            Ok(CheckResult::success("Rate limit check passed", duration))
        }
    }

    fn priority(&self) -> u32 {
        20
    }
}

/// Parameter validator
pub struct ParamValidator {
    required_params: RwLock<HashMap<String, Vec<String>>>,
}

impl ParamValidator {
    pub fn new() -> Self {
        Self {
            required_params: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_required_param(&self, interface: impl Into<String>, param: impl Into<String>) {
        let interface = interface.into();
        let param = param.into();
        let mut params = self.required_params.write();
        params.entry(interface).or_insert_with(Vec::new).push(param);
    }
}

impl Default for ParamValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for ParamValidator {
    fn name(&self) -> &str {
        "param_validator"
    }

    fn check(&self, ctx: &CheckContext) -> FrameworkResult<CheckResult> {
        let start = Instant::now();
        let required = self.required_params.read();
        
        if let Some(params) = required.get(&ctx.target_interface) {
            for param in params {
                if !ctx.params.contains_key(param) {
                    let duration = start.elapsed().as_secs_f64() * 1000.0;
                    return Ok(CheckResult::failure(
                        format!("Missing required parameter: {}", param),
                        duration,
                    ));
                }
            }
        }
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        Ok(CheckResult::success("Parameter validation passed", duration))
    }

    fn priority(&self) -> u32 {
        30
    }
}

/// Middleware chain
pub struct MiddlewareChain {
    checkers: RwLock<Vec<Arc<dyn Checker>>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            checkers: RwLock::new(Vec::new()),
        }
    }

    /// Add checker
    pub fn add_checker(&self, checker: Arc<dyn Checker>) {
        let mut checkers = self.checkers.write();
        checkers.push(checker);
        checkers.sort_by_key(|c| c.priority());
    }

    /// Execute inspection chain
    pub fn check(&self, ctx: &CheckContext) -> FrameworkResult<CheckResult> {
        let checkers = self.checkers.read();
        let start = Instant::now();
        
        for checker in checkers.iter() {
            let result = checker.check(ctx)?;
            if !result.passed {
                log::warn!(
                    "Check failed at {}: {}",
                    checker.name(),
                    result.message
                );
                return Ok(result);
            }
            log::debug!(
                "Check passed at {}: {} ({:.2}ms)",
                checker.name(),
                result.message,
                result.duration_ms
            );
        }
        
        let total_duration = start.elapsed().as_secs_f64() * 1000.0;
        Ok(CheckResult::success("All checks passed", total_duration))
    }

    /// Get checker count
    pub fn checker_count(&self) -> usize {
        self.checkers.read().len()
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelist_checker() {
        let checker = WhitelistChecker::new();
        checker.add_caller("test_caller");
        
        let ctx = CheckContext::new("test_caller", "test_interface");
        let result = checker.check(&ctx).unwrap();
        assert!(result.passed);
        
        let ctx = CheckContext::new("unknown_caller", "test_interface");
        let result = checker.check(&ctx).unwrap();
        assert!(!result.passed);
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, 1000);
        
        for i in 0..3 {
            let ctx = CheckContext::new("caller", "interface")
                .with_param("i", i.to_string());
            let result = limiter.check(&ctx).unwrap();
            assert!(result.passed);
        }
        
        let ctx = CheckContext::new("caller", "interface");
        let result = limiter.check(&ctx).unwrap();
        assert!(!result.passed);
    }

    #[test]
    fn test_param_validator() {
        let validator = ParamValidator::new();
        validator.add_required_param("test_interface", "param1");
        validator.add_required_param("test_interface", "param2");
        
        let ctx = CheckContext::new("caller", "test_interface")
            .with_param("param1", "value1");
        let result = validator.check(&ctx).unwrap();
        assert!(!result.passed);
        
        let ctx = CheckContext::new("caller", "test_interface")
            .with_param("param1", "value1")
            .with_param("param2", "value2");
        let result = validator.check(&ctx).unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_middleware_chain() {
        let chain = MiddlewareChain::new();
        chain.add_checker(Arc::new(WhitelistChecker::new()));
        chain.add_checker(Arc::new(RateLimiter::new(10, 1000)));
        
        assert_eq!(chain.checker_count(), 2);
    }
}
