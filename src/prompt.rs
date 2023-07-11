use crate::Status;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{backend::Backend, layout::Rect, terminal::Frame, widgets::StatefulWidget};

/// A prompt that can be drawn to a terminal.
pub trait Prompt: StatefulWidget {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the [`StatefulWidget`] trait implementation as we need the [`Frame`]
    /// to set the cursor position.
    ///
    /// [`StatefulWidget`]: ratatui::widgets::StatefulWidget
    /// [`Frame`]: ratatui::Frame
    fn draw<B: Backend>(self, frame: &mut Frame<B>, area: Rect, state: &mut Self::State);
}

/// The focus state of a prompt.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum Focus {
    #[default]
    Unfocused,
    Focused,
}
impl Focus {
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        matches!(self, Self::Focused)
    }
}

/// The state of a prompt.
///
/// Keybindings:
/// - Enter: Complete
/// - Esc | Ctrl+C: Abort
/// - Left | Ctrl+B: Move cursor left
/// - Right | Ctrl+F: Move cursor right
/// - Home | Ctrl+A: Move cursor to start of line
/// - End | Ctrl+E: Move cursor to end of line
/// - Backspace | Ctrl+H: Delete character before cursor
/// - Delete | Ctrl+D: Delete character after cursor
/// - Ctrl+K: Delete from cursor to end of line
/// - Ctrl+U: Delete from cursor to start of line
pub trait State {
    /// The status of the prompt.
    fn status(&self) -> Status;

    /// A mutable reference to the status of the prompt.
    fn status_mut(&mut self) -> &mut Status;

    /// The focus state of the prompt.
    fn focus(&self) -> Focus;

    /// A mutable reference to the focus state of the prompt.
    fn focus_mut(&mut self) -> &mut Focus;

    /// The position of the cursor in the prompt.
    fn position(&self) -> usize;

    /// A mutable reference to the position of the cursor in the prompt.
    fn position_mut(&mut self) -> &mut usize;

    /// The cursor position of the prompt.
    fn cursor(&self) -> (u16, u16);

    /// A mutable reference to the cursor position of the prompt.
    fn cursor_mut(&mut self) -> &mut (u16, u16);

    /// The value of the prompt.
    fn value(&self) -> &str;

    /// A mutable reference to the value of the prompt.
    fn value_mut(&mut self) -> &mut String;

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        match (key_event.code, key_event.modifiers) {
            (KeyCode::Enter, _) => self.complete(),
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.abort(),
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => self.move_left(),
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.move_right(),
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.move_start(),
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => self.move_end(),
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.backspace();
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.delete(),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.kill(),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.truncate(),
            (KeyCode::Char(c), KeyModifiers::NONE) => self.push(c),
            _ => {}
        }
    }

    fn complete(&mut self) {
        *self.status_mut() = Status::Done;
    }

    fn abort(&mut self) {
        *self.status_mut() = Status::Aborted;
    }

    fn delete(&mut self) {
        let position = self.position();
        if position == self.value().len() {
            return;
        }
        self.value_mut().remove(position);
    }

    fn backspace(&mut self) {
        let position = self.position().saturating_sub(1);
        if position == self.value().len() {
            return;
        }
        *self.position_mut() = position;
        self.value_mut().remove(position);
    }

    fn move_right(&mut self) {
        if self.position() == self.value().len() {
            return;
        }
        *self.position_mut() = self.position().saturating_add(1);
    }

    fn move_left(&mut self) {
        *self.position_mut() = self.position().saturating_sub(1);
    }

    fn move_end(&mut self) {
        *self.position_mut() = self.value().len();
    }

    fn move_start(&mut self) {
        *self.position_mut() = 0;
    }

    fn kill(&mut self) {
        let position = self.position();
        self.value_mut().truncate(position);
    }

    fn truncate(&mut self) {
        self.value_mut().clear();
        *self.position_mut() = 0;
    }

    fn push(&mut self, c: char) {
        let position = self.position();
        self.value_mut().insert(position, c);
        *self.position_mut() = self.position().saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{
        style::{Color, Style},
        text::Span,
    };

    use super::*;

    #[test]
    fn status_symbols() {
        assert_eq!(
            Status::Pending.symbol(),
            Span::styled("?", Style::new().fg(Color::Cyan))
        );
        assert_eq!(
            Status::Aborted.symbol(),
            Span::styled("✖", Style::new().fg(Color::Red))
        );
        assert_eq!(
            Status::Done.symbol(),
            Span::styled("✔", Style::new().fg(Color::Green))
        );
    }
}
