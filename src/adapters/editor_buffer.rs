use crate::ports::editor_buffer::EditorBufferPort;
use std::io::{self, stdout};

/// Represents the buffer holding the contents to be displayed in the editor.
pub struct EditorBuffer {
    buffer: String,
}

impl EditorBufferPort for EditorBuffer {
    /// Creates a new empty editor buffer.
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    /// Appends a single character to the editor buffer.
    fn append_char(&mut self, ch: char) {
        self.buffer.push(ch)
    }

    /// Appends a string to the editor buffer.
    fn append_str(&mut self, str: &str) {
        self.buffer.push_str(str)
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
