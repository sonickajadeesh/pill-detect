use serde::Deserialize;
use serde_json::Value;

use crate::modules::api::get_api_key;

#[derive(Debug, Clone, Deserialize)]
pub struct MedicineIdentification {
    pub found: bool,
    pub product: String,
    pub generic: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MedicineInformation {
    pub uses: String,
    pub dosage: String,
    pub side_effects: String,
    pub warnings: String,
    pub prescription: bool,
}

async fn prompt_ai(search_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
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

pub async fn identify_medicine(
    term: &str,
) -> Result<MedicineIdentification, Box<dyn std::error::Error>> {
    // let prompt = format!(
    //     r#"Use web search and identify medicine: "{}". Determine product name and generic name. If not confident, return false. Return JSON only: {{"found":true | false,"product":"","generic":""}}"#,
    //     term
    // );

    // let response = prompt_ai(&prompt).await?;

    let response = r#"{"found": true,
      "product": "Dolo 650",
      "generic": "Paracetamol"
      }"#;

    let medicine: MedicineIdentification = serde_json::from_str(&response)?;

    Ok(medicine)
}

pub async fn research_medicine(
    generic: &str,
) -> Result<MedicineInformation, Box<dyn std::error::Error>> {
    // let prompt = format!(
    //     r#"Determine uses, typical dosage, common side effects, warnings and prescription requirement in India of this medicine: {}. Do not guess. Use web search and reliable sources. Return JSON only: {{"uses":"","dosage":"","side_effects":"","warnings":"","prescription": true | false}}"#,
    //     generic
    // );

    // let response = prompt_ai(&prompt).await?;

    let response = r#"{
        "uses": "Relief of mild to moderate pain and reduction of fever.",
        "dosage": "Adults: typically 500–1000 mg every 4–6 hours as needed; do not exceed the recommended daily maximum.",
        "side_effects": "Usually well tolerated; nausea, stomach discomfort, or allergic reactions may occur.",
        "warnings": "Do not exceed the recommended dose. Excessive paracetamol can cause serious liver damage.",
        "prescription": false
    }"#;

    let information: MedicineInformation = serde_json::from_str(&response)?;

    Ok(information)
}
