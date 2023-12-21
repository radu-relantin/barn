use crate::adapters::editor_buffer::EditorBuffer;
use crate::ports::editor::EditorDomainPort;
use crate::ports::editor_buffer::EditorBufferPort;
use crossterm::{queue, terminal};
use std::io::{self, stdout, Write};

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
    window_size: (usize, usize),
    buffer: EditorBuffer,
}

impl EditorDomainPort for EditorDomain {
    fn new(window_size: (usize, usize)) -> Self {
        Self {
            window_size,
            buffer: EditorBuffer::new(),
        }
    }

    fn get_buffer(&mut self) -> &mut EditorBuffer {
        &mut self.buffer
    }

    fn get_window_size() -> io::Result<(usize, usize)> {
        terminal::size().map(|(w, h)| (w as usize, h as usize))
    }

    fn draw_rows(&mut self) -> io::Result<()> {
        let screen_rows = self.window_size.1;
        let screen_columns = self.window_size.0;
        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                // Draw a welcome message in the center of the screen
                let mut welcome = format!("BARN Editor --- Version {}", env!("CARGO_PKG_VERSION"));
                if welcome.len() > screen_columns {
                    // truncate the welcome message if it's too long
                    welcome.truncate(screen_columns)
                }
                // center the welcome message
                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.buffer.append_char('~');
                    padding -= 1
                }
                // add padding
                (0..padding).for_each(|_| self.buffer.append_char(' '));
                // add the welcome message
                self.buffer.append_str(&welcome);
            } else {
                // otherwise draw a tilde
                self.buffer.append_char('~')
            }

            queue!(
                self.buffer,
                terminal::Clear(terminal::ClearType::UntilNewLine)
            )?;

            if i < screen_rows - 1 {
                self.buffer.append_str("\r\n");
            }
            stdout().flush()?;
        }
        Ok(())
    }
}
