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

pub fn build_translation_prompt_with_signature(code: &str, from: Language, to: Language, type_signature: Option<&str>) -> String {
    let mut extra_rules = String::new();

    // Add type signature hint if provided
    if let Some(sig) = type_signature {
        extra_rules.push_str(&format!(
            r#"

FUNCTION SIGNATURE (use these types for the target language):
{}
- Translate these types to idiomatic {} equivalents (e.g., int[] -> Vec<i32> in Rust, List[int] in Python, number[] in TypeScript)"#,
            sig, to.display_name()
        ));
    }

    // Add mandatory syntax example for target language
    let syntax_example = match to {
        Language::Python => r#"

PYTHON SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
def function_name(param: str) -> str:
    # comment
    return param.upper()
REQUIRED: Use 'def', colons, proper indentation, NO braces, NO semicolons"#,
        Language::JavaScript => r#"

JAVASCRIPT SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
function functionName(param) {
    // comment
    return param.toUpperCase();
}
REQUIRED: Use 'function', braces {}, semicolons, camelCase"#,
        Language::TypeScript => r#"

TYPESCRIPT SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
function functionName(param: string): string {
    // comment
    return param.toUpperCase();
}
REQUIRED: Use 'function', type annotations with colons, braces {}"#,
        Language::Rust => r#"

RUST SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
pub fn function_name(param: String) -> String {
    // comment
    param.to_uppercase()
}
REQUIRED: Use 'fn', braces {}, NO semicolon on return, snake_case"#,
        Language::Go => r#"

GO SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
func functionName(param string) string {
    // comment
    return strings.ToUpper(param)
}
REQUIRED: Use 'func', return type AFTER params, braces {}"#,
        Language::Java => r#"

JAVA SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
public String functionName(String param) {
    // comment
    return param.toUpperCase();
}
REQUIRED: Use 'public', return type BEFORE name, braces {}, semicolons"#,
        Language::Swift => r#"

SWIFT SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
func functionName(_ param: String) -> String {
    // comment
    return param.uppercased()
}
REQUIRED: Use 'func', arrow '->' NOT '→', braces {}"#,
        Language::Kotlin => r#"

KOTLIN SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
fun functionName(param: String): String {
    // comment
    return param.uppercase()
}
REQUIRED: Use 'fun', colon before return type, braces {}"#,
        Language::Haskell => r#"

HASKELL SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
functionName :: String -> String
functionName param = 
    -- comment
    map toUpper param
REQUIRED: Type signature on separate line, NO braces, NO semicolons"#,
        Language::Lua => r#"

LUA SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
function functionName(param)
    -- comment
    return param:upper()
end
REQUIRED: Use 'function', 'end' keyword, NO braces, colon for methods"#,
        Language::OCaml => r#"

OCAML SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
let function_name param : string =
  (* comment *)
  String.uppercase_ascii param
REQUIRED: Use 'let', NO braces, NO semicolons at end"#,
        Language::Elixir => r#"

ELIXIR SYNTAX EXAMPLE (YOU MUST USE THIS EXACT FORMAT):
def function_name(param) do
  # comment
  String.reverse(param)
end
REQUIRED: Use 'def', 'do/end' NOT braces, Module.function() calls"#,
    };
    extra_rules.push_str(syntax_example);

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
        r#"You are a fast, precise code translator. Think concisely and respond immediately.

TASK: Translate {} to {} code.

SPEED INSTRUCTIONS:
- Use minimal internal reasoning
- Focus only on syntax conversion
- Output immediately without lengthy analysis

CRITICAL RULES:
1. SYNTACTICALLY CORRECT: Every line MUST be valid {} syntax - verify syntax before output
2. PRESERVE LOGIC: Keep identical algorithmic structure and logic
3. IDIOMATIC CODE: Use proper {} idioms, methods, and standard library
4. NO IMPROVEMENTS: Don't fix bugs, complete code, or add features
5. PRESERVE INCOMPLETENESS: If code is unfinished, keep it unfinished
6. LITERAL TRANSLATION: Same variable names, same structure, same flow
{}

FORBIDDEN CHARACTERS AND PATTERNS:
- NEVER use '→' (mathematical arrow) - use actual language syntax
- NEVER mix syntax from different languages
- NEVER use mathematical notation in place of code
- NEVER invent syntax that doesn't exist in {}
- If unsure, use the SYNTAX EXAMPLE above as reference

VALIDATION CHECKLIST BEFORE OUTPUT:
✓ Function declaration uses correct {} keyword and syntax
✓ Type annotations match {} conventions
✓ Control structures (if/for/while) use correct {} syntax
✓ Method calls use correct {} syntax (dot notation, Module.function, etc.)
✓ Braces/indentation matches {} requirements
✓ ALL characters are valid ASCII code (no '→', '←', '∀', etc.)

INPUT ({} code):
{}

OUTPUT REQUIREMENTS:
- ONLY the translated {} code
- NO markdown, NO explanations, NO code fences
- MUST be syntactically valid {} that can compile/run
- Must match the SYNTAX EXAMPLE format above
- Start output immediately"#,
        from.display_name(),
        to.display_name(),
        to.display_name(),
        to.display_name(),
        extra_rules,
        to.display_name(),
        to.display_name(),
        to.display_name(),
        to.display_name(),
        to.display_name(),
        to.display_name(),
        from.display_name(),
        code,
        to.display_name(),
        to.display_name()
    )
}
