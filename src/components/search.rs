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
        main { class: "min-h-[96vh] max-w-[900px] mx-auto bg-slate-50 px-6 py-12",

            h1 { class: "text-[32px] font-bold tracking-tight text-slate-900",
                "Medicine Information 💊"
            }

            p { class: "mt-2 mb-8 text-base text-slate-500",
                "Search for a medicine to learn more about it."
            }

            section { class: "rounded-[14px] border border-slate-200 bg-white p-5",

                form {
                    class: "flex gap-2.5",

                    onsubmit: move |event| {
                        event.prevent_default();

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

                    input {
                        class: "min-w-0 flex-1 rounded-[10px] border border-slate-300 bg-white px-3.5 py-3 text-[15px] text-slate-900 outline-none placeholder:text-slate-400 focus:border-blue-600 focus:ring-3 focus:ring-blue-600/10",

                        r#type: "text",
                        placeholder: "Type a medicine name",
                        value: "{search_term}",

                        oninput: move |event| {
                            search_term.set(event.value());
                        },
                    }

                    button {
                        class: "rounded-[10px] bg-blue-600 px-5 py-3 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

                        r#type: "submit",
                        disabled: loading(),

                        if loading() {
                            "Searching..."
                        } else {
                            "🔍︎"
                        }
                    }
                }
            }

            if let Some(message) = error() {
                p { class: "mt-4 rounded-[10px] border border-red-200 bg-red-50 px-4 py-3.5 text-sm text-red-700",
                    "{message}"
                }
            }

            if let Some((product, generic)) = medicine() {
                section { class: "mt-6 rounded-[14px] border border-slate-200 bg-white p-7 text-slate-700 leading-relaxed",

                    if let Some(info) = information() {
                        h2 { class: "m-0 text-[21px] font-semibold text-slate-900",
                            "{generic}"
                        }

                        p { class: "mt-3.5", "Uses: {info.uses}" }
                        p { class: "mt-3.5", "Dosage: {info.dosage}" }
                        p { class: "mt-3.5", "Side effects: {info.side_effects}" }
                        p { class: "mt-3.5", "Warnings: {info.warnings}" }

                        p { class: "mt-3.5",
                            "Prescription required: "

                            if info.prescription {
                                "Yes"
                            } else {
                                "No"
                            }
                        }
                    } else {
                        h2 { class: "m-0 text-[21px] font-semibold text-slate-900",
                            "Is this the medicine you're looking for?"
                        }

                        p { class: "mt-3.5", "{product} → {generic}" }

                        button {
                            class: "mt-6 rounded-[10px] bg-blue-600 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

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
