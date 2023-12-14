use lazy_static::lazy_static;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;

/// Represents the different logging levels.
///
/// Log levels are used to categorize the importance of the log messages.
/// `Fatal` being the most critical, and `Trace` being the least.
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
/// It encapsulates a file handle for writing log messages and a log level
/// which acts as a filter for log messages. Only messages with a level
/// greater than or equal to the logger's level are written to the file.
pub struct Logger {
    file: Mutex<File>,
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

        Ok(Logger {
            file: Mutex::new(file),
            level,
        })
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
    /// let logger = Logger::new("app.log", LogLevel::Debug).unwrap();
    /// logger.log(LogLevel::Info, "Application started").unwrap();
    /// ```
    pub fn log(&self, level: LogLevel, message: &str) -> io::Result<()> {
        if level <= self.level {
            let mut file = self.file.lock().unwrap();
            let log_message = format!("[{:?}]: {}\n", level, message);
            file.write_all(log_message.as_bytes())
        } else {
            Ok(())
        }
    }
}

lazy_static! {
    /// Global logger instance.
    ///
    /// This static instance provides a globally accessible logger,
    /// ensuring that all parts of the application can perform logging
    /// without needing to pass around a logger instance.
    pub static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(
        Logger::new("app.log", LogLevel::Debug).expect("Failed to initialize global logger")
    );
}

/// Logs a fatal error and exits the program.
///
/// Use this macro for unrecoverable errors that should terminate the program.
/// The message will be logged at the `Fatal` level.
///
/// # Examples
///
/// ```
/// log_fatal!("Unrecoverable error: {}", "Out of memory");
/// ```
#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Fatal, &format!($($arg)*)).unwrap();
    };
}

/// Logs an error message.
///
/// Use this macro for recoverable errors that do not require program termination.
/// The message will be logged at the `Error` level.
///
/// # Examples
///
/// ```
/// log_error!("Error encountered: {}", "File not found");
/// ```
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Error, &format!($($arg)*)).unwrap();
    };
}

/// Logs a warning message.
///
/// Use this macro for potential issues that do not halt program execution but require attention.
/// The message will be logged at the `Warning` level.
///
/// # Examples
///
/// ```
/// log_warning!("Potential issue detected: {}", "Low memory");
/// ```
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Warning, &format!($($arg)*)).unwrap();
    };
}

/// Logs an informational message.
///
/// Use this macro for general informational messages.
/// The message will be logged at the `Info` level.
///
/// # Examples
///
/// ```
/// log_info!("Application status: {}", "Running smoothly");
/// ```
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Info, &format!($($arg)*)).unwrap();
    };
}

/// Logs a debug message.
///
/// Use this macro for detailed system information useful for debugging.
/// The message will be logged at the `Debug` level.
///
/// # Examples
///
/// ```
/// log_debug!("Debug info: {}", "Here's some detailed information");
/// ```
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Debug, &format!($($arg)*)).unwrap();
    };
}

/// Logs a trace message.
///
/// Use this macro for tracing the program flow, typically having the highest verbosity.
/// The message will be logged at the `Trace` level.
///
/// # Examples
///
/// ```
/// log_trace!("Function trace: {}", "Entered function X");
/// ```
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::logger::GLOBAL_LOGGER.lock().unwrap().log($crate::logger::LogLevel::Trace, &format!($($arg)*)).unwrap();
    };
}
