use base64::{Engine, engine::general_purpose::STANDARD};
use dioxus::prelude::*;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

const API_KEY: &str = "API_KEY"; // this is the field name for API Key in database
const MODEL_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash-lite:generateContent";

pub async fn get_api_key() -> Result<String, Box<dyn Error>> {
    let key: String = document::eval(&format!(
        r#"
        const key = localStorage.getItem("{API_KEY}");

        if (key) return key;

        const entered = window.prompt("Enter your Gemini API key", "");

        if (entered?.trim()) {{
            localStorage.setItem("{API_KEY}", entered.trim());
            return entered.trim();
        }}

        return "";
        "#
    ))
    .join()
    .await?;

    if key.trim().is_empty() {
        return Err("No Gemini API key provided".into());
    }

    Ok(key)
}

pub async fn clear_api_key() -> Result<String, Box<dyn Error>> {
    let key: String = document::eval(&format!(
        r#"
        if (!window.confirm("Clear the stored Gemini API key?")) return "";

        localStorage.removeItem("{API_KEY}");

        const entered = window.prompt("Enter your new Gemini API key", "");

        if (entered?.trim()) {{
            localStorage.setItem("{API_KEY}", entered.trim());
            return entered.trim();
        }}

        return "";
        "#
    ))
    .join()
    .await?;

    if key.trim().is_empty() {
        return Err("API key was not changed.".into());
    }

    Ok(key)
}

async fn generate_content(parts: Vec<Value>) -> Result<String, Box<dyn Error>> {
    let api_key = get_api_key().await?;

    let response = Client::new()
        .post(MODEL_URL)
        .header("x-goog-api-key", &api_key)
        .json(&json!({
            "contents": [{
                "parts": parts
            }]
        }))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("Gemini API error ({}): {}", status, body).into());
    }

    let response: Value = serde_json::from_str(&body)?;

    response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|text| {
            text.trim()
                .strip_prefix("```json")
                .unwrap_or(text)
                .strip_suffix("```")
                .unwrap_or(text)
                .trim()
                .to_owned()
        })
        .ok_or_else(|| "Gemini response did not contain generated text".into())
}

pub async fn prompt_ai(prompt: &str) -> Result<String, Box<dyn Error>> {
    generate_content(vec![json!({ "text": prompt })]).await
}

pub async fn prompt_image(
    image_bytes: &[u8],
    mime_type: &str,
    prompt: &str,
) -> Result<String, Box<dyn Error>> {
    generate_content(vec![
        json!({ "text": prompt }),
        json!({
            "inline_data": {
                "mime_type": mime_type,
                "data": STANDARD.encode(image_bytes)
            }
        }),
    ])
    .await
}
