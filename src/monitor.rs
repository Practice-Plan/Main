//! Application monitoring module
//! 
//! Provides application status monitoring, performance tracking, and event tracing functionality

use crate::error::{FrameworkError, FrameworkResult};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use crossbeam_channel::{bounded, Sender, Receiver};

/// Monitor event types
#[derive(Debug, Clone)]
pub enum MonitorEvent {
    /// Application started
    AppStarted { app_id: String, timestamp: i64 },
    /// Application stopped
    AppStopped { app_id: String, timestamp: i64 },
    /// Performance metric
    PerformanceMetric { name: String, value: f64, unit: String },
    /// Error event
    Error { code: i32, message: String },
    /// Custom event
    Custom { event_type: String, data: HashMap<String, String> },
}

/// Application state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    /// Uninitialized
    Uninitialized,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Stopped
    Stopped,
    /// Error state
    Error,
}

/// Application information
#[derive(Debug, Clone)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub state: AppState,
    pub start_time: Option<Instant>,
    pub metadata: HashMap<String, String>,
}

impl AppInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            state: AppState::Uninitialized,
            start_time: None,
            metadata: HashMap::new(),
        }
    }

    pub fn running_time(&self) -> Option<Duration> {
        self.start_time.map(|t| t.elapsed())
    }
}

/// Monitor configuration
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Event buffer size
    pub buffer_size: usize,
    /// Whether to enable performance monitoring
    pub enable_performance: bool,
    /// Whether to enable error tracking
    pub enable_error_tracking: bool,
    /// Sampling interval (milliseconds)
    pub sampling_interval_ms: u64,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            enable_performance: true,
            enable_error_tracking: true,
            sampling_interval_ms: 1000,
        }
    }
}

/// Monitor
pub struct Monitor {
    config: MonitorConfig,
    apps: RwLock<HashMap<String, AppInfo>>,
    event_sender: Sender<MonitorEvent>,
    event_receiver: Arc<Receiver<MonitorEvent>>,
    metrics: RwLock<HashMap<String, Vec<f64>>>,
}

impl Monitor {
    /// Create a new monitor
    pub fn new(config: MonitorConfig) -> Self {
        let (sender, receiver) = bounded(config.buffer_size);
        
        Self {
            config,
            apps: RwLock::new(HashMap::new()),
            event_sender: sender,
            event_receiver: Arc::new(receiver),
            metrics: RwLock::new(HashMap::new()),
        }
    }

    /// Register application
    pub fn register_app(&self, app: AppInfo) -> FrameworkResult<()> {
        let mut apps = self.apps.write();
        if apps.contains_key(&app.id) {
            return Err(FrameworkError::MonitorError(format!(
                "App {} already registered", app.id
            )));
        }
        let app_id = app.id.clone();
        apps.insert(app_id.clone(), app);
        log::info!("App registered: {}", app_id);
        Ok(())
    }

    /// Start application monitoring
    pub fn start_app(&self, app_id: &str) -> FrameworkResult<()> {
        let mut apps = self.apps.write();
        let app = apps.get_mut(app_id)
            .ok_or_else(|| FrameworkError::MonitorError(format!("App {} not found", app_id)))?;
        
        app.state = AppState::Running;
        app.start_time = Some(Instant::now());
        
        let _ = self.event_sender.try_send(MonitorEvent::AppStarted {
            app_id: app_id.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        
        log::info!("App started: {}", app_id);
        Ok(())
    }

    /// Stop application monitoring
    pub fn stop_app(&self, app_id: &str) -> FrameworkResult<()> {
        let mut apps = self.apps.write();
        let app = apps.get_mut(app_id)
            .ok_or_else(|| FrameworkError::MonitorError(format!("App {} not found", app_id)))?;
        
        app.state = AppState::Stopped;
        app.start_time = None;
        
        let _ = self.event_sender.try_send(MonitorEvent::AppStopped {
            app_id: app_id.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        
        log::info!("App stopped: {}", app_id);
        Ok(())
    }

    /// Get application information
    pub fn get_app_info(&self, app_id: &str) -> Option<AppInfo> {
        self.apps.read().get(app_id).cloned()
    }

    /// Record performance metric
    pub fn record_metric(&self, name: impl Into<String>, value: f64) {
        if !self.config.enable_performance {
            return;
        }
        
        let name = name.into();
        let mut metrics = self.metrics.write();
        metrics.entry(name.clone()).or_insert_with(Vec::new).push(value);
        
        let _ = self.event_sender.try_send(MonitorEvent::PerformanceMetric {
            name,
            value,
            unit: "ms".to_string(),
        });
    }

    /// Record error
    pub fn record_error(&self, code: i32, message: impl Into<String>) {
        if !self.config.enable_error_tracking {
            return;
        }
        
        let message = message.into();
        let _ = self.event_sender.try_send(MonitorEvent::Error { code, message });
    }

    /// Get metric statistics
    pub fn get_metric_stats(&self, name: &str) -> Option<MetricStats> {
        let metrics = self.metrics.read();
        metrics.get(name).map(|values| {
            if values.is_empty() {
                return MetricStats {
                    count: 0,
                    min: 0.0,
                    max: 0.0,
                    avg: 0.0,
                };
            }
            
            let count = values.len();
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let avg = values.iter().sum::<f64>() / count as f64;
            
            MetricStats { count, min, max, avg }
        })
    }

    /// Get event receiver
    pub fn event_receiver(&self) -> Arc<Receiver<MonitorEvent>> {
        self.event_receiver.clone()
    }

    /// Get all application IDs
    pub fn list_apps(&self) -> Vec<String> {
        self.apps.read().keys().cloned().collect()
    }
}

/// Metric statistics
#[derive(Debug, Clone)]
pub struct MetricStats {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let monitor = Monitor::new(MonitorConfig::default());
        assert!(monitor.list_apps().is_empty());
    }

    #[test]
    fn test_app_registration() {
        let monitor = Monitor::new(MonitorConfig::default());
        let app = AppInfo::new("test_app", "Test App", "1.0.0");
        
        assert!(monitor.register_app(app).is_ok());
        assert_eq!(monitor.list_apps().len(), 1);
    }

    #[test]
    fn test_app_lifecycle() {
        let monitor = Monitor::new(MonitorConfig::default());
        let app = AppInfo::new("test_app", "Test App", "1.0.0");
        
        monitor.register_app(app).unwrap();
        monitor.start_app("test_app").unwrap();
        
        let info = monitor.get_app_info("test_app").unwrap();
        assert_eq!(info.state, AppState::Running);
        assert!(info.running_time().is_some());
        
        monitor.stop_app("test_app").unwrap();
        let info = monitor.get_app_info("test_app").unwrap();
        assert_eq!(info.state, AppState::Stopped);
    }

    #[test]
    fn test_metric_recording() {
        let monitor = Monitor::new(MonitorConfig::default());
        
        monitor.record_metric("response_time", 100.0);
        monitor.record_metric("response_time", 150.0);
        monitor.record_metric("response_time", 120.0);
        
        let stats = monitor.get_metric_stats("response_time").unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.min, 100.0);
        assert_eq!(stats.max, 150.0);
        assert!((stats.avg - 123.333).abs() < 0.01);
    }
}
