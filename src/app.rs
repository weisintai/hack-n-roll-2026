use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use crate::languages::{convert_code, Language};
use crate::problem::{run_tests_on_piston, Problem, TestResults};

// Configuration constants
const LANGUAGE_CHANGE_INTERVAL_SECS: u64 = 45;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Coding,
    Transitioning(f32), // 0.0 to 1.0 progress
    Compiling(f32),     // 0.0 to 1.0 progress (simulated)
    Running,            // Executing on Piston
    Finished(String),   // Final status message
    Results(TestResults),
}

#[derive(Debug, Clone)]
pub enum ExecutionEvent {
    Log(OutputLine),
    Finished(TestResults),
}

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub is_error: bool,
}

pub struct App {
    pub problem: Problem,
    pub code: String,
    pub cursor_position: usize,
    pub current_language: Language,
    pub state: AppState,
    pub last_randomize: Instant,
    pub randomize_interval: Duration,
    pub test_results: Option<TestResults>,
    pub scroll_offset: usize,
    pub transition_start: Option<Instant>,
    pub glitch_frame: usize,
    
    // Async execution
    pub output_rx: Option<mpsc::Receiver<ExecutionEvent>>,
    pub execution_output: Vec<OutputLine>,
    pub execution_progress: f32,
    pub show_output_panel: bool,
}

impl App {
    pub fn new() -> Self {
        let current_language = Language::Python;
        let problem = Problem::two_sum();
        
        Self {
            problem,
            code: String::from("def two_sum(nums, target):\n    # Write your solution here\n    pass\n"),
            cursor_position: 0,
            current_language,
            state: AppState::Coding,
            last_randomize: Instant::now(),
            randomize_interval: Duration::from_secs(LANGUAGE_CHANGE_INTERVAL_SECS),
            test_results: None,
            scroll_offset: 0,
            transition_start: None,
            glitch_frame: 0,
            output_rx: None,
            execution_output: Vec::new(),
            execution_progress: 0.0,
            show_output_panel: false,
        }
    }

    pub fn tick(&mut self) {
        self.glitch_frame = (self.glitch_frame + 1) % 10;

        match self.state {
            AppState::Coding => {
                let elapsed = self.last_randomize.elapsed();
                if elapsed >= self.randomize_interval {
                    self.start_transition();
                }
            }
            AppState::Transitioning(_progress) => {
                if let Some(start) = self.transition_start {
                    let elapsed = start.elapsed().as_secs_f32();
                    let new_progress = (elapsed / 2.0).min(1.0); // 2 second transition
                    
                    if new_progress >= 1.0 {
                        self.complete_transition();
                    } else {
                        self.state = AppState::Transitioning(new_progress);
                    }
                }
            }
            AppState::Compiling(mut progress) => {
                // Simulate compilation progress
                progress += 0.02;
                if progress >= 1.0 {
                    self.state = AppState::Running;
                    self.execution_progress = 0.0;
                } else {
                    self.state = AppState::Compiling(progress);
                }
            }
            _ => {}
        }
    }

