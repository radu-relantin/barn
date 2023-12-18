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
struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
        }
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != self.screen_columns - 1 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::End => self.cursor_x = self.screen_columns - 1,
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!(),
        }
    }
}
/// Represents the output of the editor.
struct Output {
    window_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
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
            cursor_controller: CursorController::new(window_size),
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

        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.editor_contents,
            crossterm::cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            crossterm::cursor::Show
        )
        .unwrap();
        self.editor_contents.flush().unwrap();
    }

    /// Draws rows on the screen, each row starting with a '~'.
    fn draw_rows(&mut self) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let screen_rows = self.window_size.1;
        let screen_columns = self.window_size.0;

        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                let mut welcome = format!("Barn Editor --- Version {}", VERSION);
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns)
                }
                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome);
            } else {
                self.editor_contents.push('~');
            }

            // self.editor_contents.push('~');
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

    fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
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
    fn process_keypress(&mut self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),

            KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End),
                modifiers: KeyModifiers::NONE,
                ..
            } => self.output.move_cursor(direction),

            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            } => (0..self.output.window_size.1).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),

            _ => (),
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
