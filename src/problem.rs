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
    output: String,
    code: Option<i32>,
    signal: Option<String>,
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
        Language::Go => generate_go_harness(&code, &test_cases_json),
        Language::Java => generate_java_harness(&code, &test_cases_json),
        Language::Rust => {
            // Rust on Piston is limited (no full Cargo support)
            // We'll provide a basic wrapper but note limitations
            send_log("Note: Rust on Piston has limited support (no Cargo dependencies)".to_string(), false);
            generate_rust_harness(&code, &test_cases_json)
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
        const nums = JSON.parse(tc.nums);
        const target = parseInt(tc.target);
        const expected = JSON.parse(tc.expected);
        
        let actual = null;
        // Try to find the solution function dynamically
        if (typeof twoSum !== 'undefined') {{
            actual = twoSum(nums, target);
        }} else {{
            // Try to find any function that looks like it could be the solution
            const globalFuncs = Object.keys(this || {{}}).filter(k => typeof (this || {{}})[k] === 'function');
            if (globalFuncs.length > 0) {{
                actual = (this || {{}})[globalFuncs[0]](nums, target);
            }}
        }}
        
        if (actual === null) {{
            results.push({{ passed: false, actual: "Error: No function found" }});
            continue;
        }}
            
        const passed = JSON.stringify(actual.sort()) === JSON.stringify(expected.sort());
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
        const nums: number[] = JSON.parse(tc.nums);
        const target: number = parseInt(tc.target);
        const expected: number[] = JSON.parse(tc.expected);
        
        let actual: number[] | null = null;
        // Try to find the solution function (TypeScript uses camelCase conventionally)
        if (typeof twoSum !== 'undefined') {{
            actual = twoSum(nums, target);
        }}
        
        if (actual === null) {{
            results.push({{ passed: false, actual: "Error: No twoSum function found" }});
            continue;
        }}
            
        const passed = JSON.stringify(actual.sort()) === JSON.stringify(expected.sort());
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

fn generate_go_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    // Extract test case data
    let test_data: Vec<(String, String, String)> = test_cases.iter().map(|tc| {
        let nums = tc["nums"].as_str().unwrap_or("[]");
        let target = tc["target"].as_str().unwrap_or("0");
        let expected = tc["expected"].as_str().unwrap_or("[]");
        (nums.to_string(), target.to_string(), expected.to_string())
    }).collect();
    
    let mut test_cases_code = String::new();
    for (i, (nums, target, expected)) in test_data.iter().enumerate() {
        test_cases_code.push_str(&format!(
            "\tTestCase{{Nums: \"{}\", Target: \"{}\", Expected: \"{}\"}},\n",
            nums.replace('"', "\\\""), target.replace('"', "\\\""), expected.replace('"', "\\\"")
        ));
    }
    
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

type TestCase struct {{
	Nums     string
	Target   string
	Expected string
}}

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

func main() {{
	testCases := []TestCase{{
{}	}}
	
	results := []TestResult{{}}
	
	for _, tc := range testCases {{
		nums := parseIntArray(tc.Nums)
		expected := parseIntArray(tc.Expected)
		target, _ := strconv.Atoi(tc.Target)
		
		actual := twoSum(nums, target)
		
		// Sort for comparison
		sortedActual := make([]int, len(actual))
		copy(sortedActual, actual)
		sort.Ints(sortedActual)
		
		sortedExpected := make([]int, len(expected))
		copy(sortedExpected, expected)
		sort.Ints(sortedExpected)
		
		passed := len(sortedActual) == len(sortedExpected)
		if passed {{
			for i := range sortedActual {{
				if sortedActual[i] != sortedExpected[i] {{
					passed = false
					break
				}}
			}}
		}}
		
		actualJSON, _ := json.Marshal(actual)
		results = append(results, TestResult{{Passed: passed, Actual: string(actualJSON)}})
	}}
	
	output, _ := json.Marshal(results)
	fmt.Println(string(output))
}}
"#,
        user_code,
        test_cases_code
    )
}

