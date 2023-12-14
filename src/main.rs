mod tools;

use tools::logger::{LogLevel, Logger};

fn main() {
    let mut logger = Logger::new("app.log", LogLevel::Debug).unwrap();

    log_fatal!(logger, "Fatal error occurred: {}", "Out of memory");
    log_error!(logger, "Error encountered: {}", "File not found");
}
