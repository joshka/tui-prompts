use std::borrow::Cow;

use ratatui::prelude::*;

use crate::{prelude::*, PasswordState};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PasswordPrompt<'a> {
    /// The message to display to the user before the input.
    message: Cow<'a, str>,
    /// The block to wrap the prompt in.
    block: Option<Block<'a>>,
    // TODO style the widget
    // TODO style each element of the widget.
}

impl<'a> PasswordPrompt<'a> {
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

impl Prompt for PasswordPrompt<'_> {
    fn draw<B: Backend>(self, frame: &mut Frame<B>, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.focus == Focus::Focused {
            frame.set_cursor(state.cursor.0, state.cursor.1);
        }
    }
}

impl<'a> StatefulWidget for PasswordPrompt<'a> {
    type State = PasswordState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut area = area;
        if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            area = inner;
        }

        let masked_password = state.value.chars().map(|_| '*').collect::<String>();
        let password_len = masked_password.len();
        let line = Line::from(vec![
            state.status.symbol(),
            " ".into(),
            self.message.bold(),
            " › ".cyan().dim(),
            masked_password.into(),
        ]);
        // TODO: constrain the cursor to the rect
        state.cursor = (
            area.x
                + u16_or(
                    (line.width() - password_len + state.position).min(area.width as usize),
                    0,
                ),
            area.y,
        );
        Paragraph::new(Text::from(line)).render(area, buf);
    }
}

fn u16_or(x: usize, default: u16) -> u16 {
    x.try_into().unwrap_or(default)
}
