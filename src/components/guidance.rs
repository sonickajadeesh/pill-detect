use dioxus::prelude::*;

use crate::modules::{api::prompt_ai, prompts::guidance_prompt, utilities::markdown_to_html};

#[derive(Clone, PartialEq)]
enum MessageRole {
    User,
    Assistant,
}

#[derive(Clone, PartialEq)]
struct Message {
    role: MessageRole,
    content: String,
}

#[component]
fn MarkdownMessage(content: String) -> Element {
    let html = markdown_to_html(&content);

    rsx! {
        div { class: "markdown-content", dangerous_inner_html: "{html}" }
    }
}

#[component]
pub fn Guidance() -> Element {
    let mut input = use_signal(String::new);
    let mut messages = use_signal(Vec::<Message>::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let mut send_message = move || {
        let prompt = input().trim().to_string();

        if prompt.is_empty() || loading() {
            return;
        }

        input.set(String::new());
        error.set(None);
        loading.set(true);

        messages.write().push(Message {
            role: MessageRole::User,
            content: prompt.clone(),
        });

        let history = messages();

        spawn(async move {
            let conversation = history
                .iter()
                .map(|message| {
                    let role = match message.role {
                        MessageRole::User => "User",
                        MessageRole::Assistant => "Assistant",
                    };

                    format!("{}: {}", role, message.content)
                })
                .collect::<Vec<_>>()
                .join("\n\n");

            let prompt = guidance_prompt(&conversation);

            match prompt_ai(&prompt).await {
                Ok(result) => {
                    messages.write().push(Message {
                        role: MessageRole::Assistant,
                        content: result,
                    });

                    loading.set(false);
                }

                Err(err) => {
                    loading.set(false);
                    error.set(Some(err.to_string()));
                }
            }
        });
    };

    rsx! {
        div { class: "guidance-page",

            div { class: "guidance-container",

                div { class: "guidance-header",

                    h1 { class: "guidance-title", "Guidance 💬" }

                    p { class: "guidance-subtitle",
                        "Ask questions and have a conversation about medicines and health."
                    }
                }

                div { class: "guidance-card",

                    // Chat messages
                    div { class: "guidance-messages",

                        if messages().is_empty() {
                            div { class: "guidance-empty",

                                p { "Ask me anything about medicines or health." }
                            }
                        }

                        for message in messages() {
                            div {
                                class: match message.role {
                                    MessageRole::User => "message message-user",
                                    MessageRole::Assistant => "message message-assistant",
                                },

                                div { class: "message-content",

                                    match message.role {
                                        MessageRole::User => rsx! { "{message.content}" },
                                        MessageRole::Assistant => rsx! {
                                            MarkdownMessage { content: message.content.clone() }
                                        },
                                    }
                                }
                            }
                        }

                        if loading() {
                            div { class: "message message-assistant",

                                div { class: "message-content", "Thinking..." }
                            }
                        }
                    }

                    if let Some(message) = error() {
                        div { class: "guidance-error", "{message}" }
                    }

                    // Input
                    div { class: "guidance-input-row",

                        input {
                            class: "guidance-input",
                            r#type: "text",
                            placeholder: "Ask something...",

                            value: "{input}",

                            oninput: move |event| {
                                input.set(event.value());
                            },

                            onkeydown: move |event| {
                                if event.key() == Key::Enter {
                                    send_message();
                                }
                            },
                        }

                        button {
                            class: "guidance-button",
                            disabled: loading(),

                            onclick: move |_| {
                                send_message();
                            },

                            if loading() {
                                "..."
                            } else {
                                "Send"
                            }
                        }
                    }
                }
            }
        }
    }
}
