use crate::ports::terminal_io::{ReaderPort, WriterPort};
use crossterm::{cursor, event, execute, terminal};
use std::io::{self, stdout};
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
    /// Clears the entire screen.
    ///
    /// This function uses `crossterm` to clear the terminal screen and
    /// repositions the cursor to the top-left corner.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if an error occurs while executing the terminal commands.
    fn clear_screen(&self) -> io::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// Refreshes the terminal screen.
    ///
    /// Currently, this is implemented to simply clear the screen. In future,
    /// it could be extended to include additional functionality like redrawing
    /// screen elements.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if an error occurs during screen clearing.
    fn refresh_screen(&self) -> io::Result<()> {
        self.clear_screen()
    }
}
