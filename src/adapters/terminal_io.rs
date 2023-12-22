use crate::ports::editor_buffer::EditorBufferPort;
use crate::ports::terminal_io::{CursorEventTypes, EventReader, ReaderPort, WriterPort};
use crossterm::{event, terminal};
use std::io::{self};
use std::time::Duration;

pub struct CrosstermEventReader;

impl EventReader for CrosstermEventReader {
    fn poll_event(&self, timeout: Duration) -> io::Result<bool> {
        event::poll(timeout)
    }

    fn read_event(&self) -> io::Result<event::Event> {
        event::read()
    }
}

pub struct ReaderAdapter<E: EventReader> {
    event_reader: E,
}

impl<E: EventReader> ReaderAdapter<E> {
    // This method is an associated function of ReaderAdapter and is not part of ReaderPort trait
    pub fn new(event_reader: E) -> ReaderAdapter<E> {
        ReaderAdapter { event_reader }
    }
}

impl<E: EventReader> ReaderPort for ReaderAdapter<E> {
    fn read_key(&self) -> io::Result<event::KeyEvent> {
        loop {
            if self.event_reader.poll_event(Duration::from_millis(500))? {
                if let event::Event::Key(event) = self.event_reader.read_event()? {
                    return Ok(event);
                }
            }
        }
    }
}

macro_rules! queue_cursor_events {
    ($buffer:expr, $events:expr) => {{
        let mut res = Ok(());

        for event in $events {
            res = match event {
                CursorEventTypes::MoveTo(x, y) => $buffer.move_cursor_to(*x, *y),
                CursorEventTypes::Show => $buffer.show_cursor(),
                CursorEventTypes::Hide => $buffer.hide_cursor(),
                CursorEventTypes::None => continue, // Skip the None event
            };

            if res.is_err() {
                break;
            }
        }

        res
    }};
}

pub struct WriterAdapter;

impl WriterPort for WriterAdapter {
    fn clear_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: terminal::ClearType,
    ) -> io::Result<()> {
        buffer.hide_cursor()?;
        buffer.clear_screen(clear_type)
    }

    fn cursor_event(
        &self,
        buffer: &mut dyn EditorBufferPort,
        cursor_events: &[CursorEventTypes],
    ) -> io::Result<()> {
        queue_cursor_events!(buffer, cursor_events)
    }

    fn flush(&self, buffer: &mut dyn EditorBufferPort) -> io::Result<()> {
        buffer.flush()
    }

    // clear_type is an optional parameter with a defalt value of All.
    fn reset_screen(
        &self,
        buffer: &mut dyn EditorBufferPort,
        clear_type: Option<terminal::ClearType>,
    ) -> io::Result<()> {
        self.clear_screen(buffer, clear_type.unwrap_or(terminal::ClearType::All))?;
        self.cursor_event(buffer, &[CursorEventTypes::MoveTo(0, 0)])
    }
}

#[cfg(test)]
pub struct MockEventReader {
    events: std::cell::RefCell<Vec<io::Result<event::Event>>>,
}

#[cfg(test)]
impl MockEventReader {
    pub fn new(events: Vec<io::Result<event::Event>>) -> Self {
        Self {
            events: std::cell::RefCell::new(events),
        }
    }
}

#[cfg(test)]
impl EventReader for MockEventReader {
    fn poll_event(&self, _timeout: Duration) -> io::Result<bool> {
        Ok(!self.events.borrow().is_empty())
    }

    fn read_event(&self) -> io::Result<event::Event> {
        self.events
            .borrow_mut()
            .pop()
            .unwrap_or(Ok(event::Event::Key(event::KeyEvent::new(
                event::KeyCode::Null,
                event::KeyModifiers::NONE,
            ))))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_read_key() {
        let mock_events = vec![Ok(event::Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
        )))];
        let mock_reader = MockEventReader::new(mock_events);
        let reader_adapter = ReaderAdapter::new(mock_reader);

        let key_event = reader_adapter.read_key().unwrap();
        assert_eq!(
            key_event,
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)
        );
    }
}