    pub fn poll_execution(&mut self) {
        let mut should_close = false;
        if let Some(rx) = &mut self.output_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    ExecutionEvent::Log(line) => {
                        self.execution_output.push(line);
                        // Auto-scroll
                        if self.execution_output.len() > 10 {
                           self.scroll_offset = self.execution_output.len() - 10;
                        }
                    }
                    ExecutionEvent::Finished(results) => {
                        self.test_results = Some(results.clone());
                        self.state = AppState::Results(results);
                        should_close = true;
                    }
                }
            }
        }
        
        if should_close {
            self.output_rx = None;
        }
    }

    fn start_transition(&mut self) {
        self.transition_start = Some(Instant::now());
        self.state = AppState::Transitioning(0.0);
    }

    fn complete_transition(&mut self) {
        // Randomize language
        let new_lang = self.current_language.random_except();
        self.code = convert_code(&self.code, self.current_language, new_lang);
        self.current_language = new_lang;
        
        // Reset timer
        self.last_randomize = Instant::now();
        self.state = AppState::Coding;
        self.transition_start = None;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Coding => self.handle_coding_key(key),
            AppState::Results(_) => self.handle_results_key(key),
            AppState::Finished(_) => self.handle_finished_key(key),
             _ => {}, // Ignore input during transitions and execution
        }
    }

    fn handle_coding_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Cmd+S or Ctrl+S to submit
            (KeyModifiers::SUPER, KeyCode::Char('s')) | (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                self.submit();
            }
            // Cmd+R or Ctrl+R to run (show output)
            (KeyModifiers::SUPER, KeyCode::Char('r')) | (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                self.show_output_panel = true;
                self.run_code();
            }
            // Regular typing
            (_, KeyCode::Char(c)) if !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::SUPER) => {
                self.insert_char(c);
            }
            (_, KeyCode::Enter) => {
                self.insert_char('\n');
            }
            (_, KeyCode::Backspace) => {
                self.delete_char();
            }
            (_, KeyCode::Tab) => {
                self.insert_char('\t');
            }
            (_, KeyCode::Left) => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            (_, KeyCode::Right) => {
                if self.cursor_position < self.code.len() {
                    self.cursor_position += 1;
                }
            }
            (_, KeyCode::Up) => {
                self.move_cursor_up();
            }
            (_, KeyCode::Down) => {
                self.move_cursor_down();
            }
            _ => {}
        }
    }

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char('r') => {
                // Restart
                *self = App::new();
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                // Keep results visible, could add exit logic here
            }
            _ => {}
        }
    }

    fn handle_finished_key(&mut self, key: KeyEvent) {
         match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                 self.state = AppState::Coding; // Return to editor on error/finish without results
            }
            _ => {}
         }
    }


    fn insert_char(&mut self, c: char) {
        self.code.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let prev_char_boundary = self.code[..self.cursor_position]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.code.remove(prev_char_boundary);
            self.cursor_position = prev_char_boundary;
        }
    }

    fn move_cursor_up(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        let mut current_line = 0;
        let mut col_in_line = 0;

        for (i, line) in lines.iter().enumerate() {
            if current_pos + line.len() >= self.cursor_position {
                current_line = i;
                col_in_line = self.cursor_position - current_pos;
                break;
            }
            current_pos += line.len() + 1; // +1 for newline
        }

        if current_line > 0 {
            let prev_line = lines[current_line - 1];
            let new_col = col_in_line.min(prev_line.len());
            let mut new_pos = 0;
            for (i, _) in lines.iter().enumerate() {
                if i == current_line - 1 {
                    new_pos += new_col;
                    break;
                }
                new_pos += lines[i].len() + 1;
            }
            self.cursor_position = new_pos;
        }
    }

    fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        let mut current_line = 0;
        let mut col_in_line = 0;

        for (i, line) in lines.iter().enumerate() {
            if current_pos + line.len() >= self.cursor_position {
                current_line = i;
                col_in_line = self.cursor_position - current_pos;
                break;
            }
            current_pos += line.len() + 1;
        }

        if current_line < lines.len() - 1 {
            let next_line = lines[current_line + 1];
            let new_col = col_in_line.min(next_line.len());
            let mut new_pos = 0;
            for (i, _) in lines.iter().enumerate() {
                if i == current_line + 1 {
                    new_pos += new_col;
                    break;
                }
                new_pos += lines[i].len() + 1;
            }
            self.cursor_position = new_pos;
        }
    }

    fn run_code(&mut self) {
        self.execution_output.clear();
        self.execution_output.push(OutputLine { 
            text: "Running code on Piston API...".to_string(), 
            is_error: false 
        });

        let (tx, rx) = mpsc::channel(32);
        self.output_rx = Some(rx);
        
        // Clone data for async task
        let code = self.code.clone();
        let problem = self.problem.clone();
        let language = self.current_language;
        
        // Spawn async execution (don't change state to Results)
        tokio::spawn(async move {
             let _results = run_tests_on_piston(code, problem, language, tx.clone()).await;
             // Just log completion, don't send Results event
             let _ = tx.send(ExecutionEvent::Log(OutputLine {
                 text: "Execution completed.".to_string(),
                 is_error: false
             })).await;
        });
    }

    fn submit(&mut self) {
        self.state = AppState::Compiling(0.0);
        self.execution_output.clear();
        self.execution_output.push(OutputLine { 
            text: "Compiling and sending to Piston API...".to_string(), 
            is_error: false 
        });

        let (tx, rx) = mpsc::channel(32);
        self.output_rx = Some(rx);
        
        // Clone data for async task
        let code = self.code.clone();
        let problem = self.problem.clone();
        let language = self.current_language;
        
        // Spawn async token
        tokio::spawn(async move {
             let results = run_tests_on_piston(code, problem, language, tx.clone()).await;
             
             // Send results
             let _ = tx.send(ExecutionEvent::Finished(results)).await;
        });
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match &self.state {
            AppState::Coding => self.render_coding(frame),
            AppState::Transitioning(progress) => self.render_transition(frame, *progress),
            AppState::Compiling(progress) => self.render_compiling(frame, *progress),
            AppState::Running => self.render_running(frame),
            AppState::Finished(msg) => self.render_finished(frame, msg),
            AppState::Results(results) => self.render_results(frame, results),
        }
    }
    
    fn render_compiling(&self, frame: &mut Frame, progress: f32) {
        let size = frame.size();
        let area = centered_rect(60, 20, size);
        
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Compiling "))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent((progress * 100.0) as u16);
            
        frame.render_widget(gauge, area);
    }

    fn render_running(&self, frame: &mut Frame) {
        let size = frame.size();
        let area = centered_rect(80, 60, size);
        
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Executing on Piston Container ")
            .border_style(Style::default().fg(Color::Yellow));
            
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let lines: Vec<Line> = self.execution_output.iter().map(|line| {
            Line::from(Span::styled(
                &line.text, 
                if line.is_error { Style::default().fg(Color::Red) } else { Style::default().fg(Color::White) }
            ))
        }).collect();
        
        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset as u16, 0));
            
        frame.render_widget(paragraph, inner_area);
    }

    fn render_finished(&self, frame: &mut Frame, msg: &str) {
         let size = frame.size();
        let area = centered_rect(60, 20, size);
        
        let text = vec![
            Line::from(Span::styled("Execution Finished", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(msg),
            Line::from(""),
            Line::from(Span::styled("Press Enter to continue", Style::default().fg(Color::DarkGray))),
        ];
        
        let p = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)))
            .alignment(Alignment::Center);
            
        frame.render_widget(p, area);
    }


    fn render_coding(&mut self, frame: &mut Frame) {
        let size = frame.size();
        
        // Main layout: header + content + footer
        let main_chunks = if self.show_output_panel {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),   // Header
                    Constraint::Min(10),     // Content (problem + editor)
                    Constraint::Length(12),  // Output panel
                    Constraint::Length(2),   // Footer
                ])
                .split(size)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Min(0),     // Content
                    Constraint::Length(2),  // Footer
                ])
                .split(size)
        };

        // Header with arcade styling
        self.render_header(frame, main_chunks[0]);

        // Split content: 1/3 problem, 2/3 editor
        let content_area = if self.show_output_panel { main_chunks[1] } else { main_chunks[1] };
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(67),
            ])
            .split(content_area);

        // Render problem description
        self.render_problem(frame, content_chunks[0]);

        // Render code editor
        self.render_editor(frame, content_chunks[1]);

        // Render output panel if visible
        if self.show_output_panel {
            self.render_output_panel(frame, main_chunks[2]);
        }

        // Footer with timer
        let footer_idx = if self.show_output_panel { 3 } else { 2 };
        self.render_footer(frame, main_chunks[footer_idx]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = vec![
            Span::styled("╔═══════════════════════════════════════╗", Style::default().fg(Color::Cyan)),
            Span::raw("\n"),
            Span::styled("║ ", Style::default().fg(Color::Cyan)),
            Span::styled("⚡ CODE ARCADE ⚡ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("LANGUAGE ROULETTE", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::styled(" ║", Style::default().fg(Color::Cyan)),
            Span::raw("\n"),
            Span::styled("╚═══════════════════════════════════════╝", Style::default().fg(Color::Cyan)),
        ];
        
        let header = Paragraph::new(Line::from(title))
            .alignment(Alignment::Center);
        
        frame.render_widget(header, area);
    }

    fn render_problem(&self, frame: &mut Frame, area: Rect) {
        let mut text = vec![
            Line::from(vec![
                Span::styled(&self.problem.title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Description:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
        ];

        for line in self.problem.description.lines() {
            text.push(Line::from(Span::styled(line, Style::default().fg(Color::White))));
        }

        text.push(Line::from(""));
        text.push(Line::from(Span::styled("Examples:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
        text.push(Line::from(""));

        for example in &self.problem.examples {
            for line in example.lines() {
                text.push(Line::from(Span::styled(line, Style::default().fg(Color::Gray))));
            }
            text.push(Line::from(""));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(Span::styled(" PROBLEM ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    }

    fn render_editor(&self, frame: &mut Frame, area: Rect) {
        let lines: Vec<Line> = self.code.lines().enumerate().map(|(i, line)| {
            let line_num = format!("{:>3} ", i + 1);
            let mut spans = vec![
                Span::styled(line_num, Style::default().fg(Color::DarkGray)),
            ];
            spans.push(Span::styled(line, Style::default().fg(Color::White)));
            Line::from(spans)
        }).collect();

        let title = format!(" EDITOR - {} ", self.current_language.display_name());
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .title(Span::styled(title, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    }

    fn render_output_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Output (Cmd+R to run) ")
            .border_style(Style::default().fg(Color::Green));
            
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let lines: Vec<Line> = self.execution_output.iter().map(|line| {
            Line::from(Span::styled(
                &line.text, 
                if line.is_error { 
                    Style::default().fg(Color::Red) 
                } else { 
                    Style::default().fg(Color::White) 
                }
            ))
        }).collect();
        
        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset as u16, 0));
            
        frame.render_widget(paragraph, inner_area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let elapsed = self.last_randomize.elapsed();
        let remaining = self.randomize_interval.saturating_sub(elapsed);
        let secs = remaining.as_secs();
        
        let timer_color = if secs < 10 {
            Color::Red
        } else if secs < 20 {
            Color::Yellow
        } else {
            Color::Green
        };

        let mut footer_spans = vec![
            Span::styled("⏱ NEXT RANDOMIZE: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}s ", secs), Style::default().fg(timer_color).add_modifier(Modifier::BOLD)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Cmd+S ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("Submit ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Cmd+R ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ];
        
        if self.show_output_panel {
            footer_spans.push(Span::styled("Run", Style::default().fg(Color::White)));
        } else {
            footer_spans.push(Span::styled("Run & Show Output", Style::default().fg(Color::White)));
        }

        let footer = Paragraph::new(Line::from(footer_spans))
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black));
        
        frame.render_widget(footer, area);
    }

    fn render_transition(&self, frame: &mut Frame, progress: f32) {
        let size = frame.size();
        
        // Create glitch effect
        let glitch_chars = ["█", "▓", "▒", "░", "▀", "▄", "▌", "▐"];
        let char_idx = (progress * glitch_chars.len() as f32) as usize % glitch_chars.len();
        
        let mut lines = vec![];
        let height = size.height as usize;
        let width = size.width as usize;
        
        for i in 0..height {
            let intensity = ((i as f32 / height as f32) - progress).abs();
            let color = if intensity < 0.1 {
                Color::Cyan
            } else if intensity < 0.3 {
                Color::Magenta
            } else {
                Color::Blue
            };
            
            let mut line_text = String::new();
            for _ in 0..width {
                if rand::random::<f32>() < progress {
                    line_text.push_str(glitch_chars[char_idx]);
                } else {
                    line_text.push(' ');
                }
            }
            
            lines.push(Line::from(Span::styled(line_text, Style::default().fg(color))));
        }
        
        // Overlay transition message
        let message = vec![
            Line::from(""),
            Line::from(Span::styled("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓", Style::default().fg(Color::Cyan))),
            Line::from(""),
            Line::from(Span::styled("   RANDOMIZING CODE...   ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled(format!("   PROGRESS: {}%   ", (progress * 100.0) as u8), Style::default().fg(Color::White))),
            Line::from(""),
            Line::from(Span::styled("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓", Style::default().fg(Color::Cyan))),
        ];
        
        let bg = Paragraph::new(lines);
        frame.render_widget(bg, size);
        
        let popup_area = centered_rect(30, 20, size);
        let popup = Paragraph::new(message)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
        
        frame.render_widget(popup, popup_area);
    }

    fn render_results(&self, frame: &mut Frame, results: &TestResults) {
        let size = frame.size();
        
        let score_percent = (results.passed as f32 / results.total as f32 * 100.0) as u8;
        let score_color = if score_percent >= 80 {
            Color::Green
        } else if score_percent >= 50 {
            Color::Yellow
        } else {
            Color::Red
        };

            let mut text = vec![
                Line::from(""),
                Line::from(Span::styled("╔═══════════════════════════════════════╗", Style::default().fg(Color::Cyan))),
                Line::from(Span::styled("║          RESULTS SCREEN              ║", Style::default().fg(Color::Cyan))),
                Line::from(Span::styled("╚═══════════════════════════════════════╝", Style::default().fg(Color::Cyan))),
                Line::from(""),
                Line::from(vec![
                    Span::styled("SCORE: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{}/{} ", results.passed, results.total), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("({}%)", score_percent), Style::default().fg(score_color)),
                ]),
                Line::from(""),
                Line::from(Span::styled("═══════════════════════════════════════", Style::default().fg(Color::DarkGray))),
                Line::from(""),
            ];

            for result in &results.details {
                let status = if result.passed {
                    Span::styled("✓ PASS", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled("✗ FAIL", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                };

                text.push(Line::from(vec![
                    status,
                    Span::styled(format!(" Test #{}", result.case_number), Style::default().fg(Color::White)),
                ]));
                text.push(Line::from(vec![
                    Span::styled("  Input: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&result.input, Style::default().fg(Color::Gray)),
                ]));
                text.push(Line::from(vec![
                    Span::styled("  Expected: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&result.expected, Style::default().fg(Color::Gray)),
                ]));
                if !result.passed {
                    text.push(Line::from(vec![
                        Span::styled("  Got: ", Style::default().fg(Color::Red)),
                        Span::styled(&result.actual, Style::default().fg(Color::Gray)),
                    ]));
                }
                text.push(Line::from(""));
            }

            text.push(Line::from(""));
            text.push(Line::from(Span::styled("═══════════════════════════════════════", Style::default().fg(Color::DarkGray))));
            text.push(Line::from(""));
            text.push(Line::from(Span::styled("Press 'R' to restart or 'Q' to quit", Style::default().fg(Color::Yellow))));

            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, size);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
