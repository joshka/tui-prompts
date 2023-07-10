use std::{borrow::Cow, vec};

use crate::{prelude::*, TextState};

use itertools::Itertools;
use ratatui::prelude::*;

/// A prompt widget that displays a message and a text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextPrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    /// The block to wrap the prompt in.
    block: Option<Block<'a>>,
    // TODO style the widget
    // TODO style each element of the widget.
}

impl<'a> TextPrompt<'a> {
    #[must_use]
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
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

    // TODO: this is a mess, clean it up.
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut area = area;
        if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            area = inner;
        }

        let width = area.width as usize;
        let height = area.height as usize;

        // The first line will have the parts that are static and the first part of the value that
        // fits in the area.
        let mut first_line = Line::from(vec![
            state.status.symbol(),
            " ".into(),
            self.message.bold(),
            " › ".cyan().dim(),
        ]);

        let static_part_width = first_line.width();
        let remaining_width_on_first_line = width - static_part_width;
        let remaining_span: Span = state
            .value
            .chars()
            .take(remaining_width_on_first_line)
            .collect::<String>()
            .into();
        let skip = remaining_span.width();
        first_line.spans.push(remaining_span);

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
            .skip(skip)
            .chunks(width)
            .into_iter()
            .map(|c| Line::from(c.collect::<String>()))
            .take(height - 1)
            .collect_vec();
        lines.insert(0, first_line);
        let position = u16_or(state.position + static_part_width, u16::MAX);
        let width = u16_or(width, 1);
        // TODO constrain to area.
        state.cursor = (area.x + position % width, area.y + position / width);
        Paragraph::new(lines).render(area, buf);
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
        let prompt = TextPrompt::new("Enter your name");
        assert_eq!(prompt.message, "Enter your name");
    }

    #[test]
    fn render() {
        let prompt = TextPrompt::new("prompt");
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
        let prompt = TextPrompt::new("prompt");
        let mut state = TextState::new().with_status(Status::Complete);
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
        let prompt = TextPrompt::new("prompt");
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
        let prompt = TextPrompt::new("prompt");
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
        let prompt = TextPrompt::new("prompt")
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
