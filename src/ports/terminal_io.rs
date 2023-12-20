use crossterm::event;
use std::io;

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
    fn clear_screen(&self) -> io::Result<()>;
    fn draw_rows(&self, window_size: (usize, usize)) -> io::Result<()>;
}
