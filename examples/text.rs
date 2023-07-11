mod tui;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::prelude::*;

use tui::Tui;
use tui_prompts::{prelude::*, TextPrompt, TextState};

fn main() -> Result<()> {
    run()?;
    println!(); // to prevent the cursor from being on the same line as the prompt.
    Ok(())
}

fn run() -> Result<(), color_eyre::Report> {
    let mut tui = Tui::new()?;
    let mut state = TextState::new().with_focus(Focus::Focused);
    loop {
        tui.draw(|frame| draw_ui(frame, &mut state))?;
        if state.is_finished() {
            break;
        }
        if let Event::Key(key_event) = event::read()? {
            state.handle_key_event(key_event);
        }
    }
    // prevents the tui output from being overwritten by the prompt.
    tui.backend_mut().append_lines(2)?;
    Ok(())
}

fn draw_ui<B: Backend>(frame: &mut Frame<B>, state: &mut TextState) {
    let area = Rect::new(0, 0, 25, 4);
    let prompt = TextPrompt::from("Text");
    prompt.draw(frame, area, state);
}
