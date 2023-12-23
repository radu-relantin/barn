pub trait StatusMessagePort {
    fn new(initial_message: String) -> Self
    where
        Self: Sized;
    fn set_message(&mut self, message: String);
    fn message(&mut self) -> Option<&String>;
}
