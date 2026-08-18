use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const HISTORY_STORAGE_KEY: &str = "SEARCH_HISTORY";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHistory {
    pub product: String,
    pub generic: String,
}

pub async fn load_search_history() -> Result<Vec<SearchHistory>, Box<dyn std::error::Error>> {
    let script = format!(
        r#"
        const history = localStorage.getItem("{HISTORY_STORAGE_KEY}");
        return history || "[]";
        "#
    );

    let json: String = document::eval(&script).join().await?;

    Ok(serde_json::from_str(&json)?)
}

pub async fn save_search_history(
    history: &[SearchHistory],
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(history)?;

    let script = format!(
        r#"
        localStorage.setItem("{HISTORY_STORAGE_KEY}", {json:?});
        return;
        "#
    );

    let _: () = document::eval(&script).join().await?;

    Ok(())
}

pub async fn clear_search_history() -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(
        r#"
        localStorage.removeItem("{HISTORY_STORAGE_KEY}");
        return;
        "#
    );

    let _: () = document::eval(&script).join().await?;

    Ok(())
}
