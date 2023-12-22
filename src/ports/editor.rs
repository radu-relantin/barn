use crate::adapters::editor_buffer::EditorBuffer;
use crossterm::event;
use std::io;

pub trait EditorDomainPort {
    fn new(window_size: (usize, usize)) -> Self;
    fn get_window_size() -> io::Result<(usize, usize)>;
    fn draw_rows(&mut self) -> io::Result<()>;
    fn get_buffer(&mut self) -> &mut EditorBuffer;
    fn get_cursor_position(&self) -> (usize, usize);
    fn set_cursor_position(&mut self, x: usize, y: usize);
    fn move_cursor(&mut self, direction: event::KeyCode);
}
