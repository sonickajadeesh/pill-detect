#[derive(Clone, Debug, PartialEq)]
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
