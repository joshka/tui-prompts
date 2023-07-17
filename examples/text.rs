mod tui;

use std::{thread::sleep, time::Duration};

use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tui::Tui;
use tui_prompts::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = App::new(cli);
    app.run()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct App<'a> {
    debug: bool,
    current_field: Field,
    username_state: TextState<'a>,
    password_state: TextState<'a>,
    invisible_state: TextState<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Username,
    Password,
    Invisible,
}

impl<'a> App<'a> {
    const fn new(cli: Cli) -> Self {
        Self {
            debug: cli.debug,
            current_field: Field::Username,
            username_state: TextState::new(),
            password_state: TextState::new(),
            invisible_state: TextState::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        *self.current_state().focus_state_mut() = FocusState::Focused;
        while !self.is_finished() {
            self.handle_events()?;
            tui.draw(|frame| self.draw_ui(frame))?;
        }
        tui.hide_cursor()?;
        // wait a second before exiting so the user can see the final state of the UI.
        sleep(Duration::from_secs(1));
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event);
            }
        }
        Ok(())
    }

    fn draw_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        self.draw_app(frame);
        self.draw_debug(frame);
    }

    fn draw_app<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let area = Rect {
            width: 30,
            ..frame.size()
        };
        TextPrompt::from("Username").draw(frame, area, &mut self.username_state);

        let area = Rect {
            y: area.y + self.username_state.render_height as u16,
            height: area.height - self.username_state.render_height as u16,
            ..area
        };
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, area, &mut self.password_state);

        let area = Rect {
            y: area.y + self.password_state.render_height as u16,
            height: area.height - self.password_state.render_height as u16,
            ..area
        };
        TextPrompt::from("Invisible")
            .with_render_style(TextRenderStyle::Invisible)
            .draw(frame, area, &mut self.invisible_state);

        let state = self.current_state();
        let state = format!("  Value: {}", state.value());
        let area = Rect {
            y: area.y + self.invisible_state.render_height as u16,
            height: frame.size().height - area.y - self.invisible_state.render_height as u16,
            ..frame.size()
        };
        frame.render_widget(
            Paragraph::new(state).style(Style::new().fg(Color::DarkGray)),
            area,
        );
    }

    fn draw_debug<B: Backend>(&mut self, frame: &mut Frame<B>) {
        if !self.debug {
            return;
        }
        let state = self.current_state();
        let debug = format!("{state:#?}");
        let area = Rect::new(frame.size().width - 30, 0, 30, 20);
        frame.render_widget(Paragraph::new(debug), area);
    }

    fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.invisible_state.is_finished()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (event::KeyCode::Enter, _) => {
                self.submit();
            }
            (event::KeyCode::Tab, KeyModifiers::NONE) => {
                self.current_state().blur();
                self.current_field = self.next_field();
                self.current_state().focus();
            }
            (event::KeyCode::BackTab, KeyModifiers::SHIFT) => {
                self.current_state().blur();
                self.current_field = self.prev_field();
                self.current_state().focus();
            }
            _ => {
                let state = self.current_state();
                state.handle_key_event(key_event);
            }
        }
    }

    fn submit(&mut self) {
        self.current_state().complete();
        if self.current_state().is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        }
    }

    fn next_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Invisible,
            Field::Invisible => Field::Username,
        }
    }

    fn prev_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Invisible,
            Field::Password => Field::Username,
            Field::Invisible => Field::Password,
        }
    }

    fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
            Field::Invisible => &mut self.invisible_state,
        }
    }
}
