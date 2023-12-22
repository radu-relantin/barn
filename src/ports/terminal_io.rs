use crate::ports::editor_buffer::EditorBufferPort;
use crossterm::{event, terminal};
use std::io;
use std::time;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CursorEventTypes {
    Show,
    Hide,
    MoveTo(u16, u16),
    None,
}

pub trait EventReader {
    fn poll_event(&self, timeout: time::Duration) -> io::Result<bool>;
    fn read_event(&self) -> io::Result<event::Event>;
}

pub trait ReaderPort {
    fn read_key(&self) -> io::Result<event::KeyEvent>;
}

pub trait WriterPort {
    // Change the buffer parameter type to a trait object
    fn clear_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: terminal::ClearType,
    ) -> io::Result<()>;
    fn reset_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: Option<terminal::ClearType>,
    ) -> io::Result<()>;
    fn cursor_event(
        &self,
        buffer: &mut dyn EditorBufferPort,
        cursor_events: &[CursorEventTypes],
    ) -> io::Result<()>;
    fn flush(&self, buffer: &mut dyn EditorBufferPort) -> io::Result<()>;
}
