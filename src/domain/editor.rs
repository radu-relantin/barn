use crossterm::terminal;
use std::io;

/// Represents the core business logic of the text editor.
///
/// `EditorDomain` encapsulates the fundamental operations and state management
/// related to text editing, independent of any user interface or external systems.
/// This struct should contain methods and fields necessary for performing
/// operations such as text manipulation, cursor positioning, text selection,
/// and other core editing functionalities.
///
/// As the application grows, this struct can be expanded with more methods
/// and fields to support additional editing features.
pub struct EditorDomain {
    pub window_size: (usize, usize),
}

impl EditorDomain {
    pub fn new(window_size: (usize, usize)) -> Self {
        Self { window_size }
    }

    pub fn get_window_size() -> io::Result<(usize, usize)> {
        terminal::size().map(|(w, h)| (w as usize, h as usize))
    }
}
