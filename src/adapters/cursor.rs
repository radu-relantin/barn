use crate::ports::cursor::CursorControllerPort;
use crate::ports::editor_rows::EditorRowsPort;
use crossterm::event::KeyCode;
use std::cmp;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,

    pub screen_cols: usize,
    pub screen_rows: usize,

    pub row_offset: usize,
    pub col_offset: usize,
}

impl CursorControllerPort for CursorController {
    fn new(window_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_cols: window_size.0,
            screen_rows: window_size.1,
            row_offset: 0,
            col_offset: 0,
        }
    }

    fn move_cursor(&mut self, direction: KeyCode, editor_rows: &dyn EditorRowsPort) {
        let number_of_rows = editor_rows.number_of_rows();
        match direction {
            // up
            KeyCode::Char('k') => self.cursor_y = self.cursor_y.saturating_sub(1),
            // left
            KeyCode::Char('h') => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_row(self.cursor_y).len();
                }
            }
            // down
            KeyCode::Char('j') => {
                if self.cursor_y != number_of_rows {
                    self.cursor_y += 1;
                }
            }
            // right
            KeyCode::Char('l') => {
                if self.cursor_y < number_of_rows {
                    match self.cursor_x.cmp(&editor_rows.get_row(self.cursor_y).len()) {
                        cmp::Ordering::Less => self.cursor_x += 1,
                        cmp::Ordering::Equal => {
                            self.cursor_y += 1;
                            self.cursor_x = 0
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::End => self.cursor_x = self.screen_cols - 1,
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < number_of_rows {
            editor_rows.get_row(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_len);
    }

    fn get_cursor_position(&self) -> (usize, usize) {
        (
            self.cursor_x - self.col_offset,
            self.cursor_y - self.row_offset,
        )
    }

    fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    fn get_row_offset(&self) -> usize {
        self.row_offset
    }

    fn get_col_offset(&self) -> usize {
        self.col_offset
    }

    fn scroll(&mut self) {
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }

        self.col_offset = cmp::min(self.col_offset, self.cursor_x);
        if self.cursor_x >= self.col_offset + self.screen_cols {
            self.col_offset = self.cursor_x - self.screen_cols + 1;
        }
    }
}
