use std::{borrow::Cow, vec};

use crate::prelude::*;

use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

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
    render_style: TextRenderStyle,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextRenderStyle {
    #[default]
    Default,
    Password,
    Invisible,
}

impl TextRenderStyle {
    #[must_use]
    pub fn render(&self, state: &TextState) -> String {
        match self {
            Self::Default => state.value().to_string(),
            Self::Password => "*".repeat(state.value().len()),
            Self::Invisible => String::new(),
        }
    }
}

impl<'a> TextPrompt<'a> {
    #[must_use]
    pub const fn new(message: Cow<'a, str>) -> Self {
        Self {
            message,
            block: None,
            render_style: TextRenderStyle::Default,
        }
    }

    #[must_use]
    // const causes: error[E0493]: destructor of `std::option::Option<ratatui::widgets::Block<'_>>` cannot be evaluated at compile-time
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    #[must_use]
    pub const fn with_render_style(mut self, render_style: TextRenderStyle) -> Self {
        self.render_style = render_style;
        self
    }
}

impl Prompt for TextPrompt<'_> {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the `Widget` trait implementation as we need the `Frame` to set the
    /// cursor position.
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.is_focused() {
            frame.set_cursor(state.cursor().0, state.cursor().1);
        }
    }
}

impl<'a> StatefulWidget for TextPrompt<'a> {
    type State = TextState<'a>;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_block(&mut area, buf);

        let width = area.width as usize;
        let height = area.height as usize;
        let value = self.render_style.render(state);
        let value_length = value.len();

        let line = Line::from(vec![
            state.status().symbol(),
            " ".into(),
            self.message.bold(),
            " › ".cyan().dim(),
            Span::raw(value),
        ]);
        let prompt_length = line.width() - value_length;
        let lines = wrap(line, width).take(height).collect_vec();

        // constrain the position to the area
        let position = (state.position() + prompt_length).min(area.area() as usize - 1);
        let row = position / width;
        let column = position % width;
        // sizes are already constrained to the u16 range
        #[allow(clippy::cast_possible_truncation)]
        {
            *state.cursor_mut() = (area.x + column as u16, area.y + row as u16);
        }
        Paragraph::new(lines).render(area, buf);
    }
}

/// wraps a line into multiple lines of the given width.
///
/// This is a character based wrap, not a word based wrap.
///
/// TODO: move this into the `Line` type.
fn wrap(line: Line, width: usize) -> impl Iterator<Item = Line> {
    let mut line = line;
    std::iter::from_fn(move || {
        if line.width() > width {
            let (first, second) = line_split_at(line.clone(), width);
            line = second;
            Some(first)
        } else if line.width() > 0 {
            let first = line.clone();
            line = Line::default();
            Some(first)
        } else {
            None
        }
    })
}

/// splits a line into two lines at the given position.
///
/// TODO: move this into the `Line` type.
/// TODO: fix this so that it operates on multi-width characters.
fn line_split_at(line: Line, mid: usize) -> (Line, Line) {
    let mut first = Line::default();
    let mut second = Line::default();
    first.alignment = line.alignment;
    second.alignment = line.alignment;
    for span in line.spans {
        let first_width = first.width();
        let span_width = span.width();
        if first_width + span_width <= mid {
            first.spans.push(span);
        } else if first_width < mid && first_width + span_width > mid {
            let span_mid = mid - first_width;
            let (span_first, span_second) = span_split_at(span, span_mid);
            first.spans.push(span_first);
            second.spans.push(span_second);
        } else {
            second.spans.push(span);
        }
    }
    (first, second)
}

