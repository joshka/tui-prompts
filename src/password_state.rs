use std::borrow::Cow;

use crate::{prelude::*, State};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PasswordState<'a> {
    pub status: Status,
    pub focus: Focus,
    pub position: usize,
    pub cursor: (u16, u16),
    pub value: Cow<'a, str>,
}

impl<'a> PasswordState<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = value.into();
        self
    }

    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub const fn with_focus(mut self, focus: Focus) -> Self {
        self.focus = focus;
        self
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self.status, Status::Complete | Status::Aborted)
    }
}

impl State for PasswordState<'_> {
    fn status(&self) -> Status {
        self.status
    }

    fn focus(&self) -> Focus {
        self.focus
    }

    fn position(&self) -> usize {
        self.position
    }

    fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    fn focus_mut(&mut self) -> &mut Focus {
        &mut self.focus
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }

    fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    fn cursor_mut(&mut self) -> &mut (u16, u16) {
        &mut self.cursor
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        self.value.to_mut()
    }
}
