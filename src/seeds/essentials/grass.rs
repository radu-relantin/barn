use crate::{log_debug, log_error};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// The `Grass` struct represents a basic text editor.
///
/// It manages a buffer that contains the text of a file, allowing for basic
/// operations like loading, editing, and saving text.
pub struct Grass {
    buffer: Vec<String>,
}

#[allow(dead_code)]
impl Grass {
    /// Creates a new instance of `Grass` with an empty buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// ```
    pub fn new() -> Grass {
        Grass { buffer: Vec::new() }
    }

    /// Loads the contents of a file into the buffer, creating the file if it does not exist.
    ///
    /// This function will open a file at the specified path for reading and writing.
    /// If the file does not exist, it will be created. The contents of the file, if any,
    /// are read into the buffer of the editor.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to be loaded or created.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file is opened (and created if necessary) and its contents
    /// are loaded successfully, or an `io::Error` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// editor.load_file(Path::new("example.txt")).unwrap();
    /// ```
    ///
    /// In this example, `example.txt` will be opened if it exists, or created if it does not.
    /// The contents of `example.txt` will then be loaded into the editor's buffer.
    pub fn load_file(&mut self, path: &Path) -> io::Result<()> {
        log_debug!("Opening or creating file: {}", path.display());
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
        {
            Ok(file) => file,
            Err(e) => {
                log_error!("Error opening or creating file: {}", e);
                return Err(e);
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            log_error!("Error reading file: {}", e);
            return Err(e);
        }

        self.buffer = contents.lines().map(|s| s.to_string()).collect();
        log_debug!("File opened successfully");
        Ok(())
    }

    /// Saves the buffer's contents to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file where the buffer should be saved.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file is saved successfully, or an `io::Error` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// // ... perform some editing operations ...
    /// editor.save_file(Path::new("example.txt")).unwrap();
    /// ```
    pub fn save_file(&self, path: &Path) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        for line in &self.buffer {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    /// Inserts a line of text at the specified index in the buffer.
    ///
    /// If the index is out of bounds, the line is appended to the end of the buffer.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index at which to insert the line.
    /// * `line` - The line of text to insert.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// editor.insert_line(0, "Hello, world!".to_string());
    /// ```
    pub fn insert_line(&mut self, idx: usize, line: String) {
        if idx <= self.buffer.len() {
            self.buffer.insert(idx, line);
        } else {
            self.buffer.push(line);
        }
    }

    /// Removes the line of text at the specified index in the buffer.
    ///
    /// If the index is out of bounds, no action is taken.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the line to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// // ... assume the buffer is populated ...
    /// editor.remove_line(0);
    /// ```
    pub fn remove_line(&mut self, idx: usize) {
        if idx < self.buffer.len() {
            self.buffer.remove(idx);
        }
    }

    /// Edits the line of text at the specified index in the buffer.
    ///
    /// If the index is out of bounds, no action is taken.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the line to edit.
    /// * `new_line` - The new text for the line.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut editor = Grass::new();
    /// // ... assume the buffer is populated ...
    /// editor.edit_line(0, "Edited line".to_string());
    /// ```
    pub fn edit_line(&mut self, idx: usize, new_line: String) {
        if idx < self.buffer.len() {
            self.buffer[idx] = new_line;
        }
    }

    /// Returns a reference to the current state of the buffer.
    ///
    /// This can be used to display the buffer's contents.
    ///
    /// # Returns
    ///
    /// A reference to the buffer (a vector of strings).
    ///
    /// # Examples
    ///
    /// ```
    /// let editor = Grass::new();
    /// // ... operations on the editor ...
    /// let buffer = editor.get_buffer();
    /// for line in buffer {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn get_buffer(&self) -> &Vec<String> {
        &self.buffer
    }
}
