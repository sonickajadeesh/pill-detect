pub fn sentence_case(value: &str) -> String {
    let mut chars = value.trim().chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}
