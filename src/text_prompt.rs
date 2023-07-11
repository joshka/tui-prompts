use std::{borrow::Cow, vec};

use crate::{prelude::*, TextState};

use itertools::Itertools;
use ratatui::prelude::*;

// TODO style the widget
// TODO style each element of the widget.
// TODO handle multi-line input.
// TODO handle scrolling.
// TODO handle vertical movement.
// TODO handle bracketed paste.

/// A prompt widget that displays a message and a text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextPrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    /// The block to wrap the prompt in.
    block: Option<Block<'a>>,
}

impl<'a> TextPrompt<'a> {
    #[must_use]
    pub const fn new(message: Cow<'a, str>) -> Self {
        Self {
            message,
            block: None,
        }
    }

    #[must_use]
    // const causes: error[E0493]: destructor of `std::option::Option<ratatui::widgets::Block<'_>>` cannot be evaluated at compile-time
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Prompt for TextPrompt<'_> {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the `Widget` trait implementation as we need the `Frame` to set the
    /// cursor position.
    fn draw<B: Backend>(self, frame: &mut Frame<B>, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.focus == Focus::Focused {
            frame.set_cursor(state.cursor.0, state.cursor.1);
        }
    }
}

impl<'a> StatefulWidget for TextPrompt<'a> {
    type State = TextState<'a>;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_block(&mut area, buf);

        let width = area.width as usize;
        let height = area.height as usize;

        // The first line will have the parts that are static and the first part of the value that
        // fits in the area.
        let prompt_length = format!("? {} › ", self.message).len();
        let mut first_line = Line::from(vec![
            state.status.symbol(),
            " ".into(),
            self.message.bold(),
            " › ".cyan().dim(),
        ]);

        // Add the first line of the value to the first line of the prompt.
        let first_line_length = width - prompt_length;
        let first_line_value: Span = state
            .value
            .chars()
            .take(first_line_length)
            .collect::<String>()
            .into();
        first_line.spans.push(first_line_value);

        // Each successive line will have the next part of the value that fits in the area, which
        // is calculated by skipping the characters that were already rendered and then splitting
        // the remaining characters into chunks that fit in the area. E.g. this will look like:
        // ```text
        // ? Enter your name › John Doe
        // is my name and I am 99 years
        // old.
        // ```
        let mut lines = state
            .value
            .chars()
            .skip(first_line_length)
            .chunks(width)
            .into_iter()
            .map(|c| Line::from(c.collect::<String>()))
            .take(height - 1)
            .collect_vec();
        lines.insert(0, first_line);

        // calculate the cursor position
        let position = u16_or(state.position + prompt_length, u16::MAX);
        let width = u16_or(width, 1);
        // TODO constrain to area.
        // TODO handle scrolling automatically.
        state.cursor = (area.x + position % width, area.y + position / width);
        Paragraph::new(lines).render(area, buf);
    }
}

impl<'a> TextPrompt<'a> {
    fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) {
        if let Some(block) = self.block.take() {
            let inner = block.inner(*area);
            block.render(*area, buf);
            *area = inner;
        };
    }
}

impl<T> From<T> for TextPrompt<'static>
where
    T: Into<Cow<'static, str>>,
{
    fn from(message: T) -> Self {
        Self::new(message.into())
    }
}

fn u16_or(x: usize, default: u16) -> u16 {
    x.try_into().unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;
    use ratatui::assert_buffer_eq;

    // TODO make these configurable
    const PENDING_STYLE: Style = Style::new().fg(Color::Cyan);
    const COMPLETE_STYLE: Style = Style::new().fg(Color::Green);
    const ABORTED_STYLE: Style = Style::new().fg(Color::Red);
    const MESSAGE_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
    const MESSAGE_SEPERATOR_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::DIM);

    #[test]
    fn new() {
        const PROMPT: TextPrompt<'_> = TextPrompt::new(Cow::Borrowed("Enter your name"));
        assert_eq!(PROMPT.message, "Enter your name");
    }

    #[test]
    fn from() {
        let prompt = TextPrompt::from("Enter your name");
        assert_eq!(prompt.message, "Enter your name");
    }
    #[test]
    fn render() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["? prompt ›     "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), MESSAGE_STYLE);
        expected.set_style(Rect::new(8, 0, 3, 1), MESSAGE_SEPERATOR_STYLE);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_with_done() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_status(Status::Done);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["✔ prompt ›     "]);
        expected.set_style(Rect::new(0, 0, 1, 1), COMPLETE_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), MESSAGE_STYLE);
        expected.set_style(Rect::new(8, 0, 3, 1), MESSAGE_SEPERATOR_STYLE);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_with_aborted() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_status(Status::Aborted);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["✖ prompt ›     "]);
        expected.set_style(Rect::new(0, 0, 1, 1), ABORTED_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), MESSAGE_STYLE);
        expected.set_style(Rect::new(8, 0, 3, 1), MESSAGE_SEPERATOR_STYLE);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_with_value() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["? prompt › value              "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), MESSAGE_STYLE);
        expected.set_style(Rect::new(8, 0, 3, 1), MESSAGE_SEPERATOR_STYLE);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_with_block() {
        let prompt = TextPrompt::from("prompt")
            .with_block(Block::default().borders(Borders::ALL).title("Title"));
        let mut state = TextState::new();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec![
            "┌Title────────┐",
            "│? prompt ›   │",
            "└─────────────┘",
        ]);
        expected.set_style(Rect::new(1, 1, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(3, 1, 6, 1), MESSAGE_STYLE);
        expected.set_style(Rect::new(9, 1, 3, 1), MESSAGE_SEPERATOR_STYLE);
        assert_buffer_eq!(buffer, expected);
    }
}