fn generate_java_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    // Extract test case data
    let test_data: Vec<(String, String, String)> = test_cases.iter().map(|tc| {
        let nums = tc["nums"].as_str().unwrap_or("[]");
        let target = tc["target"].as_str().unwrap_or("0");
        let expected = tc["expected"].as_str().unwrap_or("[]");
        (nums.to_string(), target.to_string(), expected.to_string())
    }).collect();
    
    let mut test_cases_code = String::new();
    for (i, (nums, target, expected)) in test_data.iter().enumerate() {
        test_cases_code.push_str(&format!(
            "        testCases[{}] = new TestCase(\"{}\", \"{}\", \"{}\");\n",
            i, nums.replace('"', "\\\""), target.replace('"', "\\\""), expected.replace('"', "\\\"")
        ));
    }
    
    // Process user code: strip imports and ensure proper structure
    let user_code_clean = user_code.lines()
        .filter(|line| !line.trim().starts_with("import ") && !line.trim().starts_with("package "))
        .collect::<Vec<_>>()
        .join("\n");
    
    // Ensure user code has proper structure or provide placeholder, and indent for class nesting
    let user_code_safe = if user_code_clean.trim().is_empty() {
        "        public int[] twoSum(int[] nums, int target) {\n            return new int[0];\n        }".to_string()
    } else {
        // Indent each line for proper class nesting (8 spaces as it's inside static class Solution which is inside Main)
        user_code_clean.lines()
            .map(|line| if line.trim().is_empty() { String::new() } else { format!("        {}", line) })
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    format!(
        r#"
import java.util.*;

public class Main {{
    static class Solution {{
{}
    }}

    static class TestCase {{
        String nums;
        String target;
        String expected;
        
        TestCase(String n, String t, String e) {{
            this.nums = n;
            this.target = t;
            this.expected = e;
        }}
    }}

    public static void main(String[] args) {{
        TestCase[] testCases = new TestCase[{}];
{}
        
        StringBuilder results = new StringBuilder("[");
        Solution solution = new Solution();
        
        for (int idx = 0; idx < testCases.length; idx++) {{
            try {{
                TestCase tc = testCases[idx];
                
                // Parse arrays manually
                int[] nums = parseIntArray(tc.nums);
                int target = Integer.parseInt(tc.target);
                int[] expected = parseIntArray(tc.expected);
                
                int[] actual = solution.twoSum(nums, target);
                
                Arrays.sort(actual);
                Arrays.sort(expected);
                boolean passed = Arrays.equals(actual, expected);
                
                if (idx > 0) results.append(",");
                results.append("{{\"passed\":");
                results.append(passed);
                results.append(",\"actual\":\"");
                results.append(Arrays.toString(actual));
                results.append("\"}}");
            }} catch (Exception e) {{
                if (idx > 0) results.append(",");
                results.append("{{\"passed\":false,\"actual\":\"Error: ");
                results.append(e.getMessage());
                results.append("\"}}");
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
}}
"#,
        user_code_safe,
        test_data.len(),
        test_cases_code
    )
}

fn generate_rust_harness(user_code: &str, test_cases: &[serde_json::Value]) -> String {
    // Piston's Rust doesn't support Cargo dependencies, so manually build test cases
    let test_data: Vec<(String, String, String)> = test_cases.iter().map(|tc| {
        let nums = tc["nums"].as_str().unwrap_or("[]");
        let target = tc["target"].as_str().unwrap_or("0");
        let expected = tc["expected"].as_str().unwrap_or("[]");
        (nums.to_string(), target.to_string(), expected.to_string())
    }).collect();
    
    // Generate test case execution code
    let mut test_execution = String::new();
    for (i, (nums, target, expected)) in test_data.iter().enumerate() {
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
        # Piston might receive string inputs differently, handle eval carefully
        nums = eval(tc["nums"]) if isinstance(tc["nums"], str) else tc["nums"]
        target = int(tc["target"])
        expected = eval(tc["expected"]) if isinstance(tc["expected"], str) else tc["expected"]
        
        actual = None
        # Try finding solution function
        if 'two_sum' in dir():
            actual = two_sum(nums, target)
        elif 'twoSum' in dir():
             actual = twoSum(nums, target)
        else:
            # Fallback search
             for name in dir():
                if name.startswith('_'): continue
                obj = eval(name)
                if callable(obj):
                     try:
                        actual = obj(nums, target)
                        break
                     except:
                        continue
        
        if actual is None:
            results.append({{"passed": False, "actual": "Error: No function found"}})
            continue
            
        passed = sorted(actual) == sorted(expected)
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
