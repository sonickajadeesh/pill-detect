use serde::Deserialize;

use crate::modules::api::prompt_ai;

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

pub async fn identify_medicine(
    term: &str,
) -> Result<MedicineIdentification, Box<dyn std::error::Error>> {
    let prompt = format!(
        r#"Use web search and identify medicine: "{}". Determine product name and generic name. If not confident, return false. Return JSON only: {{"found":true | false,"product":"","generic":""}}"#,
        term
    );

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
    let prompt = format!(
        r#"Determine uses, typical dosage, common side effects, warnings and prescription requirement in India of this medicine: {}. Do not guess. Use web search and reliable sources. Return JSON only: {{"uses":"","dosage":"","side_effects":"","warnings":"","prescription": true | false}}"#,
        generic
    );

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

pub fn guidance_prompt(conversation: &str) -> String {
    format!(
        r#"Converse naturally, using prior context for follow-ups. Be concise, clear, and medically responsible; don’t diagnose or overstate uncertain information.
        Conversation: {}
        Respond to the latest user message."#,
        conversation
    )
}
