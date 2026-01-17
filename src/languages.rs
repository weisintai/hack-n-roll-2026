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

/// Mock code converter - in production, this would call an LLM API
pub fn convert_code(code: &str, from: Language, to: Language) -> String {
    if from == to {
        return code.to_string();
    }

    // Mock conversion - just wrap the code with language-specific syntax
    match to {
        Language::JavaScript => mock_convert_to_js(code),
        Language::TypeScript => mock_convert_to_ts(code),
        Language::Python => mock_convert_to_python(code),
        Language::Rust => mock_convert_to_rust(code),
        Language::Go => mock_convert_to_go(code),
        Language::Java => mock_convert_to_java(code),
    }
}

fn mock_convert_to_js(_code: &str) -> String {
    format!(
        r#"// JavaScript conversion
function twoSum(nums, target) {{
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {{
        const complement = target - nums[i];
        if (map.has(complement)) {{
            return [map.get(complement), i];
        }}
        map.set(nums[i], i);
    }}
    return [];
}}
"#
    )
}

fn mock_convert_to_ts(_code: &str) -> String {
    format!(
        "// TypeScript conversion\nfunction twoSum(nums: number[], target: number): number[] {{\n    const map = new Map<number, number>();\n    for (let i = 0; i < nums.length; i++) {{\n        const complement = target - nums[i];\n        if (map.has(complement)) {{\n            return [map.get(complement)!, i];\n        }}\n        map.set(nums[i], i);\n    }}\n    return [];\n}}\n"
    )
}

fn mock_convert_to_python(_code: &str) -> String {
    format!(
        r#"# Python conversion
def two_sum(nums, target):
    num_map = {{}}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_map:
            return [num_map[complement], i]
        num_map[num] = i
    return []
"#
    )
}

fn mock_convert_to_rust(_code: &str) -> String {
    format!(
        r#"// Rust conversion
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {{
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {{
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {{
            return vec![j as i32, i as i32];
        }}
        map.insert(num, i);
    }}
    vec![]
}}
"#
    )
}

fn mock_convert_to_go(_code: &str) -> String {
    format!(
        r#"// Go conversion
func twoSum(nums []int, target int) []int {{
    m := make(map[int]int)
    for i, num := range nums {{
        complement := target - num
        if j, ok := m[complement]; ok {{
            return []int{{j, i}}
        }}
        m[num] = i
    }}
    return []int{{}}
}}
"#
    )
}

fn mock_convert_to_java(_code: &str) -> String {
    format!(
        r#"// Java conversion
    public int[] twoSum(int[] nums, int target) {{
        Map<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {{
            int complement = target - nums[i];
            if (map.containsKey(complement)) {{
                return new int[] {{ map.get(complement), i }};
            }}
            map.put(nums[i], i);
        }}
        return new int[] {{}};
    }}
"#
    )
}
