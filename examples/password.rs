use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Frame;
use tui::Terminal;
use tui_prompts::{PasswordPrompt, PasswordState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut stdout = std::io::stdout();
    let mut backend = CrosstermBackend::new(&mut stdout);
    execute!(backend, crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    crossterm::terminal::enable_raw_mode()?;
    // Initialize password state
    let mut password_state = PasswordState::new();

    // Main event loop
    loop {
        // Draw UI
        terminal.draw(|frame| draw_ui(frame, &mut password_state))?;

        // Handle input
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Esc {
                break;
            }
            password_state.handle_input(key_event);
        }
    }

    // Cleanup terminal
    terminal.clear()?;
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}

fn draw_ui<B>(f: &mut Frame<B>, password_state: &mut PasswordState)
where
    B: tui::backend::Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let block = Block::default()
        .title("Password Widget")
        .borders(Borders::ALL);
    let inner = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let password_widget = PasswordPrompt::new("Tell me a secret");
    f.render_stateful_widget(password_widget, inner, password_state);
}
