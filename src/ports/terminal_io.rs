use crossterm::event::KeyEvent;
use std::io::Result;

pub trait ReaderPort {
    fn read_key(&self) -> Result<KeyEvent>;
}

pub trait WriterPort {
    fn clear_screen(&self) -> Result<()>;
    fn refresh_screen(&self) -> Result<()>;
}
