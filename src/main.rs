use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, execute, queue, terminal};
use std::io;
use std::io::stdout;
use std::io::Write;
use std::time::Duration;

/// Cleanup struct to disable raw mode on drop
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen();
    }
}

struct Output {
    window_size: (usize, usize),
    editor_contents: EditorContents,
}

impl Output {
    fn new() -> Self {
        let window_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            window_size,
            editor_contents: EditorContents::new(),
        }
    }
    fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), crossterm::cursor::MoveTo(0, 0)).unwrap();
    }

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

struct Reader;

impl Reader {
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

struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> std::io::Result<bool> {
        self.output.refresh_screen();
        self.process_keypress()
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

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
