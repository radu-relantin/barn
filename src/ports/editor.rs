use crate::ports::cursor::CursorControllerPort;
use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::editor_rows::EditorRowsPort;
use crossterm::event;
use std::io;

pub trait EditorDomainPort {
    fn new(window_size: (usize, usize)) -> Self;
    fn get_window_size(&self) -> io::Result<(usize, usize)>;
    fn draw_rows(&mut self) -> io::Result<()>;
    fn get_buffer(&mut self) -> &mut dyn EditorBufferPort;
    fn get_cursor_position(&self) -> (usize, usize);
    fn set_cursor_position(&mut self, x: usize, y: usize);
    fn move_cursor(&mut self, direction: event::KeyCode);
    fn scroll(&mut self);
    fn get_cursor_controller(&mut self) -> &mut dyn CursorControllerPort;
    fn get_editor_rows(&mut self) -> &mut dyn EditorRowsPort;
    fn draw_status_bar(&mut self);
    fn draw_message_bar(&mut self);
}
