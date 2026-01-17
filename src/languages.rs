use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    TypeScript,
    Python,
    Rust,
    Go,
    Java,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![
            Language::JavaScript,
            Language::TypeScript,
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
        
        // If no other languages available, return self or a random from all
        if others.is_empty() {
            // If only one language total, just return it
            Language::all().first().copied().unwrap_or(*self)
        } else {
            *others.choose(&mut rng).unwrap()
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Python => "python",
            Language::Rust => "rust",
            Language::Go => "go",
            Language::Java => "java",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::Java => "Java",
        }
    }
}

pub fn build_translation_prompt(code: &str, from: Language, to: Language) -> String {
    let mut extra_rules = String::new();

    if from == Language::Python && to != Language::Python {
        extra_rules.push_str(
            r#"
PYTHON->BRACES RULE:
- Reconstruct { } blocks based on indentation levels.
- When indentation increases, open a new block; when it decreases, close blocks.
- Do not change control flow; only map indentation to braces."#,
        );
    }

    format!(
        r#"You are a code translator that performs LITERAL translations.

CRITICAL RULES:
1. DO NOT fix bugs - translate the code exactly as-is, including any bugs
2. DO NOT complete unfinished code - if code is incomplete, keep it incomplete
3. DO NOT add any code not in the original
4. Preserve incomplete variable names (e.g., "let fo" should stay as a partial declaration)
5. This is LIVE typing - the code is intentionally incomplete and being actively typed
6. Preserve the exact structure, even if it's syntactically invalid
7. If a line is cut off mid-word, translate the complete words and keep the partial word as-is
{}

Translate this {} code to {}:

{}

Output ONLY the translated code, no markdown formatting, no explanations, no code fences."#,
        extra_rules,
        from.display_name(),
        to.display_name(),
        code
    )
}
