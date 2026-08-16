use dioxus::prelude::*;

use crate::modules::prompts::{MedicineInformation, identify_medicine, research_medicine};

#[component]
pub fn Information() -> Element {
    let mut search_term = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let mut medicine = use_signal(|| Option::<(String, String)>::None);
    let mut information = use_signal(|| Option::<MedicineInformation>::None);

    rsx! {
        div { class: "information-page",

            div { class: "information-container",

                div { class: "information-header",

                    h1 { class: "information-title", "Medicine Information 💊" }

                    p { class: "information-subtitle",
                        "Search for a medicine to learn more about it."
                    }
                }

                // Search
                div { class: "search-card",

                    div { class: "search-row",

                        input {
                            class: "search-input",
                            r#type: "text",
                            placeholder: "Type a medicine name",
                            value: "{search_term}",

                            oninput: move |event| {
                                search_term.set(event.value());
                            },
                        }

                        button {
                            class: "search-button",
                            disabled: loading(),

                            onclick: move |_| {
                                let term = search_term().trim().to_string();

                                if term.is_empty() {
                                    return;
                                }

                                loading.set(true);
                                error.set(None);
                                medicine.set(None);
                                information.set(None);

                                spawn(async move {
                                    match identify_medicine(&term).await {
                                        Ok(result) => {
                                            loading.set(false);

                                            if result.found {
                                                medicine.set(Some((result.product, result.generic)));
                                            } else {
                                                error
                                                    .set(
                                                        Some(
                                                            "Couldn't confidently identify that medicine. Please check again."
                                                                .to_string(),
                                                        ),
                                                    );
                                            }
                                        }
                                        Err(err) => {
                                            loading.set(false);
                                            error.set(Some(err.to_string()));
                                        }
                                    }
                                });
                            },

                            if loading() {
                                "Searching..."
                            } else {
                                "🔍︎"
                            }
                        }
                    }
                }

                // Error
                if let Some(message) = error() {
                    div { class: "search-error", "{message}" }
                }

                // Medicine result
                if let Some((product, generic)) = medicine() {
                    div { class: "medicine-result",

                        if let Some(info) = information() {
                            // Full information after confirmation
                            h2 { class: "medicine-result-title", "{generic}" }

                            div { class: "medicine-details",

                                p { "Uses: {info.uses}" }

                                p { "Dosage: {info.dosage}" }

                                p { "Side effects: {info.side_effects}" }

                                p { "Warnings: {info.warnings}" }

                                p {
                                    "Prescription required: "

                                    if info.prescription {
                                        "Yes"
                                    } else {
                                        "No"
                                    }
                                }
                            }
                        } else {
                            // Confirmation before research
                            h2 { class: "medicine-result-title",
                                "Is this the medicine you're looking for?"
                            }

                            div { class: "medicine-details", "{product} -> {generic}" }

                            button {
                                class: "confirm-button",
                                disabled: loading(),

                                onclick: move |_| {

                                    let generic_name = generic.clone();

                                    loading.set(true);
                                    error.set(None);

                                    spawn(async move {
                                        match research_medicine(&generic_name).await {
                                            Ok(result) => {
                                                loading.set(false);
                                                information.set(Some(result));
                                            }

                                            Err(err) => {
                                                loading.set(false);
                                                error.set(Some(err.to_string()));
                                            }
                                        }
                                    });
                                },

                                if loading() {
                                    "Researching..."
                                } else {
                                    "Yes, continue"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
