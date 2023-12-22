use crate::ports::editor_rows::EditorRowsPort;

use std::{env, fs, path};

pub struct EditorRows {
    row_content: Vec<String>,
}

impl EditorRows {
    pub fn from_file(file: &path::Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            row_content: file_contents.lines().map(String::from).collect(),
        }
    }
}

impl EditorRowsPort for EditorRows {
    fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_content: Vec::new(),
            },
            Some(file) => EditorRows::from_file(&path::Path::new(&file)),
        }
    }
    fn number_of_rows(&self) -> usize {
        self.row_content.len()
    }

    fn get_row(&self, at: usize) -> &str {
        &self.row_content[at]
    }
}
