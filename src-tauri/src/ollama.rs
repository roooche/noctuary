use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    input: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

pub async fn generate(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let url = format!("{}/api/generate", base_url.trim_end_matches('/'));
    let body = GenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let res = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama generate request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Ollama returned {}: {}", status, text));
    }

    let parsed: GenerateResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    Ok(parsed.response)
}

pub async fn embed(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    text: &str,
) -> Result<Vec<f32>, String> {
    let url = format!("{}/api/embed", base_url.trim_end_matches('/'));
    let body = EmbedRequest {
        model: model.to_string(),
        input: text.to_string(),
    };

    let res = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama embed request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Ollama embed returned {}: {}", status, text));
    }

    let parsed: EmbedResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse embed response: {}", e))?;

    parsed
        .embeddings
        .into_iter()
        .next()
        .ok_or_else(|| "Ollama returned empty embeddings".to_string())
}
