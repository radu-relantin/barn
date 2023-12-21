use crate::ports::editor::EditorDomainPort;
use crate::ports::terminal_io::{CursorEventTypes, ReaderPort, WriterPort};
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
pub struct EditorApp<R: ReaderPort, W: WriterPort, E: EditorDomainPort> {
    reader: R,
    writer: W,
    domain: E,
}

impl<R: ReaderPort, W: WriterPort, E: EditorDomainPort> EditorApp<R, W, E> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            domain: E::new(E::get_window_size().unwrap()),
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

    pub fn run(&mut self) -> io::Result<bool> {
        self.writer
            .reset_screen(self.domain.get_buffer(), None)
            .unwrap();
        self.domain.draw_rows().unwrap();
        self.writer
            .move_cursor(
                self.domain.get_buffer(),
                &[CursorEventTypes::MoveTo(0, 0), CursorEventTypes::Show],
            )
            .unwrap();
        self.writer.flush(self.domain.get_buffer()).unwrap();
        self.process_keypress()
    }
}
