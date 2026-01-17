use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::languages::Language;

pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    pub fn highlight_line<'a>(line: &'a str, language: &Language) -> Vec<Span<'a>> {
        match language {
            Language::Python => Self::highlight_python(line),
            Language::JavaScript => Self::highlight_javascript(line),
            Language::TypeScript => Self::highlight_typescript(line),
            Language::Rust => Self::highlight_rust(line),
            Language::Go => Self::highlight_go(line),
            Language::Java => Self::highlight_java(line),
            Language::Haskell => Self::highlight_haskell(line),
            Language::Lua => Self::highlight_lua(line),
            Language::OCaml => Self::highlight_ocaml(line),
            Language::Elixir => Self::highlight_elixir(line),
            Language::Kotlin => Self::highlight_kotlin(line),
            Language::Swift => Self::highlight_swift(line),
        }
    }

    fn highlight_python(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "def", "class", "if", "else", "elif", "for", "while", "return", "import",
            "from", "as", "try", "except", "finally", "with", "pass", "break", "continue",
            "and", "or", "not", "in", "is", "lambda", "yield", "async", "await", "raise",
            "del", "global", "nonlocal", "assert",
        ];
        let builtins = ["None", "True", "False", "self", "cls"];
        let types = ["int", "str", "bool", "float", "list", "dict", "set", "tuple", "range", "len", "print"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '#',
            Color::Rgb(197, 134, 192),  // Purple keywords (like VS Code)
            Color::Rgb(78, 201, 176),   // Teal for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,                // Comments
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_javascript(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "function", "const", "let", "var", "if", "else", "for", "while", "return",
            "class", "new", "this", "super", "extends", "import", "export", "from",
            "async", "await", "try", "catch", "finally", "throw", "typeof", "instanceof",
            "break", "continue", "switch", "case", "default", "do", "of", "in",
            "static", "get", "set", "delete", "void", "yield",
        ];
        let builtins = ["null", "undefined", "true", "false", "NaN", "Infinity"];
        let types = ["Array", "Object", "String", "Number", "Boolean", "Symbol", "Promise", "Map", "Set"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_typescript(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "function", "const", "let", "var", "if", "else", "for", "while", "return",
            "class", "new", "this", "super", "extends", "import", "export", "from",
            "async", "await", "try", "catch", "finally", "throw", "typeof", "instanceof",
            "break", "continue", "switch", "case", "default", "do", "of", "in",
            "interface", "type", "enum", "namespace", "public", "private",
            "protected", "readonly", "static", "abstract", "implements", "as",
            "declare", "module", "any", "unknown", "never",
        ];
        let builtins = ["null", "undefined", "true", "false"];
        let types = ["string", "number", "boolean", "void", "Array", "Promise", "Record", "Partial"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_rust(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "fn", "let", "mut", "const", "static", "if", "else", "match", "loop", "while",
            "for", "return", "struct", "enum", "impl", "trait", "pub", "use", "mod", "crate",
            "self", "super", "async", "await", "move", "ref", "as", "where", "unsafe",
            "extern", "type", "dyn", "box", "break", "continue",
        ];
        let builtins = ["true", "false", "Some", "None", "Ok", "Err"];
        let types = ["Vec", "HashMap", "String", "str", "i32", "i64", "u32", "u64", "f32", "f64",
                    "bool", "char", "usize", "isize", "Option", "Result"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_go(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "func", "var", "const", "if", "else", "for", "range", "return", "struct",
            "interface", "type", "package", "import", "go", "defer", "chan", "select",
            "case", "default", "break", "continue", "switch", "map", "fallthrough",
        ];
        let builtins = ["true", "false", "nil", "iota"];
        let types = ["int", "int8", "int16", "int32", "int64", "uint", "uint8", "uint16", "uint32", "uint64",
                    "float32", "float64", "string", "bool", "byte", "rune", "error", "make", "new", "len", "cap"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_java(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "public", "private", "protected", "class", "interface", "extends", "implements",
            "new", "this", "super", "static", "final", "abstract", "void",
            "if", "else", "for", "while", "do", "switch", "case", "default",
            "return", "try", "catch", "finally", "throw", "throws", "import", "package",
            "break", "continue", "synchronized", "volatile", "transient", "native",
            "instanceof", "enum", "assert",
        ];
        let builtins = ["true", "false", "null"];
        let types = ["int", "long", "double", "float", "boolean", "char", "byte", "short",
                    "String", "Integer", "Long", "Double", "Float", "Boolean", "Character",
                    "List", "Map", "Set", "ArrayList", "HashMap", "HashSet", "Object"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_haskell(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "module", "where", "import", "as", "qualified", "hiding",
            "class", "instance", "data", "type", "newtype", "deriving",
            "if", "then", "else", "case", "of", "let", "in", "do",
            "where", "forall", "infixl", "infixr", "infix",
        ];
        let builtins = ["True", "False", "Nothing", "Just", "Left", "Right", "pure", "return", "fmap", "bind"];
        let types = ["Int", "Integer", "Float", "Double", "Bool", "Char", "String",
                    "Maybe", "Either", "IO", "Monad", "Functor", "Applicative", "List"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '-',  // Haskell uses -- for comments
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_lua(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "and", "break", "do", "else", "elseif", "end", "false", "for", "function",
            "if", "in", "local", "nil", "not", "or", "repeat", "return", "then",
            "true", "until", "while", "goto",
        ];
        let builtins = ["nil", "true", "false", "self", "_G", "_VERSION"];
        let types = ["print", "type", "tonumber", "tostring", "pairs", "ipairs", "require",
                    "setmetatable", "getmetatable", "next", "select", "rawget", "rawset"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '-',  // Lua uses -- for comments
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_ocaml(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "let", "rec", "in", "fun", "function", "match", "with", "if", "then", "else",
            "type", "module", "struct", "sig", "open", "include", "val", "mutable",
            "begin", "end", "for", "while", "do", "done", "to", "downto", "of",
            "and", "or", "not", "true", "false", "ref", "as", "when", "exception", "raise", "try",
        ];
        let builtins = ["true", "false", "None", "Some", "Ok", "Error"];
        let types = ["int", "float", "bool", "char", "string", "unit", "list", "array",
                    "option", "result", "ref", "List", "Array", "String"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '#',  // OCaml can use # in some contexts, but (* *) for comments
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_elixir(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "def", "defp", "defmodule", "defmacro", "defstruct", "defprotocol", "defimpl",
            "do", "end", "if", "unless", "cond", "case", "when", "receive", "after",
            "fn", "for", "try", "catch", "rescue", "raise", "throw", "with",
            "import", "alias", "require", "use", "quote", "unquote",
        ];
        let builtins = ["true", "false", "nil", "self", "__MODULE__", "__ENV__"];
        let types = ["String", "Integer", "Float", "List", "Map", "Tuple", "Atom", "GenServer",
                    "Enum", "Stream", "Process", "Agent", "Task"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '#',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_kotlin(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "fun", "val", "var", "class", "object", "interface", "data", "sealed",
            "if", "else", "when", "for", "while", "do", "return", "break", "continue",
            "try", "catch", "finally", "throw", "import", "package", "as", "in", "is",
            "public", "private", "protected", "internal", "open", "abstract", "final",
            "override", "lateinit", "by", "companion", "inline", "suspend", "const",
        ];
        let builtins = ["true", "false", "null", "this", "super", "it"];
        let types = ["Int", "Long", "Double", "Float", "Boolean", "Char", "String", "Byte", "Short",
                    "List", "Map", "Set", "MutableList", "MutableMap", "MutableSet", "Array",
                    "Unit", "Any", "Nothing"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn highlight_swift(line: &str) -> Vec<Span<'_>> {
        let keywords = [
            "func", "let", "var", "class", "struct", "enum", "protocol", "extension",
            "if", "else", "guard", "switch", "case", "default", "for", "while", "repeat",
            "return", "break", "continue", "fallthrough", "import", "in", "is", "as",
            "try", "catch", "throw", "throws", "defer", "do", "where", "typealias",
            "public", "private", "fileprivate", "internal", "open", "static", "final",
            "lazy", "mutating", "nonmutating", "override", "required", "convenience",
            "dynamic", "inout", "weak", "unowned", "indirect",
        ];
        let builtins = ["true", "false", "nil", "self", "Self", "super"];
        let types = ["Int", "Int8", "Int16", "Int32", "Int64", "UInt", "UInt8", "UInt16", "UInt32", "UInt64",
                    "Double", "Float", "Bool", "String", "Character", "Array", "Dictionary", "Set",
                    "Optional", "Any", "AnyObject", "Void"];
        
        Self::advanced_tokenize(
            line,
            &keywords,
            &builtins,
            &types,
            '/',
            Color::Rgb(197, 134, 192),  // Purple keywords
            Color::Rgb(86, 156, 214),   // Blue for builtins
            Color::Rgb(78, 201, 176),   // Teal for types
            Color::Gray,
            Color::Rgb(206, 145, 120),  // Orange strings
            Color::Rgb(181, 206, 168),  // Light green numbers
            Color::Rgb(220, 220, 170),  // Light yellow functions
        )
    }

    fn advanced_tokenize<'a>(
        line: &'a str,
        keywords: &[&str],
        builtins: &[&str],
        types: &[&str],
        comment_char: char,
        keyword_color: Color,
        builtin_color: Color,
        type_color: Color,
        comment_color: Color,
        string_color: Color,
        number_color: Color,
        function_color: Color,
    ) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        
        // Check for comment first
        if let Some(comment_pos) = line.find(comment_char) {
            if comment_char == '/' && comment_pos + 1 < line.len() && line.chars().nth(comment_pos + 1) == Some('/') {
                let before_comment = &line[..comment_pos];
                if !before_comment.is_empty() {
                    spans.extend(Self::colorize_advanced(
                        before_comment, keywords, builtins, types,
                        keyword_color, builtin_color, type_color,
                        string_color, number_color, function_color
                    ));
                }
                spans.push(Span::styled(
                    &line[comment_pos..],
                    Style::default().fg(comment_color).add_modifier(Modifier::ITALIC),
                ));
                return spans;
            } else if comment_char == '#' {
                let before_comment = &line[..comment_pos];
                if !before_comment.is_empty() {
                    spans.extend(Self::colorize_advanced(
                        before_comment, keywords, builtins, types,
                        keyword_color, builtin_color, type_color,
                        string_color, number_color, function_color
                    ));
                }
                spans.push(Span::styled(
                    &line[comment_pos..],
                    Style::default().fg(comment_color).add_modifier(Modifier::ITALIC),
                ));
                return spans;
            }
        }
        
        Self::colorize_advanced(
            line, keywords, builtins, types,
            keyword_color, builtin_color, type_color,
            string_color, number_color, function_color
        )
    }

    fn colorize_advanced<'a>(
        text: &'a str,
        keywords: &[&str],
        builtins: &[&str],
        types: &[&str],
        keyword_color: Color,
        builtin_color: Color,
        type_color: Color,
        string_color: Color,
        number_color: Color,
        function_color: Color,
    ) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_char = '\0';
        let mut string_content = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if in_string {
                string_content.push(ch);
                if ch == string_char && (i == 0 || chars[i - 1] != '\\') {
                    spans.push(Span::styled(
                        string_content.clone(),
                        Style::default().fg(string_color),
                    ));
                    string_content.clear();
                    in_string = false;
                }
            } else if ch == '"' || ch == '\'' || ch == '`' {
                if !current_word.is_empty() {
                    Self::push_advanced_word(
                        &mut spans, &current_word, keywords, builtins, types,
                        keyword_color, builtin_color, type_color, number_color, function_color,
                        i < chars.len() - 1 && chars[i + 1] == '('
                    );
                    current_word.clear();
                }
                in_string = true;
                string_char = ch;
                string_content.push(ch);
            } else if ch.is_whitespace() || "(){}[],.;:+-*/%=<>!&|".contains(ch) {
                if !current_word.is_empty() {
                    let next_is_paren = i < chars.len() - 1 && chars[i + 1] == '(';
                    Self::push_advanced_word(
                        &mut spans, &current_word, keywords, builtins, types,
                        keyword_color, builtin_color, type_color, number_color, function_color,
                        next_is_paren
                    );
                    current_word.clear();
                }
                
                // Colorize operators
                let operator_color = if "+-*/%=<>!&|".contains(ch) {
                    Color::Rgb(212, 212, 212)  // Light gray for operators
                } else {
                    Color::White
                };
                spans.push(Span::styled(ch.to_string(), Style::default().fg(operator_color)));
            } else {
                current_word.push(ch);
            }
            
            i += 1;
        }
        
        if in_string {
            spans.push(Span::styled(string_content, Style::default().fg(string_color)));
        }
        
        if !current_word.is_empty() {
            Self::push_advanced_word(
                &mut spans, &current_word, keywords, builtins, types,
                keyword_color, builtin_color, type_color, number_color, function_color,
                false
            );
        }
        
        spans
    }

    fn push_advanced_word(
        spans: &mut Vec<Span>,
        word: &str,
        keywords: &[&str],
        builtins: &[&str],
        types: &[&str],
        keyword_color: Color,
        builtin_color: Color,
        type_color: Color,
        number_color: Color,
        function_color: Color,
        next_is_paren: bool,
    ) {
        if keywords.contains(&word) {
            spans.push(Span::styled(
                word.to_string(),
                Style::default().fg(keyword_color).add_modifier(Modifier::BOLD),
            ));
        } else if builtins.contains(&word) {
            spans.push(Span::styled(
                word.to_string(),
                Style::default().fg(builtin_color).add_modifier(Modifier::BOLD),
            ));
        } else if types.contains(&word) {
            spans.push(Span::styled(
                word.to_string(),
                Style::default().fg(type_color),
            ));
        } else if next_is_paren {
            // Function call
            spans.push(Span::styled(
                word.to_string(),
                Style::default().fg(function_color),
            ));
        } else if word.chars().all(|c| c.is_numeric() || c == '.' || c == '_') && word.chars().any(|c| c.is_numeric()) {
            spans.push(Span::styled(word.to_string(), Style::default().fg(number_color)));
        } else if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            // Likely a class or type (capitalized)
            spans.push(Span::styled(word.to_string(), Style::default().fg(type_color)));
        } else {
            spans.push(Span::styled(word.to_string(), Style::default().fg(Color::Rgb(212, 212, 212))));
        }
    }
}

