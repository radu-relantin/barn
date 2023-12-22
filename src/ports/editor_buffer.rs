use crossterm::terminal;
use std::io;

pub trait EditorBufferPort: std::io::Write {
    fn new() -> Self
    where
        Self: Sized;
    fn append_char(&mut self, ch: char);
    fn append_str(&mut self, str: &str);
    fn hide_cursor(&mut self) -> io::Result<()>;
    fn clear_screen(&mut self, clear_type: terminal::ClearType) -> io::Result<()>;
    fn move_cursor_to(&mut self, x: u16, y: u16) -> io::Result<()>;
    fn show_cursor(&mut self) -> io::Result<()>;
}
