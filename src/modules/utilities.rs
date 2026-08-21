use chrono::{DateTime, Datelike, Local, NaiveDate};
use pulldown_cmark::{Parser, html};

pub fn sentence_case(value: &str) -> String {
    let mut chars = value.trim().chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

pub fn is_date_valid(date: &str) -> bool {
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

pub fn validate(
    first_name: &str,
    last_name: &str,
    sex: &str,
    date_of_birth: &str,
    blood_group: &str,
    height: &str,
    weight: &str,
) -> Option<String> {
    if first_name.trim().is_empty() {
        return Some("Please enter the patient's first name.".to_string());
    }

    if !first_name.chars().all(char::is_alphabetic) {
        return Some("First name can only contain letters.".to_string());
    }

    if last_name.trim().is_empty() {
        return Some("Please enter the patient's last name.".to_string());
    }

    if !last_name.chars().all(char::is_alphabetic) {
        return Some("Last name can only contain letters.".to_string());
    }

    if sex.is_empty() {
        return Some("Please select the patient's sex.".to_string());
    }

    if date_of_birth.is_empty() {
        return Some("Please enter the patient's date of birth.".to_string());
    }

    if !is_date_valid(date_of_birth) {
        return Some("Date of birth cannot be in the future.".to_string());
    }

    if blood_group.is_empty() {
        return Some("Please select the patient's blood group.".to_string());
    }

    if height.trim().is_empty() {
        return Some("Please enter the patient's height.".to_string());
    }

    if height.parse::<f32>().unwrap_or(0.0) <= 0.0 {
        return Some("Height must be greater than zero.".to_string());
    }

    if weight.trim().is_empty() {
        return Some("Please enter the patient's weight.".to_string());
    }

    if weight.parse::<f32>().unwrap_or(0.0) <= 0.0 {
        return Some("Weight must be greater than zero.".to_string());
    }

    None
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn format_date(date: &str) -> String {
    if let Ok(date_time) = DateTime::parse_from_rfc3339(date) {
        return date_time.format("%d-%b-%Y").to_string();
    }

    if let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        return date.format("%d-%b-%Y").to_string();
    }

    date.to_string()
}
