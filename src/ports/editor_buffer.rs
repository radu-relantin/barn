pub trait EditorBufferPort {
    fn new() -> Self;
    fn append_char(&mut self, ch: char);
    fn append_str(&mut self, str: &str);
}
