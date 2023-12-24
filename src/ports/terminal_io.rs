use crate::ports::editor_buffer::EditorBufferPort;
use crossterm::{event, terminal};
use std::io;
use std::time;

#[derive(Debug)]
pub enum EventReaderError {
    PollError(io::Error),
    ReadError(io::Error),
}

impl std::fmt::Display for EventReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventReaderError::PollError(e) => write!(f, "Polling event failed: {}", e),
            EventReaderError::ReadError(e) => write!(f, "Reading event failed: {}", e),
        }
    }
}

impl std::error::Error for EventReaderError {}

#[derive(Debug)]
pub enum WriterError {
    CursorOperationError(io::Error),
    FlushError(io::Error),
    ClearScreenError(io::Error),
}

impl std::fmt::Display for WriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriterError::CursorOperationError(e) => write!(f, "Cursor operation failed: {}", e),
            WriterError::FlushError(e) => write!(f, "Flushing buffer failed: {}", e),
            WriterError::ClearScreenError(e) => write!(f, "Clearing screen failed: {}", e),
        }
    }
}

impl std::error::Error for WriterError {}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CursorEventTypes {
    Show,
    Hide,
    MoveTo(u16, u16),
    None,
}

pub trait EventReader {
    fn poll_event(&self, timeout: time::Duration) -> Result<bool, EventReaderError>;
    fn read_event(&self) -> Result<event::Event, EventReaderError>;
}

pub trait ReaderPort {
    fn read_key(&self) -> Result<event::KeyEvent, EventReaderError>;
}

pub trait WriterPort {
    // Change the buffer parameter type to a trait object
    fn clear_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: terminal::ClearType,
    ) -> Result<(), WriterError>;
    fn cursor_event(
        &self,
        buffer: &mut dyn EditorBufferPort,
        cursor_events: &[CursorEventTypes],
    ) -> Result<(), WriterError>;
    fn flush(&self, buffer: &mut dyn EditorBufferPort) -> Result<(), WriterError>;
    fn reset_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: Option<terminal::ClearType>,
    ) -> Result<(), WriterError>;
}
