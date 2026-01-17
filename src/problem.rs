use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: Vec<String>,
    pub expected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub description: String,
    pub examples: Vec<String>,
    pub constraints: Vec<String>,
    pub test_cases: Vec<TestCase>,
}

impl Problem {
    pub fn two_sum() -> Self {
        Problem {
            title: "1. Two Sum".to_string(),
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
