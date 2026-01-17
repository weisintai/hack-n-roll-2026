use crate::languages::Language;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
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

// Streaming response types
#[derive(Deserialize)]
struct StreamChunkResponse {
    candidates: Option<Vec<Candidate>>,
}

pub struct LlmConverter {
    api_key: String,
    client: reqwest::Client,
}

impl LlmConverter {
    pub fn new() -> Result<Self, ConversionError> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|_| ConversionError::EnvError("GEMINI_API_KEY not set".to_string()))?;

        let client = reqwest::Client::new();

        Ok(Self { api_key, client })
    }

    fn build_prompt(code: &str, from: Language, to: Language) -> String {
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

    /// Starts an async conversion task that sends chunks through a channel.
    /// Returns a receiver that the caller can poll for chunks.
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
            match stream_conversion(client, api_key, prompt, tx.clone()).await {
                Ok(_) => {
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
                    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
                    api_key
                );

                let request = GeminiRequest {
                    contents: vec![Content {
                        parts: vec![Part { text: prompt }],
                    }],
                };

                let response = client
                    .post(&url)
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

async fn stream_conversion(
    client: reqwest::Client,
    api_key: String,
    prompt: String,
    tx: mpsc::Sender<StreamChunk>,
) -> Result<(), ConversionError> {
    debug_log("stream_conversion started");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent?key={}&alt=sse",
        api_key
    );

    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
    };

    let response = client
        .post(&url)
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

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    debug_log("Starting to read stream...");

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                let chunk_str = String::from_utf8_lossy(&chunk);
                buffer.push_str(&chunk_str);

                // Process each complete "data: " line in the buffer
                // SSE format: "data: {json}\n" or "data: {json}\r\n"
                while let Some(data_start) = buffer.find("data: ") {
                    // Find the end of this data line
                    let rest = &buffer[data_start + 6..];
                    if let Some(line_end) = rest.find('\n') {
                        let json_str = &rest[..line_end].trim();
                        debug_log(&format!("Processing JSON: {}...", &json_str[..json_str.len().min(80)]));

                        match serde_json::from_str::<StreamChunkResponse>(json_str) {
                            Ok(response) => {
                                if let Some(text) = response
                                    .candidates
                                    .and_then(|c| c.into_iter().next())
                                    .and_then(|c| c.content)
                                    .and_then(|c| c.parts.into_iter().next())
                                    .map(|p| p.text)
                                {
                                    debug_log(&format!("Extracted text: {:?}", &text[..text.len().min(50)]));
                                    if !text.is_empty() {
                                        if tx.send(StreamChunk::Text(text)).is_err() {
                                            debug_log("Receiver dropped!");
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                debug_log(&format!("JSON parse error: {}", e));
                            }
                        }

                        // Remove processed part from buffer
                        buffer = buffer[data_start + 6 + line_end + 1..].to_string();
                    } else {
                        // Incomplete line, wait for more data
                        break;
                    }
                }
            }
            Err(e) => {
                debug_log(&format!("Stream error: {}", e));
                return Err(ConversionError::ApiError(e.to_string()));
            }
        }
    }
    debug_log("Stream ended");

    Ok(())
}

#[derive(Debug, Clone)]
pub enum StreamChunk {
    Text(String),
    Done,
    Error(String),
}
