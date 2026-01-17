use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::languages::Language;

pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    pub fn highlight_line<'a>(line: &'a str, language: &Language) -> Vec<Span<'a>> {
        match language {
            Language::Python => Self::highlight_python(line),
            Language::JavaScript => Self::highlight_javascript(line),
            Language::Rust => Self::highlight_rust(line),
            Language::Go => Self::highlight_go(line),
            Language::Java => Self::highlight_java(line),
        }
    }

    fn highlight_python(line: &str) -> Vec<Span> {
        let keywords = [
            "def", "class", "if", "else", "elif", "for", "while", "return", "import",
            "from", "as", "try", "except", "finally", "with", "pass", "break", "continue",
            "and", "or", "not", "in", "is", "lambda", "yield", "async", "await", "None",
            "True", "False",
        ];
        
        Self::tokenize_and_color(
            line,
            &keywords,
            '#',
            Color::Green,    // Keywords
            Color::Gray,     // Comments
            Color::Yellow,   // Strings
            Color::Cyan,     // Numbers
        )
    }

    fn highlight_javascript(line: &str) -> Vec<Span> {
        let keywords = [
            "function", "const", "let", "var", "if", "else", "for", "while", "return",
            "class", "new", "this", "super", "extends", "import", "export", "from",
            "async", "await", "try", "catch", "finally", "throw", "typeof", "instanceof",
            "null", "undefined", "true", "false", "break", "continue", "switch", "case",
            "default", "do",
        ];
        
        Self::tokenize_and_color(
            line,
            &keywords,
            '/',
            Color::Magenta,  // Keywords
            Color::Gray,     // Comments
            Color::Yellow,   // Strings
            Color::Cyan,     // Numbers
        )
    }


    fn highlight_rust(line: &str) -> Vec<Span> {
        let keywords = [
            "fn", "let", "mut", "const", "static", "if", "else", "match", "loop", "while",
            "for", "return", "struct", "enum", "impl", "trait", "pub", "use", "mod", "crate",
            "self", "super", "async", "await", "move", "ref", "true", "false", "Some", "None",
            "Ok", "Err", "Vec", "HashMap", "String", "i32", "i64", "u32", "u64", "f32", "f64",
            "bool", "char", "usize",
        ];
        
        Self::tokenize_and_color(
            line,
            &keywords,
            '/',
            Color::LightRed,   // Keywords
            Color::Gray,       // Comments
            Color::Yellow,     // Strings
            Color::Cyan,       // Numbers
        )
    }

    fn highlight_go(line: &str) -> Vec<Span> {
        let keywords = [
            "func", "var", "const", "if", "else", "for", "range", "return", "struct",
            "interface", "type", "package", "import", "go", "defer", "chan", "select",
            "case", "default", "break", "continue", "switch", "map", "make", "new",
            "true", "false", "nil",
        ];
        
        Self::tokenize_and_color(
            line,
            &keywords,
            '/',
            Color::LightCyan,  // Keywords
            Color::Gray,       // Comments
            Color::Yellow,     // Strings
            Color::Cyan,       // Numbers
        )
    }

    fn highlight_java(line: &str) -> Vec<Span> {
        let keywords = [
            "public", "private", "protected", "class", "interface", "extends", "implements",
            "new", "this", "super", "static", "final", "abstract", "void", "int", "long",
            "double", "float", "boolean", "char", "String", "if", "else", "for", "while",
            "return", "try", "catch", "finally", "throw", "throws", "import", "package",
            "true", "false", "null", "break", "continue", "switch", "case", "default",
        ];
        
        Self::tokenize_and_color(
            line,
            &keywords,
            '/',
            Color::LightMagenta, // Keywords
            Color::Gray,         // Comments
            Color::Yellow,       // Strings
            Color::Cyan,         // Numbers
        )
    }

    fn tokenize_and_color<'a>(
        line: &'a str,
        keywords: &[&str],
        comment_char: char,
        keyword_color: Color,
        comment_color: Color,
        string_color: Color,
        number_color: Color,
    ) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        
        // Check for comment first
        if let Some(comment_pos) = line.find(comment_char) {
            // Handle double slash comments
            if comment_char == '/' && comment_pos + 1 < line.len() && line.chars().nth(comment_pos + 1) == Some('/') {
                let before_comment = &line[..comment_pos];
                if !before_comment.is_empty() {
                    spans.extend(Self::colorize_code(before_comment, keywords, keyword_color, string_color, number_color));
                }
                spans.push(Span::styled(
                    &line[comment_pos..],
                    Style::default().fg(comment_color).add_modifier(Modifier::ITALIC),
                ));
                return spans;
            } else if comment_char == '#' {
                let before_comment = &line[..comment_pos];
                if !before_comment.is_empty() {
                    spans.extend(Self::colorize_code(before_comment, keywords, keyword_color, string_color, number_color));
                }
                spans.push(Span::styled(
                    &line[comment_pos..],
                    Style::default().fg(comment_color).add_modifier(Modifier::ITALIC),
                ));
                return spans;
            }
        }
        
        Self::colorize_code(line, keywords, keyword_color, string_color, number_color)
    }

    fn colorize_code<'a>(
        text: &'a str,
        keywords: &[&str],
        keyword_color: Color,
        string_color: Color,
        number_color: Color,
    ) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_char = '\0';
        let mut string_content = String::new();
        
        for ch in text.chars() {
            if in_string {
                string_content.push(ch);
                if ch == string_char && !string_content.ends_with(&format!("\\{}", string_char)) {
                    spans.push(Span::styled(
                        string_content.clone(),
                        Style::default().fg(string_color),
                    ));
                    string_content.clear();
                    in_string = false;
                }
            } else if ch == '"' || ch == '\'' {
                if !current_word.is_empty() {
                    Self::push_word(&mut spans, &current_word, keywords, keyword_color, number_color);
                    current_word.clear();
                }
                in_string = true;
                string_char = ch;
                string_content.push(ch);
            } else if ch.is_whitespace() || ch == '(' || ch == ')' || ch == '{' || ch == '}' 
                   || ch == '[' || ch == ']' || ch == ',' || ch == ';' || ch == ':' || ch == '.' {
                if !current_word.is_empty() {
                    Self::push_word(&mut spans, &current_word, keywords, keyword_color, number_color);
                    current_word.clear();
                }
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White)));
            } else {
                current_word.push(ch);
            }
        }
        
        if in_string {
            spans.push(Span::styled(string_content, Style::default().fg(string_color)));
        }
        
        if !current_word.is_empty() {
            Self::push_word(&mut spans, &current_word, keywords, keyword_color, number_color);
        }
        
        spans
    }

    fn push_word(
        spans: &mut Vec<Span>,
        word: &str,
        keywords: &[&str],
        keyword_color: Color,
        number_color: Color,
    ) {
        if keywords.contains(&word) {
            spans.push(Span::styled(
                word.to_string(),
                Style::default().fg(keyword_color).add_modifier(Modifier::BOLD),
            ));
        } else if word.chars().all(|c| c.is_numeric() || c == '.') {
            spans.push(Span::styled(word.to_string(), Style::default().fg(number_color)));
        } else {
            spans.push(Span::styled(word.to_string(), Style::default().fg(Color::White)));
        }
    }
}
