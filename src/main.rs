mod adapters;
mod app;
mod domain;
mod ports;
mod toolshed;

use adapters::terminal_io;
use app::editor_app;
use crossterm::{cursor, execute, terminal};
use domain::editor;
use std::io::stdout;
use std::io::Result;
use crate::toolshed::logger;

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
fn main() -> Result<()> {
    // Initialize the cleanup struct to ensure cleanup actions are taken
    // when the program exits.
    let _clean_up = CleanUp;

    // Enable raw mode for the terminal to handle keypresses and terminal output
    // more directly.
    log_info!("Enabling raw mode");
    crossterm::terminal::enable_raw_mode()?;

    // Initialize the terminal I/O adapters.
    log_info!("Initializing terminal I/O adapters");
    let reader = terminal_io::ReaderAdapter;
    let writer = terminal_io::WriterAdapter;

    // Create an instance of the main editor application, passing the I/O adapters.
    log_info!("Initializing editor application");
    let mut editor: editor_app::EditorApp<
        terminal_io::ReaderAdapter,
        terminal_io::WriterAdapter,
        editor::EditorDomain,
    > = editor_app::EditorApp::new(reader, writer);

    // Main execution loop of the editor. This loop continues running the editor
    // until an exit condition (like pressing 'Ctrl+Q') is met.
    log_info!("Starting editor application");
    while editor.run()? {}

    // If the loop exits without error, the program exits cleanly.
    Ok(())
}
