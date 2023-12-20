use crate::ports::terminal_io::{ReaderPort, WriterPort};
use crossterm::cursor;
use crossterm::event;
use crossterm::terminal;
use std::io;
use std::time::Duration;

pub struct ReaderAdapter;

impl ReaderPort for ReaderAdapter {
    fn read_key(&self) -> io::Result<event::KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let event::Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

pub struct WriterAdapter;

impl WriterPort for WriterAdapter {
    fn clear_screen(&self) -> io::Result<()> {
        crossterm::execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
        crossterm::execute!(io::stdout(), cursor::MoveTo(0, 0))
    }

    fn refresh_screen(&self) -> io::Result<()> {
        self.clear_screen()
    }
}
