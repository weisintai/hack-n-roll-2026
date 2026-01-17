use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

// Error logging helper
fn log_error(context: &str, error: &str) {
    use std::io::Write;
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}: {}\n", timestamp, context, error);
    
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("code_arcade_errors.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

// Piston-specific error logging with full details
fn log_piston_error(language: &str, error_type: &str, details: &str) {
    use std::io::Write;
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!(
        "[{}] Piston Error - Language: {}, Type: {}\nDetails: {}\n---\n",
        timestamp, language, error_type, details
    );
    
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("piston_errors.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

// Log full Piston request/response for debugging
fn log_piston_full_exchange(language: &str, request_code: &str, response: &str) {
    use std::io::Write;
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!(
        "[{}] === Piston Full Exchange: {} ===\n\n--- Generated Code ---\n{}\n\n--- Response ---\n{}\n\n=== End Exchange ===\n\n",
        timestamp, language, request_code, response
    );
    
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("piston_full.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: Vec<String>,
    pub expected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub examples: Vec<String>,
    pub constraints: Vec<String>,
    pub test_cases: Vec<TestCase>,
    pub function_name: String,
}

impl Problem {
    pub fn all() -> Vec<Problem> {
        vec![
            Problem::two_sum(),
            Problem::reverse_string(),
            Problem::fizz_buzz(),
            Problem::palindrome_check(),
            Problem::fibonacci(),
        ]
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Problem::all().choose(&mut rng).unwrap().clone()
    }

    pub fn random_except(&self) -> Self {
        let mut rng = rand::thread_rng();
        let others: Vec<_> = Problem::all()
            .into_iter()
            .filter(|p| p.id != self.id)
            .collect();
        others.choose(&mut rng).unwrap().clone()
    }

    pub fn two_sum() -> Self {
        Problem {
            id: 1,
            title: "1. Two Sum".to_string(),
            function_name: "two_sum".to_string(),
            description: r#"Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order."#.to_string(),
            examples: vec![
                r#"Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: nums[0] + nums[1] == 9, so we return [0, 1]."#.to_string(),
                r#"Example 2:
Input: nums = [3,2,4], target = 6
Output: [1,2]"#.to_string(),
                r#"Example 3:
Input: nums = [3,3], target = 6
Output: [0,1]"#.to_string(),
            ],
            constraints: vec![
                "2 <= nums.length <= 10^4".to_string(),
                "-10^9 <= nums[i] <= 10^9".to_string(),
                "-10^9 <= target <= 10^9".to_string(),
                "Only one valid answer exists.".to_string(),
            ],
            test_cases: vec![
                TestCase {
                    input: vec!["[2,7,11,15]".to_string(), "9".to_string()],
                    expected: "[0,1]".to_string(),
                },
                TestCase {
                    input: vec!["[3,2,4]".to_string(), "6".to_string()],
                    expected: "[1,2]".to_string(),
                },
                TestCase {
                    input: vec!["[3,3]".to_string(), "6".to_string()],
                    expected: "[0,1]".to_string(),
                },
                TestCase {
                    input: vec!["[-1,-2,-3,-4,-5]".to_string(), "-8".to_string()],
                    expected: "[2,4]".to_string(),
                },
            ],
        }
    }

    pub fn reverse_string() -> Self {
        Problem {
            id: 2,
            title: "2. Reverse String".to_string(),
            function_name: "reverse_string".to_string(),
            description: r#"Write a function that reverses a string.

The input string is given as an array of characters s.

You must do this by modifying the input array in-place with O(1) extra memory."#.to_string(),
            examples: vec![
                r#"Example 1:
Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]"#.to_string(),
                r#"Example 2:
Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]"#.to_string(),
            ],
            constraints: vec![
                "1 <= s.length <= 10^5".to_string(),
                "s[i] is a printable ascii character.".to_string(),
            ],
            test_cases: vec![
                TestCase {
                    input: vec![r#"["h","e","l","l","o"]"#.to_string()],
                    expected: r#"["o","l","l","e","h"]"#.to_string(),
                },
                TestCase {
                    input: vec![r#"["H","a","n","n","a","h"]"#.to_string()],
                    expected: r#"["h","a","n","n","a","H"]"#.to_string(),
                },
            ],
        }
    }

    pub fn fizz_buzz() -> Self {
        Problem {
            id: 3,
            title: "3. Fizz Buzz".to_string(),
            function_name: "fizz_buzz".to_string(),
            description: r#"Given an integer n, return a string array answer where:

- answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
- answer[i] == "Fizz" if i is divisible by 3.
- answer[i] == "Buzz" if i is divisible by 5.
- answer[i] == i (as a string) if none of the above conditions are true."#.to_string(),
            examples: vec![
                r#"Example 1:
Input: n = 3
Output: ["1","2","Fizz"]"#.to_string(),
                r#"Example 2:
Input: n = 5
Output: ["1","2","Fizz","4","Buzz"]"#.to_string(),
                r#"Example 3:
Input: n = 15
Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]"#.to_string(),
            ],
            constraints: vec![
                "1 <= n <= 10^4".to_string(),
            ],
            test_cases: vec![
                TestCase {
                    input: vec!["3".to_string()],
                    expected: r#"["1","2","Fizz"]"#.to_string(),
                },
                TestCase {
                    input: vec!["5".to_string()],
                    expected: r#"["1","2","Fizz","4","Buzz"]"#.to_string(),
                },
                TestCase {
                    input: vec!["15".to_string()],
                    expected: r#"["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]"#.to_string(),
                },
            ],
        }
    }

    pub fn palindrome_check() -> Self {
        Problem {
            id: 4,
            title: "4. Valid Palindrome".to_string(),
            function_name: "is_palindrome".to_string(),
            description: r#"A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.

Given a string s, return true if it is a palindrome, or false otherwise."#.to_string(),
            examples: vec![
                r#"Example 1:
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome."#.to_string(),
                r#"Example 2:
Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome."#.to_string(),
                r#"Example 3:
Input: s = " "
Output: true
Explanation: After removing non-alphanumeric chars, s is ""."#.to_string(),
            ],
            constraints: vec![
                "1 <= s.length <= 2 * 10^5".to_string(),
                "s consists only of printable ASCII characters.".to_string(),
            ],
            test_cases: vec![
                TestCase {
                    input: vec![r#""A man, a plan, a canal: Panama""#.to_string()],
                    expected: "true".to_string(),
                },
                TestCase {
                    input: vec![r#""race a car""#.to_string()],
                    expected: "false".to_string(),
                },
                TestCase {
                    input: vec![r#"" ""#.to_string()],
                    expected: "true".to_string(),
                },
            ],
        }
    }

    pub fn fibonacci() -> Self {
        Problem {
            id: 5,
            title: "5. Fibonacci Number".to_string(),
            function_name: "fib".to_string(),
            description: r#"The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1.

That is:
F(0) = 0, F(1) = 1
F(n) = F(n - 1) + F(n - 2), for n > 1.

Given n, calculate F(n)."#.to_string(),
            examples: vec![
                r#"Example 1:
Input: n = 2
Output: 1
Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1."#.to_string(),
                r#"Example 2:
Input: n = 3
Output: 2
Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2."#.to_string(),
                r#"Example 3:
Input: n = 4
Output: 3
Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3."#.to_string(),
            ],
            constraints: vec![
                "0 <= n <= 30".to_string(),
            ],
            test_cases: vec![
                TestCase {
                    input: vec!["2".to_string()],
                    expected: "1".to_string(),
                },
                TestCase {
                    input: vec!["3".to_string()],
                    expected: "2".to_string(),
                },
                TestCase {
                    input: vec!["4".to_string()],
                    expected: "3".to_string(),
                },
                TestCase {
                    input: vec!["10".to_string()],
                    expected: "55".to_string(),
                },
            ],
        }
    }
}

use tokio::sync::mpsc;
use crate::app::{ExecutionEvent, OutputLine};
use crate::languages::Language;

#[derive(Serialize)]
struct PistonRequest {
    language: String,
    version: String,
    files: Vec<PistonFile>,
}

#[derive(Serialize)]
struct PistonFile {
    name: String,
    content: String,
}

#[derive(Deserialize)]
struct PistonResponse {
    run: PistonRunResult,
}

#[derive(Deserialize)]
struct PistonRunResult {
    stdout: String,
    stderr: String,
    code: Option<i32>,
}

/// Async test runner using Piston API
pub async fn run_tests_on_piston(
    code: String, 
    problem: Problem, 
    language: Language,
    tx: mpsc::Sender<ExecutionEvent>
) -> TestResults {
    
    // Helper to send output
    let send_log = |text: String, is_error: bool| {
        let tx = tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(ExecutionEvent::Log(OutputLine { text, is_error })).await;
        });
    };

    send_log(format!("Preparing {} environment...", language.display_name()), false);

    // Build test cases JSON
    let test_cases_json: Vec<serde_json::Value> = problem
        .test_cases
        .iter()
        .map(|tc| {
            match problem.id {
                1 => serde_json::json!({
                    "nums": tc.input[0],
                    "target": tc.input[1],
                    "expected": tc.expected
                }),
                2 => serde_json::json!({
                    "s": tc.input[0],
                    "expected": tc.expected
                }),
                3 => serde_json::json!({
                    "n": tc.input[0],
                    "expected": tc.expected
                }),
                4 => serde_json::json!({
                    "s": tc.input[0],
                    "expected": tc.expected
                }),
                5 => serde_json::json!({
                    "n": tc.input[0],
                    "expected": tc.expected
                }),
                _ => serde_json::json!({
                    "input": tc.input,
                    "expected": tc.expected
                })
            }
        })
        .collect();

    // Generate harness based on language
    let full_code = match language {
        Language::Python => generate_python_harness(&code, &test_cases_json),
        Language::JavaScript => generate_javascript_harness(&code, &test_cases_json),
        Language::TypeScript => generate_typescript_harness(&code, &test_cases_json),
        Language::Go => generate_go_harness(&code, &test_cases_json, problem.id),
        Language::Java => generate_java_harness(&code, &test_cases_json, problem.id),
        Language::Rust => {
            // Rust on Piston is limited (no full Cargo support)
            // We'll provide a basic wrapper but note limitations
            send_log("Note: Rust on Piston has limited support (no Cargo dependencies)".to_string(), false);
            generate_rust_harness(&code, &test_cases_json, problem.id)
        }
    };

    let (piston_lang, piston_ver, filename) = match language {
        Language::Python => ("python", "3.10.0", "solution.py"),
        Language::JavaScript => ("javascript", "18.15.0", "solution.js"),
        Language::TypeScript => ("typescript", "1.32.3", "solution.ts"),
        Language::Rust => ("rust", "1.68.2", "solution.rs"),
        Language::Go => ("go", "1.16.2", "solution.go"),
        Language::Java => ("java", "15.0.2", "Main.java"),
    };

    let request = PistonRequest {
        language: piston_lang.to_string(),
        version: piston_ver.to_string(),
        files: vec![PistonFile {
            name: filename.to_string(),
            content: full_code.clone(),
        }],
    };

    send_log("Sending code to Piston API (emkc.org)...".to_string(), false);

    // Log the full generated code for debugging
    log_piston_full_exchange(
        language.display_name(),
        &full_code,
        "[Request sent, awaiting response...]"
    );

    let client = reqwest::Client::new();
    let res = client.post("https://emkc.org/api/v2/piston/execute")
        .json(&request)
        .send()
        .await;

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                let status = response.status();
                let error_msg = format!("API Error: {}", status);
                
                // Try to get response body for detailed logging
                let body = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
                log_piston_error(
                    language.display_name(),
                    &format!("HTTP {}", status.as_u16()),
                    &body
                );
                
                log_error("Piston API", &error_msg);
                send_log(error_msg.clone(), true);
                return create_error_results(&problem, &error_msg);
            }

            match response.json::<PistonResponse>().await {
                Ok(piston_res) => {
                    // Log full response for debugging
                    let response_json = serde_json::json!({
                        "stdout": &piston_res.run.stdout,
                        "stderr": &piston_res.run.stderr,
                        "exit_code": &piston_res.run.code
                    });
                    log_piston_full_exchange(
                        language.display_name(),
                        "[See previous request]",
                        &serde_json::to_string_pretty(&response_json).unwrap_or_default()
                    );
                    
                    send_log("Execution completed.".to_string(), false);
                    
                    // Show stdout/stderr in the terminal window
                    for line in piston_res.run.stdout.lines() {
                        send_log(line.to_string(), false);
                    }
                    for line in piston_res.run.stderr.lines() {
                        send_log(line.to_string(), true);
                    }

                    // Parse JSON results from stdout
                    parse_results(&piston_res.run.stdout, &problem)
                }
                Err(e) => {
                    let error_msg = format!("Failed to parse Piston response: {}", e);
                    log_error("Piston Response Parse", &error_msg);
                    send_log(error_msg.clone(), true);
                    create_error_results(&problem, &format!("Parse Error: {}", e))
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Network Error: {}", e);
            log_error("Piston Network", &error_msg);
            send_log(error_msg.clone(), true);
            create_error_results(&problem, &format!("Network Error: {}", e))
        }
    }
}

fn generate_javascript_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    format!(
        r#"
// User's code
{}

// Test runner
const testCases = {};

const results = [];
for (let i = 0; i < testCases.length; i++) {{
    try {{
        const tc = testCases[i];
        let actual = null;
        let expected = null;
        
        // Dynamically handle different problem types
        if (tc.nums !== undefined && tc.target !== undefined) {{
            // Two Sum (problem 1)
            const nums = JSON.parse(tc.nums);
            const target = parseInt(tc.target);
            expected = JSON.parse(tc.expected);
            
            // Try to find the function
            if (typeof twoSum !== 'undefined') {{
                actual = twoSum(nums, target);
            }} else if (typeof two_sum !== 'undefined') {{
                actual = two_sum(nums, target);
            }}
        }} else if (tc.s !== undefined && tc.expected !== undefined) {{
            // String problems (problem 2 or 4)
            const s_input = JSON.parse(tc.s);
            expected = JSON.parse(tc.expected);
            
            if (Array.isArray(s_input)) {{
                // Reverse String (problem 2) - modifies in place OR returns result
                const sCopy = [...s_input];
                let result;
                if (typeof reverseString !== 'undefined') {{
                    result = reverseString(sCopy);
                    actual = result !== undefined ? result : sCopy;
                }} else if (typeof reverse_string !== 'undefined') {{
                    result = reverse_string(sCopy);
                    actual = result !== undefined ? result : sCopy;
                }}
            }} else {{
                // Palindrome check (problem 4)
                if (typeof isPalindrome !== 'undefined') {{
                    actual = isPalindrome(s_input);
                }} else if (typeof is_palindrome !== 'undefined') {{
                    actual = is_palindrome(s_input);
                }}
            }}
        }} else if (tc.n !== undefined) {{
            // Number problems (problem 3 or 5)
            const n = parseInt(tc.n);
            expected = JSON.parse(tc.expected);
            
            if (Array.isArray(expected)) {{
                // Fizz Buzz (problem 3)
                if (typeof fizzBuzz !== 'undefined') {{
                    actual = fizzBuzz(n);
                }} else if (typeof fizz_buzz !== 'undefined') {{
                    actual = fizz_buzz(n);
                }}
            }} else {{
                // Fibonacci (problem 5)
                if (typeof fibonacci !== 'undefined') {{
                    actual = fibonacci(n);
                }} else if (typeof fib !== 'undefined') {{
                    actual = fib(n);
                }}
            }}
        }}
        
        if (actual === null) {{
            results.push({{ passed: false, actual: "Error: No function found" }});
            continue;
        }}
        
        // Compare results
        let passed = false;
        if (Array.isArray(actual) && Array.isArray(expected)) {{
            // For array results, sort before comparison if they're numeric arrays
            if (actual.length > 0 && typeof actual[0] === 'number') {{
                passed = JSON.stringify(actual.sort((a,b)=>a-b)) === JSON.stringify(expected.sort((a,b)=>a-b));
            }} else {{
                passed = JSON.stringify(actual) === JSON.stringify(expected);
            }}
        }} else {{
            passed = JSON.stringify(actual) === JSON.stringify(expected);
        }}
        
        results.push({{ passed, actual: JSON.stringify(actual) }});
    }} catch (e) {{
        results.push({{ passed: false, actual: `Error: ${{e.message}}` }});
    }}
}}

console.log(JSON.stringify(results));
"#,
        user_code,
        serde_json::to_string(test_cases).unwrap_or_default()
    )
}

fn generate_typescript_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    format!(
        r#"
// User's code
{}

// Test runner
const testCases: any[] = {};

const results: any[] = [];
for (let i = 0; i < testCases.length; i++) {{
    try {{
        const tc = testCases[i];
        let actual: any = null;
        let expected: any = null;
        
        // Dynamically handle different problem types
        if (tc.nums !== undefined && tc.target !== undefined) {{
            // Two Sum (problem 1)
            const nums: number[] = JSON.parse(tc.nums);
            const target: number = parseInt(tc.target);
            expected = JSON.parse(tc.expected);
            
            // Try to find the function
            if (typeof twoSum !== 'undefined') {{
                actual = twoSum(nums, target);
            }} else if (typeof two_sum !== 'undefined') {{
                actual = two_sum(nums, target);
            }}
        }} else if (tc.s !== undefined && tc.expected !== undefined) {{
            // String problems (problem 2 or 4)
            const s_input: any = JSON.parse(tc.s);
            expected = JSON.parse(tc.expected);
            
            if (Array.isArray(s_input)) {{
                // Reverse String (problem 2) - modifies in place OR returns result
                const sCopy = [...s_input];
                let result: any;
                if (typeof reverseString !== 'undefined') {{
                    result = reverseString(sCopy);
                    actual = result !== undefined ? result : sCopy;
                }} else if (typeof reverse_string !== 'undefined') {{
                    result = reverse_string(sCopy);
                    actual = result !== undefined ? result : sCopy;
                }}
            }} else {{
                // Palindrome check (problem 4)
                if (typeof isPalindrome !== 'undefined') {{
                    actual = isPalindrome(s_input);
                }} else if (typeof is_palindrome !== 'undefined') {{
                    actual = is_palindrome(s_input);
                }}
            }}
        }} else if (tc.n !== undefined) {{
            // Number problems (problem 3 or 5)
            const n: number = parseInt(tc.n);
            expected = JSON.parse(tc.expected);
            
            if (Array.isArray(expected)) {{
                // Fizz Buzz (problem 3)
                if (typeof fizzBuzz !== 'undefined') {{
                    actual = fizzBuzz(n);
                }} else if (typeof fizz_buzz !== 'undefined') {{
                    actual = fizz_buzz(n);
                }}
            }} else {{
                // Fibonacci (problem 5)
                if (typeof fibonacci !== 'undefined') {{
                    actual = fibonacci(n);
                }} else if (typeof fib !== 'undefined') {{
                    actual = fib(n);
                }}
            }}
        }}
        
        if (actual === null) {{
            results.push({{ passed: false, actual: "Error: No function found" }});
            continue;
        }}
        
        // Compare results
        let passed = false;
        if (Array.isArray(actual) && Array.isArray(expected)) {{
            // For array results, sort before comparison if they're numeric arrays
            if (actual.length > 0 && typeof actual[0] === 'number') {{
                passed = JSON.stringify(actual.sort((a:any,b:any)=>a-b)) === JSON.stringify(expected.sort((a:any,b:any)=>a-b));
            }} else {{
                passed = JSON.stringify(actual) === JSON.stringify(expected);
            }}
        }} else {{
            passed = JSON.stringify(actual) === JSON.stringify(expected);
        }}
        
        results.push({{ passed, actual: JSON.stringify(actual) }});
    }} catch (e: any) {{
        results.push({{ passed: false, actual: `Error: ${{e.message}}` }});
    }}
}}

console.log(JSON.stringify(results));
"#,
        user_code,
        serde_json::to_string(test_cases).unwrap_or_default()
    )
}

fn generate_go_harness(user_code: &str, test_cases: &[serde_json::Value], _problem_id: usize) -> String {
    // Serialize test cases
    let test_cases_str = serde_json::to_string(test_cases).unwrap_or_default();
    
    format!(
        r#"
package main

import (
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

{}

type TestResult struct {{
	Passed bool   `json:"passed"`
	Actual string `json:"actual"`
}}

func parseIntArray(s string) []int {{
	s = strings.TrimSpace(s)
	s = strings.Trim(s, "[]")
	if s == "" {{
		return []int{{}}
	}}
	parts := strings.Split(s, ",")
	result := make([]int, len(parts))
	for i, p := range parts {{
		val, _ := strconv.Atoi(strings.TrimSpace(p))
		result[i] = val
	}}
	return result
}}

func parseStringArray(s string) []string {{
	var result []string
	json.Unmarshal([]byte(s), &result)
	return result
}}

func main() {{
	testCasesJSON := `{}`
	var testCases []map[string]interface{{}}
	json.Unmarshal([]byte(testCasesJSON), &testCases)
	
	results := []TestResult{{}}
	
	for _, tc := range testCases {{
		var passed bool
		var actualStr string
		
		// Handle different problem types based on fields in test case
		if nums, hasNums := tc["nums"]; hasNums {{
			// Problem 1: Two Sum
			numsArr := parseIntArray(nums.(string))
			target, _ := strconv.Atoi(tc["target"].(string))
			expected := parseIntArray(tc["expected"].(string))
			
			actual := twoSum(numsArr, target)
			
			sortedActual := make([]int, len(actual))
			copy(sortedActual, actual)
			sort.Ints(sortedActual)
			
			sortedExpected := make([]int, len(expected))
			copy(sortedExpected, expected)
			sort.Ints(sortedExpected)
			
			passed = len(sortedActual) == len(sortedExpected)
			if passed {{
				for i := range sortedActual {{
					if sortedActual[i] != sortedExpected[i] {{
						passed = false
						break
					}}
				}}
			}}
			
			actualJSON, _ := json.Marshal(actual)
			actualStr = string(actualJSON)
			
		}} else if s, hasS := tc["s"]; hasS {{
			sStr := s.(string)
			expected := tc["expected"].(string)
			
			// Check if it's an array or string
			if strings.HasPrefix(sStr, "[") {{
				// Problem 2: Reverse String
				var sArr []string
				json.Unmarshal([]byte(sStr), &sArr)
				
				reverseString(&sArr)
				
				var expectedArr []string
				json.Unmarshal([]byte(expected), &expectedArr)
				
				passed = len(sArr) == len(expectedArr)
				if passed {{
					for i := range sArr {{
						if sArr[i] != expectedArr[i] {{
							passed = false
							break
						}}
					}}
				}}
				
				actualJSON, _ := json.Marshal(sArr)
				actualStr = string(actualJSON)
			}} else {{
				// Problem 4: Palindrome
				sStrUnquoted := strings.Trim(sStr, "\"")
				actual := isPalindrome(sStrUnquoted)
				expectedBool := expected == "true"
				passed = actual == expectedBool
				actualStr = fmt.Sprintf("%v", actual)
			}}
			
		}} else if n, hasN := tc["n"]; hasN {{
			nInt, _ := strconv.Atoi(n.(string))
			expected := tc["expected"].(string)
			
			if strings.HasPrefix(expected, "[") {{
				// Problem 3: Fizz Buzz
				actual := fizzBuzz(nInt)
				var expectedArr []string
				json.Unmarshal([]byte(expected), &expectedArr)
				
				passed = len(actual) == len(expectedArr)
				if passed {{
					for i := range actual {{
						if actual[i] != expectedArr[i] {{
							passed = false
							break
						}}
					}}
				}}
				
				actualJSON, _ := json.Marshal(actual)
				actualStr = string(actualJSON)
			}} else {{
				// Problem 5: Fibonacci
				actual := fib(nInt)
				expectedInt, _ := strconv.Atoi(expected)
				passed = actual == expectedInt
				actualStr = strconv.Itoa(actual)
			}}
		}}
		
		results = append(results, TestResult{{Passed: passed, Actual: actualStr}})
	}}
	
	output, _ := json.Marshal(results)
	fmt.Println(string(output))
}}
"#,
        user_code,
        test_cases_str
    )
}

fn generate_java_harness(user_code: &str, test_cases: &[serde_json::Value], _problem_id: usize) -> String {
    // Process user code: strip imports and ensure proper structure
    let user_code_clean = user_code.lines()
        .filter(|line| !line.trim().starts_with("import ") && !line.trim().starts_with("package "))
        .collect::<Vec<_>>()
        .join("\n");
    
    // Indent each line for proper class nesting
    let user_code_safe = if user_code_clean.trim().is_empty() {
        "        // No user code provided\n".to_string()
    } else {
        user_code_clean.lines()
            .map(|line| if line.trim().is_empty() { String::new() } else { format!("        {}", line) })
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    // Generate test case initialization code based on problem structure
    let mut test_init = String::new();
    for (_idx, tc) in test_cases.iter().enumerate() {
        if tc.get("nums").is_some() {
            let nums = tc["nums"].as_str().unwrap_or("[]");
            let target = tc["target"].as_str().unwrap_or("0");
            let expected = tc["expected"].as_str().unwrap_or("[]");
            test_init.push_str(&format!(
                "        tests.add(new Object[]{{1, \"{}\", \"{}\", \"{}\"}});\n",
                nums.replace('"', "\\\""), target.replace('"', "\\\""), expected.replace('"', "\\\"")
            ));
        } else if tc.get("s").is_some() {
            let s = tc["s"].as_str().unwrap_or("\"\"");
            let expected = tc["expected"].as_str().unwrap_or("\"\"");
            // Determine if it's problem 2 or 4
            let prob_type = if s.starts_with('[') { 2 } else { 4 };
            test_init.push_str(&format!(
                "        tests.add(new Object[]{{{}, \"{}\", \"{}\"}});\n",
                prob_type, s.replace('"', "\\\""), expected.replace('"', "\\\"")
            ));
        } else if tc.get("n").is_some() {
            let n = tc["n"].as_str().unwrap_or("0");
            let expected = tc["expected"].as_str().unwrap_or("0");
            // Determine if it's problem 3 or 5
            let prob_type = if expected.starts_with('[') { 3 } else { 5 };
            test_init.push_str(&format!(
                "        tests.add(new Object[]{{{}, \"{}\", \"{}\"}});\n",
                prob_type, n, expected.replace('"', "\\\"")
            ));
        }
    }
    
    format!(
        r#"
import java.util.*;

public class Main {{
    static class Solution {{
{}
    }}

    public static void main(String[] args) {{
        List<Object[]> tests = new ArrayList<>();
{}
        
        StringBuilder results = new StringBuilder("[");
        Solution solution = new Solution();
        
        for (int idx = 0; idx < tests.size(); idx++) {{
            try {{
                Object[] test = tests.get(idx);
                int problemType = (Integer) test[0];
                boolean passed = false;
                String actualStr = "";
                
                if (problemType == 1) {{
                    // Two Sum
                    int[] nums = parseIntArray((String) test[1]);
                    int target = Integer.parseInt((String) test[2]);
                    int[] expected = parseIntArray((String) test[3]);
                    
                    int[] actual = solution.twoSum(nums, target);
                    Arrays.sort(actual);
                    Arrays.sort(expected);
                    passed = Arrays.equals(actual, expected);
                    actualStr = Arrays.toString(actual);
                    
                }} else if (problemType == 2) {{
                    // Reverse String
                    String[] sArr = parseStringArray((String) test[1]);
                    solution.reverseString(sArr);
                    String[] expectedArr = parseStringArray((String) test[2]);
                    passed = Arrays.equals(sArr, expectedArr);
                    actualStr = Arrays.toString(sArr);
                    
                }} else if (problemType == 3) {{
                    // Fizz Buzz
                    int n = Integer.parseInt((String) test[1]);
                    String[] actual = solution.fizzBuzz(n);
                    String[] expectedArr = parseStringArray((String) test[2]);
                    passed = Arrays.equals(actual, expectedArr);
                    actualStr = Arrays.toString(actual);
                    
                }} else if (problemType == 4) {{
                    // Palindrome
                    String s = ((String) test[1]).replaceAll("^\\\"|\\\"$", "");
                    boolean actual = solution.isPalindrome(s);
                    boolean expectedBool = Boolean.parseBoolean((String) test[2]);
                    passed = actual == expectedBool;
                    actualStr = String.valueOf(actual);
                    
                }} else if (problemType == 5) {{
                    // Fibonacci
                    int n = Integer.parseInt((String) test[1]);
                    int actual = solution.fib(n);
                    int expectedInt = Integer.parseInt((String) test[2]);
                    passed = actual == expectedInt;
                    actualStr = String.valueOf(actual);
                }}
                
                if (idx > 0) results.append(",");
                results.append("{{\\\"passed\\\":");
                results.append(passed);
                results.append(",\\\"actual\\\":\\\"");
                results.append(actualStr.replace("\\\"", "\\\\\\\\\\\""));
                results.append("\\\"}}");
            }} catch (Exception e) {{
                if (idx > 0) results.append(",");
                results.append("{{\\\"passed\\\":false,\\\"actual\\\":\\\"Error: ");
                results.append(e.getMessage());
                results.append("\\\"}}");
            }}
        }}
        results.append("]");
        
        System.out.println(results.toString());
    }}
    
    static int[] parseIntArray(String s) {{
        s = s.trim().replace("[", "").replace("]", "");
        if (s.isEmpty()) return new int[0];
        String[] parts = s.split(",");
        int[] result = new int[parts.length];
        for (int i = 0; i < parts.length; i++) {{
            result[i] = Integer.parseInt(parts[i].trim());
        }}
        return result;
    }}
    
    static String[] parseStringArray(String s) {{
        s = s.trim();
        if (s.startsWith("[")) {{
            s = s.substring(1, s.length() - 1);
        }}
        if (s.isEmpty()) return new String[0];
        String[] parts = s.split(",");
        List<String> result = new ArrayList<>();
        for (String part : parts) {{
            result.add(part.trim().replaceAll("^\\\"|\\\"$", ""));
        }}
        return result.toArray(new String[0]);
    }}
}}
"#,
        user_code_safe,
        test_init
    )
}

fn generate_rust_harness(user_code: &str, test_cases: &[serde_json::Value], _problem_id: usize) -> String {
    // Generate test case execution code based on problem structure
    let mut test_execution = String::new();
    
    for (i, tc) in test_cases.iter().enumerate() {
        if tc.get("nums").is_some() {
            // Problem 1: Two Sum
            let nums = tc["nums"].as_str().unwrap_or("[]");
            let target = tc["target"].as_str().unwrap_or("0");
            let expected = tc["expected"].as_str().unwrap_or("[]");
            test_execution.push_str(&format!(
                r#"
    // Test case {}
    {{
        let nums = vec!{};
        let target = {};
        let expected = vec!{};
        let actual = two_sum(nums, target);
        
        let mut sorted_actual = actual.clone();
        sorted_actual.sort();
        let mut sorted_expected = expected.clone();
        sorted_expected.sort();
        
        let passed = sorted_actual == sorted_expected;
        if idx > 0 {{ print!(","); }}
        print!("{{{{\"passed\":{{}},", passed);
        print!("\"actual\":\"{{:?}}\"}}}}", actual);
        idx += 1;
    }}
"#,
                i + 1, nums, target, expected
            ));
        } else if tc.get("s").is_some() {
            let s = tc["s"].as_str().unwrap_or("\"\"");
            let expected = tc["expected"].as_str().unwrap_or("\"\"");
            
            if s.starts_with('[') {
                // Problem 2: Reverse String
                test_execution.push_str(&format!(
                    r#"
    // Test case {}
    {{
        let mut s: Vec<char> = {};
        let expected: Vec<char> = {};
        reverse_string(&mut s);
        
        let passed = s == expected;
        if idx > 0 {{ print!(","); }}
        print!("{{{{\"passed\":{{}},", passed);
        print!("\"actual\":\"{{:?}}\"}}}}", s);
        idx += 1;
    }}
"#,
                    i + 1, s, expected
                ));
            } else {
                // Problem 4: Palindrome
                let s_unquoted = s.trim_matches('"');
                let expected_bool = expected == "true";
                test_execution.push_str(&format!(
                    r#"
    // Test case {}
    {{
        let s = "{}".to_string();
        let expected = {};
        let actual = is_palindrome(s);
        
        let passed = actual == expected;
        if idx > 0 {{ print!(","); }}
        print!("{{{{\"passed\":{{}},", passed);
        print!("\"actual\":\"{{}}\"}}}}", actual);
        idx += 1;
    }}
"#,
                    i + 1, s_unquoted.replace("\\", "\\\\").replace("\"", "\\\""), expected_bool
                ));
            }
        } else if tc.get("n").is_some() {
            let n = tc["n"].as_str().unwrap_or("0");
            let expected = tc["expected"].as_str().unwrap_or("0");
            
            if expected.starts_with('[') {
                // Problem 3: Fizz Buzz
                test_execution.push_str(&format!(
                    r#"
    // Test case {}
    {{
        let n = {};
        let expected: Vec<String> = {};
        let actual = fizz_buzz(n);
        
        let passed = actual == expected;
        if idx > 0 {{ print!(","); }}
        print!("{{{{\"passed\":{{}},", passed);
        print!("\"actual\":\"{{:?}}\"}}}}", actual);
        idx += 1;
    }}
"#,
                    i + 1, n, expected
                ));
            } else {
                // Problem 5: Fibonacci
                test_execution.push_str(&format!(
                    r#"
    // Test case {}
    {{
        let n = {};
        let expected = {};
        let actual = fib(n);
        
        let passed = actual == expected;
        if idx > 0 {{ print!(","); }}
        print!("{{{{\"passed\":{{}},", passed);
        print!("\"actual\":\"{{}}\"}}}}", actual);
        idx += 1;
    }}
"#,
                    i + 1, n, expected
                ));
            }
        }
    }
    
    format!(
        r#"
// User's code
{}

fn main() {{
    let mut idx = 0;
    print!("[");
{}
    println!("]")
}}
"#,
        user_code,
        test_execution
    )
}

fn generate_python_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    format!(
        r#"
import json
import sys

# User's code
{}

# Test runner
test_cases = {}

results = []
for i, tc in enumerate(test_cases):
    try:
        actual = None
        expected = None
        
        # Dynamically handle different problem types
        if "nums" in tc and "target" in tc:
            # Two Sum (problem 1)
            nums = eval(tc["nums"]) if isinstance(tc["nums"], str) else tc["nums"]
            target = int(tc["target"])
            expected = eval(tc["expected"]) if isinstance(tc["expected"], str) else tc["expected"]
            
            # Try finding solution function
            if 'two_sum' in dir():
                actual = two_sum(nums, target)
            elif 'twoSum' in dir():
                actual = twoSum(nums, target)
        
        elif "s" in tc:
            # String problems (problem 2 or 4)
            s_input = eval(tc["s"]) if isinstance(tc["s"], str) else tc["s"]
            expected = eval(tc["expected"]) if isinstance(tc["expected"], str) else tc["expected"]
            
            if isinstance(s_input, list):
                # Reverse String (problem 2) - modifies in place OR returns result
                s_copy = s_input.copy()
                if 'reverse_string' in dir():
                    result = reverse_string(s_copy)
                    actual = result if result is not None else s_copy
                elif 'reverseString' in dir():
                    result = reverseString(s_copy)
                    actual = result if result is not None else s_copy
            else:
                # Palindrome check (problem 4)
                if 'is_palindrome' in dir():
                    actual = is_palindrome(s_input)
                elif 'isPalindrome' in dir():
                    actual = isPalindrome(s_input)
        
        elif "n" in tc:
            # Number problems (problem 3 or 5)
            n = int(tc["n"])
            expected = eval(tc["expected"]) if isinstance(tc["expected"], str) else tc["expected"]
            
            if isinstance(expected, list):
                # Fizz Buzz (problem 3)
                if 'fizz_buzz' in dir():
                    actual = fizz_buzz(n)
                elif 'fizzBuzz' in dir():
                    actual = fizzBuzz(n)
            else:
                # Fibonacci (problem 5)
                if 'fibonacci' in dir():
                    actual = fibonacci(n)
                elif 'fib' in dir():
                    actual = fib(n)
        
        if actual is None:
            results.append({{"passed": False, "actual": "Error: No function found"}})
            continue
        
        # Compare results
        if isinstance(actual, list) and isinstance(expected, list):
            # For array results, sort before comparison if they're numeric
            if len(actual) > 0 and isinstance(actual[0], (int, float)):
                passed = sorted(actual) == sorted(expected)
            else:
                passed = actual == expected
        else:
            passed = actual == expected
        
        results.append({{"passed": passed, "actual": str(actual)}})
    except Exception as e:
        results.append({{"passed": False, "actual": f"Error: {{e}}"}})

print(json.dumps(results))
"#,
        user_code,
        serde_json::to_string(test_cases).unwrap_or_default()
    )
}

fn parse_results(stdout: &str, problem: &Problem) -> TestResults {
    // Find the last line that looks like a JSON array
    let json_line = stdout.lines().rev().find(|l| l.trim().starts_with('['));
    
    if let Some(line) = json_line {
        if let Ok(json_results) = serde_json::from_str::<Vec<serde_json::Value>>(line) {
             let details: Vec<TestResult> = problem
                    .test_cases
                    .iter()
                    .enumerate()
                    .map(|(i, tc)| {
                        let result = json_results.get(i);
                        let passed = result.and_then(|r| r.get("passed")).and_then(|p| p.as_bool()).unwrap_or(false);
                        let actual = result.and_then(|r| r.get("actual")).and_then(|a| a.as_str()).unwrap_or("Error").to_string();

                        TestResult {
                            case_number: i + 1,
                            passed,
                            input: tc.input.join(", "),
                            expected: tc.expected.clone(),
                            actual,
                        }
                    })
                    .collect();

            let passed_count = details.iter().filter(|r| r.passed).count();

            return TestResults {
                total: problem.test_cases.len(),
                passed: passed_count,
                failed: problem.test_cases.len() - passed_count,
                details,
            };
        }
    }
    
    create_error_results(problem, "Failed to parse test results from output")
}

fn create_error_results(problem: &Problem, error: &str) -> TestResults {
    TestResults {
        total: problem.test_cases.len(),
        passed: 0,
        failed: problem.test_cases.len(),
        details: problem
            .test_cases
            .iter()
            .enumerate()
            .map(|(i, tc)| TestResult {
                case_number: i + 1,
                passed: false,
                input: tc.input.join(", "),
                expected: tc.expected.clone(),
                actual: error.to_string(),
            })
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub details: Vec<TestResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestResult {
    pub case_number: usize,
    pub passed: bool,
    pub input: String,
    pub expected: String,
    pub actual: String,
}
