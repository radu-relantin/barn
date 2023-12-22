use crossterm::event::KeyCode;
pub trait CursorControllerPort {
    fn new(window_size: (usize, usize)) -> Self;
    fn move_cursor(&mut self, direction: KeyCode);
}
