use crossterm::event::KeyCode;
pub trait CursorControllerPort {
    fn new(window_size: (usize, usize)) -> Self
    where
        Self: Sized;
    fn move_cursor(&mut self, direction: KeyCode);
    fn get_cursor_position(&self) -> (usize, usize);
    fn set_cursor_position(&mut self, x: usize, y: usize);
}
