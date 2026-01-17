use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::time::Duration;

const DEFAULT_MODEL: &str = "gemini-3-flash-preview";

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Option<Vec<Part>>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: Option<String>,
}

pub async fn translate_code(prompt: &str) -> Result<String> {
    let api_key = env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY is not set (check your .env or environment)")?;
    let model = env::var("GEMINI_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let payload = json!({
        "contents": [
            {
                "parts": [
                    { "text": prompt }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.0,
            "maxOutputTokens": 4096,
            "thinkingConfig": {
                "thinkingLevel": "minimal"
            }
        }
    });

    let client = Client::builder()
        .timeout(Duration::from_secs(45))
        .build()
        .context("failed to build HTTP client")?;

    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&payload)
        .send()
        .await
        .context("failed to send Gemini request")?
        .error_for_status()
        .context("Gemini request returned an error status")?;

    let body: GenerateContentResponse = response
        .json()
        .await
        .context("failed to parse Gemini response")?;

    let text = body
        .candidates
        .unwrap_or_default()
        .into_iter()
        .filter_map(|candidate| candidate.content)
        .filter_map(|content| content.parts)
        .flatten()
        .filter_map(|part| part.text)
        .collect::<Vec<_>>()
        .join("");

    if text.trim().is_empty() {
        anyhow::bail!("Gemini response was empty");
    }

    Ok(text)
}
