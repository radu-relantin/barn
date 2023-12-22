use crate::ports::editor_rows::EditorRowsPort;
use crate::ports::editor_rows::RowPort;
use std::{env, fs, path, rc::Rc};

const TAB_STOP: usize = 8;

pub struct Row {
    row_content: Box<str>,
    render: String,
}

impl RowPort for Row {
    fn new(row_content: Box<str>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
    fn get_content(&self) -> &str {
        &self.row_content
    }

    fn get_render(&self) -> &String {
        &self.render
    }
}

pub struct EditorRows {
    row_content: Vec<Rc<dyn RowPort>>,
}

impl EditorRows {
    pub fn from_file(file: &path::Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");

        let row_content = file_contents
            .lines()
            .map(|line| {
                let row = Row::new(line.into(), String::new());
                Rc::new(row) as Rc<dyn RowPort>
            })
            .collect();

        Self { row_content }
    }
}

impl EditorRowsPort for EditorRows {
    fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_content: Vec::new(),
            },
            Some(file) => Self::from_file(&path::Path::new(&file)),
        }
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

    // NOTE & WARNING: Don't be this guy...
    // fn get_editor_row(&self, at: usize) -> &Row {
    //     self.row_content[at]
    //         .as_ref() // Get a reference to the `dyn RowPort`
    //         .as_any() // Call `as_any` on the `dyn RowPort` reference
    //         .downcast_ref::<Row>()
    //         .expect("Downcast to Row failed")
    // }

    // NOTE & WARNING: Be this guy instead :)
    fn get_editor_row(&self, at: usize) -> &dyn RowPort {
        &*self.row_content[at]
    }

    fn render_row(&self, row: &mut Row) {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { 8 } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_STOP != 0 {
                    row.render.push(' ');
                    index += 1
                }
            } else {
                row.render.push(c);
            }
        });
    }
}
