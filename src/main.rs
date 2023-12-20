mod adapters;
mod app;
mod domain;
mod ports;

use adapters::terminal_io;
use app::editor_app;
use crossterm::terminal;
use ports::terminal_io::WriterPort;
use std::io::Result;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal_io::WriterAdapter.clear_screen().expect("Unable to clear screen");
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
    }
}
fn main() -> Result<()> {
    let _clean_up = CleanUp;
    crossterm::terminal::enable_raw_mode()?;

    let reader = terminal_io::ReaderAdapter;
    let writer = terminal_io::WriterAdapter;
    let editor = editor_app::EditorApp::new(reader, writer);

    while editor.run()? {}

    Ok(())
}

