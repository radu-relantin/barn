pub trait EditorRowsPort {
    fn new() -> Self
    where
        Self: Sized;
    fn number_of_rows(&self) -> usize;
    fn get_row(&self) -> &str;
}
