use crossterm::event::KeyEvent;
use std::io::Result;

/// A trait defining the interface for reading key events from the terminal.
///
/// This trait abstracts the functionality for reading key events, allowing
/// different implementations (adapters) that can be used interchangeably.
/// Implementors of this trait can provide different ways of reading key events,
/// for instance, using different libraries or techniques.
pub trait ReaderPort {
    /// Reads a key event from the terminal.
    ///
    /// This method should block until a key event is available and then return it.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is `Ok(KeyEvent)` if a key event is successfully read,
    /// or an `Err` containing an `io::Error` if an error occurs during reading.
    fn read_key(&self) -> Result<KeyEvent>;
}

/// A trait defining the interface for writing to the terminal.
///
/// This trait abstracts the functionality for terminal output operations such as
/// clearing the screen. Different implementations (adapters) can provide different
/// ways of handling terminal output, potentially using various backends or libraries.
pub trait WriterPort {
    /// Clears the entire screen.
    ///
    /// Implementors should ensure that this method clears the entire terminal screen.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is `Ok(())` on successful clearing of the screen,
    /// or an `Err` containing an `io::Error` if an error occurs during the operation.
    fn clear_screen(&self) -> Result<()>;

    /// Refreshes the terminal screen.
    ///
    /// This method can be used to refresh or redraw the screen contents. Currently,
    /// it might be implemented simply as clearing the screen, but it could be
    /// extended for more sophisticated screen management.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is `Ok(())` on successful screen refresh,
    /// or an `Err` containing an `io::Error` if an error occurs during the operation.
    fn refresh_screen(&self) -> Result<()>;
}
