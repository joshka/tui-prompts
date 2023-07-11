mod tui;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::prelude::*;

use tui::Tui;
use tui_prompts::prelude::*;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;
    println!(); // to prevent the cursor from being on the same line as the prompt.
    Ok(())
}

struct App<'a> {
    username: TextState<'a>,
    password: TextState<'a>,
}

impl<'a> App<'a> {
    const fn new() -> Self {
        Self {
            username: TextState::new().with_focus(Focus::Focused),
            password: TextState::new(),
        }
    }

    fn is_finished(&self) -> bool {
        self.username.is_finished() && self.password.is_finished()
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        loop {
            tui.draw(|frame| self.draw_ui(frame))?;
            if self.is_finished() {
                break;
            }
            if let Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event);
            }
        }
        Ok(())
    }

    fn draw_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let area = Rect::new(0, 0, frame.size().width, 1);
        TextPrompt::from("Username").draw(frame, area, &mut self.username);

        let area = Rect::new(0, 1, frame.size().width, 1);
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, area, &mut self.password);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (event::KeyCode::Enter, _) => {
                if self.username.is_focused() {
                    self.username.handle_key_event(key_event);
                    *self.username.focus_mut() = Focus::Unfocused;
                    *self.password.focus_mut() = Focus::Focused;
                } else {
                    self.password.handle_key_event(key_event);
                    *self.password.focus_mut() = Focus::Unfocused;
                }
            }
            (event::KeyCode::Esc, _) => {
                *self.username.focus_mut() = Focus::Focused;
                *self.password.focus_mut() = Focus::Unfocused;
            }
            _ => {
                if self.username.is_focused() {
                    self.username.handle_key_event(key_event);
                } else {
                    self.password.handle_key_event(key_event);
                }
            }
        }
    }
}
