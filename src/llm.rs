use crate::languages::Language;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::time::Duration;
use std::io::Write;

fn debug_log(msg: &str) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/weisintai/development/hackathon/2026/hackn-n-roll/debug.log")
    {
        let _ = writeln!(file, "[LLM] {}", msg);
    }
}

#[derive(Debug)]
pub enum ConversionError {
    ApiError(String),
    EnvError(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::ApiError(e) => write!(f, "API error: {}", e),
            ConversionError::EnvError(e) => write!(f, "Environment error: {}", e),
        }
    }
}

impl std::error::Error for ConversionError {}

// Gemini API request/response types
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "thinkingConfig")]
    thinking_config: ThinkingConfig,
}

#[derive(Serialize)]
struct ThinkingConfig {
    #[serde(rename = "thinkingLevel")]
    thinking_level: &'static str,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

const GEMINI_MODEL: &str = "gemini-3-flash-preview";

pub struct LlmConverter {
    api_key: String,
    client: reqwest::Client,
}

impl LlmConverter {
    pub fn new() -> Result<Self, ConversionError> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|_| ConversionError::EnvError("GEMINI_API_KEY not set".to_string()))?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(25))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| ConversionError::ApiError(e.to_string()))?;

        Ok(Self { api_key, client })
    }

    fn build_prompt(code: &str, from: Language, to: Language) -> String {
        let mut extra_rules = String::new();
        let forbidden = forbidden_tokens(to);
        if !forbidden.is_empty() {
            extra_rules.push_str("\nLANGUAGE LOCK:\n- Output must be valid ");
            extra_rules.push_str(to.display_name());
            extra_rules.push_str(" syntax only.\n- Do NOT use syntax tokens from other languages (outside of string literals/comments). Forbidden tokens: ");
            extra_rules.push_str(&forbidden.join(", "));
            extra_rules.push_str("\n- If you would output mixed-language code, output an empty string instead.\n");
        }
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

    /// Starts an async conversion task and sends the final result through a channel.
    /// Returns a receiver that the caller can poll for the final result.
    pub fn start_streaming_conversion(
        &self,
        code: String,
        from: Language,
        to: Language,
    ) -> mpsc::Receiver<StreamChunk> {
        let (tx, rx) = mpsc::channel();
        let api_key = self.api_key.clone();
        let client = self.client.clone();
        let prompt = Self::build_prompt(&code, from, to);

        // Spawn the async task on the tokio runtime
        tokio::spawn(async move {
            match request_conversion(client, api_key, prompt).await {
                Ok(text) => {
                    if !text.is_empty() {
                        let _ = tx.send(StreamChunk::Text(text));
                    }
                    let _ = tx.send(StreamChunk::Done);
                }
                Err(e) => {
                    let _ = tx.send(StreamChunk::Error(e.to_string()));
                }
            }
        });

        rx
    }

    /// Synchronous conversion (blocks until complete) - for submit functionality
    pub fn convert_code_sync(
        &self,
        code: &str,
        from: Language,
        to: Language,
    ) -> Result<String, ConversionError> {
        if from == to {
            return Ok(code.to_string());
        }

        let api_key = self.api_key.clone();
        let client = self.client.clone();
        let prompt = Self::build_prompt(code, from, to);

        // Use tokio's block_in_place to run async code in sync context
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let url = format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                    GEMINI_MODEL
                );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
            generation_config: Some(GenerationConfig {
                thinking_config: ThinkingConfig {
                    thinking_level: "minimal",
                },
            }),
        };

                let response = client
                    .post(&url)
                    .header("x-goog-api-key", api_key)
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| ConversionError::ApiError(e.to_string()))?;

                let gemini_response: GeminiResponse = response
                    .json()
                    .await
                    .map_err(|e| ConversionError::ApiError(e.to_string()))?;

                if let Some(error) = gemini_response.error {
                    return Err(ConversionError::ApiError(error.message));
                }

                let text = gemini_response
                    .candidates
                    .and_then(|c| c.into_iter().next())
                    .and_then(|c| c.content)
                    .and_then(|c| c.parts.into_iter().next())
                    .map(|p| p.text)
                    .unwrap_or_default();

                Ok(text)
            })
        });

        result
    }
}

fn forbidden_tokens(to: Language) -> Vec<&'static str> {
    let mut tokens = Vec::new();
    if to != Language::Python {
        tokens.extend(["def ", "elif ", "self", "None", "pass"]);
    }
    if to != Language::Rust {
        tokens.extend(["fn ", "println!", "let mut", "Vec<", "::"]);
    }
    if to != Language::Go {
        tokens.extend(["func ", ":=", "package ", "fmt."]);
    }
    if to != Language::Java {
        tokens.extend(["public class", "System.out", "String", "new "]);
    }
    tokens
}

async fn request_conversion(
    client: reqwest::Client,
    api_key: String,
    prompt: String,
) -> Result<String, ConversionError> {
    debug_log("request_conversion started");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        GEMINI_MODEL
    );

                let request = GeminiRequest {
                    contents: vec![Content {
                        parts: vec![Part { text: prompt }],
                    }],
                    generation_config: Some(GenerationConfig {
                        thinking_config: ThinkingConfig {
                            thinking_level: "minimal",
                        },
                    }),
                };

    debug_log("Sending Gemini request...");
    let response = client
        .post(&url)
        .header("x-goog-api-key", api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| ConversionError::ApiError(e.to_string()))?;

    debug_log(&format!("Gemini API response status: {}", response.status()));

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        debug_log(&format!("HTTP error body: {}", error_text));
        return Err(ConversionError::ApiError(format!(
            "HTTP error: {}",
            error_text
        )));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .map_err(|e| ConversionError::ApiError(e.to_string()))?;

    if let Some(error) = gemini_response.error {
        return Err(ConversionError::ApiError(error.message));
    }

    let text = gemini_response
        .candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.content)
        .and_then(|c| c.parts.into_iter().next())
        .map(|p| p.text)
        .unwrap_or_default();

    debug_log(&format!("Extracted text: {:?}", &text[..text.len().min(50)]));
    Ok(text)
}

#[derive(Debug, Clone)]
pub enum StreamChunk {
    Text(String),
    Done,
    Error(String),
}
