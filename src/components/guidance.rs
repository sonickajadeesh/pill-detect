use dioxus::prelude::*;

use crate::{
    components::navbar::Navbar,
    modules::{
        database::{Chat, Message, MessageRole, add_chat, delete_chat, get_chats, update_chat},
        prompts::guidance,
        utilities::markdown_to_html,
    },
};

#[component]
fn MarkdownMessage(content: String) -> Element {
    let html = markdown_to_html(&content);

    rsx! {
        div {
            class: "[&_p]:mb-3 [&_p:last-child]:mb-0 [&_strong]:font-bold [&_em]:italic [&_ul]:my-2.5 [&_ul]:pl-6 [&_ol]:my-2.5 [&_ol]:pl-6 [&_li]:my-1 [&_blockquote]:my-3 [&_blockquote]:border-l-[3px] [&_blockquote]:border-slate-300 [&_blockquote]:pl-3.5 [&_blockquote]:text-slate-500 [&_code]:rounded [&_code]:bg-slate-200 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:text-[0.9em] [&_pre]:my-3 [&_pre]:max-w-full [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:bg-slate-200 [&_pre]:p-3",
            dangerous_inner_html: "{html}",
        }
    }
}

#[component]
pub fn Guidance(patient_id: String) -> Element {
    let mut active_chat = use_signal(|| Option::<u64>::None);
    let mut chats = use_signal(Vec::<Chat>::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut input = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut sidebar_open = use_signal(|| false);

    // Load chats belonging to this patient.
    let load_patient_id = patient_id.clone();

    use_effect(move || match get_chats(&load_patient_id) {
        Ok(saved_chats) => {
            chats.set(saved_chats);
            active_chat.set(None);
        }

        Err(err) => {
            error.set(Some(err));
        }
    });

    let mut create_chat = move || {
        active_chat.set(None);
        input.set(String::new());
        error.set(None);
        loading.set(false);
    };

    // Patient ID used when creating/updating chats.
    let chat_patient_id = patient_id.clone();

    let mut send_message = move || {
        let prompt = input().trim().to_string();

        if prompt.is_empty() || loading() {
            return;
        }

        let chat_id = match active_chat() {
            Some(id) => id,

            None => {
                let id = js_sys::Date::now() as u64;

                let chat = Chat {
                    id,
                    title: prompt.chars().take(40).collect(),
                    messages: Vec::new(),
                };

                chats.write().push(chat.clone());

                if let Err(err) = add_chat(&chat_patient_id, chat) {
                    error.set(Some(err));
                    return;
                }

                active_chat.set(Some(id));
                id
            }
        };

        input.set(String::new());
        error.set(None);
        loading.set(true);

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

                if let Err(err) = update_chat(&chat_patient_id, chat.clone()) {
                    error.set(Some(err));
                    loading.set(false);
                    return;
                }
            }
        }

        let history = chats()
            .iter()
            .find(|chat| chat.id == chat_id)
            .map(|chat| chat.messages.clone())
            .unwrap_or_default();

        let ai_patient_id = chat_patient_id.clone();

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

            match guidance(&conversation).await {
                Ok(result) => {
                    let mut all_chats = chats.write();

                    if let Some(chat) = all_chats.iter_mut().find(|chat| chat.id == chat_id) {
                        chat.messages.push(Message {
                            role: MessageRole::Assistant,
                            content: result,
                        });

                        if let Err(err) = update_chat(&ai_patient_id, chat.clone()) {
                            error.set(Some(err));
                        }
                    }

                    loading.set(false);
                }

                Err(err) => {
                    loading.set(false);
                    error.set(Some(err.to_string()));
                }
            }
        });
    };

    let current_messages = chats()
        .iter()
        .find(|chat| Some(chat.id) == active_chat())
        .map(|chat| chat.messages.clone())
        .unwrap_or_default();

    rsx! {
        Navbar {
            patient_id: patient_id.clone(),
        }

        main { class: "flex h-[90vh] overflow-hidden bg-slate-50 p-6",

            // Sidebar
            aside { class: if sidebar_open() { "fixed inset-y-0 left-0 z-[100] flex w-[280px] flex-shrink-0 flex-col rounded-r-[14px] border border-slate-200 bg-white p-4 transition-transform duration-200 sm:relative sm:inset-auto sm:z-auto sm:w-60 sm:rounded-[14px] sm:p-3 sm:shadow-none" } else { "fixed inset-y-0 left-0 z-[100] flex w-[280px] flex-shrink-0 -translate-x-full flex-col rounded-r-[14px] border border-slate-200 bg-white p-4 transition-transform duration-200 sm:relative sm:inset-auto sm:z-auto sm:w-60 sm:translate-x-0 sm:rounded-[14px] sm:p-3" },

                button {
                    class: "w-full rounded-[10px] bg-blue-600 px-3.5 py-2.5 text-sm font-semibold text-white hover:bg-blue-700",

                    onclick: move |_| {
                        create_chat();
                        sidebar_open.set(false);
                    },

                    "+ New chat"
                }

                div { class: "mt-4 flex-1 overflow-y-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",

                    for chat in chats() {
                        div { class: if Some(chat.id) == active_chat() { "group mb-1 flex w-full items-center overflow-hidden rounded-lg bg-blue-50" } else { "group mb-1 flex w-full items-center overflow-hidden rounded-lg hover:bg-slate-100" },

                            button {
                                class: if Some(chat.id) == active_chat() { "min-w-0 flex-1 cursor-pointer overflow-hidden text-ellipsis whitespace-nowrap bg-transparent px-3 py-2.5 text-left text-sm font-semibold text-blue-700" } else { "min-w-0 flex-1 cursor-pointer overflow-hidden text-ellipsis whitespace-nowrap bg-transparent px-3 py-2.5 text-left text-sm text-slate-600" },

                                onclick: move |_| {
                                    active_chat.set(Some(chat.id));
                                    input.set(String::new());
                                    error.set(None);
                                    sidebar_open.set(false);
                                },

                                "{chat.title}"
                            }

                            button {
                                class: "mr-0.5 h-9 w-9 flex-shrink-0 cursor-pointer rounded-md bg-transparent text-slate-400 transition-opacity hover:text-red-600 sm:opacity-0 sm:group-hover:opacity-100",

                                onclick: {
                                    let patient_id = patient_id.clone();
                                    let chat_id = chat.id;

                                    move |_| {
                                        match delete_chat(&patient_id, chat_id) {
                                            Ok(()) => {
                                                chats
                                                    .write()
                                                    .retain(|chat| chat.id != chat_id);

                                                if active_chat() == Some(chat_id) {
                                                    active_chat.set(None);
                                                    input.set(String::new());
                                                }
                                            }

                                            Err(err) => {
                                                error.set(Some(err));
                                            }
                                        }
                                    }
                                },

                                "⨯"
                            }
                        }
                    }
                }
            }

            // Mobile backdrop
            if sidebar_open() {
                div {
                    class: "fixed inset-0 z-[99] bg-black/30 sm:hidden",

                    onclick: move |_| sidebar_open.set(false),
                }
            }

            // Main
            section { class: "flex min-w-0 flex-1 flex-col pl-0 sm:pl-6",

                button {
                    class: "mb-3 flex h-10 w-10 items-center justify-center rounded-[10px] border border-slate-200 bg-white text-xl text-slate-700 sm:hidden",

                    onclick: move |_| sidebar_open.set(true),

                    "☰"
                }

                header { class: "mb-6 flex-shrink-0",

                    h1 { class: "text-[32px] font-bold tracking-tight text-slate-900",
                        "Guidance 💬"
                    }

                    p { class: "mt-2 text-base text-slate-500",
                        "Ask questions and have a conversation about medicines and health."
                    }
                }

                section { class: "flex min-h-0 flex-1 flex-col overflow-hidden",

                    div { class: "min-h-0 flex-1 overflow-y-auto px-1 pt-2 pb-6 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",

                        if current_messages.is_empty() {
                            div { class: "flex h-full items-center justify-center text-center text-slate-400",

                                p { "Ask me anything about medicines or health." }
                            }
                        }

                        for message in current_messages {
                            div {
                                class: match message.role {
                                    MessageRole::User => "mb-[18px] flex justify-end",
                                    MessageRole::Assistant => "mb-[18px] flex justify-start",
                                },

                                div {
                                    class: match message.role {
                                        MessageRole::User => {
                                            "max-w-[75%] overflow-wrap-anywhere break-words rounded-[18px] rounded-br-[5px] bg-blue-600 px-4 py-3 text-[15px] leading-[1.6] text-white"
                                        }

                                        MessageRole::Assistant => {
                                            "max-w-[75%] overflow-wrap-anywhere break-words rounded-[18px] rounded-bl-[5px] bg-slate-200 px-4 py-3 text-[15px] leading-[1.6] text-slate-700"
                                        }
                                    },

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
                            div { class: "mb-[18px] flex justify-start",

                                div { class: "rounded-[18px] rounded-bl-[5px] bg-slate-200 px-4 py-3 text-[15px] leading-[1.6] text-slate-700",
                                    "Thinking..."
                                }
                            }
                        }
                    }

                    if let Some(message) = error() {
                        div { class: "mb-3 flex-shrink-0 rounded-[10px] border border-red-200 bg-red-50 px-3.5 py-3 text-sm text-red-700",
                            "{message}"
                        }
                    }

                    form {
                        class: "flex flex-shrink-0 gap-2.5 border-t border-slate-200 pt-3",

                        onsubmit: move |event| {
                            event.prevent_default();
                            send_message();
                        },

                        input {
                            class: "min-w-0 flex-1 rounded-xl border border-slate-300 bg-white px-4 py-3 text-[15px] text-slate-900 outline-none placeholder:text-slate-400 focus:border-blue-600 focus:ring-3 focus:ring-blue-600/10",

                            r#type: "text",
                            placeholder: "Ask something...",
                            value: "{input}",

                            oninput: move |event| {
                                input.set(event.value());
                            },
                        }

                        button {
                            class: "rounded-xl bg-blue-600 px-5 py-3 text-sm font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

                            r#type: "submit",
                            disabled: loading(),

                            if loading() {
                                "..."
                            } else {
                                "Send ⮞"
                            }
                        }
                    }
                }
            }
        }
    }
}