/// splits a span into two spans at the given position.
///
/// TODO: move this into the `Span` type.
/// TODO: fix this so that it operates on multi-width characters.
#[allow(clippy::needless_pass_by_value)]
fn span_split_at(span: Span, mid: usize) -> (Span, Span) {
    let (first, second) = span.content.split_at(mid);
    let first = Span {
        content: Cow::Owned(first.into()),
        style: span.style,
    };
    let second = Span {
        content: Cow::Owned(second.into()),
        style: span.style,
    };
    (first, second)
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

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;
    use ratatui::{assert_buffer_eq, backend::TestBackend, widgets::Borders};

    // TODO make these configurable
    const PENDING_STYLE: Style = Style::new().fg(Color::Cyan);
    const COMPLETE_STYLE: Style = Style::new().fg(Color::Green);
    const ABORTED_STYLE: Style = Style::new().fg(Color::Red);

    #[test]
    fn new() {
        const PROMPT: TextPrompt<'_> = TextPrompt::new(Cow::Borrowed("Enter your name"));
        assert_eq!(PROMPT.message, "Enter your name");
        assert_eq!(PROMPT.block, None);
        assert_eq!(PROMPT.render_style, TextRenderStyle::Default);
    }

    #[test]
    fn default() {
        let prompt = TextPrompt::default();
        assert_eq!(prompt.message, "");
        assert_eq!(prompt.block, None);
        assert_eq!(prompt.render_style, TextRenderStyle::Default);
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
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
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
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_with_aborted() {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_status(Status::Aborted);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["✘ prompt ›     "]);
        expected.set_style(Rect::new(0, 0, 1, 1), ABORTED_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
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
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
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
        expected.set_style(Rect::new(3, 1, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(9, 1, 3, 1), Style::new().cyan().dim());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_password() {
        let prompt = TextPrompt::from("prompt").with_render_style(TextRenderStyle::Password);
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["? prompt › *****              "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn render_invisible() {
        let prompt = TextPrompt::from("prompt").with_render_style(TextRenderStyle::Invisible);
        let mut state = TextState::new().with_value("value");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));

        prompt.render(buffer.area, &mut buffer, &mut state);

        let mut expected = Buffer::with_lines(vec!["? prompt ›                    "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn draw_no_wrap() -> Result<(), Box<dyn std::error::Error>> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello");
        let backend = TestBackend::new(17, 2);
        let mut terminal = Terminal::new(backend)?;

        let mut expected = Buffer::with_lines(vec!["? prompt › hello ", "                 "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());

        // The cursor is not changed when the prompt is not focused.
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.backend_mut().get_cursor().unwrap(), (0, 0));

        // The cursor is changed when the prompt is focused.
        state.focus();
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.backend_mut().get_cursor().unwrap(), (11, 0));

        // The cursor is changed when the prompt is focused and the position is changed.
        *state.position_mut() = 3;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (14, 0));
        assert_eq!(terminal.get_cursor()?, (14, 0));

        // The cursor does not go beyond the end of the value.
        *state.position_mut() = 100;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (16, 0));
        assert_eq!(terminal.get_cursor()?, (16, 0));

        Ok(())
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn draw_wrapped() -> Result<(), Box<dyn std::error::Error>> {
        let prompt = TextPrompt::from("prompt");
        let mut state = TextState::new().with_value("hello world");
        let backend = TestBackend::new(17, 2);
        let mut terminal = Terminal::new(backend)?;

        let mut expected = Buffer::with_lines(vec!["? prompt › hello ", "world            "]);
        expected.set_style(Rect::new(0, 0, 1, 1), PENDING_STYLE);
        expected.set_style(Rect::new(2, 0, 6, 1), Style::new().bold());
        expected.set_style(Rect::new(8, 0, 3, 1), Style::new().cyan().dim());

        // The cursor is not changed when the prompt is not focused.
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.get_cursor()?, (0, 0));

        // The cursor is changed when the prompt is focused.
        state.focus();
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (11, 0));
        assert_eq!(terminal.get_cursor()?, (11, 0));

        // The cursor is changed when the prompt is focused and the position is changed.
        *state.position_mut() = 3;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (14, 0));
        assert_eq!(terminal.get_cursor()?, (14, 0));

        // The cursor wraps to the first column of the next line
        *state.position_mut() = 6;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (0, 1));
        assert_eq!(terminal.get_cursor()?, (0, 1));

        // The cursor continues to cover the second line
        *state.position_mut() = 7;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (1, 1));
        assert_eq!(terminal.get_cursor()?, (1, 1));

        // The cursor does not go beyond the end of the value.
        *state.position_mut() = 100;
        let frame = terminal.draw(|frame| prompt.clone().draw(frame, frame.size(), &mut state))?;
        assert_buffer_eq!(*frame.buffer, expected);
        assert_eq!(state.cursor(), (5, 1));
        assert_eq!(terminal.get_cursor()?, (5, 1));

        Ok(())
    }
}
