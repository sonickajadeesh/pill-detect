use dioxus::prelude::*;
use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};

const CHATS_STORAGE_KEY: &str = "CHATS";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    pub id: u64,
    pub title: String,
    pub messages: Vec<Message>,
}

pub async fn load_chats() -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
    let script = format!(
        r#"
        const chats = localStorage.getItem("{CHATS_STORAGE_KEY}");
        return chats || "[]";
        "#
    );

    let json: String = document::eval(&script).join().await?;

    Ok(serde_json::from_str(&json)?)
}

pub async fn save_chats(chats: &[Chat]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(chats)?;

    let script = format!(
        r#"
        localStorage.setItem("{CHATS_STORAGE_KEY}", {json:?});
        return;
        "#
    );

    let _: () = document::eval(&script).join().await?;

    Ok(())
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
