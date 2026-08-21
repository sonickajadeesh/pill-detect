use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const DB: &str = "PATIENTS";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub date_of_birth: String,
    pub blood_group: String,
    pub height: u32,
    pub weight: f32,
    pub allergies: String,
    pub medical_conditions: String,

    #[serde(default)]
    pub search_history: Vec<SearchHistory>,

    #[serde(default)]
    pub chat_history: Vec<Chat>,

    #[serde(default)]
    pub prescriptions: Vec<Prescription>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchHistory {
    pub product: String,
    pub generic: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    pub id: u64,
    pub title: String,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prescription {
    pub id: String,
    pub reason: String,
    pub prescription_text: String,
    pub created_at: String,
    pub expiry_date: String,
}

// Patient CRUD
pub fn add_patient(mut patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

    // New patients always start with empty history.
    patient.search_history = Vec::new();
    patient.chat_history = Vec::new();
    patient.prescriptions = Vec::new();

    patients.push(patient);

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to save patient: {:?}", err))
}

pub fn get_patients() -> Result<Vec<Patient>, String> {
    match LocalStorage::get(DB) {
        Ok(patients) => Ok(patients),
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Ok(Vec::new()),
        Err(err) => Err(format!("Failed to load patients: {:?}", err)),
    }
}

pub fn get_patient_id(patient_id: &str) -> Result<Patient, String> {
    let patients = get_patients()?;

    patients
        .into_iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())
}

pub fn update_patient(mut updated_patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

    let Some(patient) = patients
        .iter_mut()
        .find(|patient| patient.id == updated_patient.id)
    else {
        return Err("Patient not found.".to_string());
    };

    updated_patient.search_history = patient.search_history.clone();
    updated_patient.chat_history = patient.chat_history.clone();
    updated_patient.prescriptions = patient.prescriptions.clone();

    *patient = updated_patient;

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to update patient: {:?}", err))
}

pub fn delete_patient(patient_id: &str) -> Result<(), String> {
    let mut patients = get_patients()?;

    let original_len = patients.len();

    patients.retain(|patient| patient.id != patient_id);

    if patients.len() == original_len {
        return Err("Patient not found.".to_string());
    }

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to delete patient: {:?}", err))
}

// Medicine Information CRUD
pub fn add_search_history(patient_id: &str, search: SearchHistory) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.search_history.push(search);

    LocalStorage::set(DB, patients)
        .map_err(|err| format!("Failed to save search history: {:?}", err))
}

pub fn get_search_history(patient_id: &str) -> Result<Vec<SearchHistory>, String> {
    let patients = get_patients()?;

    let patient = patients
        .iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    Ok(patient.search_history.clone())
}

pub fn clear_search_history(patient_id: &str) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.search_history.clear();

    LocalStorage::set(DB, patients)
        .map_err(|err| format!("Failed to clear search history: {:?}", err))
}

// Symptom Guidance CRUD
pub fn add_chat(patient_id: &str, chat: Chat) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.chat_history.push(chat);

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to save chat: {:?}", err))
}

pub fn get_chats(patient_id: &str) -> Result<Vec<Chat>, String> {
    let patients = get_patients()?;

    let patient = patients
        .iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    Ok(patient.chat_history.clone())
}

pub fn update_chat(patient_id: &str, updated_chat: Chat) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    let chat = patient
        .chat_history
        .iter_mut()
        .find(|chat| chat.id == updated_chat.id)
        .ok_or_else(|| "Chat not found.".to_string())?;

    *chat = updated_chat;

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to update chat: {:?}", err))
}

pub fn delete_chat(patient_id: &str, chat_id: u64) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    let original_len = patient.chat_history.len();

    patient.chat_history.retain(|chat| chat.id != chat_id);

    if patient.chat_history.len() == original_len {
        return Err("Chat not found.".to_string());
    }

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to delete chat: {:?}", err))
}

// Prescription CRUD
pub fn add_prescription(patient_id: &str, prescription: Prescription) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.prescriptions.push(prescription);

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to save prescription: {:?}", err))
}

pub fn get_prescriptions(patient_id: &str) -> Result<Vec<Prescription>, String> {
    let patients = get_patients()?;

    let patient = patients
        .iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    Ok(patient.prescriptions.clone())
}

pub fn update_prescription(
    patient_id: &str,
    updated_prescription: Prescription,
) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    let prescription = patient
        .prescriptions
        .iter_mut()
        .find(|prescription| prescription.id == updated_prescription.id)
        .ok_or_else(|| "Prescription not found.".to_string())?;

    *prescription = updated_prescription;

    LocalStorage::set(DB, patients)
        .map_err(|err| format!("Failed to update prescription: {:?}", err))
}

pub fn delete_prescription(patient_id: &str, prescription_id: &str) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    let original_len = patient.prescriptions.len();

    patient
        .prescriptions
        .retain(|prescription| prescription.id != prescription_id);

    if patient.prescriptions.len() == original_len {
        return Err("Prescription not found.".to_string());
    }

    LocalStorage::set(DB, patients)
        .map_err(|err| format!("Failed to delete prescription: {:?}", err))
}
