use crate::ports::status_message::StatusMessagePort;
use std::time::{Duration, Instant};

pub struct StatusMessage {
    message: Option<String>,
    set_time: Option<Instant>,
}

impl StatusMessagePort for StatusMessage {
    fn new(initial_message: String) -> Self {
        Self {
            message: Some(initial_message),
            set_time: None,
        }
    }

    fn set_message(&mut self, message: String) {
        self.message = Some(message);
        self.set_time = Some(Instant::now())
    }

    fn message(&mut self) -> Option<&String> {
        self.set_time.and_then(|time| {
            if time.elapsed() > Duration::from_secs(5) {
                self.message = None;
                self.set_time = None;
                None
            } else {
                Some(self.message.as_ref().unwrap())
            }
        })
    }
}
