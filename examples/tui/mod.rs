use std::{
    io::Stderr,
    ops::{Deref, DerefMut},
};

use color_eyre::Result;
use ratatui::prelude::*;

/// A wrapper around the terminal that enables raw mode on creation and disables it on drop.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = setup_terminal()?;
        Ok(Self { terminal })
    }
}

impl Deref for Tui {
    type Target = Terminal<CrosstermBackend<Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        cleanup_terminal().unwrap();
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stderr>>> {
    let stdout = std::io::stderr();
    let backend = CrosstermBackend::new(stdout);
    crossterm::terminal::enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn cleanup_terminal() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
