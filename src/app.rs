use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

use crate::languages::{convert_code, Language};
use crate::problem::{run_tests, Problem, TestResults};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Coding,
    Transitioning(f32), // 0.0 to 1.0 progress
    Results,
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
            randomize_interval: Duration::from_secs(45),
            test_results: None,
            scroll_offset: 0,
            transition_start: None,
            glitch_frame: 0,
        }
    }

    pub fn tick(&mut self) {
        self.glitch_frame = (self.glitch_frame + 1) % 10;

        // Check if it's time to randomize
        if self.state == AppState::Coding {
            let elapsed = self.last_randomize.elapsed();
            if elapsed >= self.randomize_interval {
                self.start_transition();
            }
        }

        // Update transition progress
        if let AppState::Transitioning(_progress) = self.state {
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
            AppState::Results => self.handle_results_key(key),
            AppState::Transitioning(_) => {}, // No input during transition
        }
    }

    fn handle_coding_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Cmd+S or Ctrl+S to submit
            (KeyModifiers::SUPER, KeyCode::Char('s')) | (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                self.submit();
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

    fn submit(&mut self) {
        // Convert to Python and run tests
        let python_code = if self.current_language != Language::Python {
            convert_code(&self.code, self.current_language, Language::Python)
        } else {
            self.code.clone()
        };

        let results = run_tests(&python_code, &self.problem);
        self.test_results = Some(results);
        self.state = AppState::Results;
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match &self.state {
            AppState::Coding => self.render_coding(frame),
            AppState::Transitioning(progress) => self.render_transition(frame, *progress),
            AppState::Results => self.render_results(frame),
        }
    }

    fn render_coding(&mut self, frame: &mut Frame) {
        let size = frame.size();
        
        // Main layout: header + content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(2),  // Footer
            ])
            .split(size);

        // Header with arcade styling
        self.render_header(frame, chunks[0]);

        // Split content: 1/3 problem, 2/3 editor
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(67),
            ])
            .split(chunks[1]);

        // Render problem description
        self.render_problem(frame, content_chunks[0]);

        // Render code editor
        self.render_editor(frame, content_chunks[1]);

        // Footer with timer
        self.render_footer(frame, chunks[2]);
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

        let footer_text = vec![
            Span::styled("⏱ NEXT RANDOMIZE: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}s ", secs), Style::default().fg(timer_color).add_modifier(Modifier::BOLD)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Cmd+S ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("to Submit ", Style::default().fg(Color::White)),
        ];

        let footer = Paragraph::new(Line::from(footer_text))
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

    fn render_results(&self, frame: &mut Frame, ) {
        let size = frame.size();
        
        if let Some(results) = &self.test_results {
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
