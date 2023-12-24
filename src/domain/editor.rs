use crate::adapters::editor_buffer::EditorBuffer;
use crate::adapters::editor_rows::EditorRows;
use crate::adapters::status_message::StatusMessage;
use crate::ports::cursor::CursorControllerPort;
use crate::ports::editor::EditorDomainPort;
use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::editor_rows::EditorRowsPort;
use crate::ports::status_message::StatusMessagePort;
use crate::{adapters::cursor::CursorController, log_info};

use crossterm::{event, queue, style, terminal};

use std::cmp;
use std::io::{self, stdout, Write};

pub struct EditorDomain {
    window_size: (usize, usize),
    buffer: Box<dyn EditorBufferPort>,
    cursor_controller: Box<dyn CursorControllerPort>,
    editor_rows: Box<dyn EditorRowsPort>,
    status_message: Box<dyn StatusMessagePort>,
}

impl EditorDomainPort for EditorDomain {
    fn new(window_size: (usize, usize)) -> Self {
        log_info!(
            "Initializing editor domain with window size: {:?}",
            window_size
        );
        Self {
            window_size,
            buffer: Box::new(EditorBuffer::new()),
            cursor_controller: Box::new(CursorController::new(window_size)),
            editor_rows: Box::new(EditorRows::new()),
            status_message: Box::new(StatusMessage::new("HELP: Ctrl-Q = Quit".into())),
        }
    }

    fn draw_rows(&mut self) -> io::Result<()> {
        // Define the screen dimensions
        let screen_rows = self.window_size.1;
        let screen_columns = self.window_size.0;

        // Iterate through each row on the screen
        for i in 0..screen_rows {
            // Calculate the corresponding row in the file based on the current scroll position
            let file_row = i + self.cursor_controller.get_row_offset();

            // Check if the current screen row has a corresponding file row
            if file_row >= self.editor_rows.number_of_rows() {
                // If the file has no content and we're at the middle third of the screen
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    // Prepare a welcome message
                    let mut welcome =
                        format!("BARN Editor --- Version {}", env!("CARGO_PKG_VERSION"));

                    // Truncate the message if it exceeds screen width
                    if welcome.len() > screen_columns {
                        welcome.truncate(screen_columns);
                    }

                    // Calculate padding to center the message
                    let mut padding = (screen_columns - welcome.len()) / 2;
                    // Draw a tilde before the welcome message if there's space
                    if padding != 0 {
                        self.buffer.append_char('~');
                        padding -= 1; // Reduce padding by one for the tilde
                    }

                    // Add spaces to pad out the message
                    (0..padding).for_each(|_| self.buffer.append_char(' '));

                    // Append the welcome message
                    self.buffer.append_str(&welcome);
                } else {
                    // For empty file rows, draw a tilde
                    self.buffer.append_char('~');
                }
            } else {
                // Get the content of the file row and the column offset for horizontal scrolling
                let row = self.editor_rows.get_render(file_row);
                let col_offset = self.cursor_controller.get_col_offset();
                // Determine the length of the text to be displayed on the screen
                let len = cmp::min(row.len().saturating_sub(col_offset), screen_columns);
                let start = if len == 0 { 0 } else { col_offset };
                // Append the part of the row that fits on the screen
                self.buffer.append_str(&row[start..start + len]);
            }

            // Clear to the end of the line to remove any previous content
            queue!(
                self.buffer,
                terminal::Clear(terminal::ClearType::UntilNewLine)
            )?;

            // Move to the next line
            self.buffer.append_str("\r\n");
            stdout().flush()?; // Flush the buffer to apply changes to the terminal
        }

        Ok(())
    }

    fn draw_status_bar(&mut self) {
        // Set the style of the buffer to Reverse (inverts the foreground and background colors)
        self.buffer.set_style(style::Attribute::Reverse).unwrap();
        self.buffer.set_style(style::Attribute::Bold).unwrap();

        // Retrieve the file name, default to "[No Name]" if not available
        let file_name = self
            .editor_rows
            .get_file_name()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");

        // Create a string with the current line and total lines information
        let line_info = format!(
            "{}/{}",
            self.cursor_controller.get_cursor_position().1 + 1,
            self.editor_rows.number_of_rows()
        );

        // Create a string with file information and the number of lines
        let info = format!(
            "{} -- {} lines",
            file_name,
            self.editor_rows.number_of_rows()
        );
        // Determine the length of the info, limiting it to the window width minus the length of line_info
        let info_len = cmp::min(info.len(), self.window_size.0 - line_info.len());

        // Construct the status bar string.
        // Format the file info to occupy the left side (info_len width) and line info on the right
        let status_bar = format!(
            "{:<info_len$}{:>line_info_len$}",
            info,
            line_info,
            info_len = info_len,
            line_info_len = self.window_size.0 - info_len
        );

        // Append the constructed status bar string to the buffer
        self.buffer.append_str(&status_bar);
        // Reset the buffer's style to default
        self.buffer.set_style(style::Attribute::Reset).unwrap();
        // Append a new line character to move to the next line
        self.buffer.append_str("\r\n");
    }

    fn draw_message_bar(&mut self) {
        queue!(
            self.buffer,
            terminal::Clear(terminal::ClearType::UntilNewLine)
        )
        .unwrap();
        if let Some(msg) = self.status_message.message() {
            self.buffer
                .append_str(&msg[..cmp::min(self.window_size.0, msg.len())]);
        }
    }

    fn get_buffer(&mut self) -> &mut dyn EditorBufferPort {
        &mut *self.buffer
    }

    fn get_window_size(&self) -> io::Result<(usize, usize)> {
        terminal::size().map(|(w, h)| (w as usize, h as usize - 2))
    }

    fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor_controller.get_cursor_position()
    }

    fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_controller.set_cursor_position(x, y);
    }

    fn move_cursor(&mut self, direction: event::KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &*self.editor_rows);
    }

    fn scroll(&mut self) {
        self.cursor_controller.scroll(&*self.editor_rows);
    }

    fn get_cursor_controller(&mut self) -> &mut dyn CursorControllerPort {
        &mut *self.cursor_controller
    }

    fn get_editor_rows(&mut self) -> &mut dyn EditorRowsPort {
        &mut *self.editor_rows
    }
}
