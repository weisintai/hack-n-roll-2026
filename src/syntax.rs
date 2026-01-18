use ratatui::style::{Color, Style};
use ratatui::text::Span;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use once_cell::sync::Lazy;

use crate::languages::Language;

// Global syntax set and theme - loaded once
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

pub struct SyntectHighlighter;

impl SyntectHighlighter {
    /// Highlight a line of code using syntect
    pub fn highlight(line: &str, language: &Language) -> Vec<Span<'static>> {
        let ext = Self::get_extension(language);

        // Try extension first (most reliable), then name, then fallback
        let syntax = SYNTAX_SET
            .find_syntax_by_extension(ext)
            .or_else(|| {
                // Fallback: use similar language for unsupported ones
                let fallback_ext = Self::get_fallback_extension(language);
                SYNTAX_SET.find_syntax_by_extension(fallback_ext)
            })
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

        let theme = &THEME_SET.themes["base16-ocean.dark"];
        let mut hl = HighlightLines::new(syntax, theme);

        match hl.highlight_line(line, &SYNTAX_SET) {
            Ok(ranges) => {
                ranges.into_iter().map(|(style, text)| {
                    Span::styled(text.to_string(), syntect_to_ratatui_style(style))
                }).collect()
            }
            Err(_) => {
                vec![Span::raw(line.to_string())]
            }
        }
    }

    /// Get file extension for syntax lookup
    fn get_extension(language: &Language) -> &'static str {
        match language {
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::TypeScript => "ts",
            Language::Rust => "rs",
            Language::Go => "go",
            Language::Java => "java",
            Language::Haskell => "hs",
            Language::Lua => "lua",
            Language::OCaml => "ml",
            Language::Elixir => "ex",
            Language::Kotlin => "kt",
            Language::Swift => "swift",
        }
    }

    /// Fallback extensions for languages not in syntect defaults
    fn get_fallback_extension(language: &Language) -> &'static str {
        match language {
            // TypeScript -> JavaScript (similar syntax)
            Language::TypeScript => "js",
            // Swift -> C (similar enough for basic highlighting)
            Language::Swift => "c",
            // Kotlin -> Java (JVM language, similar syntax)
            Language::Kotlin => "java",
            // Elixir -> Ruby (similar syntax style)
            Language::Elixir => "rb",
            // OCaml -> use plain text (ML family not common)
            Language::OCaml => "ml",
            // Everything else: plain text
            _ => "txt",
        }
    }
}

/// Convert syntect style to ratatui style
fn syntect_to_ratatui_style(style: SyntectStyle) -> Style {
    let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
    Style::default().fg(fg)
}

