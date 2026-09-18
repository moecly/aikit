use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
}

fn client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .build()
        .context("failed to build HTTP client")
}

pub async fn chat(cfg: &Config, system: &str, user: &str) -> Result<String> {
    let body = ChatRequest {
        model: &cfg.model,
        messages: vec![
            Message {
                role: "system",
                content: system,
            },
            Message {
                role: "user",
                content: user,
            },
        ],
        temperature: 0.2,
    };
    let resp: ChatResponse = client()?
        .post(format!("{}/chat/completions", cfg.base_url))
        .bearer_auth(&cfg.api_key)
        .json(&body)
        .send()
        .await
        .context("model request failed; check network or AIKIT_BASE_URL")?
        .error_for_status()
        .context("model returned an error; check key, model name, or base_url")?
        .json()
        .await
        .context("failed to parse model response")?;
    resp.choices
        .into_iter()
        .next()
        .map(|c| c.message.content.trim().to_string())
        .context("model returned empty content")
}

pub async fn list_models(cfg: &Config) -> Result<Vec<String>> {
    let resp: ModelsResponse = client()?
        .get(format!("{}/models", cfg.base_url))
        .bearer_auth(&cfg.api_key)
        .send()
        .await
        .context("model list request failed; check network or AIKIT_BASE_URL")?
        .error_for_status()
        .context("failed to fetch model list; check key or base_url")?
        .json()
        .await
        .context("failed to parse model list")?;
    let mut ids: Vec<String> = resp.data.into_iter().map(|m| m.id).collect();
    ids.sort();
    Ok(ids)
}
