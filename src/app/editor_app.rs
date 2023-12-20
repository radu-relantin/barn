use crate::domain::editor;
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
    reader: R,
    writer: W,
    domain: editor::EditorDomain,
}

impl<R: ReaderPort, W: WriterPort> EditorApp<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            domain: editor::EditorDomain::new(editor::EditorDomain::get_window_size().unwrap()),
        }
    }

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

    pub fn run(&self) -> io::Result<bool> {
        self.writer.clear_screen().unwrap();
        self.writer.draw_rows(self.domain.window_size).unwrap();
        self.process_keypress()
    }
}
