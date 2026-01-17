mod app;
mod languages;
mod problem;

use anyhow::Result;
use app::{App, AppState};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    // 60 FPS tick rate
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = std::time::Instant::now();

    loop {
        // Render
        terminal.draw(|f| app.render(f))?;

        // Poll for async execution output
        app.poll_execution();

        // Calculate timeout for next tick
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Handle input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Global quit
                    if key.code == KeyCode::Esc && matches!(app.state, AppState::Results(_)) {
                        return Ok(());
                    }
                    if key.code == KeyCode::Char('q') && matches!(app.state, AppState::Results(_)) {
                        return Ok(());
                    }
                    
                    app.handle_key(key);
                }
            }
        }

        // Tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = std::time::Instant::now();
        }
    }
}

