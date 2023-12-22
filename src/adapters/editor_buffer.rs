use crate::ports::editor_buffer::EditorBufferPort;
use crossterm::{cursor, queue, terminal};
use std::io::{self, stdout};

pub struct EditorBuffer {
    buffer: String,
}

impl EditorBufferPort for EditorBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn append_char(&mut self, ch: char) {
        self.buffer.push(ch)
    }

    fn append_str(&mut self, str: &str) {
        self.buffer.push_str(str)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        queue!(self, cursor::Hide)
    }

    fn clear_screen(&mut self, clear_type: terminal::ClearType) -> io::Result<()> {
        queue!(self, terminal::Clear(clear_type))
    }

    fn move_cursor_to(&mut self, x: u16, y: u16) -> io::Result<()> {
        queue!(self, cursor::MoveTo(x, y))
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        queue!(self, cursor::Show)
    }
}

impl io::Write for EditorBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.buffer.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.buffer);
        stdout().flush()?;
        self.buffer.clear();
        out
    }
}
