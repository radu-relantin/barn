use crate::ports::editor_rows::EditorRowsPort;
use crossterm::event::KeyCode;

pub trait CursorControllerPort {
    fn new(window_size: (usize, usize)) -> Self
    where
        Self: Sized;
    fn move_cursor(&mut self, direction: KeyCode, editor_rows: &dyn EditorRowsPort);
    fn get_cursor_position(&self) -> (usize, usize);
    fn set_cursor_position(&mut self, x: usize, y: usize);
    fn get_row_offset(&self) -> usize;
    fn get_col_offset(&self) -> usize;
    fn scroll(&mut self);
}
