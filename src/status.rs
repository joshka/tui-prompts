use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// The result of a prompt.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    #[default]
    Pending,
    Aborted,
    Done,
}

impl Status {
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::Done | Self::Aborted)
    }

    #[must_use]
    pub fn symbol(&self) -> Span<'static> {
        match self {
            Self::Pending => Symbols::default().pending,
            Self::Aborted => Symbols::default().aborted,
            Self::Done => Symbols::default().done,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbols {
    pub pending: Span<'static>,
    pub aborted: Span<'static>,
    pub done: Span<'static>,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            pending: Span::styled("?", Style::new().fg(Color::Cyan)),
            aborted: Span::styled("✘", Style::new().fg(Color::Red)),
            done: Span::styled("✔", Style::new().fg(Color::Green)),
        }
    }
}
