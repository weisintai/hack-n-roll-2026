use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use crate::languages::{build_translation_prompt, Language};
use crate::llm;
use crate::problem::{run_tests_on_piston, Problem, TestResults};
use crate::syntax::SyntaxHighlighter;

// Configuration constants
const LANGUAGE_CHANGE_INTERVAL_SECS: u64 = 10;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Coding,
    Countdown(u8),           // 5, 4, 3, 2, 1
    Transitioning(f32),      // 0.0 to 1.0 progress (glitch effect)
    Revealing(f32),          // 0.0 to 1.0 progress (reveal new language/problem)
    Compiling(f32),     // 0.0 to 1.0 progress (simulated)
    Running,            // Executing on Piston
    Results(TestResults),
}

#[derive(Debug, Clone)]
pub enum ExecutionEvent {
    Log(OutputLine),
    Finished(TestResults),      // For submit - shows full results screen
    RunFinished(TestResults),    // For run - shows results in output panel
}

#[derive(Debug, Clone)]
pub enum TranslationEvent {
    Success(String),
    #[allow(dead_code)]
    Failure(String),
}

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub is_error: bool,
}

/// Generate box-drawing ASCII art for a single letter
fn get_letter_ascii(letter: char) -> Vec<String> {
    match letter.to_ascii_uppercase() {
        'A' => vec![
            " █████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "███████║".to_string(),
            "██╔══██║".to_string(),
            "██║  ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ],
        'B' => vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
        ],
        'C' => vec![
            " ██████╗".to_string(),
            "██╔════╝".to_string(),
            "██║     ".to_string(),
            "██║     ".to_string(),
            "╚██████╗".to_string(),
            " ╚═════╝".to_string(),
        ],
        'D' => vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██║  ██║".to_string(),
            "██║  ██║".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
        ],
        'E' => vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "█████╗  ".to_string(),
            "██╔══╝  ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
        ],
        'F' => vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "█████╗  ".to_string(),
            "██╔══╝  ".to_string(),
            "██║     ".to_string(),
            "╚═╝     ".to_string(),
        ],
        'G' => vec![
            " ██████╗ ".to_string(),
            "██╔════╝ ".to_string(),
            "██║  ███╗".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
        ],
        'H' => vec![
            "██╗  ██╗".to_string(),
            "██║  ██║".to_string(),
            "███████║".to_string(),
            "██╔══██║".to_string(),
            "██║  ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ],
        'I' => vec![
            "██╗".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "╚═╝".to_string(),
        ],
        'J' => vec![
            "     ██╗".to_string(),
            "     ██║".to_string(),
            "     ██║".to_string(),
            "██   ██║".to_string(),
            "╚█████╔╝".to_string(),
            " ╚════╝ ".to_string(),
        ],
        'K' => vec![
            "██╗  ██╗".to_string(),
            "██║ ██╔╝".to_string(),
            "█████╔╝ ".to_string(),
            "██╔═██╗ ".to_string(),
            "██║  ██╗".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ],
        'L' => vec![
            "██╗     ".to_string(),
            "██║     ".to_string(),
            "██║     ".to_string(),
            "██║     ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
        ],
        'M' => vec![
            "███╗   ███╗".to_string(),
            "████╗ ████║".to_string(),
            "██╔████╔██║".to_string(),
            "██║╚██╔╝██║".to_string(),
            "██║ ╚═╝ ██║".to_string(),
            "╚═╝     ╚═╝".to_string(),
        ],
        'N' => vec![
            "███╗   ██╗".to_string(),
            "████╗  ██║".to_string(),
            "██╔██╗ ██║".to_string(),
            "██║╚██╗██║".to_string(),
            "██║ ╚████║".to_string(),
            "╚═╝  ╚═══╝".to_string(),
        ],
        'O' => vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
        ],
        'P' => vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔═══╝ ".to_string(),
            "██║     ".to_string(),
            "╚═╝     ".to_string(),
        ],
        'Q' => vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "██║   ██║".to_string(),
            "██║▄▄ ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚══▀▀═╝ ".to_string(),
        ],
        'R' => vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔══██╗".to_string(),
            "██║  ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ],
        'S' => vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "███████╗".to_string(),
            "╚════██║".to_string(),
            "███████║".to_string(),
            "╚══════╝".to_string(),
        ],
        'T' => vec![
            "████████╗".to_string(),
            "╚══██╔══╝".to_string(),
            "   ██║   ".to_string(),
            "   ██║   ".to_string(),
            "   ██║   ".to_string(),
            "   ╚═╝   ".to_string(),
        ],
        'U' => vec![
            "██╗   ██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
        ],
        'V' => vec![
            "██╗   ██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██╗ ██╔╝".to_string(),
            " ╚████╔╝ ".to_string(),
            "  ╚═══╝  ".to_string(),
        ],
        'W' => vec![
            "██╗    ██╗".to_string(),
            "██║    ██║".to_string(),
            "██║ █╗ ██║".to_string(),
            "██║███╗██║".to_string(),
            "╚███╔███╔╝".to_string(),
            " ╚══╝╚══╝ ".to_string(),
        ],
        'X' => vec![
            "██╗  ██╗".to_string(),
            "╚██╗██╔╝".to_string(),
            " ╚███╔╝ ".to_string(),
            " ██╔██╗ ".to_string(),
            "██╔╝ ██╗".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ],
        'Y' => vec![
            "██╗   ██╗".to_string(),
            "╚██╗ ██╔╝".to_string(),
            " ╚████╔╝ ".to_string(),
            "  ╚██╔╝  ".to_string(),
            "   ██║   ".to_string(),
            "   ╚═╝   ".to_string(),
        ],
        'Z' => vec![
            "███████╗".to_string(),
            "╚══███╔╝".to_string(),
            "  ███╔╝ ".to_string(),
            " ███╔╝  ".to_string(),
            "███████╗".to_string(),
            "═╚═════╝".to_string(),
        ],
        ' ' => vec![
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
        ],
        '?' => vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "╚═══██╔╝ ".to_string(),
            "   ██╔╝  ".to_string(),
            "   ╚═╝   ".to_string(),
            "   ██    ".to_string(),
        ],
        _ => vec![
            "█╗  ".to_string(),
            "█║  ".to_string(),
            "█║  ".to_string(),
            "█║  ".to_string(),
            "█║  ".to_string(),
            "╚╝  ".to_string(),
        ],
    }
}

/// Generate ASCII art for a text string by combining individual letters
fn get_text_ascii(text: &str) -> Vec<String> {
    let letters: Vec<Vec<String>> = text.chars().map(get_letter_ascii).collect();
    
    if letters.is_empty() {
        return vec!["".to_string(); 6];
    }
    
    let mut result = vec![String::new(); 6];
    
    for letter_art in letters {
        for (i, line) in letter_art.iter().enumerate() {
            if i < 6 {
                result[i].push_str(line);
            }
        }
    }
    
    result
}

/// Generate ASCII art for a language name using composed letters
fn get_language_ascii(lang: &str) -> Vec<String> {
    let ascii = get_text_ascii(lang);
    // Add an empty line at the start for spacing
    let mut result = vec!["".to_string()];
    result.extend(ascii);
    result.push("".to_string());
    result
}

