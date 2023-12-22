use crate::adapters::editor_buffer::EditorBuffer;
use crossterm::{event, terminal};
use std::io;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CursorEventTypes {
    Show,
    Hide,
    MoveTo(u16, u16),
    None,
}

/// A trait defining the interface for reading key events from the terminal.
///
/// This trait abstracts the functionality for reading key events, allowing
/// different implementations (adapters) that can be used interchangeably.
/// Implementors of this trait can provide different ways of reading key events,
/// for instance, using different libraries or techniques.
pub trait ReaderPort {
    fn read_key(&self) -> io::Result<event::KeyEvent>;
}

/// A trait defining the interface for writing to the terminal.
///
/// This trait abstracts the functionality for terminal output operations such as
/// clearing the screen. Different implementations (adapters) can provide different
/// ways of handling terminal output, potentially using various backends or libraries.
pub trait WriterPort {
    fn clear_screen(
        &self,
        buffer: &mut EditorBuffer,
        clear_type: terminal::ClearType,
    ) -> io::Result<()>;
    fn cursor_event(
        &self,
        buffer: &mut EditorBuffer,
        cursor_events: &[CursorEventTypes],
    ) -> io::Result<()>;
    fn flush(&self, buffer: &mut EditorBuffer) -> io::Result<()>;
    fn reset_screen(
        &self,
        buffer: &mut EditorBuffer,
        clear_type: Option<terminal::ClearType>,
    ) -> io::Result<()>;
}
