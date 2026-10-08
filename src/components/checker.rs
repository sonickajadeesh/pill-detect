use crate::{
    components::navbar::Navbar,
    modules::{
        bodymap::{BodyMap, BodyParts},
        prompts::symptom_guidance,
        utilities::markdown_to_html,
    },
};
use dioxus::prelude::*;

#[component]
fn MarkdownMessage(content: String) -> Element {
    let html = markdown_to_html(&content);

    rsx! {
        div {
            class: "[&_p]:mb-3 [&_p:last-child]:mb-0 [&_strong]:font-bold [&_em]:italic [&_ul]:my-2.5 [&_ul]:pl-6 [&_ul]:list-disc [&_ol]:my-2.5 [&_ol]:pl-6 [&_ol]:list-decimal [&_li]:my-1 [&_blockquote]:my-3 [&_blockquote]:border-l-[3px] [&_blockquote]:border-slate-300 [&_blockquote]:pl-3.5 [&_blockquote]:text-slate-500 [&_code]:rounded [&_code]:bg-slate-200 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:text-[0.9em] [&_pre]:my-3 [&_pre]:max-w-full [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:bg-slate-200 [&_pre]:p-3",
            dangerous_inner_html: html,
        }
    }
}

#[component]
pub fn SymptomChecker(patient_id: String) -> Element {
    let mut selected = use_signal(|| None::<BodyParts>);
    let mut selected_symptoms = use_signal(Vec::<String>::new);
    let mut additional_symptoms = use_signal(String::new);

    let mut organ_dropdown_open = use_signal(|| false);
    let mut symptom_dropdown_open = use_signal(|| false);

    let mut is_submitting = use_signal(|| false);
    let mut check_symptoms = use_signal(|| None::<String>);

    // Keep this list in the same order as your body map.
    let body_parts = vec![
        BodyParts::Scalp,
        BodyParts::Forehead,
        BodyParts::Eyes,
        BodyParts::Ears,
        BodyParts::Nose,
        BodyParts::Face,
        BodyParts::Mouth,
        BodyParts::Neck,
        BodyParts::Shoulders,
        BodyParts::UpperArms,
        BodyParts::Elbows,
        BodyParts::Forearms,
        BodyParts::Wrists,
        BodyParts::Hands,
        BodyParts::Fingers,
        BodyParts::UpperChest,
        BodyParts::Sternum,
        BodyParts::Breasts,
        BodyParts::Chest,
        BodyParts::Abdomen,
        BodyParts::Pelvis,
        BodyParts::Genitals,
        BodyParts::Thighs,
        BodyParts::Knees,
        BodyParts::LowerLegs,
        BodyParts::Ankles,
        BodyParts::Feet,
        BodyParts::Toes,
    ];

    let mut select_part = move |part: BodyParts| {
        selected.set(Some(part));
        organ_dropdown_open.set(false);
        symptom_dropdown_open.set(false);
    };

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main { class: "relative flex min-h-[90vh] flex-col items-center px-6 py-6",

            h1 { class: "text-[32px] font-bold tracking-tight text-slate-900", "Symptom Guidance 🤒" }

            p { class: "mb-12 text-base text-slate-500",
                "What symptoms are you experiencing? Select an area on the body to get started."
            }

            // Main content
            div { class: "w-full max-w-5xl",

                // Model + controls
                div { class: "flex w-full flex-col items-center gap-10 lg:flex-row lg:items-center lg:justify-center",

                    // Body map
                    div { class: "shrink-0",

                        BodyMap {
                            selected: selected(),

                            on_select: move |part| {
                                select_part(part);
                            },
                        }
                    }

                    // Right-side controls
                    div { class: "w-full max-w-lg",

                        // ORGAN SELECTOR
                        div { class: "mb-6",

                            p { class: "mb-2 block text-sm font-medium text-gray-700",
                                "Select symptoms for your:"
                            }

                            div { class: "relative",

                                button {
                                    r#type: "button",

                                    class: "flex min-h-[52px] w-full items-center justify-between rounded-2xl border border-gray-200 bg-white px-4 text-left shadow-sm transition hover:border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-100",

                                    onclick: move |_| {
                                        organ_dropdown_open.set(!organ_dropdown_open());
                                        symptom_dropdown_open.set(false);
                                    },

                                    span { class: if selected().is_some() { "text-gray-900" } else { "text-sm text-gray-400" },

                                        if let Some(part) = selected() {
                                            "{part.label()}"
                                        } else {
                                            "Select a body area"
                                        }
                                    }

                                    svg {
                                        class: "h-5 w-5 shrink-0 text-gray-400 transition-transform",

                                        class: if organ_dropdown_open() { "rotate-180" } else { "" },

                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",

                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M19 9l-7 7-7-7",
                                        }
                                    }
                                }

                                // Organ dropdown
                                if organ_dropdown_open() {
                                    div { class: "absolute z-30 mt-2 w-full overflow-hidden rounded-2xl border border-gray-200 bg-white p-2 shadow-lg",

                                        div { class: "max-h-64 overflow-y-auto",

                                            for part in body_parts.iter() {
                                                {
                                                    let part = *part;
                                                    let is_selected = selected() == Some(part);

                                                    rsx! {
                                                        button {
                                                            r#type: "button",

                                                            class: if is_selected { "flex w-full items-center justify-between rounded-xl bg-blue-50 px-4 py-3 text-left text-sm font-medium text-blue-700" } else { "flex w-full items-center justify-between rounded-xl px-4 py-3 text-left text-sm text-gray-700 hover:bg-gray-50" },

                                                            onclick: move |_| {
                                                                select_part(part);
                                                            },

                                                            span { "{part.label()}" }

                                                            if is_selected {
                                                                svg {
                                                                    class: "h-5 w-5 text-blue-600",
                                                                    fill: "none",
                                                                    view_box: "0 0 24 24",
                                                                    stroke: "currentColor",

                                                                    path {
                                                                        stroke_linecap: "round",
                                                                        stroke_linejoin: "round",
                                                                        stroke_width: "2",
                                                                        d: "M5 13l4 4L19 7",
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // SYMPTOM SELECTOR
                        if let Some(part) = selected() {
                            div { class: "relative",

                                button {
                                    r#type: "button",

                                    class: "flex min-h-[56px] w-full items-center justify-between rounded-2xl border border-gray-200 bg-white px-4 text-left shadow-sm transition hover:border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-100",

                                    onclick: move |_| {
                                        symptom_dropdown_open.set(!symptom_dropdown_open());
                                        organ_dropdown_open.set(false);
                                    },

                                    div { class: "flex flex-1 flex-wrap items-center gap-2",

                                        if selected_symptoms().is_empty() {
                                            span { class: "text-sm text-gray-400", "Select symptoms" }
                                        } else {
                                            for symptom in selected_symptoms() {
                                                span { class: "rounded-full bg-blue-50 px-3 py-1.5 text-xs font-medium text-blue-700",
                                                    "{symptom}"
                                                }
                                            }
                                        }
                                    }

                                    svg {
                                        class: "ml-3 h-5 w-5 shrink-0 text-gray-400 transition-transform",

                                        class: if symptom_dropdown_open() { "rotate-180" } else { "" },

                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",

                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M19 9l-7 7-7-7",
                                        }
                                    }
                                }

                                // Symptom dropdown
                                if symptom_dropdown_open() {
                                    div { class: "absolute z-20 mt-2 w-full overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-lg",

                                        div { class: "max-h-64 overflow-y-auto p-2",

                                            for symptom in part.symptoms() {
                                                {
                                                    let symptom_str = symptom.to_string();

                                                    let is_selected = selected_symptoms()
                                                        .iter()
                                                        .any(|s| s == &symptom_str);

                                                    rsx! {
                                                        button {
                                                            r#type: "button",

                                                            class: if is_selected { "flex w-full items-center justify-between rounded-xl bg-blue-50 px-4 py-3 text-left text-sm font-medium text-blue-700" } else { "flex w-full items-center justify-between rounded-xl px-4 py-3 text-left text-sm text-gray-700 hover:bg-gray-50" },

                                                            onclick: move |_| {
                                                                selected_symptoms
                                                                    .with_mut(|symptoms| {
                                                                        if let Some(pos) = symptoms
                                                                            .iter()
                                                                            .position(|s| s == &symptom_str)
                                                                        {
                                                                            symptoms.remove(pos);
                                                                        } else {
                                                                            symptoms.push(symptom_str.clone());
                                                                        }
                                                                    });
                                                            },

                                                            span { "{symptom_str}" }

                                                            if is_selected {
                                                                svg {
                                                                    class: "h-5 w-5 text-blue-600",
                                                                    fill: "none",
                                                                    view_box: "0 0 24 24",
                                                                    stroke: "currentColor",

                                                                    path {
                                                                        stroke_linecap: "round",
                                                                        stroke_linejoin: "round",
                                                                        stroke_width: "2",
                                                                        d: "M5 13l4 4L19 7",
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Clear selection
                                        if !selected_symptoms().is_empty() {
                                            div { class: "border-t border-gray-100 px-4 py-3",

                                                button {
                                                    r#type: "button",
                                                    class: "text-xs font-medium text-gray-500 hover:text-gray-700",

                                                    onclick: move |_| {
                                                        selected_symptoms.set(Vec::new());
                                                    },

                                                    "Clear selection"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Summary
                            if !selected_symptoms().is_empty() {
                                div { class: "mt-4 flex items-center justify-between text-sm",

                                    span { class: "text-gray-500",

                                        if selected_symptoms().len() == 1 {
                                            "1 symptom selected"
                                        } else {
                                            "{selected_symptoms().len()} symptoms selected"
                                        }
                                    }

                                    button {
                                        r#type: "button",
                                        class: "font-medium text-blue-600 hover:text-blue-700",

                                        onclick: move |_| {
                                            selected_symptoms.set(Vec::new());
                                        },

                                        "Clear"
                                    }
                                }
                            }

                            // Initial state
                            if selected().is_none() {
                                div { class: "mt-4 rounded-2xl border border-gray-200 bg-white px-6 py-5 shadow-sm",

                                    p { class: "text-sm font-medium text-gray-700",
                                        "Select a body area"
                                    }

                                    p { class: "mt-1 text-sm text-gray-500",
                                        "Choose an area from the model or the dropdown above."
                                    }
                                }
                            }

                            // Additional symptoms
                            div { class: "mt-6",

                                label { class: "mb-2 block text-sm font-medium text-gray-700",
                                    "Anything else you'd like to describe?"
                                }

                                textarea {
                                    class: "min-h-[120px] w-full resize-none rounded-2xl border border-gray-200 bg-white px-4 py-3 text-sm text-gray-900 shadow-sm outline-none transition placeholder:text-gray-400 focus:border-blue-400 focus:ring-2 focus:ring-blue-100",

                                    placeholder: "Describe any other symptoms, how they feel, when they started, or anything else you think is important...",

                                    value: "{additional_symptoms()}",

                                    oninput: move |event| {
                                        additional_symptoms.set(event.value());
                                    },
                                }

                                button {
                                    r#type: "button",

                                    class: "mt-5 flex w-full items-center justify-center gap-2 rounded-2xl bg-blue-600 px-5 py-3.5 text-sm font-medium text-white shadow-sm transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 disabled:cursor-not-allowed disabled:opacity-50",

                                    disabled: is_submitting(),

                                    onclick: move |_| async move {
                                        is_submitting.set(true);
                                        check_symptoms.set(None);
                                        match symptom_guidance(&selected_symptoms(), &additional_symptoms()).await {
                                            Ok(response) => {
                                                check_symptoms.set(Some(response));
                                            }
                                            Err(error) => {
                                                check_symptoms.set(Some(format!("Unable to get guidance: {}", error)));
                                            }
                                        }
                                        is_submitting.set(false);
                                    },

                                    if is_submitting() {
                                        div { class: "h-4 w-4 animate-spin rounded-full border-2 border-white/40 border-t-white" }

                                        "Checking"
                                    } else {
                                        "Check for symptoms"
                                    }
                                }
                            }
                        }
                    }
                }

                // AI result
                if let Some(guidance) = check_symptoms() {
                    div { class: "mt-6 rounded-2xl border border-gray-200 bg-white p-5 shadow-sm",
                        div { class: "mb-3 flex items-center justify-between",

                            div { class: "text-lg font-semibold text-gray-900", "Symptom Guidance" }

                            span { class: "rounded-full bg-blue-50 px-2.5 py-1 text-xs font-medium text-blue-700",
                                "AI-assisted"
                            }
                        }
                        div { class: "text-sm leading-6 text-gray-600",
                            MarkdownMessage { content: guidance }
                        }
                    }
                }
            }
        }
    }
}
