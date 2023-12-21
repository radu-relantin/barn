use crate::adapters::editor_buffer::EditorBuffer;
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
                CursorEventTypes::MoveTo(x, y) => queue!($buffer, cursor::MoveTo(*x, *y)),
                CursorEventTypes::Show => queue!($buffer, cursor::Show),
                CursorEventTypes::Hide => queue!($buffer, cursor::Hide),
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
        buffer: &mut EditorBuffer,
        clear_type: terminal::ClearType,
    ) -> io::Result<()> {
        queue!(buffer, cursor::Hide, terminal::Clear(clear_type))
    }

    fn move_cursor(
        &self,
        buffer: &mut EditorBuffer,
        cursor_events: &[CursorEventTypes],
    ) -> io::Result<()> {
        queue_cursor_events!(buffer, cursor_events)
    }

    fn flush(&self, buffer: &mut EditorBuffer) -> io::Result<()> {
        buffer.flush()
    }

    // clear_type is an optional parameter with a defalt value of All.
    fn reset_screen(
        &self,
        buffer: &mut EditorBuffer,
        clear_type: Option<terminal::ClearType>,
    ) -> io::Result<()> {
        self.clear_screen(buffer, clear_type.unwrap_or(terminal::ClearType::All))?;
        self.move_cursor(buffer, &[CursorEventTypes::MoveTo(0, 0)])
    }
}
