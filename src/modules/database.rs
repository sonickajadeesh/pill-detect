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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchHistory {
    pub product: String,
    pub generic: String,
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
pub struct Chat {
    pub id: u64,
    pub title: String,
    pub messages: Vec<Message>,
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

// Patient Ops
pub fn add_patient(mut patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

    // New patients always start with empty history.
    patient.search_history = Vec::new();
    patient.chat_history = Vec::new();

    patients.push(patient);

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to save patient: {:?}", err))
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

// Search Ops
pub fn get_search_history(patient_id: &str) -> Result<Vec<SearchHistory>, String> {
    let patients = get_patients()?;

    let patient = patients
        .iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    Ok(patient.search_history.clone())
}

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

// Chat Ops
pub fn get_chats(patient_id: &str) -> Result<Vec<Chat>, String> {
    let patients = get_patients()?;

    let patient = patients
        .iter()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    Ok(patient.chat_history.clone())
}

pub fn add_chat(patient_id: &str, chat: Chat) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.chat_history.push(chat);

    LocalStorage::set(DB, patients).map_err(|err| format!("Failed to save chat: {:?}", err))
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
