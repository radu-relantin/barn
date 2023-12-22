use crate::adapters::editor_rows::Row;
use std::any::Any;

pub trait RowPort: Any {
    fn new(row_content: Box<str>, render: String) -> Self
    where
        Self: Sized;
    fn get_content(&self) -> &str;
    fn get_render(&self) -> &String;
}

pub trait EditorRowsPort {
    fn new() -> Self
    where
        Self: Sized;
    fn number_of_rows(&self) -> usize;
    fn get_row(&self, at: usize) -> &str;
    fn render_row(&self, row: &mut Row);
    fn get_render(&self, at: usize) -> &String;
    fn get_editor_row(&self, at: usize) -> &dyn RowPort;
}
