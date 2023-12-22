use crate::ports::editor_rows::EditorRowsPort;

struct EditorRows {
    row_contents: Vec<String>,
}

impl EditorRowsPort for EditorRows {
    fn new() -> Self {
        Self {
            row_contents: vec!["Hello World".into()],
        }
    }

    fn number_of_rows(&self) -> usize {
        1
    }

    fn get_row(&self) -> &str {
        &self.row_contents[0]
    }
}
