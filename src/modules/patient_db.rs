use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

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
}

const PATIENTS_KEY: &str = "patients";

pub fn get_patients() -> Result<Vec<Patient>, String> {
    match LocalStorage::get(PATIENTS_KEY) {
        Ok(patients) => Ok(patients),
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Ok(Vec::new()),
        Err(err) => Err(format!("Failed to load patients: {:?}", err)),
    }
}
pub fn add_patient(patient: Patient) -> Result<(), String> {
    let mut patients = get_patients()?;

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
