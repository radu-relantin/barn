use crate::ports::terminal_io::{ReaderPort, WriterPort};
use crossterm::{cursor, event, execute, terminal};
use std::io::{self, stdout, Write};
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

/// Adapter for writing to the terminal.
///
/// `WriterAdapter` implements `WriterPort` and provides functionality
/// for writing to the terminal, such as clearing the screen.
pub struct WriterAdapter;

impl WriterPort for WriterAdapter {
    fn clear_screen(&self) -> io::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&self, window_size: (usize, usize)) -> io::Result<()> {
        let screen_rows = window_size.1;
        for i in 0..screen_rows {
            print!("~");
            if i < screen_rows - 1 {
                println!("\r")
            }
            stdout().flush()?;
        }
        Ok(())
    }
}
