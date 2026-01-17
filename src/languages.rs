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
    Haskell,
    Lua,
    OCaml,
    Elixir,
    Kotlin,
    Swift,
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
            Language::Haskell,
            Language::Lua,
            Language::OCaml,
            Language::Elixir,
            Language::Kotlin,
            Language::Swift,
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

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::Haskell => "Haskell",
            Language::Lua => "Lua",
            Language::OCaml => "OCaml",
            Language::Elixir => "Elixir",
            Language::Kotlin => "Kotlin",
            Language::Swift => "Swift",
        }
    }
}

pub fn build_translation_prompt(code: &str, from: Language, to: Language) -> String {
    let mut extra_rules = String::new();

    // Add target language specific syntax rules
    match to {
        Language::Elixir => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (Elixir):
- Functions are called with Module.function(args), NOT object.method() style
- String reversal: String.reverse(s)
- List operations: Enum.map(list, fn), List.first(list)
- Pattern matching: case value do ... end
- Pipe operator: value |> function() for chaining
Example: s |> String.reverse() or String.reverse(s)"#,
            );
        }
        Language::Kotlin => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (Kotlin):
- String reversal: s.reversed()
- Array operations: array.map { }, array.filter { }
- Constructors: ClassName() or arrayOf(), listOf()
- Extension functions: value.function() NOT ClassName.new()
- When expression: when (value) { ... }
Example: val result = s.reversed()"#,
            );
        }
        Language::Swift => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (Swift):
- String reversal: String(s.reversed())
- Array operations: array.map { }, array.filter { }
- Constructors: ClassName() or []
- Optional handling: value ?? default, if let, guard let
- Switch: switch value { case ... }
Example: let result = String(s.reversed())"#,
            );
        }
        Language::Haskell => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (Haskell):
- Function application: function arg, NOT function(arg)
- List operations: map, filter, reverse, etc.
- Pattern matching: case expr of ...
- String is [Char], so reverse works directly
- Function composition: f . g
Example: reverse s"#,
            );
        }
        Language::Lua => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (Lua):
- Functions: function name(args) ... end
- String operations: string.reverse(s), string.sub()
- Tables (arrays): {1, 2, 3}, use ipairs() to iterate
- Conditionals: if condition then ... end
- Loops: for i = 1, n do ... end
Example: string.reverse(s)"#,
            );
        }
        Language::OCaml => {
            extra_rules.push_str(
                r#"

TARGET LANGUAGE SYNTAX (OCaml):
- Function application: function arg, NOT function(arg)
- String operations: String.reverse, String.concat
- List operations: List.map, List.filter, List.rev
- Pattern matching: match expr with | pattern -> result
- Let bindings: let name = value in ...
Example: String.reverse s or List.rev s"#,
            );
        }
        _ => {}
    }

    // Handle source language specific conversions
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
        r#"You are a code translator that converts code from one language to another.

CRITICAL RULES:
1. PRESERVE LOGIC: Keep the same algorithmic logic and structure
2. USE VALID SYNTAX: You MUST use syntactically correct {} syntax
3. DO NOT fix bugs - translate bugs as-is (but with valid syntax)
4. DO NOT complete unfinished code - if incomplete, keep it incomplete
5. DO NOT add any code not in the original
6. This is LIVE typing - code may be intentionally incomplete
7. Preserve incomplete lines as-is (e.g., "let fo" stays "let fo")
{}

IMPORTANT: "Literal" means preserve the LOGIC and INTENT, NOT copy syntax from the source language.
You must translate to IDIOMATIC {} syntax while preserving the original structure and incompleteness.

Translate this {} code to {}:

{}

Output ONLY the translated code, no markdown formatting, no explanations, no code fences."#,
        to.display_name(),
        extra_rules,
        to.display_name(),
        from.display_name(),
        to.display_name(),
        code
    )
}
