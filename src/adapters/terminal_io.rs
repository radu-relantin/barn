//! `terminal_io.rs`
//!
//! This module provides adapters for terminal I/O operations in the `barn` text editor.
//! It follows the hexagonal architecture pattern, separating the application logic
//! (in the domain and application layers) from external concerns like terminal I/O.
//!
//! The module consists of two primary adapters:
//! - `ReaderAdapter`: For reading user inputs (like keypresses) from the terminal.
//! - `WriterAdapter`: For writing outputs (like text and cursor movements) to the terminal.
//!
//! Additionally, it includes an `EventReader` trait that abstracts the details of event
//! polling and reading, allowing for easier testing and future extensions.

use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::terminal_io::{CursorEventTypes, EventReader, ReaderPort, WriterPort};
use crate::ports::terminal_io::{EventReaderError, WriterError};
use crossterm::{event, terminal};
use std::io;
use std::time::Duration;

/// `CrosstermEventReader` is an implementation of `EventReader` using `crossterm`.
/// It provides the functionality to poll and read terminal events.
pub struct CrosstermEventReader;

impl EventReader for CrosstermEventReader {
    fn poll_event(&self, timeout: Duration) -> Result<bool, EventReaderError> {
        event::poll(timeout).map_err(EventReaderError::PollError)
    }

    fn read_event(&self) -> Result<event::Event, EventReaderError> {
        event::read().map_err(EventReaderError::ReadError)
    }
}

/// `ReaderAdapter` provides an interface for reading key events from the terminal.
/// It utilizes an `EventReader` to abstract the specifics of event polling and reading.
pub struct ReaderAdapter<E: EventReader> {
    event_reader: E,
}

impl<E: EventReader> ReaderAdapter<E> {
    /// Constructs a new `ReaderAdapter` with the given `EventReader`.
    pub fn new(event_reader: E) -> ReaderAdapter<E> {
        ReaderAdapter { event_reader }
    }
}

impl<E: EventReader> ReaderPort for ReaderAdapter<E> {
    /// Reads a key event from the terminal.
    /// Continuously polls for an event and returns the first key event detected.
    fn read_key(&self) -> Result<event::KeyEvent, EventReaderError> {
        loop {
            if self.event_reader.poll_event(Duration::from_millis(500))? {
                if let event::Event::Key(event) = self.event_reader.read_event()? {
                    return Ok(event);
                }
            }
        }
    }
}

/// Macro to queue a series of cursor events into a given `EditorBufferPort`.
macro_rules! queue_cursor_events {
    ($buffer:expr, $events:expr) => {{
        let mut res = Ok(());

        for event in $events {
            res = match event {
                CursorEventTypes::MoveTo(x, y) => $buffer.move_cursor_to(*x, *y),
                CursorEventTypes::Show => $buffer.show_cursor(),
                CursorEventTypes::Hide => $buffer.hide_cursor(),
                CursorEventTypes::None => continue, // Skip the None event
            };

            if res.is_err() {
                break;
            }
        }

        res
    }};
}

/// `WriterAdapter` provides an interface for writing outputs (like text and cursor movements)
/// to the terminal. It uses `EditorBufferPort` to abstract the actual writing process.
pub struct WriterAdapter;

impl WriterPort for WriterAdapter {
    /// Clears the screen with the specified clear type.
    fn clear_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: terminal::ClearType,
    ) -> Result<(), WriterError> {
        buffer
            .hide_cursor()
            .map_err(WriterError::CursorOperationError)?;
        buffer
            .clear_screen(clear_type)
            .map_err(WriterError::ClearScreenError)
    }

    /// Handles a sequence of cursor events, queuing them into the buffer.
    fn cursor_event(
        &self,
        buffer: &mut dyn EditorBufferPort,
        cursor_events: &[CursorEventTypes],
    ) -> Result<(), WriterError> {
        queue_cursor_events!(buffer, cursor_events).map_err(WriterError::CursorOperationError)
    }

    /// Flushes the buffer, writing all queued content to the terminal.
    fn flush(&self, buffer: &mut dyn EditorBufferPort) -> Result<(), WriterError> {
        buffer.flush().map_err(WriterError::FlushError)
    }

    /// Resets the screen to a clear state, optionally with a specified clear type.
    /// Defaults to clearing the entire screen.
    fn reset_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: Option<terminal::ClearType>,
    ) -> Result<(), WriterError> {
        self.clear_screen(buffer, clear_type.unwrap_or(terminal::ClearType::All))?;
        self.cursor_event(buffer, &[CursorEventTypes::MoveTo(0, 0)])
    }
}

// ============================================================================
// ============================ 🧨 TEST SECTION 🧨 ============================
// ============================================================================

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ports::terminal_io::{CursorEventTypes, EventReader, ReaderPort, WriterPort};
//     use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//     use crossterm::terminal::ClearType;
//     use std::cell::RefCell;
//     use std::io::{self};
//     use std::time::Duration;

//     pub struct MockEventReader {
//         events: RefCell<Vec<io::Result<event::Event>>>,
//     }

//     impl MockEventReader {
//         pub fn new(events: Vec<io::Result<event::Event>>) -> Self {
//             Self {
//                 events: RefCell::new(events),
//             }
//         }
//     }

