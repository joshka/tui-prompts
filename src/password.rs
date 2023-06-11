use std::borrow::Cow;

use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Line, Span, Text};
use tui::widgets::{Paragraph, StatefulWidget, Widget};
use tui_textarea::TextArea;

pub struct PasswordState<'a> {
    pub masked_password: String,
    pub actual_password: String,
    pub cursor_position: (usize, usize),
    pub textarea: TextArea<'a>,
}

impl PasswordState<'_> {
    pub fn new() -> Self {
        PasswordState {
            masked_password: String::new(),
            actual_password: String::new(),
            cursor_position: (0, 0),
            textarea: TextArea::default(),
        }
    }

    pub fn handle_input(&mut self, event: crossterm::event::KeyEvent) {
        if event.code == crossterm::event::KeyCode::Enter {
            return;
        }
        self.textarea.input(event);
        self.actual_password = self.textarea.lines().join("\n");
        self.masked_password = self.actual_password.chars().map(|_| '*').collect();
        self.cursor_position = self.textarea.cursor();
    }
}

#[derive(Debug, Builder)]
pub struct PasswordPrompt<'a> {
    message: Cow<'a, str>,
}

impl Default for PasswordPrompt<'_> {
    fn default() -> Self {
        PasswordPrompt {
            message: Cow::from("Enter Password"),
        }
    }
}

impl<'a> PasswordPrompt<'a> {
    pub fn new(message: &'a str) -> PasswordPrompt<'a> {
        PasswordPrompt {
            message: Cow::from(message),
        }
    }
}

impl<'a> StatefulWidget for PasswordPrompt<'a> {
    type State = PasswordState<'a>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let line = Line::from(vec![
            Span::styled("? ", Style::default().fg(Color::Cyan)),
            Span::styled(self.message, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                " › ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ),
            Span::raw(state.masked_password.as_str()),
        ]);
        Paragraph::new(Text::from(line)).render(area, buf);
    }
}
