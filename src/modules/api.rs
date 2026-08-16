use dioxus::prelude::*;
use serde_json::Value;

const API_KEY: &str = "API_KEY";

pub async fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let key: String = document::eval(&format!(
        r#"
        const key = localStorage.getItem("{API_KEY}");

        if (key) {{
            return key;
        }}

        const entered = window.prompt(
            "Enter your Gemini API key",
            ""
        );

        if (entered && entered.trim() !== "") {{
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

pub async fn prompt_ai(search_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = get_api_key().await?;

    let r = reqwest::Client::new()
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash-lite:generateContent?key={}",
            api_key
        ))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": search_prompt
                }]
            }]
        }))
        .send()
        .await?;

    let status = r.status();
    let body = r.text().await?;

    if !status.is_success() {
        return Err(format!("Gemini API error ({}): {}", status, body).into());
    }

    let response: Value = serde_json::from_str(&body)?;

    let text = response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Gemini response did not contain generated text")?;

    let text = text
        .trim()
        .strip_prefix("```json")
        .unwrap_or(text)
        .strip_suffix("```")
        .unwrap_or(text)
        .trim();

    Ok(text.to_owned())
}
