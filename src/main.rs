use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{execute, queue};
use std::io::{self, stdout, Write};
use std::time::Duration;

/// Cleanup struct to disable raw mode on drop.
/// This ensures that the terminal is returned to its original state
/// when the program exits or panics.
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen();
    }
}

/// Represents the output of the editor.
struct Output {
    window_size: (usize, usize),
    editor_contents: EditorContents,
}

impl Output {
    /// Constructs a new Output instance with the current terminal size.
    fn new() -> Self {
        let window_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            window_size,
            editor_contents: EditorContents::new(),
        }
    }

    /// Clears the entire screen.
    fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), crossterm::cursor::MoveTo(0, 0)).unwrap();
    }

    /// Refreshes the screen with the current editor contents.
    fn refresh_screen(&mut self) {
        queue!(
            self.editor_contents,
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0)
        )
        .unwrap();

        self.draw_rows();

        queue!(
            self.editor_contents,
            crossterm::cursor::MoveTo(0, 0),
            crossterm::cursor::Show
        )
        .unwrap();
        self.editor_contents.flush().unwrap();
    }

    /// Draws rows on the screen, each row starting with a '~'.
    fn draw_rows(&mut self) {
        let screen_rows = self.window_size.1;
        for i in 0..screen_rows {
            self.editor_contents.push('~');
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
            stdout().flush().unwrap();
        }
        for _ in 0..screen_rows {
            println!("~\r");
        }
    }
}

/// Handles reading of key events.
struct Reader;

impl Reader {
    /// Reads a key event, blocking until a key is pressed.
    fn read_key(&self) -> std::io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

/// Main editor structure.
struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    /// Creates a new instance of the editor.
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    /// Processes a keypress event.
    /// Returns false if the editor should exit, true otherwise.
    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    /// Main run loop of the editor.
    /// Refreshes the screen and processes keypresses.
    fn run(&mut self) -> std::io::Result<bool> {
        self.output.refresh_screen();
        self.process_keypress()
    }
}

/// Represents the contents of the editor.
struct EditorContents {
    content: String,
}

impl EditorContents {
    /// Creates a new, empty EditorContents.
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Appends a character to the contents.
    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    /// Appends a string to the contents.
    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}
impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
