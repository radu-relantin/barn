use crate::adapters::editor_buffer::EditorBuffer;
use crate::adapters::editor_rows::EditorRows;
use crate::ports::cursor::CursorControllerPort;
use crate::ports::editor::EditorDomainPort;
use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::editor_rows::EditorRowsPort;
use crate::{adapters::cursor::CursorController, log_info};

use crossterm::{event, queue, terminal};

use std::cmp;
use std::io::{self, stdout, Write};

pub struct EditorDomain {
    window_size: (usize, usize),
    buffer: Box<dyn EditorBufferPort>,
    cursor_controller: Box<dyn CursorControllerPort>,
    editor_rows: Box<dyn EditorRowsPort>,
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
        }
    }

    fn draw_rows(&mut self) -> io::Result<()> {
        let screen_rows = self.window_size.1;
        let screen_columns = self.window_size.0;
        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.get_row_offset();
            if file_row >= self.editor_rows.number_of_rows() {
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    // Draw a welcome message in the center of the screen
                    let mut welcome =
                        format!("BARN Editor --- Version {}", env!("CARGO_PKG_VERSION"));
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
            } else {
                let row = self.editor_rows.get_render(file_row);
                let col_offset = self.cursor_controller.get_col_offset();
                let len = cmp::min(row.len().saturating_sub(col_offset), screen_columns);
                let start = if len == 0 { 0 } else { col_offset };
                self.buffer.append_str(&row[start..start + len]);
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

    fn get_buffer(&mut self) -> &mut dyn EditorBufferPort {
        &mut *self.buffer
    }

    fn get_window_size() -> io::Result<(usize, usize)> {
        terminal::size().map(|(w, h)| (w as usize, h as usize))
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
}
