use crate::ports::editor::EditorDomainPort;
use crate::ports::terminal_io::{CursorEventTypes, ReaderPort, WriterPort};
use crossterm::event;
use std::io;

pub struct EditorApp<R: ReaderPort, W: WriterPort, E: EditorDomainPort> {
    reader: R,
    writer: W,
    domain: E,
}

impl<R: ReaderPort, W: WriterPort, E: EditorDomainPort> EditorApp<R, W, E> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            domain: E::new(E::get_window_size().unwrap()),
        }
    }

    fn process_keypress(&mut self) -> io::Result<bool> {
        match self.reader.read_key()? {
            event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Ok(false),
            event::KeyEvent {
                code:
                    direction @ (event::KeyCode::Char('h')
                    | event::KeyCode::Char('j')
                    | event::KeyCode::Char('k')
                    | event::KeyCode::Char('l')),
                ..
            } => {
                self.domain.move_cursor(direction);
                Ok(true)
            }
            event::KeyEvent {
                code: val @ (event::KeyCode::PageUp | event::KeyCode::PageDown),
                ..
            } => {
                (0..E::get_window_size().unwrap().1).for_each(|_| {
                    self.domain
                        .move_cursor(if matches!(val, event::KeyCode::PageUp) {
                            event::KeyCode::Char('k')
                        } else {
                            event::KeyCode::Char('j')
                        });
                });
                Ok(true)
            }
            _ => Ok(true),
        }
    }

    pub fn run(&mut self) -> io::Result<bool> {
        let (cursor_x, cursor_y) = self.domain.get_cursor_position();

        self.writer
            .reset_screen(self.domain.get_buffer(), None)
            .unwrap();
        self.domain.draw_rows().unwrap();
        self.writer
            .cursor_event(
                self.domain.get_buffer(),
                &[
                    CursorEventTypes::MoveTo(cursor_x as u16, cursor_y as u16),
                    CursorEventTypes::Show,
                ],
            )
            .unwrap();
        self.writer.flush(self.domain.get_buffer()).unwrap();
        self.process_keypress()
    }
}
