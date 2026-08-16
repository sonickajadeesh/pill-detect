use chrono::{Datelike, Local, NaiveDate};

pub fn sentence_case(value: &str) -> String {
    let mut chars = value.trim().chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

pub fn is_valid_date(date: &str) -> bool {
    let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else {
        return false;
    };

    date <= Local::now().date_naive()
}

pub fn calculate_age(date_of_birth: &str) -> Option<u32> {
    let dob = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").ok()?;
    let today = Local::now().date_naive();

    let mut age = today.year() - dob.year();

    if (today.month(), today.day()) < (dob.month(), dob.day()) {
        age -= 1;
    }

    Some(age as u32)
}
