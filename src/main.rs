mod adapters;
mod app;
mod domain;
mod ports;
mod toolshed;

use crate::adapters::terminal_io::CrosstermEventReader;
use crate::toolshed::logger;
use adapters::terminal_io;
use app::editor_app;
use crossterm::{cursor, execute, terminal};
use domain::editor;
use ports::config::read_config;
use std::io::stdout;
use std::io::Result as IoResult;

/// A utility struct responsible for cleaning up the application state
/// when the main function exits, either normally or due to an error.
///
/// This includes tasks like clearing the screen and disabling raw mode
/// for the terminal to ensure the terminal is left in a usable state.
struct CleanUp;

impl Drop for CleanUp {
    /// The destructor method for `CleanUp`.
    ///
    /// This method is automatically called when `CleanUp` goes out of scope.
    /// It performs necessary cleanup actions like clearing the terminal screen
    /// and disabling raw mode.
    fn drop(&mut self) {
        log_info!("Cleaning up application state");
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))
            .expect("Unable to clear the screen");
        execute!(stdout(), cursor::MoveTo(0, 0)) // Move cursor to top left corner
            .expect("Unable to move cursor");
    }
}

/// The entry point for the text editor application.
///
/// This function sets up necessary components like the terminal I/O adapters
/// and the main editor application (`EditorApp`). It then enters a loop
/// where it continuously runs the editor until an exit condition is met.
///
/// # Errors
///
/// Returns an `io::Error` if there are issues with terminal I/O operations,
/// such as enabling raw mode or during the main execution loop of the editor.
fn main() -> IoResult<()> {
    // Initialize the cleanup struct to ensure cleanup actions are taken
    // when the program exits.
    let _clean_up = CleanUp;

    // Enable raw mode for the terminal to handle keypresses and terminal output
    // more directly.
    log_info!("Enabling raw mode (Zap! Pow! Bang! and there goes the keyboard)");
    crossterm::terminal::enable_raw_mode()?;

    let config = read_config("./config.toml").unwrap();
    log_info!("Config: {:?}", config);

    // Initialize the terminal I/O adapters.
    let event_reader = CrosstermEventReader;
    let reader = terminal_io::ReaderAdapter::new(event_reader);
    let writer = terminal_io::WriterAdapter;

    // Create an instance of the main editor application, passing the I/O adapters.
    let mut editor: editor_app::EditorApp<
        terminal_io::ReaderAdapter<CrosstermEventReader>,
        terminal_io::WriterAdapter,
        editor::EditorDomain,
    > = editor_app::EditorApp::new(reader, writer, config);

    // Main execution loop of the editor. This loop continues running the editor
    // until an exit condition (like pressing 'Ctrl+Q') is met.
    log_info!("Starting barn editor...");
    while editor.run()? {}

    // If the loop exits without error, the program exits cleanly.
    Ok(())
}
