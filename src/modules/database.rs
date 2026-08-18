use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const PATIENTS_KEY: &str = "PATIENTS";

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
    pub chats: Vec<Chat>,
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
    match LocalStorage::get(PATIENTS_KEY) {
        Ok(patients) => Ok(patients),
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Ok(Vec::new()),
        Err(err) => Err(format!("Failed to load patients: {:?}", err)),
    }
}

pub fn add_patient(mut patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

    // New patients always start with empty history.
    patient.search_history = Vec::new();
    patient.chats = Vec::new();

    patients.push(patient);

    LocalStorage::set(PATIENTS_KEY, patients)
        .map_err(|err| format!("Failed to save patient: {:?}", err))
}

pub fn update_patient(updated_patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

    let Some(patient) = patients
        .iter_mut()
        .find(|patient| patient.id == updated_patient.id)
    else {
        return Err("Patient not found.".to_string());
    };

    *patient = updated_patient;

    LocalStorage::set(PATIENTS_KEY, patients)
        .map_err(|err| format!("Failed to update patient: {:?}", err))
}

pub fn delete_patient(patient_id: &str) -> Result<(), String> {
    let mut patients = get_patients()?;

    let original_len = patients.len();

    patients.retain(|patient| patient.id != patient_id);

    if patients.len() == original_len {
        return Err("Patient not found.".to_string());
    }

    LocalStorage::set(PATIENTS_KEY, patients)
        .map_err(|err| format!("Failed to delete patient: {:?}", err))
}

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

    LocalStorage::set(PATIENTS_KEY, patients)
        .map_err(|err| format!("Failed to save search history: {:?}", err))
}

pub fn clear_search_history(patient_id: &str) -> Result<(), String> {
    let mut patients = get_patients()?;

    let patient = patients
        .iter_mut()
        .find(|patient| patient.id == patient_id)
        .ok_or_else(|| "Patient not found.".to_string())?;

    patient.search_history.clear();

    LocalStorage::set(PATIENTS_KEY, patients)
        .map_err(|err| format!("Failed to clear search history: {:?}", err))
}
