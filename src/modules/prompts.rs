use serde::{Deserialize, Serialize};

use crate::modules::api::{prompt_ai, prompt_image};

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

#[derive(Debug, Clone, Deserialize)]
pub struct PrescriptionAnalysis {
    pub medications: Vec<PrescriptionMedication>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrescriptionMedication {
    pub name: String,
    pub strength: String,
    pub dosage: String,
    pub duration: String,
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteraction {
    pub r#type: String,
    pub drugs: Vec<String>,
    pub severity: String,
    pub interaction: String,
    pub effects: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteractionResponse {
    pub interactions: Vec<DrugInteraction>,
}

pub async fn identify_medicine(
    term: &str,
) -> Result<MedicineIdentification, Box<dyn std::error::Error>> {
    let prompt = format!(
        r#"
Use web search and identify medicine: "{}".
Determine product name and generic name.
If not confident, return false. Return JSON only: {{"found":true | false,"product":"","generic":""}}
        "#,
        term
    );
    let response = prompt_ai(&prompt).await?;
    let medicine: MedicineIdentification = serde_json::from_str(&response)?;

    Ok(medicine)
}

pub async fn identify_medicine_image(
    image_bytes: &[u8],
    mime_type: &str,
) -> Result<MedicineIdentification, Box<dyn std::error::Error>> {
    let prompt = r#"
Identify the medicine shown in this image which may contain a tablet strip, medicine box, medicine bottle, or pill container.
Read the medicine label, brand name, generic name, strength, and other printed data. If you cannot confidently identify, return found as false.
Determine product name and generic name.

Return JSON only:
{"found":true|false,"product":"","generic":""}
    "#;

    let response = crate::modules::api::prompt_image(image_bytes, mime_type, prompt).await?;

    let identification: MedicineIdentification = serde_json::from_str(&response)
        .map_err(|err| format!("Failed to parse medicine identification response: {err}"))?;

    Ok(identification)
}

pub async fn research_medicine(
    generic: &str,
) -> Result<MedicineInformation, Box<dyn std::error::Error>> {
    let prompt = format!(
        r#"
Determine uses, typical dosage, common side effects, warnings and prescription requirement in India of this medicine: {}.
Do not guess. Use web search and reliable sources.
Return JSON only: {{"uses":"","dosage":"","side_effects":"","warnings":"","prescription": true | false}}
    "#,
        generic
    );

    let response = prompt_ai(&prompt).await?;
    let information: MedicineInformation = serde_json::from_str(&response)?;

    Ok(information)
}

pub async fn guidance(conversation: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!(
        r#"
Converse naturally, using prior context for follow-ups.
Be concise, clear, and medically responsible.
Do not diagnose or overstate uncertain information.
Do not invent medical information.

Conversation: {conversation}

Respond to the latest user message.
    "#
    );

    let response = prompt_ai(&prompt).await?;

    Ok(response)
}

pub async fn analyze_prescription(
    image_bytes: &[u8],
    mime_type: &str,
) -> Result<PrescriptionAnalysis, Box<dyn std::error::Error>> {
    let prompt = r#"
Analyze this prescription image and extract every medication prescribed.
Do not invent information. If a field cannot be read, return an empty string.
Keep dosage instructions exactly as written, including multiple dosing times.
Do not include total quantity in dosage.
Return JSON only:
{
    "medications": [
        {
            "name": "",
            "strength": "",
            "dosage": "",
            "duration": "",
            "instructions": ""
        }
    ]
}
    "#;

    let response = prompt_image(image_bytes, mime_type, prompt).await?;
    let analysis: PrescriptionAnalysis = serde_json::from_str(&response)?;

    Ok(analysis)
}

pub async fn check_drug_interactions(
    medicines: Vec<String>,
    allergies: Vec<String>,
    medical_conditions: Vec<String>,
) -> Result<DrugInteractionResponse, Box<dyn std::error::Error>> {
    let medicine_list = medicines
        .iter()
        .map(|medicine| format!("- {medicine}"))
        .collect::<Vec<_>>()
        .join("\n");

    let allergy_list = if allergies.is_empty() {
        "- None recorded".to_string()
    } else {
        allergies
            .iter()
            .map(|allergy| format!("- {allergy}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let condition_list = if medical_conditions.is_empty() {
        "- None recorded".to_string()
    } else {
        medical_conditions
            .iter()
            .map(|condition| format!("- {condition}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let prompt = format!(
        r#"
Check these medicines for clinically relevant safety concerns.

Medicines:
{medicine_list}

Allergies:
{allergy_list}

Medical conditions:
{condition_list}

Return ONLY valid JSON:

{{
    "interactions": [
        {{
            "type": "drug-drug | allergy | condition",
            "drugs": ["Drug A", "Drug B", ...],
            "severity": "High | Moderate | Low",
            "interaction": "...",
            "effects": "...",
            "recommendation": "..."
        }}
    ]
}}

Check all medicines against each other, the recorded allergies, and medical conditions.
Include every clinically relevant concern. Do not invent interactions.
If there are no concerns, return an empty "interactions" array.
Keep the response concise and patient-friendly.
"#,
        medicine_list = medicine_list,
        allergy_list = allergy_list,
        condition_list = condition_list,
    );

    let response = prompt_ai(&prompt).await?;

    let response = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let result: DrugInteractionResponse = serde_json::from_str(response)?;

    Ok(result)
}
