use dioxus::prelude::*;

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
