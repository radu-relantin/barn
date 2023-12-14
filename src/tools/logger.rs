use std::fs::{File, OpenOptions};
use std::io::{self, Write};

/// Represents the different logging levels.
///
/// The log levels are ordered by their importance, with `Fatal` being the most critical
/// and `Trace` being the least.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

/// The `Logger` struct provides functionalities for logging messages.
///
/// It allows logging messages at various levels and writes them to a specified file.
/// The logger ensures that messages are appended to the file, preserving existing content.
pub struct Logger {
    file: File,
    level: LogLevel,
}

impl Logger {
    /// Initializes a new instance of `Logger`.
    ///
    /// Opens or creates a file at the specified path for logging. The log level
    /// determines the minimum level of messages that will be logged.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file where log messages will be written.
    /// * `level` - The minimum `LogLevel` to log.
    ///
    /// # Returns
    ///
    /// Returns `Logger` instance wrapped in `io::Result` to handle potential I/O errors.
    ///
    /// # Examples
    ///
    /// ```
    /// let logger = Logger::new("app.log", LogLevel::Info).unwrap();
    /// ```
    pub fn new(file_path: &str, level: LogLevel) -> io::Result<Logger> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;

        Ok(Logger { file, level })
    }

    /// Logs a message at the specified log level.
    ///
    /// If the specified log level is greater than or equal to the logger's level,
    /// the message will be written to the log file.
    ///
    /// # Arguments
    ///
    /// * `level` - The `LogLevel` at which to log the message.
    /// * `message` - The message to log.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the message was successfully written, or an `io::Error` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut logger = Logger::new("app.log", LogLevel::Debug).unwrap();
    /// logger.log(LogLevel::Info, "Application started").unwrap();
    /// ```
    pub fn log(&mut self, level: LogLevel, message: &str) -> io::Result<()> {
        if level <= self.level {
            let log_message = format!("[{:?}]: {}\n", level, message);
            self.file.write_all(log_message.as_bytes())
        } else {
            Ok(())
        }
    }
}

/// Logs a fatal error and exits the program.
///
/// This macro should be used for unrecoverable errors.
///
/// # Examples
///
/// ```
/// log_fatal!(logger, "Unrecoverable error: {}", "Out of memory");
/// ```
#[macro_export]
macro_rules! log_fatal {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Fatal, &format!($($arg)*)).unwrap();
    };
}

/// Logs an error message.
///
/// This macro should be used for recoverable errors.
///
/// # Examples
///
/// ```
/// log_error!(logger, "Error encountered: {}", "File not found");
/// ```
#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Error, &format!($($arg)*)).unwrap();
    };
}

/// Logs a warning message.
///
/// This macro should be used for potential issues that do not halt program execution.
///
/// # Examples
///
/// ```
/// log_warning!(logger, "Potential issue detected: {}", "Low memory");
/// ```
#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Warning, &format!($($arg)*)).unwrap();
    };
}

/// Logs an informational message.
///
/// This macro should be used for general informational messages.
///
/// # Examples
///
/// ```
/// log_info!(logger, "Application status: {}", "Running smoothly");
/// ```
#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Info, &format!($($arg)*)).unwrap();
    };
}

/// Logs a debug message.
///
/// This macro should be used for detailed system information useful for debugging.
///
/// # Examples
///
/// ```
/// log_debug!(logger, "Debug info: {}", "Here's some detailed information");
/// ```
#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Debug, &format!($($arg)*)).unwrap();
    };
}

/// Logs a trace message.
///
/// This macro should be used for tracing the program flow, typically having the highest verbosity.
///
/// # Examples
///
/// ```
/// log_trace!(logger, "Function trace: {}", "Entered function X");
/// ```
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Trace, &format!($($arg)*)).unwrap();
    };
}
