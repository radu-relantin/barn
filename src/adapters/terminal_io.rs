use crate::log_info;
use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::terminal_io::{CursorEventTypes, ReaderPort, WriterPort};
use crossterm::{cursor, event, queue, terminal};
use std::io::{self, Write};
use std::time::Duration;

/// Adapter for reading key events from the terminal.
///
/// `ReaderAdapter` implements `ReaderPort` and provides functionality
/// for reading key events from the terminal using the `crossterm` crate.
pub struct ReaderAdapter;

impl ReaderPort for ReaderAdapter {
    /// Reads a key event from the terminal.
    ///
    /// This function blocks until a key event is available or the specified
    /// timeout (500ms) is reached. It returns a `KeyEvent` on success.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if an error occurs while polling or reading the event.
    fn read_key(&self) -> io::Result<event::KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let event::Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

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
/// Adapter for writing to the terminal.
///
/// `WriterAdapter` implements `WriterPort` and provides functionality
/// for writing to the terminal, such as clearing the screen.
pub struct WriterAdapter;

impl WriterPort for WriterAdapter {
    fn clear_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: terminal::ClearType,
    ) -> io::Result<()> {
        log_info!("Clearing screen, type: {:?}", clear_type);
        buffer.hide_cursor()?;
        buffer.clear_screen(clear_type)
    }

    fn cursor_event(
        &self,
        buffer: &mut dyn EditorBufferPort,
        cursor_events: &[CursorEventTypes],
    ) -> io::Result<()> {
        log_info!("Moving cursor, events: {:?}", cursor_events);
        queue_cursor_events!(buffer, cursor_events)
    }

    fn flush(&self, buffer: &mut dyn EditorBufferPort) -> io::Result<()> {
        log_info!("Flushing buffer");
        buffer.flush()
    }

    // clear_type is an optional parameter with a defalt value of All.
    fn reset_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: Option<terminal::ClearType>,
    ) -> io::Result<()> {
        log_info!("Resetting screen, type: {:?}", clear_type);
        self.clear_screen(buffer, clear_type.unwrap_or(terminal::ClearType::All))?;
        self.cursor_event(buffer, &[CursorEventTypes::MoveTo(0, 0)])
    }
}
