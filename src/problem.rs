use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

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

/// Mock test runner - in production, this would execute the code
pub fn run_tests(code: &str, problem: &Problem) -> TestResults {
    // Mock: just check if code is not empty
    let passed = if code.trim().is_empty() {
        0
    } else {
        // Simulate some passing tests
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=problem.test_cases.len())
    };

    TestResults {
        total: problem.test_cases.len(),
        passed,
        failed: problem.test_cases.len() - passed,
        details: problem
            .test_cases
            .iter()
            .enumerate()
            .map(|(i, tc)| {
                let success = i < passed;
                TestResult {
                    case_number: i + 1,
                    passed: success,
                    input: tc.input.join(", "),
                    expected: tc.expected.clone(),
                    actual: if success {
                        tc.expected.clone()
                    } else {
                        "[]".to_string()
                    },
                }
            })
            .collect(),
    }
}

#[derive(Debug, Clone)]
pub struct TestResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub details: Vec<TestResult>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub case_number: usize,
    pub passed: bool,
    pub input: String,
    pub expected: String,
    pub actual: String,
}