/// Generate starter code template for a problem in a specific language
fn get_starter_code(problem: &Problem, language: Language) -> String {
    let func_name = &problem.function_name;
    
    match language {
        Language::Python => {
            let args = match problem.id {
                1 => "nums, target",
                2 | 4 => "s",
                3 | 5 => "n",
                _ => "..."
            };
            format!("def {}({}):\n    # Write your solution here\n    pass\n", func_name, args)
        },
        Language::JavaScript => {
            let args = match problem.id {
                1 => "nums, target",
                2 | 4 => "s",
                3 | 5 => "n",
                _ => "..."
            };
            format!("function {}({}) {{\n    // Write your solution here\n    \n}}\n", func_name, args)
        },
        Language::TypeScript => {
            let (args, ret) = match problem.id {
                1 => ("nums: number[], target: number", "number[]"),
                2 => ("s: string[]", "void"),
                3 => ("n: number", "string[]"),
                4 => ("s: string", "boolean"),
                5 => ("n: number", "number"),
                _ => ("...", "any")
            };
            format!("function {}({}): {} {{\n    // Write your solution here\n    \n}}\n", func_name, args, ret)
        },
        Language::Rust => {
            let (args, ret) = match problem.id {
                1 => ("nums: Vec<i32>, target: i32", "Vec<i32>"),
                2 => ("s: &mut Vec<char>", ""),
                3 => ("n: i32", "Vec<String>"),
                4 => ("s: String", "bool"),
                5 => ("n: i32", "i32"),
                _ => ("...", "()")
            };
            let ret_str = if ret.is_empty() { String::new() } else { format!(" -> {}", ret) };
            let body = if ret.is_empty() { "" } else { "    todo!()\n" };
            format!("pub fn {}({}){} {{\n    // Write your solution here\n{}}}\n", func_name, args, ret_str, body)
        },
        Language::Go => {
            let (args, ret) = match problem.id {
                1 => ("nums []int, target int", "[]int"),
                2 => ("s []string", ""),
                3 => ("n int", "[]string"),
                4 => ("s string", "bool"),
                5 => ("n int", "int"),
                _ => ("...", "")
            };
            
            let ret_str = if ret.is_empty() { String::new() } else { format!(" {}", ret) };
            let return_stmt = match problem.id {
                1 | 2 | 3 => "    return nil\n",
                4 => "    return false\n",
                5 => "    return 0\n",
                _ => ""
            };
            
            format!("func {}({}){} {{\n    // Write your solution here\n{}}}\n", func_name, args, ret_str, return_stmt)
        },
        Language::Java => {
            let (args, ret, return_stmt) = match problem.id {
                1 => ("int[] nums, int target", "int[]", "return new int[0];"),
                2 => ("char[] s", "void", ""),
                3 => ("int n", "List<String>", "return new ArrayList<>();"),
                4 => ("String s", "boolean", "return false;"),
                5 => ("int n", "int", "return 0;"),
                _ => ("...", "Object", "return null;")
            };
            
            format!("public {} {}({}) {{\n    // Write your solution here\n    {}\n}}\n", ret, func_name, args, return_stmt)
        },
        Language::Haskell => {
            let (args, ret) = match problem.id {
                1 => ("nums target", "[Int] -> Int -> [Int]"),
                2 => ("s", "[Char] -> [Char]"),
                3 => ("n", "Int -> [String]"),
                4 => ("s", "String -> Bool"),
                5 => ("n", "Int -> Int"),
                _ => ("...", "a -> b")
            };
            format!("{} :: {}\n{} {} = \n    -- Write your solution here\n    undefined\n", func_name, ret, func_name, args)
        },
        Language::Lua => {
            let args = match problem.id {
                1 => "nums, target",
                2 | 4 => "s",
                3 | 5 => "n",
                _ => "..."
            };
            format!("function {}({})\n    -- Write your solution here\n    \nend\n", func_name, args)
        },
        Language::OCaml => {
            let (args, ret) = match problem.id {
                1 => ("nums target", "int list -> int -> int list"),
                2 => ("s", "char list -> char list"),
                3 => ("n", "int -> string list"),
                4 => ("s", "string -> bool"),
                5 => ("n", "int -> int"),
                _ => ("...", "'a -> 'b")
            };
            format!("let {} {} : {} =\n  (* Write your solution here *)\n  failwith \"Not implemented\"\n", func_name, args, ret)
        },
        Language::Elixir => {
            let args = match problem.id {
                1 => "nums, target",
                2 | 4 => "s",
                3 | 5 => "n",
                _ => "..."
            };
            format!("def {}({}) do\n  # Write your solution here\n  \nend\n", func_name, args)
        },
        Language::Kotlin => {
            let (args, ret, return_stmt) = match problem.id {
                1 => ("nums: IntArray, target: Int", "IntArray", "return intArrayOf()"),
                2 => ("s: CharArray", "Unit", ""),
                3 => ("n: Int", "List<String>", "return emptyList()"),
                4 => ("s: String", "Boolean", "return false"),
                5 => ("n: Int", "Int", "return 0"),
                _ => ("...", "Any", "return null")
            };
            let ret_prefix = if ret == "Unit" { "" } else { ": " };
            let body = if return_stmt.is_empty() { 
                "    // Write your solution here\n".to_string() 
            } else { 
                format!("    // Write your solution here\n    {}\n", return_stmt)
            };
            format!("fun {}({}){}{} {{\n{}}}\n", func_name, args, ret_prefix, ret, body)
        },
        Language::Swift => {
            let (args, ret, return_stmt) = match problem.id {
                1 => ("_ nums: [Int], _ target: Int", "[Int]", "return []"),
                2 => ("_ s: inout [Character]", "Void", ""),
                3 => ("_ n: Int", "[String]", "return []"),
                4 => ("_ s: String", "Bool", "return false"),
                5 => ("_ n: Int", "Int", "return 0"),
                _ => ("...", "Any", "return nil")
            };
            let ret_str = if ret == "Void" { String::new() } else { format!(" -> {}", ret) };
            let body = if return_stmt.is_empty() {
                "    // Write your solution here\n".to_string()
            } else {
                format!("    // Write your solution here\n    {}\n", return_stmt)
            };
            format!("func {}({}){} {{\n{}}}\n", func_name, args, ret_str, body)
        },
    }
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
    pub editor_area: Rect,
    pub countdown_start: Option<Instant>,
    pub pending_language: Option<Language>,
    pub pending_problem: Option<Problem>,
    pub translation_rx: Option<mpsc::Receiver<TranslationEvent>>,
    pub pending_translation: Option<TranslationEvent>,
    pub code_sent_for_translation: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let current_language = Language::Python;
        let problem = Problem::random();
        
        Self {
            problem: problem.clone(),
            code: get_starter_code(&problem, current_language),
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
            editor_area: Rect::default(),
            countdown_start: None,
            pending_language: None,
            pending_problem: None,
            translation_rx: None,
            pending_translation: None,
            code_sent_for_translation: None,
        }
    }

    pub fn tick(&mut self) {
        self.glitch_frame = (self.glitch_frame + 1) % 10;

        match self.state {
            AppState::Coding => {
                let elapsed = self.last_randomize.elapsed();
                // Start countdown 5 seconds before randomize time
                let countdown_threshold = self.randomize_interval.saturating_sub(Duration::from_secs(5));
                if elapsed >= countdown_threshold && self.countdown_start.is_none() {
                    self.start_countdown();
                }
            }
            AppState::Countdown(count) => {
                // Use the actual remaining time to stay in sync with the footer timer
                let elapsed = self.last_randomize.elapsed();
                let remaining = self.randomize_interval.saturating_sub(elapsed);
                let new_count = remaining.as_secs() as u8;
                
                if new_count == 0 || remaining.is_zero() {
                    self.start_transition();
                } else if new_count != count {
                    self.state = AppState::Countdown(new_count);
                }
            }
            AppState::Transitioning(_progress) => {
                if let Some(start) = self.transition_start {
                    let elapsed = start.elapsed().as_secs_f32();
                    let new_progress = (elapsed / 1.5).min(1.0); // 1.5s transition
                    
                    if new_progress >= 1.0 {
                        self.start_reveal();
                    } else {
                        self.state = AppState::Transitioning(new_progress);
                    }
                }
            }
            AppState::Revealing(_progress) => {
                if let Some(start) = self.transition_start {
                    let elapsed = start.elapsed().as_secs_f32();
                    let new_progress = (elapsed / 3.0).min(1.0); // 3s reveal
                    
                    if new_progress >= 1.0 {
                        if self.translation_ready() {
                            self.complete_transition();
                        } else {
                            // Keep showing the final reveal (don't restart animation)
                            // Just stay at progress 0.99 to show the language while waiting
                            self.state = AppState::Revealing(0.99);
                        }
                    } else {
                        self.state = AppState::Revealing(new_progress);
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
                        // Submit mode - show full results screen
                        self.test_results = Some(results.clone());
                        self.state = AppState::Results(results);
                        should_close = true;
                    }
                    ExecutionEvent::RunFinished(results) => {
                        // Run mode - show results inline in output panel
                        self.test_results = Some(results.clone());
                        
                        // Add blank line
                        self.execution_output.push(OutputLine { 
                            text: "".to_string(), 
                            is_error: false 
                        });
                        
                        // Add results summary
                        let score_text = format!(
                            "RESULTS: {}/{} tests passed ({}%)", 
                            results.passed, 
                            results.total,
                            (results.passed * 100) / results.total.max(1)
                        );
                        self.execution_output.push(OutputLine { 
                            text: score_text, 
                            is_error: results.passed != results.total 
                        });
                        
                        self.execution_output.push(OutputLine { 
                            text: "─".repeat(60), 
                            is_error: false 
                        });
                        
                        // Add individual test results
                        for detail in &results.details {
                            let status = if detail.passed { "✓ PASS" } else { "✗ FAIL" };
                            let status_line = format!("{} Test #{}", status, detail.case_number);
                            self.execution_output.push(OutputLine { 
                                text: status_line, 
                                is_error: !detail.passed 
                            });
                            
                            if !detail.passed {
                                self.execution_output.push(OutputLine { 
                                    text: format!("  Input: {}", detail.input), 
                                    is_error: false 
                                });
                                self.execution_output.push(OutputLine { 
                                    text: format!("  Expected: {}", detail.expected), 
                                    is_error: false 
                                });
                                self.execution_output.push(OutputLine { 
                                    text: format!("  Got: {}", detail.actual), 
                                    is_error: true 
                                });
                            }
                        }
                        should_close = true;
                    }
                }
            }
        }
        
        if should_close {
            self.output_rx = None;
        }

    }

    pub fn poll_translation(&mut self) {
        let mut completed = None;
        if let Some(rx) = &mut self.translation_rx {
            while let Ok(event) = rx.try_recv() {
                completed = Some(event);
            }
        }

        if let Some(event) = completed {
            self.pending_translation = Some(event);
            self.translation_rx = None;
        }
    }

    fn translation_ready(&self) -> bool {
        self.pending_translation.is_some()
    }

    fn start_llm_translation(&mut self) {
        self.pending_translation = None;
        self.translation_rx = None;

        let target_language = match self.pending_language {
            Some(lang) => lang,
            None => return,
        };

        let code = self.code.clone();
        self.code_sent_for_translation = Some(code.clone());
        let from = self.current_language;
        let to = target_language;
        if from == to {
            self.pending_translation = Some(TranslationEvent::Success(code));
            return;
        }

        let prompt = build_translation_prompt(&code, from, to);
        let (tx, rx) = mpsc::channel(1);
        self.translation_rx = Some(rx);

        tokio::spawn(async move {
            let result = llm::translate_code(&prompt).await;
            let event = match result {
                Ok(translated) => TranslationEvent::Success(translated),
                Err(err) => TranslationEvent::Failure(err.to_string()),
            };
            let _ = tx.send(event).await;
        });
    }

    fn start_countdown(&mut self) {
        self.countdown_start = Some(Instant::now());
        self.state = AppState::Countdown(5);
        // Pre-select new language now so we can show it during reveal
        self.pending_language = Some(self.current_language.random_except());
        // Start translation early for faster response
        self.start_llm_translation();
    }

    fn start_transition(&mut self) {
        self.transition_start = Some(Instant::now());
        self.state = AppState::Transitioning(0.0);
        // If user typed during countdown, restart translation with latest code
        let code_changed = self.code_sent_for_translation.as_ref() != Some(&self.code);
        if code_changed {
            self.start_llm_translation();
        }
    }

    fn start_reveal(&mut self) {
        self.transition_start = Some(Instant::now());
        self.state = AppState::Revealing(0.0);
    }

    fn complete_transition(&mut self) {
        // Apply the pending language only (keep the same problem)
        if let Some(new_lang) = self.pending_language.take() {
            if let Some(result) = self.pending_translation.take() {
                match result {
                    TranslationEvent::Success(translated) => {
                        self.code = translated;
                    }
                    TranslationEvent::Failure(_) => {
                        // Keep the existing code if translation fails
                    }
                }
            }
            self.current_language = new_lang;
        } 
        
        // Clear any pending problem (not used in auto-transition)
        self.pending_problem = None;
        self.translation_rx = None;
        self.pending_translation = None;
        
        // Reset timer and state
        self.last_randomize = Instant::now();
        self.state = AppState::Coding;
        self.transition_start = None;
        self.countdown_start = None;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Coding | AppState::Countdown(_) => self.handle_coding_key(key),
            AppState::Results(_) => self.handle_results_key(key),
             _ => {}, // Ignore input during transitions and execution
        }
    }

    fn randomize_problem(&mut self) {
        let new_problem = self.problem.random_except();
        self.problem = new_problem.clone();
        self.code = get_starter_code(&new_problem, self.current_language);
        self.cursor_position = 0;
    }

    fn handle_coding_key(&mut self, key: KeyEvent) {
        
        // Smart detection: Try Cmd (SUPER) first, then Ctrl
        // Some terminals (with config) can pass through Cmd keys
        // Most terminals pass through Ctrl/Alt keys
        let is_cmd = key.modifiers.contains(KeyModifiers::SUPER);
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);
        let is_shift = key.modifiers.contains(KeyModifiers::SHIFT);
        
        // Use Cmd OR Ctrl (whichever is available) for line/editing commands
        let has_modifier = is_cmd || is_ctrl;
        
        if has_modifier && !is_alt {
            match key.code {
                // Cmd/Ctrl+S to submit
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    self.submit();
                    return;
                }
                // Cmd/Ctrl+Q to quit (handled in main.rs, but listed here for consistency)
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return; // Let main.rs handle the quit
                }
                // Cmd/Ctrl+R to randomize problem
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    self.randomize_problem();
                    return;
                }
                // Cmd/Ctrl+C to run (show output)
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    self.show_output_panel = true;
                    self.run_code();
                }
                // Cmd/Ctrl+A: move to start of line (like bash/zsh)
                KeyCode::Char('a') | KeyCode::Char('A') => {
                    self.move_to_line_start();
                    return;
                }
                // Cmd/Ctrl+E: move to end of line (like bash/zsh)
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    self.move_to_line_end();
                    return;
                }
                // Cmd/Ctrl+U: delete from cursor to start of line (like bash/zsh)
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    self.delete_to_start_of_line();
                    return;
                }
                // Cmd/Ctrl+K: delete from cursor to end of line (like bash/zsh)
                KeyCode::Char('k') | KeyCode::Char('K') => {
                    self.delete_to_end_of_line();
                    return;
                }
                // Cmd/Ctrl+W: delete previous word (like bash/zsh)
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    self.delete_word_backward();
                    return;
                }
                // Cmd/Ctrl+D: delete character under cursor
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    if self.cursor_position < self.code.len() {
                        self.code.remove(self.cursor_position);
                    }
                    return;
                }
                // Cmd/Ctrl+Left: move to start of line (macOS style)
                KeyCode::Left if is_cmd => {
                    self.move_to_line_start();
                    return;
                }
                // Cmd/Ctrl+Right: move to end of line (macOS style)
                KeyCode::Right if is_cmd => {
                    self.move_to_line_end();
                    return;
                }
                // Cmd+Up: move to start of document (macOS style)
                KeyCode::Up if is_cmd => {
                    self.cursor_position = 0;
                    return;
                }
                // Cmd+Down: move to end of document (macOS style)
                KeyCode::Down if is_cmd => {
                    self.cursor_position = self.code.len();
                    return;
                }
                _ => {}
            }
        }
        
        // Word navigation with Alt (Option) key
        if is_alt && !has_modifier {
            match key.code {
                // Alt+F or Alt+Right: move forward by word
                KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Right => {
                    self.move_word_right();
                    return;
                }
                // Alt+B or Alt+Left: move backward by word
                KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Left => {
                    self.move_word_left();
                    return;
                }
                // Alt+D: delete next word
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    self.delete_word_forward();
                    return;
                }
                // Alt+Backspace: delete previous word
                KeyCode::Backspace => {
                    self.delete_word_backward();
                    return;
                }
                _ => {}
            }
        }
        
        // Tab for indent/unindent
        if key.code == KeyCode::Tab && !has_modifier && !is_alt {
            if is_shift {
                self.unindent_line();
            } else {
                // Insert 4 spaces instead of tab character for consistent rendering
                self.code.insert_str(self.cursor_position, "    ");
                self.cursor_position += 4;
            }
            return;
        }
        
        // Regular key handling (no modifiers or only shift)
        if !has_modifier && !is_alt {
            match key.code {
                KeyCode::Char(c) => {
                    self.insert_char(c);
                }
                KeyCode::Enter => {
                    self.insert_newline_with_indent();
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < self.code.len() {
                        self.cursor_position += 1;
                    }
                }
                KeyCode::Up => {
                    self.move_cursor_up();
                }
                KeyCode::Down => {
                    self.move_cursor_down();
                }
                _ => {}
            }
        }
    }

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char('r') => {
                // Restart with same problem and code - just go back to coding
                self.state = AppState::Coding;
                self.test_results = None;
                self.execution_output.clear();
                self.show_output_panel = false;
                self.execution_progress = 0.0;
                self.output_rx = None;
                self.last_randomize = Instant::now(); // Reset timer
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                // Keep results visible, could add exit logic here
            }
            _ => {}
        }
    }


    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
        if self.state != AppState::Coding {
            return;
        }

        match mouse.kind {
            MouseEventKind::Down(_) | MouseEventKind::Up(_) => {
                // Check if click is in editor area
                let click_x = mouse.column;
                let click_y = mouse.row;
                
                // Account for border (1 char) and line numbers (4 chars: " 99 ")
                if click_x >= self.editor_area.x + 1 + 4 
                    && click_x < self.editor_area.x + self.editor_area.width - 1
                    && click_y >= self.editor_area.y + 1
                    && click_y < self.editor_area.y + self.editor_area.height - 1 {
                    
                    let line_num = (click_y - self.editor_area.y - 1) as usize;
                    let col_in_line = (click_x - self.editor_area.x - 1 - 4) as usize;
                    
                    // Calculate position in code string
                    let lines: Vec<&str> = self.code.split('\n').collect();
                    if line_num < lines.len() {
                        let mut pos = 0;
                        for (i, line) in lines.iter().enumerate() {
                            if i < line_num {
                                pos += line.len() + 1; // +1 for newline
                            } else {
                                pos += col_in_line.min(line.len());
                                break;
                            }
                        }
                        self.cursor_position = pos.min(self.code.len());
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                // Scroll up (move cursor up)
                self.move_cursor_up();
            }
            MouseEventKind::ScrollDown => {
                // Scroll down (move cursor down)
                self.move_cursor_down();
            }
            _ => {}
        }
    }

    fn insert_char(&mut self, c: char) {
        self.code.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    fn insert_newline_with_indent(&mut self) {
        // Find the current line
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        let mut current_line = "";
        
        for line in lines.iter() {
            if current_pos + line.len() >= self.cursor_position {
                current_line = line;
                break;
            }
            current_pos += line.len() + 1; // +1 for newline
        }
        
        // Count leading spaces
        let indent = current_line.chars().take_while(|&c| c == ' ').count();
        
        // Insert newline + same indentation
        self.code.insert(self.cursor_position, '\n');
        self.cursor_position += 1;
        
        if indent > 0 {
            let indent_str = " ".repeat(indent);
            self.code.insert_str(self.cursor_position, &indent_str);
            self.cursor_position += indent;
        }
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            // Check if we're in leading whitespace and can delete a full indent
            let before_cursor = &self.code[..self.cursor_position];
            
            // Find the start of the current line
            let line_start = before_cursor.rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line_before_cursor = &self.code[line_start..self.cursor_position];
            
            // If we're in leading spaces and have at least 4 spaces, delete 4
            if line_before_cursor.chars().all(|c| c == ' ') && line_before_cursor.len() >= 4 {
                // Delete 4 spaces at once
                for _ in 0..4 {
                    if self.cursor_position > line_start {
                        self.code.remove(self.cursor_position - 1);
                        self.cursor_position -= 1;
                    }
                }
            } else {
                // Normal backspace - delete one character
                let prev_char_boundary = self.code[..self.cursor_position]
                    .char_indices()
                    .last()
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                self.code.remove(prev_char_boundary);
                self.cursor_position = prev_char_boundary;
            }
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

    /// Shared helper to execute code and run tests
    fn execute_code(&mut self, is_submit: bool) {
        self.execution_output.clear();
        self.execution_output.push(OutputLine { 
            text: if is_submit { 
                "Compiling and sending to Piston API...".to_string() 
            } else { 
                "Running code on Piston API...".to_string() 
            }, 
            is_error: false 
        });

        let (tx, rx) = mpsc::channel(32);
        self.output_rx = Some(rx);
        
        // Clone data for async task
        let code = self.code.clone();
        let problem = self.problem.clone();
        let language = self.current_language;
        
        // Spawn async execution
        tokio::spawn(async move {
            let results = run_tests_on_piston(code, problem, language, tx.clone()).await;
            
            // Send different event based on mode
            let event = if is_submit {
                ExecutionEvent::Finished(results)
            } else {
                ExecutionEvent::RunFinished(results)
            };
            let _ = tx.send(event).await;
        });
    }

    fn run_code(&mut self) {
        self.execute_code(false);  // false = run mode (inline results)
    }

    fn move_to_line_start(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        
        for line in lines.iter() {
            if current_pos + line.len() >= self.cursor_position {
                self.cursor_position = current_pos;
                return;
            }
            current_pos += line.len() + 1;
        }
    }

    fn move_to_line_end(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        
        for line in lines.iter() {
            if current_pos + line.len() >= self.cursor_position {
                self.cursor_position = current_pos + line.len();
                return;
            }
            current_pos += line.len() + 1;
        }
    }

    fn clear_to_end_of_line(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        let mut current_line = 0;
        
        for (i, line) in lines.iter().enumerate() {
            if current_pos + line.len() >= self.cursor_position {
                current_line = i;
                break;
            }
            current_pos += line.len() + 1;
        }
        
        // Delete from cursor to end of current line
        let line_end = current_pos + lines[current_line].len();
        if self.cursor_position < line_end {
            self.code.drain(self.cursor_position..line_end);
        }
    }

    fn delete_to_end_of_line(&mut self) {
        self.clear_to_end_of_line();
    }

    fn delete_to_start_of_line(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        
        for line in lines.iter() {
            if current_pos + line.len() >= self.cursor_position {
                // Delete from start of line to cursor
                if self.cursor_position > current_pos {
                    self.code.drain(current_pos..self.cursor_position);
                    self.cursor_position = current_pos;
                }
                return;
            }
            current_pos += line.len() + 1;
        }
    }

    fn unindent_line(&mut self) {
        let lines: Vec<&str> = self.code.split('\n').collect();
        let mut current_pos = 0;
        
        for line in lines.iter() {
            if current_pos + line.len() >= self.cursor_position {
                // Remove up to 4 spaces or 1 tab from start of line
                if line.starts_with("    ") {
                    self.code.drain(current_pos..current_pos + 4);
                    if self.cursor_position > current_pos {
                        self.cursor_position = (self.cursor_position - 4).max(current_pos);
                    }
                } else if line.starts_with('\t') {
                    self.code.remove(current_pos);
                    if self.cursor_position > current_pos {
                        self.cursor_position = (self.cursor_position - 1).max(current_pos);
                    }
                }
                return;
            }
            current_pos += line.len() + 1;
        }
    }

    fn move_word_left(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        
        let chars: Vec<char> = self.code.chars().collect();
        let mut pos = self.cursor_position.saturating_sub(1);
        
        // Skip whitespace
        while pos > 0 && chars.get(pos).map_or(false, |c| c.is_whitespace()) {
            pos -= 1;
        }
        
        // Skip word characters
        while pos > 0 && chars.get(pos).map_or(false, |c| !c.is_whitespace()) {
            pos -= 1;
        }
        
        // Move one position forward if we're not at the start
        if pos > 0 || chars.get(0).map_or(false, |c| c.is_whitespace()) {
            pos += 1;
        }
        
        self.cursor_position = pos;
    }

    fn move_word_right(&mut self) {
        let chars: Vec<char> = self.code.chars().collect();
        if self.cursor_position >= chars.len() {
            return;
        }
        
        let mut pos = self.cursor_position;
        
        // Skip current word
        while pos < chars.len() && !chars[pos].is_whitespace() {
            pos += 1;
        }
        
        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        
        self.cursor_position = pos;
    }

    fn delete_word_backward(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        
        let start_pos = self.cursor_position;
        self.move_word_left();
        let end_pos = self.cursor_position;
        
        if end_pos < start_pos {
            self.code.drain(end_pos..start_pos);
        }
    }

    fn delete_word_forward(&mut self) {
        let chars: Vec<char> = self.code.chars().collect();
        if self.cursor_position >= chars.len() {
            return;
        }
        
        let start_pos = self.cursor_position;
        let mut end_pos = start_pos;
        
        // Skip current word
        while end_pos < chars.len() && !chars[end_pos].is_whitespace() {
            end_pos += 1;
        }
        
        if end_pos > start_pos {
            self.code.drain(start_pos..end_pos);
        }
    }

    fn submit(&mut self) {
        self.state = AppState::Compiling(0.0);
        self.execute_code(true);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match &self.state {
            AppState::Coding => self.render_coding(frame),
            AppState::Countdown(count) => self.render_countdown(frame, *count),
            AppState::Transitioning(progress) => self.render_transition(frame, *progress),
            AppState::Revealing(progress) => self.render_reveal(frame, *progress),
            AppState::Compiling(progress) => self.render_compiling(frame, *progress),
            AppState::Running => self.render_running(frame),
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

        // Store editor area for mouse clicks
        self.editor_area = content_area;

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
        // Calculate cursor line and column
        let mut cursor_line = 0;
        let mut cursor_col = 0;
        let mut char_count = 0;
        
        for (line_idx, line) in self.code.split('\n').enumerate() {
            let line_len = line.len() + 1; // +1 for newline
            if char_count + line_len > self.cursor_position {
                cursor_line = line_idx;
                cursor_col = self.cursor_position - char_count;
                break;
            }
            char_count += line_len;
        }
        
        // Render lines with syntax highlighting and cursor
        let lines: Vec<Line> = self.code.split('\n').enumerate().map(|(i, line)| {
            let line_num = format!("{:>3} ", i + 1);
            let mut spans = vec![
                Span::styled(line_num, Style::default().fg(Color::DarkGray)),
            ];
            
            // Apply syntax highlighting
            let highlighted = SyntaxHighlighter::highlight_line(line, &self.current_language);
            
            // Insert cursor if this is the cursor line
            if i == cursor_line {
                let mut char_pos = 0;
                let mut inserted = false;
                let mut final_spans = Vec::new();
                
                for span in highlighted {
                    let span_text = span.content.to_string();
                    let span_len = span_text.len();
                    
                    if !inserted && char_pos + span_len > cursor_col {
                        // Split this span to insert cursor
                        let before_cursor = &span_text[..cursor_col.saturating_sub(char_pos)];
                        let at_cursor_char = span_text.chars().nth(cursor_col - char_pos).unwrap_or(' ');
                        let after_cursor = &span_text[(cursor_col - char_pos + at_cursor_char.len_utf8()).min(span_len)..];
                        
                        if !before_cursor.is_empty() {
                            final_spans.push(Span::styled(before_cursor.to_string(), span.style));
                        }
                        
                        // Cursor with inverted colors - show the actual character under cursor
                        final_spans.push(Span::styled(
                            at_cursor_char.to_string(),
                            Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD),
                        ));
                        
                        if !after_cursor.is_empty() {
                            final_spans.push(Span::styled(after_cursor.to_string(), span.style));
                        }
                        
                        inserted = true;
                    } else {
                        final_spans.push(span);
                    }
                    
                    char_pos += span_len;
                }
                
                // If cursor is at the end of the line, add a space with inverted colors (not a block)
                if !inserted {
                    final_spans.push(Span::styled(
                        " ",
                        Style::default().fg(Color::Black).bg(Color::White),
                    ));
                }
                
                spans.extend(final_spans);
            } else {
                spans.extend(highlighted);
            }
            
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
            .title(" Output (^C to run) ")
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
            Span::styled("⏱ ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}s", secs), Style::default().fg(timer_color).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("^S ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("Submit", Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("^R ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::styled("New Problem", Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("^A/E ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Line", Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Alt+←/→ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("Word", Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("^C ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("Compile", Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("^Q ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("Quit", Style::default().fg(Color::White)),
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

    fn render_countdown(&mut self, frame: &mut Frame, count: u8) {
        let size = frame.size();
        
        // First render the normal coding view so user can see their code
        self.render_coding(frame);
        
        // Then overlay the big countdown
        let color = match count {
            5 => Color::Green,
            4 => Color::Yellow,
            3 => Color::Yellow,
            2 => Color::Rgb(255, 165, 0), // Orange
            1 => Color::Red,
            _ => Color::White,
        };

        // Big ASCII art numbers using the standardized function
        let big_number = match count {
            5 => self.get_ascii_number(5),
            4 => self.get_ascii_number(4),
            3 => self.get_ascii_number(3),
            2 => self.get_ascii_number(2),
            1 => self.get_ascii_number(1),
            _ => self.get_ascii_number(0),
        };

        let popup_area = centered_rect(50, 36, size);
        let popup_height = popup_area.height as usize;
        
        // Calculate content height for vertical centering
        let title_lines = 1;  // Warning message
        let ascii_number_lines = 6;  // Big number (now 6 lines)
        let help_text_lines = 1;  // "Keep typing" message
        let spacing = 3;  // Empty lines between sections (extra padding)
        let total_content_height = title_lines + ascii_number_lines + help_text_lines + spacing;
        
        // Calculate vertical padding (accounting for borders)
        let available_height = popup_height.saturating_sub(2); // Subtract borders
        let vertical_padding = if available_height > total_content_height {
            // Add one extra top padding line so content sits slightly lower
            (available_height - total_content_height) / 2 + 1
        } else {
            0
        };
        
        let mut countdown_text = vec![];
        
        // Add top padding for vertical centering
        for _ in 0..vertical_padding {
            countdown_text.push(Line::from(""));
        }
        
        countdown_text.push(Line::from(Span::styled(
                "YOUR CODE WILL BECOME A RANDOM LANGUAGE. DO NOT RESIST.",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
            )));
        countdown_text.push(Line::from(""));
        countdown_text.push(Line::from(""));
        
        // Add the big number
        for line in big_number {
            countdown_text.push(Line::from(Span::styled(
                line,
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            )));
        }
        
        // Extra padding line to avoid clipping the bottom of ASCII art
        countdown_text.push(Line::from(""));
        countdown_text.push(Line::from(Span::styled(
            "Keep typing! Your code will be translated.",
            Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)
        )));
        
        // Clear the area for solid background
        frame.render_widget(Clear, popup_area);
        
        let popup = Paragraph::new(countdown_text)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(100, 100, 120)))
                .style(Style::default().bg(Color::Black)));
        
        frame.render_widget(popup, popup_area);
    }

    fn render_reveal(&self, frame: &mut Frame, progress: f32) {
        let size = frame.size();
        
        // Get the pending language name
        let lang_name = self.pending_language
            .as_ref()
            .map(|l| l.display_name())
            .unwrap_or("???");
        
        // Create glitch effect background (same as transition)
        let glitch_chars = ["█", "▓", "▒", "░", "▄", "▀", "▌", "▐"];
        let mut bg_lines = Vec::new();
        let char_idx = (self.glitch_frame % glitch_chars.len()) as usize;
        
        let height = size.height as usize;
        let width = size.width as usize;
        
        // Use a decreasing glitch intensity as reveal progresses
        let glitch_intensity = 0.8 - (progress * 0.5);
        
        for i in 0..height {
            let intensity = ((i as f32 / height as f32) - 0.5).abs();
            let wave = (i as f32 * 0.1 + progress * 10.0).sin();
            let phase = (self.glitch_frame as f32 * 0.1 + i as f32 * 0.05).sin();
            
            // Generate random rainbow colors - full spectrum
            let hue_base = (i as f32 * 7.0 + self.glitch_frame as f32 * 3.0) % 360.0;
            let hue_offset = wave * 60.0 + phase * 40.0;
            let hue = (hue_base + hue_offset).rem_euclid(360.0);
            
            // Vary saturation and brightness based on intensity
            let saturation = if intensity < 0.1 {
                0.9 + rand::random::<f32>() * 0.1  // Very saturated near progress
            } else if intensity < 0.3 {
                0.6 + rand::random::<f32>() * 0.3  // Medium saturation
            } else {
                0.3 + rand::random::<f32>() * 0.4  // Lower saturation
            };
            
            let brightness = if intensity < 0.1 {
                0.8 + rand::random::<f32>() * 0.2  // Bright near progress
            } else if intensity < 0.3 {
                0.5 + rand::random::<f32>() * 0.3  // Medium brightness
            } else {
                0.2 + rand::random::<f32>() * 0.3  // Dimmer background
            };
            
            // Convert HSV to RGB
            let c = brightness * saturation;
            let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
            let m = brightness - c;
            
            let (r, g, b) = if hue < 60.0 {
                (c, x, 0.0)
            } else if hue < 120.0 {
                (x, c, 0.0)
            } else if hue < 180.0 {
                (0.0, c, x)
            } else if hue < 240.0 {
                (0.0, x, c)
            } else if hue < 300.0 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };
            
            let color = Color::Rgb(
                ((r + m) * 255.0) as u8,
                ((g + m) * 255.0) as u8,
                ((b + m) * 255.0) as u8
            );
            
            let mut line_text = String::new();
            for _ in 0..width {
                if rand::random::<f32>() < glitch_intensity {
                    line_text.push_str(glitch_chars[char_idx]);
                } else {
                    line_text.push(' ');
                }
            }
            
            bg_lines.push(Line::from(Span::styled(line_text, Style::default().fg(color))));
        }
        
        let bg = Paragraph::new(bg_lines);
        frame.render_widget(bg, size);
        
        // Build the overlay content
        let mut message = vec![];
        
        // Spinning/slot machine effect for first part of reveal
        if progress < 0.5 {
            // Slot machine spinning effect for language only
            let languages = Language::all();
            let spin_idx = ((progress * 30.0) as usize) % languages.len();
            let display_lang = languages[spin_idx].display_name();
            
            message.push(Line::from(Span::styled(
                "╔══════════════════════════════════════════════════════════════════╗",
                Style::default().fg(Color::Cyan)
            )));
            message.push(Line::from(Span::styled(
                "║           RNG IS SELECTING YOUR NEW LINGUA FRANCA...             ║",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            )));
            message.push(Line::from(Span::styled(
                "╚══════════════════════════════════════════════════════════════════╝",
                Style::default().fg(Color::Cyan)
            )));
            message.push(Line::from(""));
            
            // Big ASCII display of spinning language
            let ascii_art = get_language_ascii(display_lang);
            
            // Generate random rainbow color for each frame
            let hue = (self.glitch_frame as f32 * 17.0 + progress * 360.0) % 360.0;
            let saturation = 0.8 + rand::random::<f32>() * 0.2;
            let brightness = 0.7 + rand::random::<f32>() * 0.3;
            
            let c = brightness * saturation;
            let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
            let m = brightness - c;
            
            let (r, g, b) = if hue < 60.0 {
                (c, x, 0.0)
            } else if hue < 120.0 {
                (x, c, 0.0)
            } else if hue < 180.0 {
                (0.0, c, x)
            } else if hue < 240.0 {
                (0.0, x, c)
            } else if hue < 300.0 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };
            
            let color = Color::Rgb(
                ((r + m) * 255.0) as u8,
                ((g + m) * 255.0) as u8,
                ((b + m) * 255.0) as u8
            );
            
            for line in ascii_art {
                message.push(Line::from(Span::styled(
                    line,
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                )));
            }
        } else {
            // Final reveal with dramatic pause
            let reveal_progress = (progress - 0.5) * 2.0; // 0.0 to 1.0 for second half
            
            message.push(Line::from(Span::styled(
                "╔══════════════════════════════════════════════════════════════════╗",
                Style::default().fg(Color::Green)
            )));
            message.push(Line::from(Span::styled(
                "║                       YOUR NEW LANGUAGE!                         ║",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            )));
            message.push(Line::from(Span::styled(
                "╚══════════════════════════════════════════════════════════════════╝",
                Style::default().fg(Color::Green)
            )));
            message.push(Line::from(""));
            
            // Show language with dramatic effect - BIG ASCII ART
            if reveal_progress > 0.3 {
                let ascii_art = get_language_ascii(lang_name);
                for line in ascii_art {
                    message.push(Line::from(Span::styled(
                        line,
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    )));
                }
            } else {
                // Show big ASCII question marks
                let question_marks = get_text_ascii("? ? ?");
                for line in question_marks {
                    message.push(Line::from(Span::styled(
                        line,
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                    )));
                }
            }
            
            if reveal_progress > 0.8 {
                message.push(Line::from(""));
                // Show loading animation if translation isn't ready yet
                if !self.translation_ready() {
                    // Spinner animation
                    let spinner = match self.glitch_frame % 8 {
                        0 => "⠋",
                        1 => "⠙",
                        2 => "⠹",
                        3 => "⠸",
                        4 => "⠼",
                        5 => "⠴",
                        6 => "⠦",
                        _ => "⠧",
                    };

                    // Animated progress bar (bounces back and forth)
                    let bar_width = 20;
                    let pos = (self.glitch_frame % 16) as usize;
                    let bounce_pos = if pos < 8 { pos } else { 16 - pos };
                    let bar: String = (0..bar_width).map(|i| {
                        let dist = (i as i32 - (bounce_pos as i32 * 2 + 2)).abs();
                        if dist == 0 { '█' }
                        else if dist == 1 { '▓' }
                        else if dist == 2 { '▒' }
                        else if dist == 3 { '░' }
                        else { '·' }
                    }).collect();

                    message.push(Line::from(Span::styled(
                        format!("{} TRANSLATING CODE {}", spinner, spinner),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    )));
                    message.push(Line::from(""));
                    message.push(Line::from(Span::styled(
                        format!("┌{}┐", "─".repeat(bar_width + 2)),
                        Style::default().fg(Color::Magenta)
                    )));
                    message.push(Line::from(Span::styled(
                        format!("│ {} │", bar),
                        Style::default().fg(Color::Magenta)
                    )));
                    message.push(Line::from(Span::styled(
                        format!("└{}┘", "─".repeat(bar_width + 2)),
                        Style::default().fg(Color::Magenta)
                    )));
                    message.push(Line::from(""));
                } else {
                    message.push(Line::from(Span::styled(
                        "GET READY TO LEARN A NEW LANGUAGE, BUDDY!",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    )));
                }
            }
        }
        
        // Render popup with black background for readability
        let popup_area = centered_rect(75, 50, size);
        let popup_height = popup_area.height as usize;
        
        // Calculate vertical centering
        let content_height = message.len();
        let available_height = popup_height.saturating_sub(2); // Subtract borders
        let vertical_padding = if available_height > content_height {
            (available_height - content_height) / 2
        } else {
            0
        };
        
        // Add top padding
        let mut centered_message = vec![];
        for _ in 0..vertical_padding {
            centered_message.push(Line::from(""));
        }
        centered_message.extend(message);
        
        frame.render_widget(Clear, popup_area);
        let popup = Paragraph::new(centered_message)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::default().bg(Color::Black)));
        
        frame.render_widget(popup, popup_area);
    }

    fn render_transition(&self, frame: &mut Frame, _progress: f32) {
        let size = frame.size();
        let progress = if let AppState::Transitioning(p) = self.state {
            p
        } else {
            0.0
        };
        
        // Create glitch effect background
        let glitch_chars = ["█", "▓", "▒", "░", "▄", "▀", "▌", "▐"];
        let mut lines = Vec::new();
        let char_idx = (self.glitch_frame % glitch_chars.len()) as usize;
        
        let height = size.height as usize;
        let width = size.width as usize;
        
        for i in 0..height {
            let intensity = ((i as f32 / height as f32) - progress).abs();
            let wave = (i as f32 * 0.1 + progress * 10.0).sin();
            let phase = (self.glitch_frame as f32 * 0.1 + i as f32 * 0.05).sin();
            
            // Generate random rainbow colors - full spectrum
            let hue_base = (i as f32 * 7.0 + self.glitch_frame as f32 * 3.0) % 360.0;
            let hue_offset = wave * 60.0 + phase * 40.0;
            let hue = (hue_base + hue_offset).rem_euclid(360.0);
            
            // Vary saturation and brightness based on intensity
            let saturation = if intensity < 0.1 {
                0.9 + rand::random::<f32>() * 0.1  // Very saturated near progress
            } else if intensity < 0.3 {
                0.6 + rand::random::<f32>() * 0.3  // Medium saturation
            } else {
                0.3 + rand::random::<f32>() * 0.4  // Lower saturation
            };
            
            let brightness = if intensity < 0.1 {
                0.8 + rand::random::<f32>() * 0.2  // Bright near progress
            } else if intensity < 0.3 {
                0.5 + rand::random::<f32>() * 0.3  // Medium brightness
            } else {
                0.2 + rand::random::<f32>() * 0.3  // Dimmer background
            };
            
            // Convert HSV to RGB
            let c = brightness * saturation;
            let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
            let m = brightness - c;
            
            let (r, g, b) = if hue < 60.0 {
                (c, x, 0.0)
            } else if hue < 120.0 {
                (x, c, 0.0)
            } else if hue < 180.0 {
                (0.0, c, x)
            } else if hue < 240.0 {
                (0.0, x, c)
            } else if hue < 300.0 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };
            
            let color = Color::Rgb(
                ((r + m) * 255.0) as u8,
                ((g + m) * 255.0) as u8,
                ((b + m) * 255.0) as u8
            );
            
            let mut line_text = String::new();
            for j in 0..width {
                let density = progress + (j as f32 / width as f32 * 0.3);
                if rand::random::<f32>() < density {
                    line_text.push_str(glitch_chars[char_idx]);
                } else {
                    line_text.push(' ');
                }
            }
            
            lines.push(Line::from(Span::styled(line_text, Style::default().fg(color))));
        }
        
        let bg = Paragraph::new(lines);
        frame.render_widget(bg, size);
        
        // Overlay language roulette on top of glitch effect
        let languages = Language::all();
        let spin_idx = ((progress * 30.0) as usize) % languages.len();
        let display_lang = languages[spin_idx].display_name();
        
        // Get ASCII art for the spinning language
        let ascii_art = get_language_ascii(display_lang);
        
        // Build the overlay message with ASCII art
        let mut message = vec![
            Line::from(Span::styled(
                "╔══════════════════════════════════════════════════════════════════╗",
                Style::default().fg(Color::Cyan)
            )),
            Line::from(Span::styled(
                "║              CONTEMPLATING LANGUAGE CHOICES...                   ║",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            )),
            Line::from(Span::styled(
                "╚══════════════════════════════════════════════════════════════════╝",
                Style::default().fg(Color::Cyan)
            )),
            Line::from(""),
        ];
        
        // Add ASCII art lines with random rainbow colors
        let hue = (self.glitch_frame as f32 * 17.0 + progress * 360.0) % 360.0;
        let saturation = 0.8 + rand::random::<f32>() * 0.2;
        let brightness = 0.7 + rand::random::<f32>() * 0.3;
        
        let c = brightness * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = brightness - c;
        
        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        let color = Color::Rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8
        );
        
        for line in ascii_art {
            message.push(Line::from(Span::styled(
                line,
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            )));
        }
        
        let popup_area = centered_rect(75, 50, size);
        let popup_height = popup_area.height as usize;
        
        // Calculate vertical centering
        let content_height = message.len();
        let available_height = popup_height.saturating_sub(2); // Subtract borders
        let vertical_padding = if available_height > content_height {
            (available_height - content_height) / 2
        } else {
            0
        };
        
        // Add top padding
        let mut centered_message = vec![];
        for _ in 0..vertical_padding {
            centered_message.push(Line::from(""));
        }
        centered_message.extend(message);
        
        frame.render_widget(Clear, popup_area);
        let popup = Paragraph::new(centered_message)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().bg(Color::Black)));
        
        frame.render_widget(popup, popup_area);
    }

    fn render_results(&self, frame: &mut Frame, results: &TestResults) {
        let size = frame.size();
        
        let score_percent = (results.passed as f32 / results.total as f32 * 100.0) as u8;
        let (score_color, score_msg) = if score_percent == 100 {
            (Color::Rgb(255, 215, 0), "PERFECT!") // Gold
        } else if score_percent >= 80 {
            (Color::Green, "GREAT!")
        } else if score_percent >= 50 {
            (Color::Yellow, "NICE TRY!")
        } else {
            (Color::Red, "GAME OVER")
        };

        // Create centered layout with border colors
        let border_color = if score_percent == 100 {
            Color::Rgb(255, 215, 0) // Gold
        } else if score_percent >= 80 {
            Color::Green
        } else if score_percent >= 50 {
            Color::Yellow
        } else {
            Color::Red
        };

        // Main layout: horizontal split for main area and scoreboard
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(65),  // Main score area
                Constraint::Percentage(35),  // Scoreboard
            ])
            .split(centered_rect(95, 90, size));

        // Main score area
        let main_area_height = main_layout[0].height as usize;
        
        // Calculate content height
        let status_lines = 1;  // Status message line
        let ascii_digit_lines = 6;  // ASCII number lines
        let summary_lines = 1;  // Summary message
        let controls_lines = 1;  // Controls message
        let spacing = 8;  // Total spacing between sections
        let total_content_height = status_lines + ascii_digit_lines + summary_lines + controls_lines + spacing;
        
        // Calculate vertical padding to center the content
        let vertical_padding = if main_area_height > total_content_height {
            (main_area_height - total_content_height) / 2
        } else {
            1
        };
        
        let mut main_text = vec![];
        
        // Add top padding for vertical centering
        for _ in 0..vertical_padding {
            main_text.push(Line::from(""));
        }

        // Status message
        main_text.push(Line::from(Span::styled(score_msg, Style::default().fg(score_color).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED))));
        main_text.push(Line::from(""));
        main_text.push(Line::from(""));
        
        // Percentage in mega size - only show necessary digits
        let percent_symbol = self.get_ascii_percent();
        
        if score_percent == 100 {
            // Show all three digits for 100%
            let digit_100 = self.get_ascii_number(1);
            let digit_10 = self.get_ascii_number(0);
            let digit_1 = self.get_ascii_number(0);
            
            for i in 0..6 {
                main_text.push(Line::from(vec![
                    Span::styled(digit_100[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(digit_10[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(digit_1[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(" ".to_string(), Style::default()),
                    Span::styled(percent_symbol[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                ]));
            }
        } else if score_percent >= 10 {
            // Show two digits for 10-99%
            let digit_10 = self.get_ascii_number((score_percent / 10) % 10);
            let digit_1 = self.get_ascii_number(score_percent % 10);
            
            for i in 0..6 {
                main_text.push(Line::from(vec![
                    Span::styled(digit_10[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(digit_1[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(" ".to_string(), Style::default()),
                    Span::styled(percent_symbol[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                ]));
            }
        } else {
            // Show one digit for 0-9%
            let digit_1 = self.get_ascii_number(score_percent % 10);
            
            for i in 0..6 {
                main_text.push(Line::from(vec![
                    Span::styled(digit_1[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                    Span::styled(" ".to_string(), Style::default()),
                    Span::styled(percent_symbol[i].clone(), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
                ]));
            }
        }

        main_text.push(Line::from(""));
        main_text.push(Line::from(""));
        
        // Summary message
        let summary = format!("You passed {} out of {} test cases!", results.passed, results.total);
        main_text.push(Line::from(Span::styled(summary, Style::default().fg(Color::White))));
        
        main_text.push(Line::from(""));
        main_text.push(Line::from(""));
        main_text.push(Line::from(""));
        main_text.push(Line::from(Span::styled("Press 'R' to restart  •  Press 'Q' to quit", Style::default().fg(Color::Rgb(255, 165, 0)))));

        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD))
            .title(" FINAL SCORE ")
            .title_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD));

        let main_paragraph = Paragraph::new(main_text)
            .block(main_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        // Scoreboard area
        let mut scoreboard_text = vec![
            Line::from(""),
        ];

        for result in &results.details {
            let status_symbol = if result.passed { "✓" } else { "✗" };
            let status_color = if result.passed { Color::Green } else { Color::Red };
            
            scoreboard_text.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(status_symbol, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" Test #{}", result.case_number), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
            
            // Compact display - use owned String
            let input_display = if result.input.len() > 30 {
                format!("{}...", &result.input[..27])
            } else {
                result.input.clone()
            };
            
            scoreboard_text.push(Line::from(vec![
                Span::styled("  In: ", Style::default().fg(Color::DarkGray)),
                Span::styled(input_display, Style::default().fg(Color::Gray)),
            ]));
            
            if result.passed {
                scoreboard_text.push(Line::from(vec![
                    Span::styled("  ✓  ", Style::default().fg(Color::Green)),
                    Span::styled(result.expected.clone(), Style::default().fg(Color::Green)),
                ]));
            } else {
                scoreboard_text.push(Line::from(vec![
                    Span::styled("  Expected: ", Style::default().fg(Color::Cyan)),
                    Span::styled(result.expected.clone(), Style::default().fg(Color::White)),
                ]));
                scoreboard_text.push(Line::from(vec![
                    Span::styled("  Got: ", Style::default().fg(Color::Red)),
                    Span::styled(result.actual.clone(), Style::default().fg(Color::White)),
                ]));
            }
            scoreboard_text.push(Line::from(""));
        }

        let scoreboard_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));

        let scoreboard_paragraph = Paragraph::new(scoreboard_text)
            .block(scoreboard_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((0, 0));

        frame.render_widget(main_paragraph, main_layout[0]);
        frame.render_widget(scoreboard_paragraph, main_layout[1]);
    }

    fn get_ascii_number(&self, digit: u8) -> [String; 6] {
        match digit {
            0 => [
                " ██████╗ ".to_string(),
                "██╔═══██╗".to_string(),
                "██║   ██║".to_string(),
                "██║   ██║".to_string(),
                "╚██████╔╝".to_string(),
                " ╚═════╝ ".to_string(),
            ],
            1 => [
                "  ██╗".to_string(),
                " ███║".to_string(),
                "  ██║".to_string(),
                "  ██║".to_string(),
                "  ██║".to_string(),
                "  ╚═╝".to_string(),
            ],
            2 => [
                "██████╗ ".to_string(),
                "╚════██╗".to_string(),
                " █████╔╝".to_string(),
                "██╔═══╝ ".to_string(),
                "███████╗".to_string(),
                "╚══════╝".to_string(),
            ],
            3 => [
                "██████╗ ".to_string(),
                "╚════██╗".to_string(),
                " █████╔╝".to_string(),
                " ╚═══██╗".to_string(),
                "██████╔╝".to_string(),
                "╚═════╝ ".to_string(),
            ],
            4 => [
                "██╗  ██╗".to_string(),
                "██║  ██║".to_string(),
                "███████║".to_string(),
                "╚════██║".to_string(),
                "     ██║".to_string(),
                "     ╚═╝".to_string(),
            ],
            5 => [
                "███████╗".to_string(),
                "██╔════╝".to_string(),
                "███████╗".to_string(),
                "╚════██║".to_string(),
                "███████║".to_string(),
                "╚══════╝".to_string(),
            ],
            6 => [
                " ██████╗ ".to_string(),
                "██╔════╝ ".to_string(),
                "███████╗ ".to_string(),
                "██╔═══██╗".to_string(),
                "╚██████╔╝".to_string(),
                " ╚═════╝ ".to_string(),
            ],
            7 => [
                "███████╗".to_string(),
                "╚════██║".to_string(),
                "    ██╔╝".to_string(),
                "   ██╔╝ ".to_string(),
                "  ██╔╝  ".to_string(),
                "  ╚═╝   ".to_string(),
            ],
            8 => [
                " ██████╗ ".to_string(),
                "██╔═══██╗".to_string(),
                "╚██████╔╝".to_string(),
                "██╔═══██╗".to_string(),
                "╚██████╔╝".to_string(),
                " ╚═════╝ ".to_string(),
            ],
            9 => [
                " ██████╗ ".to_string(),
                "██╔═══██╗".to_string(),
                "╚██████╔╝".to_string(),
                " ╚════██║".to_string(),
                " █████╔╝".to_string(),
                " ╚════╝ ".to_string(),
            ],
            _ => [
                "   ".to_string(),
                "   ".to_string(),
                "   ".to_string(),
                "   ".to_string(),
                "   ".to_string(),
                "   ".to_string(),
            ],
        }
    }

    fn get_ascii_percent(&self) -> [String; 6] {
        [
            "██╗ ██╗".to_string(),
            "██║██╔╝".to_string(),
            "╚═██╔╝ ".to_string(),
            " ██╔╝██╗".to_string(),
            "██╔╝ ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
        ]
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
