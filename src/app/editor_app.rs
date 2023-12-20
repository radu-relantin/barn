use crate::domain::editor::EditorDomain;
use crate::ports::terminal_io::{ReaderPort, WriterPort};
use crossterm::event;

pub struct EditorApp<R: ReaderPort, W: WriterPort> {
    reader: R,
    writer: W,
    domain: EditorDomain,
}

impl<R: ReaderPort, W: WriterPort> EditorApp<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            domain: EditorDomain::new(),
        }
    }

    pub fn run(&self) -> std::io::Result<bool> {
        self.writer.refresh_screen()?;
        self.process_keypress()
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
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
