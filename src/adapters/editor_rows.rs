use crate::ports::editor_rows::EditorRowsPort;
use crate::ports::editor_rows::RowPort;
use std::{env, fs, path, rc::Rc};

pub const TAB_STOP: usize = 8;

pub struct Row {
    row_content: Box<str>,
    render: String,
}

impl RowPort for Row {
    fn new(row_content: Box<str>, _render: String) -> Self {
        let mut row = Self {
            row_content,
            render: String::new(),
        };
        row.update_render();
        row
    }
    fn get_content(&self) -> &str {
        &self.row_content
    }

    fn get_render(&self) -> &String {
        &self.render
    }
}

impl Row {
    fn update_render(&mut self) {
        let mut index = 0;
        self.render.clear();
        self.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                self.render.push(' ');
                while index % TAB_STOP != 0 {
                    self.render.push(' ');
                    index += 1;
                }
            } else {
                self.render.push(c);
            }
        });
    }
}

pub struct EditorRows {
    row_content: Vec<Rc<dyn RowPort>>,
    file_name: Option<path::PathBuf>,
}

impl EditorRows {
    pub fn from_file(file: path::PathBuf) -> Self {
        let file_content = fs::read_to_string(&file).expect("Unable to read file");

        let row_content = file_content
            .lines()
            .map(|line| {
                let row = Row::new(line.into(), String::new());
                Rc::new(row) as Rc<dyn RowPort>
            })
            .collect();

        Self {
            row_content,
            file_name: Some(file),
        }
    }
}

impl EditorRowsPort for EditorRows {
    fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_content: Vec::new(),
                file_name: None,
            },
            Some(file) => Self::from_file(file.into()),
        }
    }

    fn render_row(&self, row: &mut Row) {
        row.update_render();
    }

    fn number_of_rows(&self) -> usize {
        self.row_content.len()
    }

    fn get_row(&self, at: usize) -> &str {
        &self.row_content[at].get_content()
    }

    fn get_render(&self, at: usize) -> &String {
        &self.row_content[at].get_render()
    }

    fn get_editor_row(&self, at: usize) -> &dyn RowPort {
        &*self.row_content[at]
    }

    fn get_file_name(&self) -> Option<&path::PathBuf> {
        self.file_name.as_ref()
    }
}