//     impl EventReader for MockEventReader {
//         fn poll_event(&self, _timeout: Duration) -> Result<bool, EventReaderError> {
//             Ok(!self.events.borrow().is_empty())
//         }

//         fn read_event(&self) -> Result<event::Event, EventReaderError> {
//             self.events
//                 .borrow_mut()
//                 .pop()
//                 .unwrap_or(Err(EventReaderError::ReadError(io::Error::new(
//                     io::ErrorKind::Other,
//                     "No events",
//                 ))))
//         }
//     }

//     #[test]
//     fn test_read_key() {
//         let mock_events = vec![Ok(event::Event::Key(KeyEvent::new(
//             KeyCode::Char('a'),
//             KeyModifiers::NONE,
//         )))];
//         let mock_reader = MockEventReader::new(mock_events);
//         let reader_adapter = ReaderAdapter::new(mock_reader);

//         let key_event = reader_adapter.read_key().unwrap();
//         assert_eq!(
//             key_event,
//             KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)
//         );
//     }

//     #[cfg(test)]
//     struct MockEditorBuffer {
//         content: RefCell<String>,
//         cursor_pos: RefCell<(u16, u16)>,
//         is_cursor_hidden: RefCell<bool>,
//     }

//     #[cfg(test)]
//     impl MockEditorBuffer {
//         fn new() -> Self {
//             Self {
//                 content: RefCell::new(String::new()),
//                 cursor_pos: RefCell::new((0, 0)),
//                 is_cursor_hidden: RefCell::new(false),
//             }
//         }

//         fn is_content_empty(&self) -> bool {
//             self.content.borrow().is_empty()
//         }

//         fn is_cursor_at(&self, x: u16, y: u16) -> bool {
//             *self.cursor_pos.borrow() == (x, y)
//         }
//     }

//     #[cfg(test)]
//     impl io::Write for MockEditorBuffer {
//         fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//             let string = String::from_utf8_lossy(buf);
//             self.content.borrow_mut().push_str(&string);
//             Ok(buf.len())
//         }

//         fn flush(&mut self) -> io::Result<()> {
//             Ok(())
//         }
//     }

//     #[cfg(test)]
//     impl EditorBufferPort for MockEditorBuffer {
//         fn new() -> Self {
//             Self::new()
//         }

//         fn append_char(&mut self, ch: char) {
//             self.content.borrow_mut().push(ch);
//         }

//         fn append_str(&mut self, str: &str) {
//             self.content.borrow_mut().push_str(str);
//         }

//         fn hide_cursor(&mut self) -> io::Result<()> {
//             *self.is_cursor_hidden.borrow_mut() = true;
//             Ok(())
//         }

//         fn clear_screen(&mut self, clear_type: terminal::ClearType) -> io::Result<()> {
//             match clear_type {
//                 terminal::ClearType::All => self.content.borrow_mut().clear(),
//                 _ => {}
//             }
//             Ok(())
//         }

//         fn move_cursor_to(&mut self, x: u16, y: u16) -> io::Result<()> {
//             *self.cursor_pos.borrow_mut() = (x, y);
//             Ok(())
//         }

//         fn show_cursor(&mut self) -> io::Result<()> {
//             *self.is_cursor_hidden.borrow_mut() = false;
//             Ok(())
//         }

//         fn set_style(&mut self, style: crossterm::style::Attribute) -> io::Result<()> {
//             match style {
//                 crossterm::style::Attribute::Reset => {}
//                 _ => {}
//             }
//             Ok(())
//         }
//     }

//     #[test]
//     fn test_clear_screen() {
//         let mut buffer = MockEditorBuffer::new();
//         buffer.content.borrow_mut().push_str("Some content");
//         assert!(!buffer.is_content_empty());

//         let writer = WriterAdapter;
//         writer.clear_screen(&mut buffer, ClearType::All).unwrap();
//         assert!(buffer.is_content_empty());
//     }

//     #[test]
//     fn test_cursor_event() {
//         let mut buffer = MockEditorBuffer::new();
//         assert!(!buffer.is_cursor_at(5, 5));

//         let writer = WriterAdapter;
//         writer
//             .cursor_event(&mut buffer, &[CursorEventTypes::MoveTo(5, 5)])
//             .unwrap();
//         assert!(buffer.is_cursor_at(5, 5));
//     }

//     #[test]
//     fn test_flush() {
//         let mut buffer = MockEditorBuffer::new();
//         buffer.content.borrow_mut().push_str("Changed");

//         let writer = WriterAdapter;
//         assert!(writer.flush(&mut buffer).is_ok());
//         // Additional checks can be added here if needed
//     }

//     #[test]
//     fn test_reset_screen() {
//         let mut buffer = MockEditorBuffer::new();
//         buffer.content.borrow_mut().push_str("Content to clear");
//         assert!(!buffer.is_content_empty());

//         let writer = WriterAdapter;
//         writer.reset_screen(&mut buffer, None).unwrap();
//         assert!(buffer.is_content_empty());
//         assert!(buffer.is_cursor_at(0, 0));
//     }
// }
