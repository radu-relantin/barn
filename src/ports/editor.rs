use crate::adapters::editor_buffer::EditorBuffer;
use std::io;

pub trait EditorDomainPort {
    fn new(window_size: (usize, usize)) -> Self;
    fn get_window_size() -> io::Result<(usize, usize)>;
    fn draw_rows(&mut self) -> io::Result<()>;
    fn get_buffer(&mut self) -> &mut EditorBuffer;
}
