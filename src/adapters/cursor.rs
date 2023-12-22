use crate::ports::cursor::CursorControllerPort;
use crossterm::event::KeyCode;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,

    pub screen_cols: usize,
    pub screen_rows: usize,
}

impl CursorControllerPort for CursorController {
    fn new(window_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_cols: window_size.0,
            screen_rows: window_size.1,
        }
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Char('k') => self.cursor_y -= self.cursor_y.saturating_sub(1),
            KeyCode::Char('h') => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Char('j') => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Char('l') => {
                if self.cursor_x != self.screen_cols - 1 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::End => self.cursor_x = self.screen_cols - 1,
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!(),
        }
    }

    fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}
