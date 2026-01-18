mod app;
mod audio;
mod languages;
mod llm;
mod problem;
mod syntax;

use anyhow::Result;
use app::{App, AppState};
use audio::AudioPlayer;
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
    dotenvy::dotenv().ok();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
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
    
    // Audio player for SFX
    let mut audio_player = AudioPlayer::new();
    let mut audio_playing = false;
    let mut prev_state_is_countdown = false;
    let mut prev_state_is_submitting = false;

    loop {
        // Render
        terminal.draw(|f| app.render(f))?;

        // Poll for async execution output
        app.poll_execution();
        app.poll_translation();
        
        // Handle audio: play different sounds based on app state
        if let Some(ref mut player) = audio_player {
            let is_countdown = matches!(app.state, AppState::Countdown(_));
            let is_transitioning = matches!(app.state, AppState::Transitioning(_));
            let is_submitting = matches!(app.state, AppState::Compiling(_) | AppState::Running | AppState::RevealingResults(_, _) | AppState::Results(_));
            
            // Language revealed during reveal phase (progress > 0.65)
            let language_revealed = match app.state {
                AppState::Revealing(progress) => progress > 0.65,
                _ => false,
            };
            
            // Play countdown sound when countdown window appears
            if is_countdown && !prev_state_is_countdown {
                player.play_countdown_sfx();
                prev_state_is_countdown = true;
            } else if !is_countdown {
                prev_state_is_countdown = false;
            }
            
            // Play start sound when transitioning begins (countdown hits 0)
            if is_transitioning && !audio_playing {
                player.play_start_sfx();
                audio_playing = true;
            } else if language_revealed && audio_playing {
                // Stop start sound and play end sound when language appears
                player.play_end_sfx();
                audio_playing = false;
            }
            
            // Play submission/results sound when compiling/running/results (sending to Piston onwards)
            if is_submitting && !prev_state_is_submitting {
                player.play_submission_sfx();
                prev_state_is_submitting = true;
            } else if !is_submitting {
                prev_state_is_submitting = false;
            }
        }

        // Calculate timeout for next tick
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Handle input
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        // Global quit with Cmd+Q or Ctrl+Q
                        if (key.modifiers.contains(KeyModifiers::SUPER) || key.modifiers.contains(KeyModifiers::CONTROL)) 
                            && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q')) {
                            return Ok(());
                        }

                        // Quit from results screen
                        if matches!(app.state, AppState::Results(_)) {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                                // Stop audio before quitting
                                if let Some(ref mut player) = audio_player {
                                    player.stop();
                                }
                                return Ok(());
                            }
                            // Stop audio on restart (R key)
                            if key.code == KeyCode::Enter || key.code == KeyCode::Char('r') {
                                if let Some(ref mut player) = audio_player {
                                    player.stop();
                                }
                                prev_state_is_submitting = false; // Reset state tracker
                            }
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

        // Tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = std::time::Instant::now();
        }
    }
}
