use dioxus::prelude::*;

use crate::modules::{
    api::prompt_ai,
    chats::{Chat, Message, MessageRole, load_chats, markdown_to_html, save_chats},
    prompts::guidance_prompt,
};

#[component]
fn MarkdownMessage(content: String) -> Element {
    let html = markdown_to_html(&content);

    rsx! {
        div { class: "markdown-content", dangerous_inner_html: "{html}" }
    }
}

#[component]
pub fn Guidance() -> Element {
    let mut active_chat = use_signal(|| Option::<u64>::None);
    let mut chats = use_signal(Vec::<Chat>::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut input = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut sidebar_open = use_signal(|| false);

    // Load saved chats when the component starts.
    use_effect(move || {
        spawn(async move {
            match load_chats().await {
                Ok(saved_chats) => {
                    chats.set(saved_chats);
                    active_chat.set(None);
                }

                Err(err) => {
                    error.set(Some(err.to_string()));
                }
            }
        });
    });

    // Create a new conversation.
    let mut create_chat = move || {
        active_chat.set(None);
        input.set(String::new());
        error.set(None);
        loading.set(false);
    };

    // Send a message.
    let mut send_message = move || {
        let prompt = input().trim().to_string();

        if prompt.is_empty() || loading() {
            return;
        }

        // If there is no active chat, create one automatically.
        let chat_id = match active_chat() {
            Some(id) => id,

            None => {
                let id = js_sys::Date::now() as u64;

                chats.write().push(Chat {
                    id,
                    title: prompt.chars().take(40).collect(),
                    messages: Vec::new(),
                });

                active_chat.set(Some(id));

                id
            }
        };

        input.set(String::new());
        error.set(None);
        loading.set(true);

        // Add the user's message to the active chat.
        {
            let mut all_chats = chats.write();

            if let Some(chat) = all_chats.iter_mut().find(|chat| chat.id == chat_id) {
                if chat.messages.is_empty() {
                    chat.title = prompt.chars().take(40).collect();
                }

                chat.messages.push(Message {
                    role: MessageRole::User,
                    content: prompt.clone(),
                });
            }
        }

        // Get the current conversation history.
        let history = chats()
            .iter()
            .find(|chat| chat.id == chat_id)
            .map(|chat| chat.messages.clone())
            .unwrap_or_default();

        // Save the user's message immediately.
        let saved_chats = chats();

        spawn(async move {
            let _ = save_chats(&saved_chats).await;
        });

        // Ask Gemini.
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
                    let mut all_chats = chats.write();

                    if let Some(chat) = all_chats.iter_mut().find(|chat| chat.id == chat_id) {
                        chat.messages.push(Message {
                            role: MessageRole::Assistant,
                            content: result,
                        });
                    }

                    loading.set(false);

                    // Save the assistant response.
                    let saved_chats = all_chats.clone();

                    drop(all_chats);

                    spawn(async move {
                        let _ = save_chats(&saved_chats).await;
                    });
                }

                Err(err) => {
                    loading.set(false);
                    error.set(Some(err.to_string()));
                }
            }
        });
    };

    // Get messages for the currently selected chat.
    let current_messages = chats()
        .iter()
        .find(|chat| Some(chat.id) == active_chat())
        .map(|chat| chat.messages.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "guidance-page",

            // Sidebar
            aside {
                class: if sidebar_open() {
                    "guidance-sidebar sidebar-open"
                } else {
                    "guidance-sidebar"
                },

                button {
                    class: "new-chat-button",

                    onclick: move |_| {
                        create_chat();
                        sidebar_open.set(false);
                    },

                    "+ New chat"
                }

                div {
                    class: "chat-list",

                    for chat in chats() {
                        div {
                            class: if Some(chat.id) == active_chat() {
                                "chat-list-item active"
                            } else {
                                "chat-list-item"
                            },

                            button {
                                class: "chat-select-button",

                                onclick: move |_| {
                                    active_chat.set(Some(chat.id));
                                    input.set(String::new());
                                    error.set(None);
                                    sidebar_open.set(false);
                                },

                                "{chat.title}"
                            }

                            button {
                                class: "chat-delete-button",

                                onclick: move |_| {
                                    let chat_id = chat.id;

                                    chats.write().retain(|chat| chat.id != chat_id);

                                    if active_chat() == Some(chat_id) {
                                        active_chat.set(None);
                                        input.set(String::new());
                                    }

                                    let saved_chats = chats();

                                    spawn(async move {
                                        let _ = save_chats(&saved_chats).await;
                                    });
                                },

                                "⨯"
                            }
                        }
                    }
                }
            }

            if sidebar_open() {
                div {
                    class: "sidebar-backdrop",

                    onclick: move |_| {
                        sidebar_open.set(false);
                    },
                }
            }

            // Main chat
            div { class: "guidance-main",

                button {
                    class: "mobile-sidebar-button",

                    onclick: move |_| {
                        sidebar_open.set(true);
                    },

                    "☰"
                }

                div { class: "guidance-header",

                    h1 { class: "guidance-title", "Guidance 💬" }

                    p { class: "guidance-subtitle",
                        "Ask questions and have a conversation about medicines and health."
                    }
                }

                div { class: "guidance-card",

                    // Messages
                    div { class: "guidance-messages",

                        if current_messages.is_empty() {
                            div { class: "guidance-empty",

                                p { "Ask me anything about medicines or health." }
                            }
                        }

                        for message in current_messages {
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

                    // Error
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
