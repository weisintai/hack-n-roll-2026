use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    Python,
    Rust,
    Go,
    Java,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![
            Language::JavaScript,
            Language::Python,
            Language::Rust,
            Language::Go,
            Language::Java,
        ]
    }

    pub fn random_except(&self) -> Language {
        let mut rng = rand::thread_rng();
        let others: Vec<_> = Language::all()
            .into_iter()
            .filter(|l| l != self)
            .collect();
        *others.choose(&mut rng).unwrap()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::JavaScript => "javascript",
            Language::Python => "python",
            Language::Rust => "rust",
            Language::Go => "go",
            Language::Java => "java",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::JavaScript => "JavaScript",
            Language::Python => "Python",
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::Java => "Java",
        }
    }

    fn score_hint(&self, code: &str) -> i32 {
        fn bump(score: &mut i32, code: &str, token: &str, weight: i32) {
            if code.contains(token) {
                *score += weight;
            }
        }

        let mut score = 0;
        match self {
            Language::Python => {
                bump(&mut score, code, "def ", 3);
                bump(&mut score, code, "class ", 2);
                bump(&mut score, code, "import ", 2);
                bump(&mut score, code, "from ", 1);
                bump(&mut score, code, "self", 2);
                bump(&mut score, code, "None", 2);
                bump(&mut score, code, "True", 1);
                bump(&mut score, code, "False", 1);
                bump(&mut score, code, "elif ", 1);
                bump(&mut score, code, "pass", 1);
                bump(&mut score, code, "#", 1);
            }
            Language::JavaScript => {
                bump(&mut score, code, "function ", 2);
                bump(&mut score, code, "const ", 2);
                bump(&mut score, code, "let ", 2);
                bump(&mut score, code, "var ", 1);
                bump(&mut score, code, "=>", 2);
                bump(&mut score, code, "console.log", 2);
                bump(&mut score, code, "undefined", 1);
                bump(&mut score, code, "null", 1);
                bump(&mut score, code, "export ", 1);
                bump(&mut score, code, "import ", 1);
            }
            Language::Rust => {
                bump(&mut score, code, "fn ", 3);
                bump(&mut score, code, "let ", 2);
                bump(&mut score, code, "mut ", 1);
                bump(&mut score, code, "println!", 3);
                bump(&mut score, code, "Vec<", 2);
                bump(&mut score, code, "Option<", 2);
                bump(&mut score, code, "::", 2);
                bump(&mut score, code, "->", 1);
                bump(&mut score, code, "usize", 2);
                bump(&mut score, code, "i32", 2);
                bump(&mut score, code, "i64", 2);
                bump(&mut score, code, "pub ", 1);
                bump(&mut score, code, "impl ", 2);
            }
            Language::Go => {
                bump(&mut score, code, "func ", 3);
                bump(&mut score, code, "package ", 2);
                bump(&mut score, code, "fmt.", 2);
                bump(&mut score, code, ":=", 2);
                bump(&mut score, code, "[]int", 2);
                bump(&mut score, code, "[]string", 2);
                bump(&mut score, code, "map[", 2);
                bump(&mut score, code, "struct {", 2);
                bump(&mut score, code, "interface {", 2);
                bump(&mut score, code, "nil", 1);
            }
            Language::Java => {
                bump(&mut score, code, "public class", 3);
                bump(&mut score, code, "public static", 2);
                bump(&mut score, code, "System.out", 3);
                bump(&mut score, code, "new ", 1);
                bump(&mut score, code, "String", 2);
                bump(&mut score, code, "int ", 1);
                bump(&mut score, code, "boolean", 1);
                bump(&mut score, code, "void ", 1);
                bump(&mut score, code, "package ", 2);
                bump(&mut score, code, "import ", 1);
                bump(&mut score, code, "extends ", 1);
                bump(&mut score, code, "implements ", 1);
            }
        }
        score
    }

    pub fn detect_best_language(code: &str) -> Option<Language> {
        let trimmed = code.trim();
        if trimmed.is_empty() {
            return None;
        }

        let mut best = None;
        let mut best_score = 0;
        let mut second_best = 0;

        for lang in Language::all() {
            let score = lang.score_hint(trimmed);
            if score > best_score {
                second_best = best_score;
                best_score = score;
                best = Some(lang);
            } else if score > second_best {
                second_best = score;
            }
        }

        if best_score < 2 || (best_score - second_best) < 2 {
            None
        } else {
            best
        }
    }
}

// Code conversion is now handled by src/llm.rs using the Gemini API
