mod app;
mod languages;
mod llm;
mod problem;
mod syntax;

use anyhow::Result;
use app::{App, AppState};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Create app
    let mut app = App::new();

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| app.render(f))?;

        // Handle input with timeout for smooth animations
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        
                        // Global quit with Cmd+Q or Ctrl+Q
                        if (key.modifiers.contains(KeyModifiers::SUPER) || key.modifiers.contains(KeyModifiers::CONTROL)) 
                            && key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                        
                        // Quit from results screen
                        if key.code == KeyCode::Esc && app.state == AppState::Results {
                            return Ok(());
                        }
                        if key.code == KeyCode::Char('q') && app.state == AppState::Results {
                            return Ok(());
                        }
                        
                        app.handle_key(key);
                    }
                }
                Event::Mouse(mouse) => {
                    app.handle_mouse(mouse);
                }
                _ => {}
            }
        }

        // Tick for animations and timer
        app.tick();
    }
}
