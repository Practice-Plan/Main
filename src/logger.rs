//! Logging module
//! 
//! Provides high-performance logging functionality with multiple log levels and output targets

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use parking_lot::Mutex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::path::PathBuf;

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Log level
    pub level: LevelFilter,
    /// Log file path (optional)
    pub log_file: Option<PathBuf>,
    /// Whether to output to console
    pub console_output: bool,
    /// Log format
    pub format: LogFormat,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
            log_file: None,
            console_output: true,
            format: LogFormat::Default,
        }
    }
}

/// Log format
#[derive(Debug, Clone)]
pub enum LogFormat {
    /// Default format
    Default,
    /// JSON format
    Json,
    /// Compact format
    Compact,
}

/// Framework logger
pub struct FrameworkLogger {
    config: LoggerConfig,
    file: Option<Mutex<File>>,
}

impl FrameworkLogger {
    /// Create a new logger
    pub fn new(config: LoggerConfig) -> Result<Arc<Self>, std::io::Error> {
        let file = if let Some(ref path) = config.log_file {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            Some(Mutex::new(file))
        } else {
            None
        };

        Ok(Arc::new(Self { config, file }))
    }

    /// Format log message
    fn format_message(&self, record: &Record) -> String {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        
        match self.config.format {
            LogFormat::Default => {
                format!(
                    "[{}] [{}] [{}:{}] {}",
                    timestamp,
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                )
            }
            LogFormat::Json => {
                format!(
                    r#"{{"timestamp":"{}","level":"{}","module":"{}","file":"{}","line":{},"message":"{}"}}"#,
                    timestamp,
                    record.level(),
                    record.module_path().unwrap_or("unknown"),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                )
            }
            LogFormat::Compact => {
                format!("[{}] {} - {}", record.level(), record.module_path().unwrap_or(""), record.args())
            }
        }
    }
}

impl Log for FrameworkLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.config.level
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = self.format_message(record);

        // Output to console
        if self.config.console_output {
            println!("{}", message);
        }

        // Output to file
        if let Some(ref file) = self.file {
            let mut file = file.lock();
            let _ = writeln!(file, "{}", message);
        }
    }

    fn flush(&self) {
        if let Some(ref file) = self.file {
            let mut file = file.lock();
            let _ = file.flush();
        }
    }
}

/// Initialize logging system
pub fn init_logger(config: LoggerConfig) -> Result<(), SetLoggerError> {
    let logger = FrameworkLogger::new(config).expect("Failed to create logger");
    // Convert Arc to Box for set_boxed_logger
    let logger_box: Box<dyn Log> = Box::new(FrameworkLoggerWrapper {
        inner: logger,
    });
    log::set_boxed_logger(logger_box)?;
    log::set_max_level(LevelFilter::Trace);
    Ok(())
}

/// Wrapper to convert Arc<FrameworkLogger> to Box<dyn Log>
struct FrameworkLoggerWrapper {
    inner: Arc<FrameworkLogger>,
}

impl Log for FrameworkLoggerWrapper {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        self.inner.log(record)
    }

    fn flush(&self) {
        self.inner.flush()
    }
}

/// Convenience function: Initialize default logger
pub fn init_default_logger() -> Result<(), SetLoggerError> {
    init_logger(LoggerConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;

    #[test]
    fn test_logger_config_default() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, LevelFilter::Info);
        assert!(config.console_output);
    }

    #[test]
    fn test_logger_format() {
        let config = LoggerConfig {
            log_file: None,
            console_output: false,
            ..Default::default()
        };
        let logger = FrameworkLogger::new(config).unwrap();
        
        let record = log::Record::builder()
            .args(format_args!("test message"))
            .level(Level::Info)
            .file(Some("test.rs"))
            .line(Some(42))
            .build();
        
        let msg = logger.format_message(&record);
        assert!(msg.contains("test message"));
        assert!(msg.contains("INFO"));
    }
}
