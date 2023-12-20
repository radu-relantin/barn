use crate::domain::editor::EditorDomain;
use crate::ports::terminal_io::{ReaderPort, WriterPort};
use crossterm::event;
use std::io;

/// Represents the application layer of the text editor.
///
/// `EditorApp` orchestrates the interaction between the input/output operations
/// (handled by ReaderPort and WriterPort) and the core business logic of the editor
/// (encapsulated in EditorDomain).
///
/// Type parameters:
/// - `R`: The type that implements the `ReaderPort` trait for reading input.
/// - `W`: The type that implements the `WriterPort` trait for writing output.
pub struct EditorApp<R: ReaderPort, W: WriterPort> {
    // Reader component to handle input operations.
    reader: R,
    // Writer component to handle output operations.
    writer: W,
    // Domain component encapsulating the core business logic of the editor.
    domain: EditorDomain,
}

impl<R: ReaderPort, W: WriterPort> EditorApp<R, W> {
    /// Constructs a new instance of `EditorApp`.
    ///
    /// This function takes two arguments: an implementor of `ReaderPort` for input
    /// handling, and an implementor of `WriterPort` for output handling. It also
    /// initializes the `EditorDomain`.
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            domain: EditorDomain::new(),
        }
    }

    /// Runs the main loop of the editor application.
    ///
    /// This function refreshes the screen and processes keypresses in a loop.
    /// It returns a `Result` indicating whether the application should continue running.
    ///
    /// Returns `Ok(false)` if the application should exit (e.g., on pressing 'Ctrl+Q').
    pub fn run(&self) -> io::Result<bool> {
        self.writer.refresh_screen()?;
        self.process_keypress()
    }

    /// Processes a single keypress.
    ///
    /// This private function reads a keypress using the `ReaderPort` and acts accordingly.
    /// For example, it returns `Ok(false)` if 'Ctrl+Q' is pressed, indicating the application
    /// should terminate.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if there is an error reading the keypress.
    fn process_keypress(&self) -> io::Result<bool> {
        match self.reader.read_key()? {
            event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Ok(false),
            _ => Ok(true),
        }
    }
}
